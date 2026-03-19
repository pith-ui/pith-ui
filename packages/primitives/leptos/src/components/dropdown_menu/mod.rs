//! Dropdown menu triggered by a button.
//!
//! A menu that appears when a trigger button is clicked. Supports nested
//! submenus, checkbox and radio items, keyboard navigation, and typeahead.
//! Built on top of the [`menu`](crate::menu) primitive.
//!
//! Implements the [WAI-ARIA Menu Button pattern](https://www.w3.org/WAI/ARIA/apd/patterns/menubutton/).
//!
//! # Anatomy
//!
//! ```text
//! <DropdownMenu>
//!     <DropdownMenuTrigger />
//!     <DropdownMenuPortal>
//!         <DropdownMenuContent>
//!             <DropdownMenuItem />
//!             <DropdownMenuCheckboxItem />
//!             <DropdownMenuRadioGroup>
//!                 <DropdownMenuRadioItem />
//!             </DropdownMenuRadioGroup>
//!             <DropdownMenuSub>
//!                 <DropdownMenuSubTrigger />
//!                 <DropdownMenuSubContent />
//!             </DropdownMenuSub>
//!             <DropdownMenuSeparator />
//!             <DropdownMenuLabel />
//!         </DropdownMenuContent>
//!     </DropdownMenuPortal>
//! </DropdownMenu>
//! ```
//!
//! # Features
//!
//! - Controlled and uncontrolled open state
//! - Nested submenus
//! - Checkbox and radio menu items
//! - Typeahead character search
//! - Keyboard navigation (arrow keys, Home, End)
//! - Focus management and dismiss handling
//!
//! # Keyboard Interactions
//!
//! | Key | Action |
//! |-----|--------|
//! | Space / Enter | Opens menu or activates focused item |
//! | ArrowDown | Opens menu or focuses next item |
//! | ArrowUp | Focuses previous item |
//! | ArrowRight | Opens submenu |
//! | ArrowLeft | Closes submenu |
//! | Escape | Closes menu |

use std::cell::Cell;
use std::rc::Rc;

pub use crate::menu::CheckedState;
use crate::menu::*;
use crate::support::compose_refs::use_composed_refs;
use crate::support::id::use_id;
use crate::support::primitive::{
    Primitive, adapt_callback, compose_callbacks, data_attr, prop_or, prop_or_default,
    wrap_callback,
};
use crate::support::use_controllable_state::{UseControllableStateParams, use_controllable_state};
use leptos::{context::Provider, ev, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::JsCast;

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
