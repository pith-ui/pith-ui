//! Password field with visibility toggle.
//!
//! A password input that can switch between hidden and visible text.
//! Preserves cursor position and text selection when toggling visibility.
//!
//! # Anatomy
//!
//! ```text
//! <PasswordToggleField>
//!     <PasswordToggleFieldInput />
//!     <PasswordToggleFieldSlot />
//!         <PasswordToggleFieldToggle />
//!     </PasswordToggleFieldSlot>
//! </PasswordToggleField>
//! ```
//!
//! # Features
//!
//! - Toggle between `type="password"` and `type="text"`
//! - Controlled and uncontrolled visibility state
//! - Preserves cursor position and selection on toggle
//! - Configurable autocomplete (`current-password` or `new-password`)
//!
//! # Data Attributes
//!
//! **PasswordToggleFieldToggle:**
//!
//! | Attribute | Values |
//! |-----------|--------|
//! | `data-state` | `visible`, `hidden` |

use crate::support::compose_refs::use_composed_refs;
use crate::support::id::use_id;
use crate::support::primitive::{Primitive, VoidPrimitive, adapt_callback, compose_callbacks};
use crate::support::use_controllable_state::{UseControllableStateParams, use_controllable_state};
use leptos::{
    attribute_interceptor::AttributeInterceptor, context::Provider, ev, html, prelude::*,
};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::{JsCast, closure::Closure};

/// Autocomplete hint for the password field.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum AutoComplete {
    /// Hint for existing password fields (default).
    #[default]
    CurrentPassword,
    /// Hint for new/changed password fields.
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
