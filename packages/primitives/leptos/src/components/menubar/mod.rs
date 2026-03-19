//! Horizontal menu bar with dropdown menus.
//!
//! A visually persistent menu bar where each top-level item opens a dropdown
//! menu. Supports roving focus across menu bar items, keyboard navigation
//! within menus, and all menu item types (regular, checkbox, radio, sub).
//!
//! Implements the [WAI-ARIA Menu Bar pattern](https://www.w3.org/WAI/ARIA/apd/patterns/menubar/).
//!
//! # Anatomy
//!
//! ```text
//! <Menubar>
//!     <MenubarMenu>
//!         <MenubarTrigger />
//!         <MenubarPortal>
//!             <MenubarContent>
//!                 <MenubarItem />
//!                 <MenubarCheckboxItem />
//!                 <MenubarRadioGroup>
//!                     <MenubarRadioItem />
//!                 </MenubarRadioGroup>
//!                 <MenubarSub>
//!                     <MenubarSubTrigger />
//!                     <MenubarSubContent />
//!                 </MenubarSub>
//!                 <MenubarSeparator />
//!             </MenubarContent>
//!         </MenubarPortal>
//!     </MenubarMenu>
//! </Menubar>
//! ```
//!
//! # Features
//!
//! - Roving focus across top-level triggers
//! - Arrow keys move between menus
//! - Full submenu support
//! - Checkbox and radio menu items
//! - Typeahead character search
//!
//! # Keyboard Interactions
//!
//! | Key | Action |
//! |-----|--------|
//! | ArrowRight | Opens next menu / enters submenu |
//! | ArrowLeft | Opens previous menu / exits submenu |
//! | ArrowDown | Opens menu or focuses next item |
//! | ArrowUp | Focuses previous item |
//! | Escape | Closes current menu |
//! | Enter / Space | Activates focused item |

use std::cell::Cell;
use std::marker::PhantomData;
use std::rc::Rc;

use crate::internal::utils::wrap_array;
pub use crate::menu::CheckedState;
use crate::menu::*;
use crate::support::collection::{
    CollectionItemSlot, CollectionItemValue, CollectionProvider, CollectionSlot, use_collection,
};
use crate::support::compose_refs::use_composed_refs;
use crate::support::direction::{Direction, use_direction};
use crate::support::id::use_id;
use crate::support::primitive::{
    Primitive, adapt_callback, compose_callbacks, data_attr, prop_or, prop_or_default,
    wrap_callback,
};
use crate::support::roving_focus::{Orientation, RovingFocusGroup, RovingFocusGroupItem};
use crate::support::use_controllable_state::{UseControllableStateParams, use_controllable_state};
use leptos::{context::Provider, ev, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::JsCast;

mod menubar;
mod menubar_content;
mod menubar_items;

pub use menubar::*;
pub use menubar_content::*;
pub use menubar_items::*;

/* -------------------------------------------------------------------------------------------------
 * Shared types
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Debug)]
struct ItemData {
    value: String,
    disabled: bool,
}

const ITEM_DATA_PHANTOM: PhantomData<ItemData> = PhantomData;

#[derive(Clone)]
struct MenubarContextValue {
    value: Signal<String>,
    dir: Signal<Direction>,
    r#loop: Signal<bool>,
    on_menu_open: Callback<String>,
    on_menu_close: Callback<()>,
    on_menu_toggle: Callback<String>,
}

#[derive(Clone)]
struct MenubarMenuContextValue {
    value: String,
    trigger_id: ReadSignal<String>,
    trigger_ref: AnyNodeRef,
    content_id: ReadSignal<String>,
    was_keyboard_trigger_open_ref: SendWrapper<Rc<Cell<bool>>>,
}
