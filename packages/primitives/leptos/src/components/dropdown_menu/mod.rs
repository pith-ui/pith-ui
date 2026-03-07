use std::cell::Cell;
use std::rc::Rc;

pub use crate::menu::CheckedState;
use crate::menu::*;
use crate::support::compose_refs::use_composed_refs;
use crate::support::id::use_id;
use crate::support::primitive::{
    Primitive, compose_callbacks, data_attr, prop_or, prop_or_default, wrap_callback,
};
use crate::support::use_controllable_state::{UseControllableStateParams, use_controllable_state};
use leptos::{context::Provider, ev, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::JsCast;

/* -------------------------------------------------------------------------------------------------
 * Context
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy)]
struct DropdownMenuContextValue {
    trigger_id: ReadSignal<String>,
    trigger_ref: AnyNodeRef,
    content_id: ReadSignal<String>,
    open: Signal<bool>,
    on_open_change: Callback<bool>,
    on_open_toggle: Callback<()>,
    modal: Signal<bool>,
}

/* -------------------------------------------------------------------------------------------------
 * Sub-modules
 * -----------------------------------------------------------------------------------------------*/

mod dropdown_menu;
mod dropdown_menu_content;

pub use dropdown_menu::*;
pub use dropdown_menu_content::*;
