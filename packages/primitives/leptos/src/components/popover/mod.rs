use crate::support::aria_hidden::{hide_others, unhide_others};
use crate::support::compose_refs::use_composed_refs;
use crate::support::dismissable_layer::DismissableLayer;
use crate::support::focus_guards::use_focus_guards;
use crate::support::focus_scope::FocusScope;
use crate::support::id::use_id;
use crate::support::popper::{
    Popper, PopperAnchor, PopperArrow, PopperContent, UpdatePositionStrategy, provide_popper_scope,
    use_popper_scope,
};
use crate::support::portal::{ScopedPortal, resolve_force_mount};
use crate::support::presence::Presence;
use crate::support::primitive::{
    Primitive, adapt_callback, compose_callbacks, open_closed_state, prop_or_default,
};
use crate::support::scroll_lock::use_body_scroll_lock;
use crate::support::use_controllable_state::{UseControllableStateParams, use_controllable_state};
use leptos::{
    attribute_interceptor::AttributeInterceptor, context::Provider, ev, html, prelude::*,
};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::closure::Closure;

mod popover;
mod popover_content;

pub use crate::support::popper::{Align, Padding, Side, Sticky};
pub use popover::*;
pub use popover_content::*;

/* -------------------------------------------------------------------------------------------------
 * Shared types
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy)]
struct PopoverContextValue {
    trigger_ref: AnyNodeRef,
    content_id: ReadSignal<String>,
    open: Signal<bool>,
    on_open_change: Callback<bool>,
    on_open_toggle: Callback<()>,
    has_custom_anchor: RwSignal<bool>,
    modal: Signal<bool>,
}

/// Shared callback props for popover content variants, stored to avoid ownership issues
/// when forwarding `Option<Callback<...>>` through multiple component layers.
#[derive(Clone, Copy)]
struct ContentCallbacks {
    on_open_auto_focus: StoredValue<Option<Callback<web_sys::Event>>>,
    on_close_auto_focus: StoredValue<Option<Callback<web_sys::Event>>>,
    on_escape_key_down: StoredValue<Option<Callback<web_sys::KeyboardEvent>>>,
    on_pointer_down_outside: StoredValue<Option<Callback<web_sys::CustomEvent>>>,
    on_focus_outside: StoredValue<Option<Callback<web_sys::CustomEvent>>>,
    on_interact_outside: StoredValue<Option<Callback<web_sys::CustomEvent>>>,
}
