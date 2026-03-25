use pith_virtual_core::Rect;
use send_wrapper::SendWrapper;
use web_sys::{
    AddEventListenerOptions, Element, HtmlElement, ResizeObserver, ResizeObserverBoxOptions,
    ResizeObserverEntry, ResizeObserverOptions, ResizeObserverSize, Window,
    wasm_bindgen::{JsCast, closure::Closure},
};

/// Observe the scroll container's rect (width/height) via `ResizeObserver`.
///
/// Calls `cb` immediately with the current dimensions, then on every change.
/// Returns a cleanup function that disconnects the observer.
pub fn observe_element_rect(element: HtmlElement, cb: impl Fn(Rect) + 'static) -> impl FnOnce() {
    // Provide dimensions immediately.
    let width = element.offset_width() as f64;
    let height = element.offset_height() as f64;
    cb(Rect {
        width: width.round(),
        height: height.round(),
    });

    let resize_closure: Closure<dyn Fn(Vec<ResizeObserverEntry>)> =
        Closure::new(move |entries: Vec<ResizeObserverEntry>| {
            if let Some(entry) = entries.first() {
                let border_size_entry = entry.border_box_size().at(0);
                if let Some(size) = border_size_entry.dyn_ref::<ResizeObserverSize>() {
                    cb(Rect {
                        width: size.inline_size().round(),
                        height: size.block_size().round(),
                    });
                }
            }
        });

    let observer = ResizeObserver::new(resize_closure.into_js_value().unchecked_ref())
        .expect("ResizeObserver should be created");

    let options = ResizeObserverOptions::new();
    options.set_box(ResizeObserverBoxOptions::BorderBox);
    observer.observe_with_options(element.as_ref(), &options);

    let observer = SendWrapper::new(observer);
    move || {
        observer.disconnect();
    }
}

/// Observe the scroll container's scroll offset via `scroll` (and optionally
/// `scrollend`) event listeners.
///
/// Calls `cb(offset, is_scrolling)` on every scroll event.
/// Returns a cleanup function that removes the listeners.
pub fn observe_element_offset(
    element: Element,
    horizontal: bool,
    is_rtl: bool,
    use_scrollend_event: bool,
    is_scrolling_reset_delay: u32,
    cb: impl Fn(f64, bool) + 'static,
) -> impl FnOnce() {
    let cb = std::rc::Rc::new(cb);
    let element_clone = element;
    let target_window = web_sys::window();

    // Debounce timer for isScrolling reset (fallback when scrollend not used).
    let timeout_id = std::rc::Rc::new(std::cell::Cell::new(None::<i32>));

    // Check if scrollend is supported.
    let supports_scrollend = target_window
        .as_ref()
        .map(|w| js_sys::Reflect::has(w, &"onscrollend".into()).unwrap_or(false))
        .unwrap_or(false);
    let use_scrollend = use_scrollend_event && supports_scrollend;

    // Scroll handler.
    let cb_scroll = cb.clone();
    let el_scroll = element_clone.clone();
    let timeout_id_scroll = timeout_id.clone();
    let cb_debounce = cb.clone();
    let window_debounce = target_window.clone();

    let scroll_handler: Closure<dyn Fn()> = Closure::new(move || {
        let offset = if horizontal {
            let raw = el_scroll.scroll_left() as f64;
            if is_rtl { -raw } else { raw }
        } else {
            el_scroll.scroll_top() as f64
        };

        cb_scroll(offset, true);

        // Debounced isScrolling reset (unless using scrollend).
        if !use_scrollend {
            if let Some(id) = timeout_id_scroll.get()
                && let Some(w) = &window_debounce
            {
                w.clear_timeout_with_handle(id);
            }

            let cb_reset = cb_debounce.clone();
            let el_reset = el_scroll.clone();
            let h = horizontal;
            let rtl = is_rtl;
            let timeout_id_inner = timeout_id_scroll.clone();

            if let Some(w) = &window_debounce {
                let id = w
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        Closure::once(move || {
                            let offset = if h {
                                let raw = el_reset.scroll_left() as f64;
                                if rtl { -raw } else { raw }
                            } else {
                                el_reset.scroll_top() as f64
                            };
                            cb_reset(offset, false);
                            timeout_id_inner.set(None);
                        })
                        .into_js_value()
                        .unchecked_ref(),
                        is_scrolling_reset_delay as i32,
                    )
                    .ok();
                timeout_id_scroll.set(id);
            }
        }
    });

    // Scrollend handler (if supported).
    let cb_end = cb.clone();
    let el_end = element_clone.clone();

    let scrollend_handler: Option<Closure<dyn Fn()>> = if use_scrollend {
        Some(Closure::new(move || {
            let offset = if horizontal {
                let raw = el_end.scroll_left() as f64;
                if is_rtl { -raw } else { raw }
            } else {
                el_end.scroll_top() as f64
            };
            cb_end(offset, false);
        }))
    } else {
        None
    };

    // Register listeners.
    let passive_opts = AddEventListenerOptions::new();
    passive_opts.set_passive(true);

    element_clone
        .add_event_listener_with_callback_and_add_event_listener_options(
            "scroll",
            scroll_handler.as_ref().unchecked_ref(),
            &passive_opts,
        )
        .ok();

    if let Some(ref handler) = scrollend_handler {
        element_clone
            .add_event_listener_with_callback_and_add_event_listener_options(
                "scrollend",
                handler.as_ref().unchecked_ref(),
                &passive_opts,
            )
            .ok();
    }

    // Wrap closures to prevent them from being dropped.
    let scroll_handler = SendWrapper::new(scroll_handler);
    let scrollend_handler = scrollend_handler.map(SendWrapper::new);
    let element = SendWrapper::new(element_clone);
    let window_cleanup = target_window.map(SendWrapper::new);
    let timeout_id_cleanup = timeout_id;

    move || {
        element
            .remove_event_listener_with_callback("scroll", scroll_handler.as_ref().unchecked_ref())
            .ok();

        if let Some(ref handler) = scrollend_handler {
            element
                .remove_event_listener_with_callback("scrollend", handler.as_ref().unchecked_ref())
                .ok();
        }

        // Clear any pending timeout.
        if let Some(id) = timeout_id_cleanup.get()
            && let Some(w) = &window_cleanup
        {
            w.clear_timeout_with_handle(id);
        }
    }
}

/// Observe the window's size via the `resize` event.
#[allow(dead_code)]
pub fn observe_window_rect(window: &Window, cb: impl Fn(Rect) + 'static) -> impl FnOnce() {
    // Provide dimensions immediately.
    cb(Rect {
        width: window
            .inner_width()
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0),
        height: window
            .inner_height()
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0),
    });

    let resize_handler: Closure<dyn Fn()> = {
        let window = window.clone();
        Closure::new(move || {
            cb(Rect {
                width: window
                    .inner_width()
                    .ok()
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0),
                height: window
                    .inner_height()
                    .ok()
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0),
            });
        })
    };

    let passive_opts = AddEventListenerOptions::new();
    passive_opts.set_passive(true);

    window
        .add_event_listener_with_callback_and_add_event_listener_options(
            "resize",
            resize_handler.as_ref().unchecked_ref(),
            &passive_opts,
        )
        .ok();

    let resize_handler = SendWrapper::new(resize_handler);
    let window = SendWrapper::new(window.clone());

    move || {
        window
            .remove_event_listener_with_callback("resize", resize_handler.as_ref().unchecked_ref())
            .ok();
    }
}

/// Observe the window's scroll offset via `scroll` (and optionally
/// `scrollend`) event.
#[allow(dead_code)]
pub fn observe_window_offset(
    window: &Window,
    horizontal: bool,
    use_scrollend_event: bool,
    is_scrolling_reset_delay: u32,
    cb: impl Fn(f64, bool) + 'static,
) -> impl FnOnce() {
    let cb = std::rc::Rc::new(cb);
    let window_clone = window.clone();

    let timeout_id = std::rc::Rc::new(std::cell::Cell::new(None::<i32>));

    let supports_scrollend = js_sys::Reflect::has(window, &"onscrollend".into()).unwrap_or(false);
    let use_scrollend = use_scrollend_event && supports_scrollend;

    // Scroll handler.
    let cb_scroll = cb.clone();
    let w_scroll = window_clone.clone();
    let timeout_id_scroll = timeout_id.clone();
    let cb_debounce = cb.clone();
    let w_debounce = window_clone.clone();

    let scroll_handler: Closure<dyn Fn()> = Closure::new(move || {
        let offset = if horizontal {
            w_scroll.scroll_x().unwrap_or(0.0)
        } else {
            w_scroll.scroll_y().unwrap_or(0.0)
        };
        cb_scroll(offset, true);

        if !use_scrollend {
            if let Some(id) = timeout_id_scroll.get() {
                w_debounce.clear_timeout_with_handle(id);
            }

            let cb_reset = cb_debounce.clone();
            let w_reset = w_debounce.clone();
            let h = horizontal;
            let tid = timeout_id_scroll.clone();

            let id = w_debounce
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    Closure::once(move || {
                        let offset = if h {
                            w_reset.scroll_x().unwrap_or(0.0)
                        } else {
                            w_reset.scroll_y().unwrap_or(0.0)
                        };
                        cb_reset(offset, false);
                        tid.set(None);
                    })
                    .into_js_value()
                    .unchecked_ref(),
                    is_scrolling_reset_delay as i32,
                )
                .ok();
            timeout_id_scroll.set(id);
        }
    });

    // Scrollend handler.
    let cb_end = cb.clone();
    let w_end = window_clone.clone();

    let scrollend_handler: Option<Closure<dyn Fn()>> = if use_scrollend {
        Some(Closure::new(move || {
            let offset = if horizontal {
                w_end.scroll_x().unwrap_or(0.0)
            } else {
                w_end.scroll_y().unwrap_or(0.0)
            };
            cb_end(offset, false);
        }))
    } else {
        None
    };

    let passive_opts = AddEventListenerOptions::new();
    passive_opts.set_passive(true);

    window
        .add_event_listener_with_callback_and_add_event_listener_options(
            "scroll",
            scroll_handler.as_ref().unchecked_ref(),
            &passive_opts,
        )
        .ok();

    if let Some(ref handler) = scrollend_handler {
        window
            .add_event_listener_with_callback_and_add_event_listener_options(
                "scrollend",
                handler.as_ref().unchecked_ref(),
                &passive_opts,
            )
            .ok();
    }

    let scroll_handler = SendWrapper::new(scroll_handler);
    let scrollend_handler = scrollend_handler.map(SendWrapper::new);
    let window = SendWrapper::new(window.clone());
    let timeout_id_cleanup = timeout_id;
    let window_cleanup = SendWrapper::new(window_clone);

    move || {
        window
            .remove_event_listener_with_callback("scroll", scroll_handler.as_ref().unchecked_ref())
            .ok();

        if let Some(ref handler) = scrollend_handler {
            window
                .remove_event_listener_with_callback("scrollend", handler.as_ref().unchecked_ref())
                .ok();
        }

        if let Some(id) = timeout_id_cleanup.get() {
            window_cleanup.clear_timeout_with_handle(id);
        }
    }
}
