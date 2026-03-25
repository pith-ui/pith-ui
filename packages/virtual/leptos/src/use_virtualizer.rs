use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use leptos::prelude::*;
use pith_virtual_core::{Rect, VirtualItem, Virtualizer, VirtualizerOptions};
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::JsCast;

use crate::handle::VirtualizerHandle;
use crate::observers;

type CleanupHandle = Arc<Mutex<Option<SendWrapper<Box<dyn FnOnce()>>>>>;

/// Options for [`use_virtualizer`].
pub struct UseVirtualizerOptions {
    pub count: Signal<usize>,
    pub get_scroll_element: Signal<Option<web_sys::Element>>,
    pub estimate_size: Rc<dyn Fn(usize) -> f64>,

    pub overscan: Option<usize>,
    pub horizontal: Option<bool>,
    pub padding_start: Option<f64>,
    pub padding_end: Option<f64>,
    pub scroll_padding_start: Option<f64>,
    pub scroll_padding_end: Option<f64>,
    pub initial_offset: Option<f64>,
    pub initial_rect: Option<Rect>,
    pub get_item_key: Option<Rc<dyn Fn(usize) -> usize>>,
    pub range_extractor: Option<Rc<dyn Fn(pith_virtual_core::Range) -> Vec<usize>>>,
    pub scroll_margin: Option<f64>,
    pub gap: Option<f64>,
    pub index_attribute: Option<String>,
    pub initial_measurements_cache: Option<Vec<VirtualItem>>,
    pub lanes: Option<usize>,
    pub is_scrolling_reset_delay: Option<u32>,
    pub enabled: Option<bool>,
    pub is_rtl: Option<bool>,
    pub use_scrollend_event: Option<bool>,
    pub debug: Option<bool>,
}

impl UseVirtualizerOptions {
    fn build_core_options(&self, count: usize) -> VirtualizerOptions {
        let estimate_size = self.estimate_size.clone();
        let mut opts = VirtualizerOptions {
            count,
            estimate_size: Box::new(move |i| estimate_size(i)),
            ..Default::default()
        };

        macro_rules! set_opt {
            ($field:ident) => {
                if let Some(v) = self.$field {
                    opts.$field = v;
                }
            };
            ($field:ident, clone) => {
                if let Some(ref v) = self.$field {
                    opts.$field = v.clone();
                }
            };
            ($field:ident, rc_fn $sig:ty) => {
                if let Some(ref v) = self.$field {
                    let v = v.clone();
                    opts.$field = Box::new(move |i: $sig| v(i));
                }
            };
        }

        set_opt!(overscan);
        set_opt!(horizontal);
        set_opt!(padding_start);
        set_opt!(padding_end);
        set_opt!(scroll_padding_start);
        set_opt!(scroll_padding_end);
        set_opt!(scroll_margin);
        set_opt!(gap);
        set_opt!(index_attribute, clone);
        set_opt!(initial_measurements_cache, clone);
        set_opt!(lanes);
        set_opt!(is_scrolling_reset_delay);
        set_opt!(enabled);
        set_opt!(is_rtl);
        set_opt!(debug);
        set_opt!(get_item_key, rc_fn usize);

        if let Some(ref v) = self.range_extractor {
            let v = v.clone();
            opts.range_extractor = Box::new(move |r| v(r));
        }

        opts
    }
}

/// Create a virtualizer for element-based scrolling.
///
/// Returns a [`VirtualizerHandle`] whose `get_virtual_items()` /
/// `get_total_size()` methods read directly from the core and subscribe
/// to a version counter for reactivity. This follows the same pattern as
/// TanStack Virtual's React adapter: the virtualizer is a mutable object
/// and the view reads from it imperatively.
pub fn use_virtualizer(options: UseVirtualizerOptions) -> VirtualizerHandle {
    let horizontal = options.horizontal.unwrap_or(false);
    let is_rtl = options.is_rtl.unwrap_or(false);
    let use_scrollend = options.use_scrollend_event.unwrap_or(false);
    let reset_delay = options.is_scrolling_reset_delay.unwrap_or(150);

    let core_options = options.build_core_options(options.count.get_untracked());
    let core = Arc::new(Mutex::new(SendWrapper::new(Virtualizer::new(core_options))));

    let handle = VirtualizerHandle {
        core: core.clone(),
        scroll_element: Arc::new(Mutex::new(None)),
        item_observer: Arc::new(Mutex::new(None)),
        elements_cache: Arc::new(Mutex::new(SendWrapper::new(HashMap::new()))),
        horizontal,
        version: RwSignal::new(0u64),
    };

    let rect_cleanup: CleanupHandle = Arc::new(Mutex::new(None));
    let offset_cleanup: CleanupHandle = Arc::new(Mutex::new(None));

    // Effect: watch for scroll element / count changes, set up observers.
    let handle_effect = handle.clone();
    let rect_cleanup_effect = rect_cleanup.clone();
    let offset_cleanup_effect = offset_cleanup.clone();

    Effect::new(move |_| {
        let scroll_element = options.get_scroll_element.get();
        let count = options.count.get();

        // Update core options with current reactive values.
        {
            let core_opts = options.build_core_options(count);
            handle_effect.core.lock().unwrap().set_options(core_opts);
        }

        // Clean up previous observers.
        if let Some(cleanup) = rect_cleanup_effect.lock().unwrap().take() {
            cleanup.take()();
        }
        if let Some(cleanup) = offset_cleanup_effect.lock().unwrap().take() {
            cleanup.take()();
        }

        let Some(element) = scroll_element else {
            *handle_effect.scroll_element.lock().unwrap() = None;
            handle_effect.notify();
            return;
        };

        *handle_effect.scroll_element.lock().unwrap() = Some(SendWrapper::new(element.clone()));

        // Re-observe any cached item elements.
        {
            let cache = handle_effect.elements_cache.lock().unwrap();
            if !cache.is_empty() {
                handle_effect.ensure_item_observer();
                if let Some(obs) = handle_effect.item_observer.lock().unwrap().as_ref() {
                    for el in cache.values() {
                        crate::measure::observe_item(obs, el.unchecked_ref());
                    }
                }
            }
        }

        // Set up rect observer.
        if let Ok(html_el) = element.clone().dyn_into::<web_sys::HtmlElement>() {
            let core_rect = handle_effect.core.clone();
            let handle_rect = handle_effect.clone();

            let cleanup = observers::observe_element_rect(html_el, move |rect| {
                core_rect.lock().unwrap().set_scroll_rect(rect);
                handle_rect.notify();
            });

            *rect_cleanup_effect.lock().unwrap() =
                Some(SendWrapper::new(Box::new(cleanup) as Box<dyn FnOnce()>));
        }

        // Set up offset observer.
        {
            let core_offset = handle_effect.core.clone();
            let handle_offset = handle_effect.clone();

            let cleanup = observers::observe_element_offset(
                element.clone(),
                horizontal,
                is_rtl,
                use_scrollend,
                reset_delay,
                move |offset, is_scrolling| {
                    let changed = core_offset
                        .lock()
                        .unwrap()
                        .set_scroll_offset(offset, is_scrolling);

                    if core_offset.lock().unwrap().has_pending_scroll() {
                        handle_offset.schedule_reconciliation();
                    }

                    if changed {
                        handle_offset.notify();
                    }
                },
            );

            *offset_cleanup_effect.lock().unwrap() =
                Some(SendWrapper::new(Box::new(cleanup) as Box<dyn FnOnce()>));
        }

        // Initial notification.
        handle_effect.notify();
    });

    // Cleanup on unmount.
    let handle_cleanup = handle.clone();
    let rect_cleanup_unmount = rect_cleanup;
    let offset_cleanup_unmount = offset_cleanup;

    on_cleanup(move || {
        if let Some(cleanup) = rect_cleanup_unmount.lock().unwrap().take() {
            cleanup.take()();
        }
        if let Some(cleanup) = offset_cleanup_unmount.lock().unwrap().take() {
            cleanup.take()();
        }
        handle_cleanup.cleanup();
    });

    handle
}

/// Options for [`use_window_virtualizer`].
pub struct UseWindowVirtualizerOptions {
    pub count: Signal<usize>,
    pub estimate_size: Rc<dyn Fn(usize) -> f64>,
    pub overscan: Option<usize>,
    pub horizontal: Option<bool>,
    pub padding_start: Option<f64>,
    pub padding_end: Option<f64>,
    pub scroll_padding_start: Option<f64>,
    pub scroll_padding_end: Option<f64>,
    pub initial_offset: Option<f64>,
    pub initial_rect: Option<Rect>,
    pub get_item_key: Option<Rc<dyn Fn(usize) -> usize>>,
    pub range_extractor: Option<Rc<dyn Fn(pith_virtual_core::Range) -> Vec<usize>>>,
    pub scroll_margin: Option<f64>,
    pub gap: Option<f64>,
    pub index_attribute: Option<String>,
    pub initial_measurements_cache: Option<Vec<VirtualItem>>,
    pub lanes: Option<usize>,
    pub is_scrolling_reset_delay: Option<u32>,
    pub enabled: Option<bool>,
    pub is_rtl: Option<bool>,
    pub use_scrollend_event: Option<bool>,
    pub debug: Option<bool>,
}

/// Create a virtualizer for window-based scrolling.
pub fn use_window_virtualizer(options: UseWindowVirtualizerOptions) -> VirtualizerHandle {
    let window_signal =
        Signal::derive(|| web_sys::window().map(|w| w.unchecked_into::<web_sys::Element>()));

    use_virtualizer(UseVirtualizerOptions {
        count: options.count,
        get_scroll_element: window_signal,
        estimate_size: options.estimate_size,
        overscan: options.overscan,
        horizontal: options.horizontal,
        padding_start: options.padding_start,
        padding_end: options.padding_end,
        scroll_padding_start: options.scroll_padding_start,
        scroll_padding_end: options.scroll_padding_end,
        initial_offset: options
            .initial_offset
            .or_else(|| web_sys::window().map(|w| w.scroll_y().unwrap_or(0.0))),
        initial_rect: options.initial_rect,
        get_item_key: options.get_item_key,
        range_extractor: options.range_extractor,
        scroll_margin: options.scroll_margin,
        gap: options.gap,
        index_attribute: options.index_attribute,
        initial_measurements_cache: options.initial_measurements_cache,
        lanes: options.lanes,
        is_scrolling_reset_delay: options.is_scrolling_reset_delay,
        enabled: options.enabled,
        is_rtl: options.is_rtl,
        use_scrollend_event: options.use_scrollend_event,
        debug: options.debug,
    })
}
