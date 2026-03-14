use crate::support::aria_hidden::{hide_others, unhide_others};
use crate::support::compose_refs::use_composed_refs;
use crate::support::dismissable_layer::DismissableLayer;
use crate::support::focus_guards::use_focus_guards;
use crate::support::focus_scope::FocusScope;
use crate::support::id::use_id;
use crate::support::portal::{ScopedPortal, resolve_force_mount};
use crate::support::presence::Presence;
use crate::support::primitive::{
    Primitive, adapt_callback, compose_callbacks, open_closed_state, prop_or, prop_or_default,
};
use crate::support::scroll_lock::use_body_scroll_lock;
use crate::support::use_controllable_state::{UseControllableStateParams, use_controllable_state};
use leptos::{
    attribute_interceptor::AttributeInterceptor, context::Provider, ev, html, prelude::*,
};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::{JsCast, closure::Closure};

mod dialog;
mod dialog_content;
mod dialog_parts;

pub use dialog::*;
pub use dialog_content::*;
pub use dialog_parts::*;

/* -------------------------------------------------------------------------------------------------
 * DialogContextValue (shared context)
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone)]
struct DialogContextValue {
    trigger_ref: AnyNodeRef,
    content_ref: AnyNodeRef,
    content_id: ReadSignal<String>,
    title_id: ReadSignal<String>,
    description_id: ReadSignal<String>,
    open: Signal<bool>,
    on_open_change: Callback<bool>,
    on_open_toggle: Callback<()>,
    modal: Signal<bool>,
}
