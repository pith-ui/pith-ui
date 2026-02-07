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
