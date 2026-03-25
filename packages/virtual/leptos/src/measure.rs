use web_sys::{
    Element, HtmlElement, ResizeObserver, ResizeObserverBoxOptions, ResizeObserverEntry,
    ResizeObserverOptions, ResizeObserverSize, wasm_bindgen::JsCast,
};

/// Measure an element's size using a `ResizeObserverEntry` (if available) or
/// falling back to `offsetWidth`/`offsetHeight`.
pub fn measure_element(
    element: &HtmlElement,
    entry: Option<&ResizeObserverEntry>,
    horizontal: bool,
) -> f64 {
    if let Some(entry) = entry {
        let border_box = entry.border_box_size().at(0);
        if let Some(size) = border_box.dyn_ref::<ResizeObserverSize>() {
            let value = if horizontal {
                size.inline_size()
            } else {
                size.block_size()
            };
            return value.round();
        }
    }

    if horizontal {
        element.offset_width() as f64
    } else {
        element.offset_height() as f64
    }
}

/// Read the item index from a DOM element's `data-index` (or custom) attribute.
pub fn index_from_element(element: &Element, attribute: &str) -> Option<usize> {
    element
        .get_attribute(attribute)
        .and_then(|s| s.parse::<usize>().ok())
}

/// Create a shared `ResizeObserver` that calls `on_resize` for each observed
/// entry. The callback receives `(element, entry)`.
///
/// Returns `(observer, cleanup_fn)`.
pub fn create_item_observer(
    on_resize: impl Fn(&HtmlElement, &ResizeObserverEntry) + 'static,
) -> ResizeObserver {
    let closure: wasm_bindgen::closure::Closure<dyn Fn(Vec<ResizeObserverEntry>)> =
        wasm_bindgen::closure::Closure::new(move |entries: Vec<ResizeObserverEntry>| {
            for entry in &entries {
                let target = entry.target();
                if !target.is_connected() {
                    continue;
                }
                if let Ok(html_el) = target.dyn_into::<HtmlElement>() {
                    on_resize(&html_el, entry);
                }
            }
        });

    ResizeObserver::new(closure.into_js_value().unchecked_ref())
        .expect("ResizeObserver should be created")
}

/// Observe an element with border-box sizing.
pub fn observe_item(observer: &ResizeObserver, element: &Element) {
    let options = ResizeObserverOptions::new();
    options.set_box(ResizeObserverBoxOptions::BorderBox);
    observer.observe_with_options(element, &options);
}
