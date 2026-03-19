use web_sys::wasm_bindgen::JsCast;
use web_sys::{Document, Element, Event, HtmlElement, Node, Window};

pub fn can_use_dom() -> bool {
    web_sys::window().and_then(|w| w.document()).is_some()
}

pub fn compose_event_handlers<E: Clone + Into<Event>>(
    original_event_handler: Option<fn(E)>,
    our_event_handler: Option<fn(E)>,
    check_for_default_prevented: Option<bool>,
) -> impl Fn(E) {
    let check_for_default_prevented = check_for_default_prevented.unwrap_or(true);

    move |event: E| {
        if let Some(original_event_handler) = original_event_handler {
            original_event_handler(event.clone());
        }

        if (!check_for_default_prevented || !event.clone().into().default_prevented())
            && let Some(our_event_handler) = our_event_handler
        {
            our_event_handler(event);
        }
    }
}

pub fn get_owner_window(element: Option<&Node>) -> Result<Window, &'static str> {
    if !can_use_dom() {
        return Err("Cannot access window outside of the DOM");
    }

    let window = element
        .and_then(|el| el.owner_document())
        .and_then(|doc| doc.default_view())
        .or_else(web_sys::window);

    window.ok_or("Cannot access window outside of the DOM")
}

pub fn get_owner_document(element: Option<&Node>) -> Result<Document, &'static str> {
    if !can_use_dom() {
        return Err("Cannot access document outside of the DOM");
    }

    let document = element
        .and_then(|el| el.owner_document())
        .or_else(|| web_sys::window().and_then(|w| w.document()));

    document.ok_or("Cannot access document outside of the DOM")
}

/// Lifted from https://github.com/ariakit/ariakit/blob/main/packages/ariakit-core/src/utils/dom.ts#L37
/// MIT License, Copyright (c) AriaKit.
pub fn get_active_element(node: Option<&Node>, active_descendant: bool) -> Option<HtmlElement> {
    let document = get_owner_document(node).ok()?;
    let active_element = document.active_element()?;

    if active_element.node_name().is_empty() {
        return None;
    }

    if is_frame(&active_element) {
        let frame: &web_sys::HtmlIFrameElement = active_element.unchecked_ref();
        let body = frame.content_document()?.body()?;
        return get_active_element(Some(body.as_ref()), active_descendant);
    }

    if active_descendant
        && let Some(id) = active_element.get_attribute("aria-activedescendant")
        && !id.is_empty()
    {
        let owner_doc = get_owner_document(Some(active_element.as_ref())).ok()?;
        if let Some(element) = owner_doc.get_element_by_id(&id) {
            return element.dyn_into::<HtmlElement>().ok();
        }
    }

    active_element.dyn_into::<HtmlElement>().ok()
}

pub fn is_frame(element: &Element) -> bool {
    element.tag_name() == "IFRAME"
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn can_use_dom_returns_true() {
        assert!(can_use_dom());
    }

    #[wasm_bindgen_test]
    fn is_frame_with_iframe() {
        let doc = web_sys::window().unwrap().document().unwrap();
        let iframe = doc.create_element("iframe").unwrap();
        assert!(is_frame(&iframe));
    }

    #[wasm_bindgen_test]
    fn is_frame_with_div() {
        let doc = web_sys::window().unwrap().document().unwrap();
        let div = doc.create_element("div").unwrap();
        assert!(!is_frame(&div));
    }

    #[wasm_bindgen_test]
    fn get_owner_document_with_element() {
        let doc = web_sys::window().unwrap().document().unwrap();
        let div = doc.create_element("div").unwrap();
        doc.body().unwrap().append_child(&div).unwrap();
        let result = get_owner_document(Some(div.unchecked_ref()));
        assert!(result.is_ok());
        div.remove();
    }

    #[wasm_bindgen_test]
    fn get_owner_document_with_none() {
        let result = get_owner_document(None);
        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    fn get_owner_window_with_element() {
        let doc = web_sys::window().unwrap().document().unwrap();
        let div = doc.create_element("div").unwrap();
        doc.body().unwrap().append_child(&div).unwrap();
        let result = get_owner_window(Some(div.unchecked_ref()));
        assert!(result.is_ok());
        div.remove();
    }

    #[wasm_bindgen_test]
    fn get_owner_window_with_none() {
        let result = get_owner_window(None);
        assert!(result.is_ok());
    }

    thread_local! {
        static ORIGINAL_CALLED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
        static OUR_CALLED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    }

    fn original_handler(_: Event) {
        ORIGINAL_CALLED.with(|c| c.set(true));
    }

    fn our_handler(_: Event) {
        OUR_CALLED.with(|c| c.set(true));
    }

    fn preventing_handler(e: Event) {
        e.prevent_default();
    }

    #[wasm_bindgen_test]
    fn compose_event_handlers_both_called() {
        ORIGINAL_CALLED.with(|c| c.set(false));
        OUR_CALLED.with(|c| c.set(false));

        let handler = compose_event_handlers(Some(original_handler), Some(our_handler), None);

        let event = Event::new("click").unwrap();
        handler(event);

        assert!(ORIGINAL_CALLED.with(|c| c.get()));
        assert!(OUR_CALLED.with(|c| c.get()));
    }

    #[wasm_bindgen_test]
    fn compose_event_handlers_default_prevented_skips_ours() {
        OUR_CALLED.with(|c| c.set(false));

        let handler = compose_event_handlers(Some(preventing_handler), Some(our_handler), None);

        let mut init = web_sys::EventInit::new();
        init.cancelable(true);
        let event = Event::new_with_event_init_dict("click", &init).unwrap();
        handler(event);

        assert!(!OUR_CALLED.with(|c| c.get()));
    }
}
