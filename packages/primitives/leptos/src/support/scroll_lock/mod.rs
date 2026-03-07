use leptos::prelude::*;

/// Sets `overflow: hidden` on `<body>` while the calling component is mounted,
/// compensating for scrollbar width to prevent layout shift, then restores the
/// original values on cleanup.
pub fn use_body_scroll_lock() {
    let original_overflow: RwSignal<Option<String>> = RwSignal::new(None);
    let original_padding_right: RwSignal<Option<String>> = RwSignal::new(None);

    Effect::new(move |_| {
        if let Some(body) = document().body() {
            let style = body.style();
            let prev_overflow = style.get_property_value("overflow").unwrap_or_default();
            let prev_padding_right = style
                .get_property_value("padding-right")
                .unwrap_or_default();
            original_overflow.set(Some(prev_overflow));

            // Measure scrollbar width: difference between window inner width and document client width.
            let scrollbar_width = window()
                .inner_width()
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0)
                - document()
                    .document_element()
                    .map(|el| el.client_width() as f64)
                    .unwrap_or(0.0);

            // Parse any existing padding-right value and add the scrollbar width.
            let existing_padding: f64 = prev_padding_right
                .trim_end_matches("px")
                .parse()
                .unwrap_or(0.0);

            original_padding_right.set(Some(prev_padding_right));

            let _ = style.set_property(
                "padding-right",
                &format!("{}px", existing_padding + scrollbar_width),
            );
            let _ = style.set_property("overflow", "hidden");
        }
    });

    on_cleanup(move || {
        if let Some(body) = document().body() {
            let style = body.style();

            if let Some(original) = original_overflow.get_untracked() {
                if original.is_empty() {
                    let _ = style.remove_property("overflow");
                } else {
                    let _ = style.set_property("overflow", &original);
                }
            }

            if let Some(original) = original_padding_right.get_untracked() {
                if original.is_empty() {
                    let _ = style.remove_property("padding-right");
                } else {
                    let _ = style.set_property("padding-right", &original);
                }
            }
        }
    });
}
