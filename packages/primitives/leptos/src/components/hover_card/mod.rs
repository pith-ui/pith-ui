use crate::support::compose_refs::use_composed_refs;
use crate::support::dismissable_layer::DismissableLayer;
use crate::support::popper::{
    Align, Padding, Popper, PopperAnchor, PopperArrow, PopperContent, Side, Sticky,
    UpdatePositionStrategy, provide_popper_scope, use_popper_scope,
};
use crate::support::portal::{ScopedPortal, resolve_force_mount};
use crate::support::presence::Presence;
use crate::support::primitive::{Primitive, compose_callbacks, open_closed_state};
use crate::support::use_controllable_state::{UseControllableStateParams, use_controllable_state};
use leptos::{
    attribute_interceptor::AttributeInterceptor, context::Provider, ev, html, prelude::*,
};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::closure::Closure;

mod hover_card;
mod hover_card_content;

pub use hover_card::*;
pub use hover_card_content::*;

/* -------------------------------------------------------------------------------------------------
 * Shared types and helpers
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy)]
struct HoverCardContextValue {
    open: Signal<bool>,
    #[allow(dead_code)]
    on_open_change: Callback<bool>,
    on_open: Callback<()>,
    on_close: Callback<()>,
    on_dismiss: Callback<()>,
    has_selection_ref: RwSignal<bool>,
    is_pointer_down_on_content_ref: RwSignal<bool>,
}

#[derive(Clone, Copy)]
struct ContentCallbacks {
    on_escape_key_down: StoredValue<Option<Callback<web_sys::KeyboardEvent>>>,
    on_pointer_down_outside: StoredValue<Option<Callback<web_sys::CustomEvent>>>,
    on_focus_outside: StoredValue<Option<Callback<web_sys::CustomEvent>>>,
    on_interact_outside: StoredValue<Option<Callback<web_sys::CustomEvent>>>,
}

/// Returns a list of nodes that can be in the tab sequence.
/// See: https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker
fn get_tabbable_nodes(container: &web_sys::HtmlElement) -> Vec<web_sys::HtmlElement> {
    let mut nodes = Vec::new();

    let accept_node_closure: Closure<dyn Fn(web_sys::Node) -> u32> =
        Closure::new(move |node: web_sys::Node| -> u32 {
            if let Some(html_element) = node.dyn_ref::<web_sys::HtmlElement>()
                && html_element.tab_index() >= 0
            {
                // NodeFilter.FILTER_ACCEPT
                return 1;
            }
            // NodeFilter.FILTER_SKIP
            3
        });

    let node_filter = web_sys::NodeFilter::new();
    node_filter.set_accept_node(accept_node_closure.as_ref().unchecked_ref());

    let walker = document()
        // 0x01 is NodeFilter.SHOW_ELEMENT
        .create_tree_walker_with_what_to_show_and_filter(container, 0x1, Some(&node_filter))
        .expect("Tree walker should be created.");

    while let Some(node) = walker
        .next_node()
        .expect("Tree walker should return a next node.")
    {
        if let Ok(element) = node.dyn_into::<web_sys::HtmlElement>() {
            nodes.push(element);
        }
    }

    nodes
}

fn document() -> web_sys::Document {
    web_sys::window()
        .expect("Window should exist.")
        .document()
        .expect("Document should exist.")
}

fn set_timeout(f: impl Fn() + 'static, delay: i32) -> i32 {
    let closure = Closure::once_into_js(f);
    web_sys::window()
        .expect("Window should exist.")
        .set_timeout_with_callback_and_timeout_and_arguments_0(closure.unchecked_ref(), delay)
        .expect("setTimeout should succeed.")
}

fn clear_timeout(handle: StoredValue<Option<i32>>) {
    if let Some(id) = handle.get_value() {
        web_sys::window()
            .expect("Window should exist.")
            .clear_timeout_with_handle(id);
        handle.set_value(None);
    }
}
