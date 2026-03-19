//! Vertically stacked expandable sections.
//!
//! A set of collapsible content panels where one or multiple can be expanded.
//! Built on top of [`Collapsible`](crate::collapsible) with coordinated state,
//! keyboard navigation, and ARIA accordion semantics.
//!
//! # Anatomy
//!
//! ```text
//! <Accordion>              // or AccordionSingle / AccordionMultiple
//!     <AccordionItem>
//!         <AccordionHeader>
//!             <AccordionTrigger />
//!         </AccordionHeader>
//!         <AccordionContent />
//!     </AccordionItem>
//! </Accordion>
//! ```
//!
//! # Features
//!
//! - Single or multiple expanded items
//! - Controlled and uncontrolled expanded state
//! - Keyboard navigation between triggers (arrow keys, Home, End)
//! - Horizontal and vertical orientation
//! - RTL support
//! - CSS animation support via `--radix-collapsible-content-height` / `--radix-collapsible-content-width`
//!
//! # Keyboard Interactions
//!
//! | Key | Action |
//! |-----|--------|
//! | Space / Enter | Toggles the focused item |
//! | ArrowDown | Focuses next trigger (vertical) |
//! | ArrowUp | Focuses previous trigger (vertical) |
//! | ArrowRight | Focuses next trigger (horizontal) |
//! | ArrowLeft | Focuses previous trigger (horizontal) |
//! | Home | Focuses first trigger |
//! | End | Focuses last trigger |
//!
//! # Data Attributes
//!
//! **AccordionItem, AccordionTrigger, AccordionContent:**
//!
//! | Attribute | Values |
//! |-----------|--------|
//! | `data-state` | `open`, `closed` |
//! | `data-disabled` | Present when disabled |
//! | `data-orientation` | `horizontal`, `vertical` |

use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

use crate::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use crate::support::collection::{
    CollectionItemSlot, CollectionProvider, CollectionSlot, use_collection,
};
use crate::support::compose_refs::use_composed_refs;
use crate::support::direction::{Direction, use_direction};
use crate::support::id::use_id;
use crate::support::primitive::{
    Primitive, adapt_callback, data_attr, open_closed_state, prop_or_default,
};
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

const ACCORDION_KEYS: &[&str] = &[
    "Home",
    "End",
    "ArrowDown",
    "ArrowUp",
    "ArrowLeft",
    "ArrowRight",
];

/// The selection mode of an accordion.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AccordionType {
    /// Only one item can be expanded at a time.
    Single,
    /// Multiple items can be expanded simultaneously.
    Multiple,
}

/// The orientation of an accordion.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Orientation {
    /// Horizontal accordion with left/right arrow navigation.
    Horizontal,
    /// Vertical accordion with up/down arrow navigation (default).
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

#[derive(Clone, Copy)]
pub(crate) struct AccordionValueContextValue {
    pub(crate) value: Signal<Vec<String>>,
    pub(crate) on_item_open: Callback<String>,
    pub(crate) on_item_close: Callback<String>,
}

#[derive(Clone, Copy)]
pub(crate) struct AccordionCollapsibleContextValue {
    pub(crate) collapsible: Signal<bool>,
}

#[derive(Clone, Copy)]
pub(crate) struct AccordionImplContextValue {
    pub(crate) disabled: Signal<bool>,
    pub(crate) orientation: Signal<Orientation>,
}

#[derive(Clone, Copy)]
pub(crate) struct AccordionItemContextValue {
    pub(crate) open: Signal<bool>,
    pub(crate) disabled: Signal<bool>,
    pub(crate) trigger_id: ReadSignal<String>,
    #[allow(dead_code)]
    pub(crate) item_value: StoredValue<String>,
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
        on_change: adapt_callback(on_value_change),
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
        on_change: adapt_callback(on_value_change),
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
    use any_spawner::Executor;

    struct NoopExecutor;

    impl any_spawner::CustomExecutor for NoopExecutor {
        fn spawn(&self, _fut: any_spawner::PinnedFuture<()>) {}
        fn spawn_local(&self, _fut: any_spawner::PinnedLocalFuture<()>) {}
        fn poll_local(&self) {}
    }

    fn with_owner<T>(f: impl FnOnce() -> T) -> T {
        let _ = Executor::init_custom_executor(NoopExecutor);
        let owner = Owner::new_root(None);
        owner.with(f)
    }

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

    // ── Single mode state machine ───────────────────────────

    #[test]
    fn single_open_sets_value() {
        with_owner(|| {
            let (ctx, _) = create_single_contexts(
                MaybeProp::from(None::<String>),
                MaybeProp::from(None::<String>),
                None,
                MaybeProp::from(Some(true)),
            );

            assert_eq!(ctx.value.get(), Vec::<String>::new());

            ctx.on_item_open.run("item-1".into());
            assert_eq!(ctx.value.get(), vec!["item-1"]);
        });
    }

    #[test]
    fn single_open_replaces_previous() {
        with_owner(|| {
            let (ctx, _) = create_single_contexts(
                MaybeProp::from(None::<String>),
                MaybeProp::from(None::<String>),
                None,
                MaybeProp::from(Some(true)),
            );

            ctx.on_item_open.run("item-1".into());
            ctx.on_item_open.run("item-2".into());
            assert_eq!(ctx.value.get(), vec!["item-2"]);
        });
    }

    #[test]
    fn single_close_clears_when_collapsible() {
        with_owner(|| {
            let (ctx, _) = create_single_contexts(
                MaybeProp::from(None::<String>),
                MaybeProp::from(None::<String>),
                None,
                MaybeProp::from(Some(true)),
            );

            ctx.on_item_open.run("item-1".into());
            ctx.on_item_close.run("item-1".into());
            assert_eq!(ctx.value.get(), Vec::<String>::new());
        });
    }

    #[test]
    fn single_close_noop_when_not_collapsible() {
        with_owner(|| {
            let (ctx, _) = create_single_contexts(
                MaybeProp::from(None::<String>),
                MaybeProp::from(None::<String>),
                None,
                MaybeProp::from(Some(false)),
            );

            ctx.on_item_open.run("item-1".into());
            ctx.on_item_close.run("item-1".into());
            assert_eq!(ctx.value.get(), vec!["item-1"]);
        });
    }

    #[test]
    fn single_collapsible_defaults_to_false() {
        with_owner(|| {
            let (ctx, collapsible_ctx) = create_single_contexts(
                MaybeProp::from(None::<String>),
                MaybeProp::from(None::<String>),
                None,
                MaybeProp::from(None::<bool>),
            );

            assert!(!collapsible_ctx.collapsible.get());

            // Close should be a no-op since collapsible defaults to false.
            ctx.on_item_open.run("item-1".into());
            ctx.on_item_close.run("item-1".into());
            assert_eq!(ctx.value.get(), vec!["item-1"]);
        });
    }

    #[test]
    fn single_default_value() {
        with_owner(|| {
            let (ctx, _) = create_single_contexts(
                MaybeProp::from(None::<String>),
                MaybeProp::from(Some("item-2".to_string())),
                None,
                MaybeProp::from(Some(true)),
            );

            assert_eq!(ctx.value.get(), vec!["item-2"]);
        });
    }

    #[test]
    fn single_on_value_change_fires_in_controlled_mode() {
        with_owner(|| {
            let received = StoredValue::new(String::new());

            let on_change = Callback::new(move |v: String| {
                received.set_value(v);
            });

            // Use a controlled prop so on_change fires synchronously
            // (uncontrolled on_change goes through an Effect which doesn't
            // run under the NoopExecutor).
            let (ctx, _) = create_single_contexts(
                MaybeProp::from(Some(String::new())),
                MaybeProp::from(None::<String>),
                Some(on_change),
                MaybeProp::from(Some(true)),
            );

            ctx.on_item_open.run("item-1".into());
            assert_eq!(received.get_value(), "item-1");
        });
    }

    // ── Multiple mode state machine ─────────────────────────

    #[test]
    fn multiple_open_appends() {
        with_owner(|| {
            let (ctx, _) = create_multiple_contexts(
                MaybeProp::from(None::<Vec<String>>),
                MaybeProp::from(None::<Vec<String>>),
                None,
            );

            ctx.on_item_open.run("item-1".into());
            ctx.on_item_open.run("item-2".into());
            assert_eq!(ctx.value.get(), vec!["item-1", "item-2"]);
        });
    }

    #[test]
    fn multiple_close_removes_one() {
        with_owner(|| {
            let (ctx, _) = create_multiple_contexts(
                MaybeProp::from(None::<Vec<String>>),
                MaybeProp::from(None::<Vec<String>>),
                None,
            );

            ctx.on_item_open.run("item-1".into());
            ctx.on_item_open.run("item-2".into());
            ctx.on_item_open.run("item-3".into());
            ctx.on_item_close.run("item-2".into());
            assert_eq!(ctx.value.get(), vec!["item-1", "item-3"]);
        });
    }

    #[test]
    fn multiple_close_last_item_empties() {
        with_owner(|| {
            let (ctx, _) = create_multiple_contexts(
                MaybeProp::from(None::<Vec<String>>),
                MaybeProp::from(None::<Vec<String>>),
                None,
            );

            ctx.on_item_open.run("item-1".into());
            ctx.on_item_close.run("item-1".into());
            assert_eq!(ctx.value.get(), Vec::<String>::new());
        });
    }

    #[test]
    fn multiple_always_collapsible() {
        with_owner(|| {
            let (_, collapsible_ctx) = create_multiple_contexts(
                MaybeProp::from(None::<Vec<String>>),
                MaybeProp::from(None::<Vec<String>>),
                None,
            );

            assert!(collapsible_ctx.collapsible.get());
        });
    }

    #[test]
    fn multiple_default_values() {
        with_owner(|| {
            let (ctx, _) = create_multiple_contexts(
                MaybeProp::from(None::<Vec<String>>),
                MaybeProp::from(Some(vec!["a".to_string(), "b".to_string()])),
                None,
            );

            assert_eq!(ctx.value.get(), vec!["a", "b"]);
        });
    }

    #[test]
    fn multiple_on_value_change_fires_in_controlled_mode() {
        with_owner(|| {
            let received = StoredValue::new(Vec::<String>::new());

            let on_change = Callback::new(move |v: Vec<String>| {
                received.set_value(v);
            });

            // Use a controlled prop so on_change fires synchronously.
            let (ctx, _) = create_multiple_contexts(
                MaybeProp::from(Some(Vec::<String>::new())),
                MaybeProp::from(None::<Vec<String>>),
                Some(on_change),
            );

            ctx.on_item_open.run("item-1".into());
            assert_eq!(received.get_value(), vec!["item-1"]);

            ctx.on_item_open.run("item-2".into());
            // Controlled mode: the prop still returns empty, so
            // on_item_open reads the prop (empty) and appends.
            assert_eq!(received.get_value(), vec!["item-2"]);
        });
    }
}
