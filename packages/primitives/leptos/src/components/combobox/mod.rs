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
//! - Auto-highlight first matching item while filtering (`auto_highlight`)
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
use pith_virtual_leptos::VirtualizerHandle;
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;

mod combobox;
mod combobox_chips;
mod combobox_content;
mod combobox_item;
mod combobox_portal;
mod combobox_separator;
mod combobox_virtual;

pub use crate::support::popper::{Align, Padding, Side, Sticky};
pub use combobox::*;
pub use combobox_chips::*;
pub use combobox_content::*;
pub use combobox_item::*;
pub use combobox_portal::*;
pub use combobox_separator::*;
pub use combobox_virtual::*;
pub use pith_virtual_leptos::VirtualItem;

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

/// Alias for the collection getter stored by `ComboboxInput`.
type GetItems = StoredValue<SendWrapper<Box<dyn Fn() -> Vec<CollectionItemValue<ComboboxItemData>>>>>;

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
    /// Whether the first matching item is highlighted automatically while filtering.
    auto_highlight: bool,
    /// Whether virtual scrolling is enabled.
    virtualized: bool,
    /// In virtual mode: the highlighted item's virtual index.
    highlighted_virtual_index: RwSignal<Option<usize>>,
    /// In virtual mode: total item count (set by ComboboxVirtualItems).
    virtual_item_count: RwSignal<usize>,
    /// In virtual mode: the virtualizer handle (set by ComboboxVirtualItems).
    virtualizer: StoredValue<Option<VirtualizerHandle>>,
}

impl ComboboxContextValue {
    /// Move focus to the combobox input element.
    fn focus_input(&self) {
        if let Some(input_el) = self.input_ref.get_untracked() {
            let el: &web_sys::HtmlElement = (*input_el).unchecked_ref();
            let _ = el.focus();
        }
    }

    /// Close the popup and clear active descendant highlight.
    fn dismiss(&self) {
        self.on_open_change.run(false);
        self.active_descendant_id.set(None);
        if self.virtualized {
            self.highlighted_virtual_index.set(None);
        }
    }
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
    on_item_text_change: Callback<Option<SendWrapper<web_sys::HtmlElement>>>,
}

#[derive(Clone, Copy)]
struct ComboboxGroupContextValue {
    id: ReadSignal<String>,
}

/// Provided by `ComboboxVirtualItems` around each rendered item so
/// `ComboboxItem` can discover its virtual index without an explicit prop.
#[derive(Clone, Copy)]
struct ComboboxVirtualItemIndex(usize);

/* -------------------------------------------------------------------------------------------------
 * Utilities
 * -----------------------------------------------------------------------------------------------*/

/// Visually hidden styles for the bubble input element
const VISUALLY_HIDDEN_STYLES_STR: &str = "position: absolute; border: 0; width: 1px; height: 1px; padding: 0; margin: -1px; overflow: hidden; clip: rect(0, 0, 0, 0); white-space: nowrap; word-wrap: normal;";

/// Check whether the given node ref currently holds DOM focus.
fn is_ref_focused(node_ref: AnyNodeRef) -> bool {
    node_ref.get_untracked().is_some_and(|el| {
        let el: &web_sys::Element = (*el).unchecked_ref();
        web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.active_element())
            .is_some_and(|active| &active == el)
    })
}

/// Remove a chip at `idx` from `values`, update the highlight index, and
/// push the new values through context. Returns the new highlight index.
fn remove_chip_at(
    context: &ComboboxContextValue,
    idx: usize,
    mut values: Vec<String>,
) {
    if idx >= values.len() {
        return;
    }
    values.remove(idx);
    let next = if values.is_empty() {
        None
    } else if idx >= values.len() {
        Some(values.len() - 1)
    } else {
        Some(idx)
    };
    context.highlighted_chip_index.set(next);
    context.on_values_change.run(values);
}
