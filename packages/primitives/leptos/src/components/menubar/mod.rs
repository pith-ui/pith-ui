use std::cell::Cell;
use std::marker::PhantomData;
use std::rc::Rc;

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
use radix_utils::wrap_array;
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
    value: Memo<String>,
    trigger_id: ReadSignal<String>,
    trigger_ref: AnyNodeRef,
    content_id: ReadSignal<String>,
    was_keyboard_trigger_open_ref: SendWrapper<Rc<Cell<bool>>>,
}
