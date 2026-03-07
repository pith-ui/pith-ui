use crate::support::compose_refs::use_composed_refs;
use crate::support::id::use_id;
use crate::support::primitive::{Primitive, VoidPrimitive, compose_callbacks};
use crate::support::use_controllable_state::{UseControllableStateParams, use_controllable_state};
use leptos::{
    attribute_interceptor::AttributeInterceptor, context::Provider, ev, html, prelude::*,
};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::{JsCast, closure::Closure};

/* -------------------------------------------------------------------------------------------------
 * Types
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum AutoComplete {
    #[default]
    CurrentPassword,
    NewPassword,
}

impl AutoComplete {
    fn as_str(&self) -> &'static str {
        match self {
            AutoComplete::CurrentPassword => "current-password",
            AutoComplete::NewPassword => "new-password",
        }
    }
}

#[derive(Clone)]
struct InternalFocusState {
    click_triggered: bool,
    selection_start: Option<u32>,
    selection_end: Option<u32>,
}

const INITIAL_FOCUS_STATE: InternalFocusState = InternalFocusState {
    click_triggered: false,
    selection_start: None,
    selection_end: None,
};

/* -------------------------------------------------------------------------------------------------
 * Context
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone)]
struct PasswordToggleFieldContextValue {
    input_id: Signal<String>,
    input_ref: AnyNodeRef,
    visible: Signal<bool>,
    set_visible: Callback<Option<bool>>,
    sync_input_id: Callback<Option<String>>,
    focus_state: StoredValue<InternalFocusState>,
}

/* -------------------------------------------------------------------------------------------------
 * Components
 * -----------------------------------------------------------------------------------------------*/

mod password_toggle_field;
mod password_toggle_field_input;
mod password_toggle_field_slot;

pub use password_toggle_field::*;
pub use password_toggle_field_input::*;
pub use password_toggle_field_slot::*;
