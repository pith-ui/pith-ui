pub use crate::menu::CheckedState;
use crate::menu::*;
use crate::support::primitive::{
    Primitive, adapt_callback, compose_callbacks, data_attr, prop_or, prop_or_default,
    wrap_callback,
};
use crate::support::use_controllable_state::{UseControllableStateParams, use_controllable_state};
use leptos::{context::Provider, ev, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use wasm_bindgen::{JsCast, closure::Closure};

mod context_menu;
mod context_menu_content;

pub use context_menu::*;
pub use context_menu_content::*;

/* -------------------------------------------------------------------------------------------------
 * Shared types
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy)]
struct ContextMenuContextValue {
    open: Signal<bool>,
    on_open_change: Callback<bool>,
    modal: Signal<bool>,
}

#[derive(Clone, Copy, Default)]
struct Point {
    x: f64,
    y: f64,
}

/// A virtual element for floating-ui that returns a zero-size rect at a given point.
/// Stores raw coordinates so `PartialEq` detects changes when the point moves,
/// which triggers floating-ui to re-compute the position.
#[derive(Clone, Debug, PartialEq)]
struct PointVirtualElement {
    x: f64,
    y: f64,
}

impl PopperVirtualElement<web_sys::Element> for PointVirtualElement {
    fn get_bounding_client_rect(&self) -> ClientRectObject {
        ClientRectObject {
            x: self.x,
            y: self.y,
            width: 0.0,
            height: 0.0,
            top: self.y,
            right: self.x,
            bottom: self.y,
            left: self.x,
        }
    }

    fn get_client_rects(&self) -> Option<Vec<ClientRectObject>> {
        None
    }

    fn context_element(&self) -> Option<web_sys::Element> {
        None
    }
}
