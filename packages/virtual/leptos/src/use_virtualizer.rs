use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use pith_virtual_core::{Rect, VirtualItem, Virtualizer, VirtualizerOptions};
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::{JsCast, closure::Closure};

use crate::handle::VirtualizerHandle;
use crate::observers;

type CleanupHandle = Arc<Mutex<Option<SendWrapper<Box<dyn FnOnce()>>>>>;

/// Options for [`use_virtualizer`].
///
/// Only `count`, `scroll_ref`, and `estimate_size` are required. All other
/// fields have sensible defaults via `Default`.
///
/// # Example
/// ```rust,ignore
/// let virtualizer = use_virtualizer(UseVirtualizerOptions {
///     count: Signal::from(10_000),
///     scroll_ref,
///     estimate_size: Rc::new(|_| 35.0),
///     ..Default::default()
/// });
/// ```
pub struct UseVirtualizerOptions {
    // -- Required --
    /// Total number of items.
    pub count: Signal<usize>,
    /// NodeRef for the scroll container element.
    pub scroll_ref: AnyNodeRef,
    /// Estimate the size (px) of the item at the given index.
    pub estimate_size: Rc<dyn Fn(usize) -> f64>,

    // -- Optional (all have defaults) --
    /// Whether to automatically measure item elements that have the
    /// `data-index` attribute inside the `container_ref` element.
    /// Default: `true`.
    pub measure: bool,
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

impl Default for UseVirtualizerOptions {
    fn default() -> Self {
        Self {
            count: Signal::from(0),
            scroll_ref: AnyNodeRef::new(),
            estimate_size: Rc::new(|_| 50.0),
            measure: true,
            overscan: None,
            horizontal: None,
            padding_start: None,
            padding_end: None,
            scroll_padding_start: None,
            scroll_padding_end: None,
            initial_offset: None,
            initial_rect: None,
            get_item_key: None,
            range_extractor: None,
            scroll_margin: None,
            gap: None,
            index_attribute: None,
            initial_measurements_cache: None,
            lanes: None,
            is_scrolling_reset_delay: None,
            enabled: None,
            is_rtl: None,
            use_scrollend_event: None,
            debug: None,
        }
    }
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
/// The returned [`VirtualizerHandle`] provides reactive `get_virtual_items()`
/// / `get_total_size()` methods that read directly from the core and subscribe
/// to a version counter for reactivity.
///
/// # Automatic measurement
///
/// When `options.measure` is `true` (the default), the virtualizer
/// automatically measures item elements after each render via
/// `requestAnimationFrame`. To use this, place `node_ref=virtualizer.container_ref()`
/// on the element that directly contains your virtual items, and set
/// `data-index=item.index` on each item element.
///
/// # Example
/// ```rust,ignore
/// let scroll_ref = AnyNodeRef::new();
/// let virtualizer = use_virtualizer(UseVirtualizerOptions {
///     count: Signal::from(10_000),
///     scroll_ref,
///     estimate_size: Rc::new(|_| 35.0),
///     ..Default::default()
/// });
///
/// view! {
///     <div node_ref=scroll_ref style="height: 400px; overflow: auto;">
///         <div style:height=move || format!("{}px", virtualizer.get_total_size())>
///             <div node_ref=virtualizer.container_ref()>
///                 {move || virtualizer.get_virtual_items().into_iter().map(|item| {
///                     view! { <div data-index=item.index>{"Row "}{item.index}</div> }
///                 }).collect_view()}
///             </div>
///         </div>
///     </div>
/// }
/// ```
pub fn use_virtualizer(options: UseVirtualizerOptions) -> VirtualizerHandle {
    let horizontal = options.horizontal.unwrap_or(false);
    let is_rtl = options.is_rtl.unwrap_or(false);
    let use_scrollend = options.use_scrollend_event.unwrap_or(false);
    let reset_delay = options.is_scrolling_reset_delay.unwrap_or(150);
    let should_measure = options.measure;
    let scroll_ref = options.scroll_ref;

    // Derive the scroll element signal from the NodeRef.
    let get_scroll_element: Signal<Option<web_sys::Element>> = Signal::derive(move || {
        scroll_ref
            .get()
            .and_then(|n| n.dyn_into::<web_sys::Element>().ok())
    });

    let core_options = options.build_core_options(options.count.get_untracked());
    let core = Arc::new(Mutex::new(SendWrapper::new(Virtualizer::new(core_options))));

    let container_ref = AnyNodeRef::new();

    let handle = VirtualizerHandle {
        core: core.clone(),
        scroll_element: Arc::new(Mutex::new(None)),
        item_observer: Arc::new(Mutex::new(None)),
        elements_cache: Arc::new(Mutex::new(SendWrapper::new(HashMap::new()))),
        horizontal,
        version: RwSignal::new(0u64),
        container_ref,
    };

    let rect_cleanup: CleanupHandle = Arc::new(Mutex::new(None));
    let offset_cleanup: CleanupHandle = Arc::new(Mutex::new(None));

    // Effect: watch for scroll element / count changes, set up observers.
    let handle_effect = handle.clone();
    let rect_cleanup_effect = rect_cleanup.clone();
    let offset_cleanup_effect = offset_cleanup.clone();

    Effect::new(move |_| {
        let scroll_element = get_scroll_element.get();
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

    // Automatic measurement Effect: after each render, scan the container
    // for elements with `data-index` and measure them via RAF.
    if should_measure {
        let handle_measure = handle.clone();
        Effect::new(move |_| {
            // Subscribe so this re-runs on any state change.
            handle_measure.track();

            let handle_raf = handle_measure.clone();
            let closure: Closure<dyn FnMut()> = Closure::once(move || {
                handle_raf.measure_container_children();
            });

            if let Some(window) = web_sys::window() {
                window
                    .request_animation_frame(closure.as_ref().unchecked_ref())
                    .ok();
            }
            closure.forget();
        });
    }

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
///
/// Same as [`UseVirtualizerOptions`] but without `scroll_ref` (defaults to
/// the browser window).
pub struct UseWindowVirtualizerOptions {
    pub count: Signal<usize>,
    pub estimate_size: Rc<dyn Fn(usize) -> f64>,
    pub measure: bool,
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

impl Default for UseWindowVirtualizerOptions {
    fn default() -> Self {
        Self {
            count: Signal::from(0),
            estimate_size: Rc::new(|_| 50.0),
            measure: true,
            overscan: None,
            horizontal: None,
            padding_start: None,
            padding_end: None,
            scroll_padding_start: None,
            scroll_padding_end: None,
            initial_offset: None,
            initial_rect: None,
            get_item_key: None,
            range_extractor: None,
            scroll_margin: None,
            gap: None,
            index_attribute: None,
            initial_measurements_cache: None,
            lanes: None,
            is_scrolling_reset_delay: None,
            enabled: None,
            is_rtl: None,
            use_scrollend_event: None,
            debug: None,
        }
    }
}

/// Create a virtualizer for window-based scrolling.
pub fn use_window_virtualizer(options: UseWindowVirtualizerOptions) -> VirtualizerHandle {
    // Window scroll element derived internally.
    let window_ref = AnyNodeRef::new();
    // TODO: proper window-based scrolling with observe_window_rect/offset.
    // For now, cast window to Element for the element-based path.

    use_virtualizer(UseVirtualizerOptions {
        count: options.count,
        scroll_ref: window_ref,
        estimate_size: options.estimate_size,
        measure: options.measure,
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
