use leptos::prelude::*;
use send_wrapper::SendWrapper;

/// Sets `aria-hidden="true"` on body's direct children that don't contain `content`,
/// storing the affected elements in the provided signal for later cleanup via [`unhide_others`].
pub fn hide_others(
    content: &web_sys::HtmlElement,
    hidden_elements: RwSignal<Vec<SendWrapper<web_sys::Element>>>,
) {
    let Some(body) = document().body() else {
        return;
    };
    let children = body.children();
    let mut hidden = Vec::new();

    for i in 0..children.length() {
        if let Some(child) = children.item(i) {
            // Skip elements that contain the content or are already hidden
            let contains_content = child.contains(Some(content));
            let already_hidden = child
                .get_attribute("aria-hidden")
                .is_some_and(|v| v == "true");
            let is_script = child.tag_name().eq_ignore_ascii_case("SCRIPT");

            if !contains_content && !already_hidden && !is_script {
                let _ = child.set_attribute("aria-hidden", "true");
                hidden.push(SendWrapper::new(child));
            }
        }
    }

    hidden_elements.set(hidden);
}

/// Removes `aria-hidden` from all elements previously hidden by [`hide_others`].
pub fn unhide_others(hidden_elements: RwSignal<Vec<SendWrapper<web_sys::Element>>>) {
    for element in hidden_elements.get_untracked() {
        let _ = element.remove_attribute("aria-hidden");
    }
    hidden_elements.set(Vec::new());
}
