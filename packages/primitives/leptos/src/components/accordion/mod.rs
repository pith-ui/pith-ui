use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

use crate::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use crate::support::collection::{
    CollectionItemSlot, CollectionProvider, CollectionSlot, use_collection,
};
use crate::support::compose_refs::use_composed_refs;
use crate::support::direction::{Direction, use_direction};
use crate::support::id::use_id;
use crate::support::primitive::{Primitive, data_attr, open_closed_state, prop_or_default};
use crate::support::use_controllable_state::{UseControllableStateParams, use_controllable_state};
use leptos::{
    attribute_interceptor::AttributeInterceptor, context::Provider, ev, html, prelude::*,
};
use leptos_node_ref::AnyNodeRef;
use web_sys::wasm_bindgen::JsCast;

mod accordion;
mod accordion_item;

pub use accordion::*;
pub use accordion_item::*;

/* -------------------------------------------------------------------------------------------------
 * Constants
 * -----------------------------------------------------------------------------------------------*/

const ACCORDION_KEYS: &[&str] = &[
    "Home",
    "End",
    "ArrowDown",
    "ArrowUp",
    "ArrowLeft",
    "ArrowRight",
];

/* -------------------------------------------------------------------------------------------------
 * Types
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AccordionType {
    Single,
    Multiple,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Orientation {
    Horizontal,
    #[default]
    Vertical,
}

impl Display for Orientation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Orientation::Horizontal => "horizontal",
                Orientation::Vertical => "vertical",
            }
        )
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
struct ItemData;

const ITEM_DATA_PHANTOM: PhantomData<ItemData> = PhantomData;

/* -------------------------------------------------------------------------------------------------
 * Contexts
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone)]
struct AccordionValueContextValue {
    value: Signal<Vec<String>>,
    on_item_open: Callback<String>,
    on_item_close: Callback<String>,
}

#[derive(Clone)]
struct AccordionCollapsibleContextValue {
    collapsible: Signal<bool>,
}

#[derive(Clone)]
struct AccordionImplContextValue {
    disabled: Signal<bool>,
    orientation: Signal<Orientation>,
}

#[derive(Clone)]
struct AccordionItemContextValue {
    open: Signal<bool>,
    disabled: Signal<bool>,
    trigger_id: ReadSignal<String>,
    #[allow(dead_code)]
    item_value: StoredValue<String>,
}

/* -------------------------------------------------------------------------------------------------
 * Context Creation Helpers
 * -----------------------------------------------------------------------------------------------*/

fn create_single_contexts(
    value: MaybeProp<String>,
    default_value: MaybeProp<String>,
    on_value_change: Option<Callback<String>>,
    collapsible: MaybeProp<bool>,
) -> (AccordionValueContextValue, AccordionCollapsibleContextValue) {
    let collapsible_flag = prop_or_default(collapsible);

    let (value_signal, set_value) = use_controllable_state(UseControllableStateParams {
        prop: value,
        default_prop: default_value,
        on_change: on_value_change.map(|on_value_change| {
            Callback::new(move |value: Option<String>| {
                if let Some(value) = value {
                    on_value_change.run(value);
                }
            })
        }),
    });

    // Wrap single value in a Vec for the shared AccordionValue context.
    let value_as_vec = Signal::derive(move || single_value_to_vec(value_signal.get()));

    let on_item_open = Callback::new(move |item_value: String| {
        set_value.run(Some(item_value));
    });

    let on_item_close = Callback::new(move |_: String| {
        if collapsible_flag.get() {
            set_value.run(Some(String::new()));
        }
    });

    (
        AccordionValueContextValue {
            value: value_as_vec,
            on_item_open,
            on_item_close,
        },
        AccordionCollapsibleContextValue {
            collapsible: collapsible_flag,
        },
    )
}

fn create_multiple_contexts(
    value: MaybeProp<Vec<String>>,
    default_value: MaybeProp<Vec<String>>,
    on_value_change: Option<Callback<Vec<String>>>,
) -> (AccordionValueContextValue, AccordionCollapsibleContextValue) {
    let (value_signal, set_value) = use_controllable_state(UseControllableStateParams {
        prop: value,
        default_prop: default_value,
        on_change: on_value_change.map(|on_value_change| {
            Callback::new(move |value: Option<Vec<String>>| {
                if let Some(value) = value {
                    on_value_change.run(value);
                }
            })
        }),
    });

    let value_vec = Signal::derive(move || value_signal.get().unwrap_or_default());

    let on_item_open = Callback::new(move |item_value: String| {
        let mut current = value_signal.get().unwrap_or_default();
        current.push(item_value);
        set_value.run(Some(current));
    });

    let on_item_close = Callback::new(move |item_value: String| {
        set_value.run(Some(remove_item(
            value_signal.get().unwrap_or_default(),
            &item_value,
        )));
    });

    (
        AccordionValueContextValue {
            value: value_vec,
            on_item_open,
            on_item_close,
        },
        AccordionCollapsibleContextValue {
            // Multiple mode is always collapsible.
            collapsible: Signal::derive(|| true),
        },
    )
}

/* -------------------------------------------------------------------------------------------------
 * Utility Functions
 * -----------------------------------------------------------------------------------------------*/

fn single_value_to_vec(value: Option<String>) -> Vec<String> {
    match value {
        None => vec![],
        Some(v) if v.is_empty() => vec![],
        Some(v) => vec![v],
    }
}

fn remove_item(values: Vec<String>, item: &str) -> Vec<String> {
    values.into_iter().filter(|v| v != item).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── single_value_to_vec ─────────────────────────────────

    #[test]
    fn single_value_none() {
        assert_eq!(single_value_to_vec(None), Vec::<String>::new());
    }

    #[test]
    fn single_value_empty() {
        assert_eq!(
            single_value_to_vec(Some(String::new())),
            Vec::<String>::new()
        );
    }

    #[test]
    fn single_value_present() {
        assert_eq!(single_value_to_vec(Some("item-1".into())), vec!["item-1"]);
    }

    // ── remove_item ─────────────────────────────────────────

    #[test]
    fn remove_existing_item() {
        let result = remove_item(vec!["a".into(), "b".into(), "c".into()], "b");
        assert_eq!(result, vec!["a", "c"]);
    }

    #[test]
    fn remove_nonexistent_item() {
        let result = remove_item(vec!["a".into(), "b".into()], "z");
        assert_eq!(result, vec!["a", "b"]);
    }

    #[test]
    fn remove_from_empty() {
        let result = remove_item(vec![], "a");
        assert!(result.is_empty());
    }

    #[test]
    fn remove_last_item() {
        let result = remove_item(vec!["a".into()], "a");
        assert!(result.is_empty());
    }

    #[test]
    fn remove_duplicate_items() {
        let result = remove_item(vec!["a".into(), "b".into(), "a".into()], "a");
        assert_eq!(result, vec!["b"]);
    }
}
