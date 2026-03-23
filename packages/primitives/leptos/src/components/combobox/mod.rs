//! Combobox primitive for filtering and selecting from a list.
//!
//! Combines a text input for filtering with a dropdown listbox for selection.
//! Supports both single-select and multi-select modes with full keyboard
//! navigation and accessibility.
//!
//! Implements the [WAI-ARIA Combobox pattern](https://www.w3.org/WAI/ARIA/apd/patterns/combobox/).
//!
//! # Anatomy
//!
//! ```text
//! <Combobox>
//!     <ComboboxAnchor>
//!         <ComboboxChips>
//!             <ComboboxChip>
//!                 <ComboboxChipRemove />
//!             </ComboboxChip>
//!         </ComboboxChips>
//!         <ComboboxInput />
//!         <ComboboxTrigger>
//!             <ComboboxIcon />
//!         </ComboboxTrigger>
//!         <ComboboxClear />
//!     </ComboboxAnchor>
//!     <ComboboxPortal>
//!         <ComboboxContent>
//!             <ComboboxViewport>
//!                 <ComboboxItem>
//!                     <ComboboxItemText />
//!                     <ComboboxItemIndicator />
//!                 </ComboboxItem>
//!                 <ComboboxGroup>
//!                     <ComboboxLabel />
//!                     <ComboboxItem />
//!                 </ComboboxGroup>
//!                 <ComboboxSeparator />
//!             </ComboboxViewport>
//!         </ComboboxContent>
//!     </ComboboxPortal>
//! </Combobox>
//! ```
//!
//! # Features
//!
//! - Controlled and uncontrolled value/open/input state
//! - Single-select and multi-select modes
//! - Consumer-managed filtering via `on_input_value_change`
//! - Keyboard navigation with `aria-activedescendant`
//! - Multi-select chip display
//! - Native form participation via hidden input
//! - Full keyboard navigation
//!
//! # Keyboard Interactions
//!
//! | Key | Action |
//! |-----|--------|
//! | ArrowDown | Opens popup or highlights next item |
//! | ArrowUp | Highlights previous item |
//! | Enter | Selects highlighted item |
//! | Escape | Closes popup |
//! | Typing | Filters via input value change callback |
//!
//! # Data Attributes
//!
//! **ComboboxInput:**
//!
//! | Attribute | Values |
//! |-----------|--------|
//! | `data-state` | `open`, `closed` |
//! | `data-disabled` | Present when disabled |
//!
//! **ComboboxItem:**
//!
//! | Attribute | Values |
//! |-----------|--------|
//! | `data-state` | `checked`, `unchecked` |
//! | `data-highlighted` | Present when active descendant |
//! | `data-disabled` | Present when disabled |

use std::marker::PhantomData;

use crate::support::collection::{
    CollectionItemSlot, CollectionItemValue, CollectionProvider, CollectionSlot,
    provide_collection_scope, use_collection, use_collection_scope,
};
use crate::support::compose_refs::use_composed_refs;
use crate::support::direction::{Direction, use_direction};
use crate::support::dismissable_layer::DismissableLayer;
use crate::support::id::use_id;
use crate::support::popper::{
    Popper, PopperAnchor, PopperArrow, PopperContent, UpdatePositionStrategy, provide_popper_scope,
    use_popper_scope,
};
use crate::support::portal::ScopedPortal;
use crate::support::primitive::{Primitive, VoidPrimitive, data_attr, prop_or_default};
use crate::support::use_controllable_state::{UseControllableStateParams, use_controllable_state};
use leptos::{
    attribute_interceptor::AttributeInterceptor, context::Provider, ev, html, prelude::*,
};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;

mod combobox;
mod combobox_chips;
mod combobox_content;
mod combobox_item;
mod combobox_portal;
mod combobox_separator;

pub use crate::support::popper::{Align, Padding, Side, Sticky};
pub use combobox::*;
pub use combobox_chips::*;
pub use combobox_content::*;
pub use combobox_item::*;
pub use combobox_portal::*;
pub use combobox_separator::*;

/* -------------------------------------------------------------------------------------------------
 * Collection ItemData
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Debug)]
pub struct ComboboxItemData {
    pub value: String,
    pub disabled: bool,
    pub text_value: String,
}

const ITEM_DATA_PHANTOM: PhantomData<ComboboxItemData> = PhantomData;

/* -------------------------------------------------------------------------------------------------
 * Contexts
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy)]
struct ComboboxContextValue {
    input_ref: AnyNodeRef,
    trigger_ref: AnyNodeRef,
    content_id: ReadSignal<String>,
    value: Signal<Option<String>>,
    values: Signal<Vec<String>>,
    on_value_change: Callback<String>,
    on_values_change: Callback<Vec<String>>,
    input_value: Signal<String>,
    on_input_value_change: Callback<String>,
    open: Signal<bool>,
    on_open_change: Callback<bool>,
    disabled: Signal<bool>,
    #[allow(dead_code)]
    required: Signal<bool>,
    dir: Signal<Direction>,
    active_descendant_id: RwSignal<Option<String>>,
    /// Index of the currently highlighted chip in multi-select mode.
    /// `None` means no chip is highlighted (focus is on the input).
    highlighted_chip_index: RwSignal<Option<usize>>,
    multiple: bool,
}

#[derive(Clone, Copy)]
struct ComboboxContentContextValue {
    #[allow(dead_code)]
    content_ref: AnyNodeRef,
    viewport_ref: AnyNodeRef,
    #[allow(dead_code)]
    is_positioned: ReadSignal<bool>,
}

#[derive(Clone)]
struct ComboboxItemContextValue {
    #[allow(dead_code)]
    value: String,
    #[allow(dead_code)]
    disabled: bool,
    text_id: ReadSignal<String>,
    is_selected: Signal<bool>,
}

#[derive(Clone, Copy)]
struct ComboboxGroupContextValue {
    id: ReadSignal<String>,
}

/* -------------------------------------------------------------------------------------------------
 * Utilities
 * -----------------------------------------------------------------------------------------------*/

/// Visually hidden styles for the bubble input element
const VISUALLY_HIDDEN_STYLES_STR: &str = "position: absolute; border: 0; width: 1px; height: 1px; padding: 0; margin: -1px; overflow: hidden; clip: rect(0, 0, 0, 0); white-space: nowrap; word-wrap: normal;";
