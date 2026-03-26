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
/// Create with [`UseVirtualizerOptions::new`] and customize with builder
/// methods. Only `count`, `scroll_ref`, and `estimate_size` are required.
///
/// # Example
/// ```rust,ignore
/// let virtualizer = use_virtualizer(
///     UseVirtualizerOptions::new(10_000, scroll_ref, |_| 35.0)
///         .overscan(5)
/// );
/// ```
pub struct UseVirtualizerOptions {
    count: Signal<usize>,
    scroll_ref: AnyNodeRef,
    estimate_size: Rc<dyn Fn(usize) -> f64>,
    measure: bool,
    overscan: Option<usize>,
    horizontal: Option<bool>,
    padding_start: Option<f64>,
    padding_end: Option<f64>,
    scroll_padding_start: Option<f64>,
    scroll_padding_end: Option<f64>,
    initial_offset: Option<f64>,
    initial_rect: Option<Rect>,
    get_item_key: Option<Rc<dyn Fn(usize) -> usize>>,
    range_extractor: Option<Rc<dyn Fn(pith_virtual_core::Range) -> Vec<usize>>>,
    scroll_margin: Option<f64>,
    gap: Option<f64>,
    index_attribute: Option<String>,
    initial_measurements_cache: Option<Vec<VirtualItem>>,
    lanes: Option<usize>,
    is_scrolling_reset_delay: Option<u32>,
    enabled: Option<bool>,
    is_rtl: Option<bool>,
    use_scrollend_event: Option<bool>,
    debug: Option<bool>,
}

impl UseVirtualizerOptions {
    /// Create options with the three required fields.
    ///
    /// - `count`: total number of items (accepts `usize`, `Signal<usize>`,
    ///   `ReadSignal<usize>`, etc.)
    /// - `scroll_ref`: `AnyNodeRef` placed on the scroll container element
    /// - `estimate_size`: function returning the estimated pixel size for
    ///   each item index
    pub fn new(
        count: impl Into<Signal<usize>>,
        scroll_ref: AnyNodeRef,
        estimate_size: impl Fn(usize) -> f64 + 'static,
    ) -> Self {
        Self {
            count: count.into(),
            scroll_ref,
            estimate_size: Rc::new(estimate_size),
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

    /// Number of items to render outside the visible range. Default: 1.
    pub fn overscan(mut self, v: usize) -> Self {
        self.overscan = Some(v);
        self
    }

    /// Enable horizontal scrolling. Default: false.
    pub fn horizontal(mut self, v: bool) -> Self {
        self.horizontal = Some(v);
        self
    }

    /// Padding before the first item in pixels.
    pub fn padding_start(mut self, v: f64) -> Self {
        self.padding_start = Some(v);
        self
    }

    /// Padding after the last item in pixels.
    pub fn padding_end(mut self, v: f64) -> Self {
        self.padding_end = Some(v);
        self
    }

    /// Scroll padding applied when aligning to start.
    pub fn scroll_padding_start(mut self, v: f64) -> Self {
        self.scroll_padding_start = Some(v);
        self
    }

    /// Scroll padding applied when aligning to end.
    pub fn scroll_padding_end(mut self, v: f64) -> Self {
        self.scroll_padding_end = Some(v);
        self
    }

    /// Initial scroll offset in pixels.
    pub fn initial_offset(mut self, v: f64) -> Self {
        self.initial_offset = Some(v);
        self
    }

    /// Initial container dimensions.
    pub fn initial_rect(mut self, v: Rect) -> Self {
        self.initial_rect = Some(v);
        self
    }

    /// Custom key extractor. Default: identity (index as key).
    pub fn get_item_key(mut self, f: impl Fn(usize) -> usize + 'static) -> Self {
        self.get_item_key = Some(Rc::new(f));
        self
    }

    /// Custom range extractor.
    pub fn range_extractor(
        mut self,
        f: impl Fn(pith_virtual_core::Range) -> Vec<usize> + 'static,
    ) -> Self {
        self.range_extractor = Some(Rc::new(f));
        self
    }

    /// Offset from the scroll container edge to the first item in pixels.
    pub fn scroll_margin(mut self, v: f64) -> Self {
        self.scroll_margin = Some(v);
        self
    }

    /// Gap between items within a lane in pixels.
    pub fn gap(mut self, v: f64) -> Self {
        self.gap = Some(v);
        self
    }

    /// DOM attribute used to read item index from elements. Default: "data-index".
    pub fn index_attribute(mut self, v: impl Into<String>) -> Self {
        self.index_attribute = Some(v.into());
        self
    }

    /// Pre-populated measurement cache.
    pub fn initial_measurements_cache(mut self, v: Vec<VirtualItem>) -> Self {
        self.initial_measurements_cache = Some(v);
        self
    }

    /// Number of columns/lanes for multi-column layouts. Default: 1.
    pub fn lanes(mut self, v: usize) -> Self {
        self.lanes = Some(v);
        self
    }

    /// Debounce delay in ms for detecting scroll-end. Default: 150.
    pub fn is_scrolling_reset_delay(mut self, v: u32) -> Self {
        self.is_scrolling_reset_delay = Some(v);
        self
    }

    /// Whether the virtualizer is active. Default: true.
    pub fn enabled(mut self, v: bool) -> Self {
        self.enabled = Some(v);
        self
    }

    /// Right-to-left layout. Default: false.
    pub fn is_rtl(mut self, v: bool) -> Self {
        self.is_rtl = Some(v);
        self
    }

    /// Use native `scrollend` event instead of debounced timeout.
    pub fn use_scrollend_event(mut self, v: bool) -> Self {
        self.use_scrollend_event = Some(v);
        self
    }

    /// Disable automatic measurement of container children.
    pub fn measure(mut self, v: bool) -> Self {
        self.measure = v;
        self
    }

    /// Enable debug logging.
    pub fn debug(mut self, v: bool) -> Self {
        self.debug = Some(v);
        self
    }

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
/// # Example
/// ```rust,ignore
/// let scroll_ref = AnyNodeRef::new();
/// let virtualizer = use_virtualizer(
///     UseVirtualizerOptions::new(10_000, scroll_ref, |_| 35.0)
///         .overscan(5)
/// );
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

    // Automatic measurement Effect.
    if should_measure {
        let handle_measure = handle.clone();
        Effect::new(move |_| {
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

/// Options for [`use_window_virtualizer`]. Same as [`UseVirtualizerOptions`]
/// but without `scroll_ref`.
pub struct UseWindowVirtualizerOptions {
    count: Signal<usize>,
    estimate_size: Rc<dyn Fn(usize) -> f64>,
    measure: bool,
    overscan: Option<usize>,
    horizontal: Option<bool>,
    initial_offset: Option<f64>,
    // ... remaining fields omitted for brevity; extend as needed
}

impl UseWindowVirtualizerOptions {
    /// Create window virtualizer options.
    pub fn new(
        count: impl Into<Signal<usize>>,
        estimate_size: impl Fn(usize) -> f64 + 'static,
    ) -> Self {
        Self {
            count: count.into(),
            estimate_size: Rc::new(estimate_size),
            measure: true,
            overscan: None,
            horizontal: None,
            initial_offset: None,
        }
    }

    /// Number of items to render outside the visible range.
    pub fn overscan(mut self, v: usize) -> Self {
        self.overscan = Some(v);
        self
    }
}

/// Create a virtualizer for window-based scrolling.
pub fn use_window_virtualizer(options: UseWindowVirtualizerOptions) -> VirtualizerHandle {
    let window_ref = AnyNodeRef::new();

    let mut opts = UseVirtualizerOptions::new(options.count, window_ref, {
        let f = options.estimate_size;
        move |i| f(i)
    });
    opts.measure = options.measure;
    opts.overscan = options.overscan;
    opts.horizontal = options.horizontal;
    opts.initial_offset = options
        .initial_offset
        .or_else(|| web_sys::window().map(|w| w.scroll_y().unwrap_or(0.0)));

    use_virtualizer(opts)
}
