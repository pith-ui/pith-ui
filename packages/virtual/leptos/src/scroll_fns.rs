use pith_virtual_core::ScrollBehavior;
use web_sys::{Element, ScrollToOptions, Window};

/// Convert our `ScrollBehavior` to the web-sys `ScrollBehavior`.
fn to_web_behavior(behavior: ScrollBehavior) -> web_sys::ScrollBehavior {
    match behavior {
        ScrollBehavior::Auto => web_sys::ScrollBehavior::Auto,
        ScrollBehavior::Smooth => web_sys::ScrollBehavior::Smooth,
        ScrollBehavior::Instant => web_sys::ScrollBehavior::Instant,
    }
}

/// Scroll an element to the given offset.
pub fn element_scroll(
    element: &Element,
    offset: f64,
    horizontal: bool,
    behavior: Option<ScrollBehavior>,
    adjustments: Option<f64>,
) {
    let to_offset = offset + adjustments.unwrap_or(0.0);
    let opts = ScrollToOptions::new();
    if horizontal {
        opts.set_left(to_offset);
    } else {
        opts.set_top(to_offset);
    }
    if let Some(behavior) = behavior {
        opts.set_behavior(to_web_behavior(behavior));
    }
    element.scroll_to_with_scroll_to_options(&opts);
}

/// Get the maximum scroll offset for an element.
pub fn element_max_scroll_offset(element: &Element, horizontal: bool) -> f64 {
    if horizontal {
        (element.scroll_width() - element.client_width()) as f64
    } else {
        (element.scroll_height() - element.client_height()) as f64
    }
}

/// Scroll the window to the given offset.
#[allow(dead_code)]
pub fn window_scroll(
    window: &Window,
    offset: f64,
    horizontal: bool,
    behavior: Option<ScrollBehavior>,
    adjustments: Option<f64>,
) {
    let to_offset = offset + adjustments.unwrap_or(0.0);
    let opts = ScrollToOptions::new();
    if horizontal {
        opts.set_left(to_offset);
    } else {
        opts.set_top(to_offset);
    }
    if let Some(behavior) = behavior {
        opts.set_behavior(to_web_behavior(behavior));
    }
    window.scroll_to_with_scroll_to_options(&opts);
}

/// Get the maximum scroll offset for the window.
#[allow(dead_code)]
pub fn window_max_scroll_offset(window: &Window, horizontal: bool) -> f64 {
    let doc = window
        .document()
        .and_then(|d| d.document_element())
        .expect("document element should exist");

    if horizontal {
        (doc.scroll_width() as f64)
            - window
                .inner_width()
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0)
    } else {
        (doc.scroll_height() as f64)
            - window
                .inner_height()
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0)
    }
}
