use leptos::prelude::*;

/// Sets `overflow: hidden` on `<body>` while the calling component is mounted,
/// then restores the original value on cleanup.
pub fn use_body_scroll_lock() {
    let original_overflow: RwSignal<Option<String>> = RwSignal::new(None);

    Effect::new(move |_| {
        if let Some(body) = document().body() {
            let style = body.style();
            let current = style.get_property_value("overflow").unwrap_or_default();
            original_overflow.set(Some(current));
            let _ = style.set_property("overflow", "hidden");
        }
    });

    on_cleanup(move || {
        if let Some(original) = original_overflow.get_untracked()
            && let Some(body) = document().body()
        {
            let _ = body.style().set_property("overflow", &original);
        }
    });
}
