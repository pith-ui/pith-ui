// TODO: remove
#![expect(dead_code)]

use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

use crate::internal::utils::{Point, is_point_in_polygon, wrap_array};
use crate::support::aria_hidden::{hide_others, unhide_others};
use crate::support::collection::{
    CollectionItemSlot, CollectionProvider, CollectionSlot, use_collection,
};
use crate::support::compose_refs::use_composed_refs;
pub use crate::support::direction::Direction;
use crate::support::direction::use_direction;
use crate::support::dismissable_layer::DismissableLayer;
use crate::support::focus_guards::use_focus_guards;
use crate::support::focus_scope::FocusScope;
use crate::support::forwarded_attrs::ForwardedAttrs;
use crate::support::id::use_id;
pub use crate::support::popper::{
    Align, ClientRectObject, Padding, PopperVirtualElement, Side as PopperSide, Sticky,
    set_popper_virtual_ref,
};
use crate::support::popper::{Popper, PopperAnchor, PopperArrow, PopperContent};
use crate::support::portal::ScopedPortal;
use crate::support::presence::Presence;
use crate::support::primitive::{
    Primitive, compose_callbacks, data_attr, open_closed_state, prop_or, prop_or_default,
    wrap_callback,
};
use crate::support::roving_focus::{Orientation, RovingFocusGroup, RovingFocusGroupItem};
use leptos::{
    attribute_interceptor::AttributeInterceptor, context::Provider, ev, html, prelude::*,
};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{AddEventListenerOptions, CustomEvent, CustomEventInit, EventListenerOptions};

// ── Sub-modules ─────────────────────────────────────────────────────────────

mod menu;
mod menu_checkbox_item;
mod menu_content;
mod menu_item;
mod menu_separator;
mod menu_sub;

pub use menu::*;
pub use menu_checkbox_item::*;
pub use menu_content::*;
pub use menu_item::*;
pub use menu_separator::*;
pub use menu_sub::*;

// Re-import pub(super) items so sibling sub-modules can access them via `use super::*;`.
use menu_content::MenuContentImpl;
use menu_item::MenuItemImpl;

// ── Constants ───────────────────────────────────────────────────────────────

const SELECTION_KEYS: [&str; 2] = ["Enter", " "];
const FIRST_KEYS: [&str; 3] = ["ArrowDown", "PageUp", "Home"];
const LAST_KEYS: [&str; 3] = ["ArrowUp", "PageDown", "End"];
const FIRST_LAST_KEYS: [&str; 6] = ["ArrowDown", "PageUp", "Home", "ArrowUp", "PageDown", "End"];

// ── Shared helper functions ─────────────────────────────────────────────────

fn sub_open_keys(dir: Direction) -> &'static [&'static str] {
    match dir {
        Direction::Ltr => &["Enter", " ", "ArrowRight"],
        Direction::Rtl => &["Enter", " ", "ArrowLeft"],
    }
}

fn sub_close_keys(dir: Direction) -> &'static [&'static str] {
    match dir {
        Direction::Ltr => &["ArrowLeft"],
        Direction::Rtl => &["ArrowRight"],
    }
}

// ── Shared types ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CheckedState {
    False,
    True,
    Indeterminate,
}

impl From<bool> for CheckedState {
    fn from(value: bool) -> Self {
        if value {
            CheckedState::True
        } else {
            CheckedState::False
        }
    }
}

fn is_indeterminate(checked: CheckedState) -> bool {
    checked == CheckedState::Indeterminate
}

fn get_checked_state(checked: CheckedState) -> &'static str {
    match checked {
        CheckedState::Indeterminate => "indeterminate",
        CheckedState::True => "checked",
        CheckedState::False => "unchecked",
    }
}

// ── Context structs ─────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
struct ItemIndicatorContextValue {
    checked: Signal<CheckedState>,
}

#[derive(Clone, Copy)]
struct RadioGroupContextValue {
    value: Signal<Option<String>>,
    on_value_change: Callback<String>,
}

#[derive(Clone, Copy)]
struct MenuSubContextValue {
    content_id: ReadSignal<String>,
    trigger_id: ReadSignal<String>,
    trigger_ref: AnyNodeRef,
}

#[derive(Clone, Debug)]
struct ItemData {
    disabled: bool,
    text_value: String,
}

const ITEM_DATA_PHANTHOM: PhantomData<ItemData> = PhantomData;

#[derive(Clone, Copy)]
struct MenuContextValue {
    open: Signal<bool>,
    content_ref: AnyNodeRef,
    on_open_change: Callback<bool>,
    /// Direct reference to the Menu's own Popper anchor ref, so that
    /// `MenuAnchor` can set it without relying on `expect_context::<PopperContextValue>()`.
    /// This avoids context shadowing when another Popper (e.g., Tooltip) is nested
    /// between the Menu's Popper and the MenuAnchor.
    popper_anchor_ref: AnyNodeRef,
}

#[derive(Clone, Copy)]
struct MenuRootContextValue {
    is_using_keyboard: Signal<bool>,
    dir: Signal<Direction>,
    modal: Signal<bool>,
    on_close: Callback<()>,
}

#[derive(Clone, Copy)]
struct MenuContentContextValue {
    on_item_enter: Callback<ev::PointerEvent>,
    on_item_leave: Callback<ev::PointerEvent>,
    on_trigger_leave: Callback<ev::PointerEvent>,
    search: RwSignal<String>,
    pointer_grace_timer: RwSignal<u64>,
    on_pointer_grace_intent_change: Callback<Option<GraceIntent>>,
}

// ── Grace area / pointer tracking types ─────────────────────────────────────

type Polygon = Vec<Point>;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Side {
    Left,
    Right,
}

#[derive(Clone, Debug)]
struct GraceIntent {
    area: Polygon,
    side: Side,
}

// ── Shared utility functions ────────────────────────────────────────────────

fn focus_first(candidates: Vec<web_sys::HtmlElement>) {
    let previously_focused_element = document().active_element();
    for candidate in candidates {
        // If focus is already where we want to go, we don't want to keep going through the candidates.
        if previously_focused_element.as_ref() == candidate.dyn_ref::<web_sys::Element>() {
            return;
        }

        candidate.focus().expect("Element should be focused.");
        if document().active_element() != previously_focused_element {
            return;
        }
    }
}

/// This is the "meat" of the typeahead matching logic. It takes in all the values,
/// the search and the current match, and returns the next match (or `None`).
///
/// We normalize the search because if a user has repeatedly pressed a character,
/// we want the exact same behavior as if we only had that one character
/// (ie. cycle through options starting with that character)
///
/// We also reorder the values by wrapping the array around the current match.
/// This is so we always look forward from the current match, and picking the first
/// match will always be the correct one.
///
/// Finally, if the normalized search is exactly one character, we exclude the
/// current match from the values because otherwise it would be the first to match always
/// and focus would never move. This is as opposed to the regular case, where we
/// don't want focus to move if the current match still matches.
fn get_next_match(
    values: Vec<String>,
    search: String,
    current_match: Option<String>,
) -> Option<String> {
    let is_repeated =
        search.chars().count() > 1 && search.chars().all(|c| c == search.chars().next().unwrap());
    let normilized_search = if is_repeated {
        search.chars().take(1).collect()
    } else {
        search
    };
    let current_match_index = current_match
        .as_ref()
        .and_then(|current_match| values.iter().position(|value| value == current_match));
    let mut wrapped_values =
        wrap_array(&mut values.clone(), current_match_index.unwrap_or(0)).to_vec();
    let exclude_current_match = normilized_search.chars().count() == 1;
    if exclude_current_match {
        wrapped_values.retain(|v| {
            current_match
                .as_ref()
                .is_none_or(|current_match| v != current_match)
        });
    }
    let next_match = wrapped_values.into_iter().find(|value| {
        value
            .to_lowercase()
            .starts_with(&normilized_search.to_lowercase())
    });

    if next_match != current_match {
        next_match
    } else {
        None
    }
}

fn is_pointer_in_grace_area(event: &ev::PointerEvent, area: Option<Polygon>) -> bool {
    if let Some(area) = area {
        let cursor_pos = Point {
            x: event.client_x() as f64,
            y: event.client_y() as f64,
        };
        is_point_in_polygon(&cursor_pos, &area)
    } else {
        false
    }
}

fn when_mouse<H: Fn(ev::PointerEvent) + Send + Sync + 'static>(
    handler: H,
) -> Callback<ev::PointerEvent> {
    Callback::new(move |event: ev::PointerEvent| {
        if event.pointer_type() == "mouse" {
            handler(event);
        }
    })
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── sub_open_keys ───────────────────────────────────────

    #[test]
    fn sub_open_keys_ltr() {
        let keys = sub_open_keys(Direction::Ltr);
        assert!(keys.contains(&"Enter"));
        assert!(keys.contains(&" "));
        assert!(keys.contains(&"ArrowRight"));
        assert!(!keys.contains(&"ArrowLeft"));
    }

    #[test]
    fn sub_open_keys_rtl() {
        let keys = sub_open_keys(Direction::Rtl);
        assert!(keys.contains(&"Enter"));
        assert!(keys.contains(&" "));
        assert!(keys.contains(&"ArrowLeft"));
        assert!(!keys.contains(&"ArrowRight"));
    }

    // ── sub_close_keys ──────────────────────────────────────

    #[test]
    fn sub_close_keys_ltr() {
        let keys = sub_close_keys(Direction::Ltr);
        assert!(keys.contains(&"ArrowLeft"));
        assert!(!keys.contains(&"ArrowRight"));
    }

    #[test]
    fn sub_close_keys_rtl() {
        let keys = sub_close_keys(Direction::Rtl);
        assert!(keys.contains(&"ArrowRight"));
        assert!(!keys.contains(&"ArrowLeft"));
    }

    // ── is_indeterminate ────────────────────────────────────

    #[test]
    fn is_indeterminate_true_for_indeterminate() {
        assert!(is_indeterminate(CheckedState::Indeterminate));
    }

    #[test]
    fn is_indeterminate_false_for_true() {
        assert!(!is_indeterminate(CheckedState::True));
    }

    #[test]
    fn is_indeterminate_false_for_false() {
        assert!(!is_indeterminate(CheckedState::False));
    }

    // ── get_checked_state ───────────────────────────────────

    #[test]
    fn checked_state_indeterminate() {
        assert_eq!(
            get_checked_state(CheckedState::Indeterminate),
            "indeterminate"
        );
    }

    #[test]
    fn checked_state_true() {
        assert_eq!(get_checked_state(CheckedState::True), "checked");
    }

    #[test]
    fn checked_state_false() {
        assert_eq!(get_checked_state(CheckedState::False), "unchecked");
    }

    // ── From<bool> for CheckedState ─────────────────────────

    #[test]
    fn checked_state_from_true() {
        assert_eq!(CheckedState::from(true), CheckedState::True);
    }

    #[test]
    fn checked_state_from_false() {
        assert_eq!(CheckedState::from(false), CheckedState::False);
    }

    // ── get_next_match ──────────────────────────────────────

    #[test]
    fn next_match_empty_search_matches_first() {
        // Empty string is a prefix of every value, so the first value matches.
        let values = vec!["Apple".into(), "Banana".into()];
        assert_eq!(
            get_next_match(values, "".into(), None),
            Some("Apple".into())
        );
    }

    #[test]
    fn next_match_single_char_cycles_through_matches() {
        let values = vec!["Apple".into(), "Avocado".into(), "Banana".into()];
        // Starting with no current match, picks first "a" match
        let result = get_next_match(values.clone(), "a".into(), None);
        assert_eq!(result, Some("Apple".into()));

        // With "Apple" as current match, cycles to "Avocado"
        let result = get_next_match(values.clone(), "a".into(), Some("Apple".into()));
        assert_eq!(result, Some("Avocado".into()));

        // With "Avocado" as current match, wraps to "Apple"
        let result = get_next_match(values.clone(), "a".into(), Some("Avocado".into()));
        assert_eq!(result, Some("Apple".into()));
    }

    #[test]
    fn next_match_repeated_chars_normalized() {
        // "aaa" is treated the same as "a"
        let values = vec!["Apple".into(), "Avocado".into(), "Banana".into()];
        let result = get_next_match(values.clone(), "aaa".into(), None);
        assert_eq!(result, Some("Apple".into()));

        // With current match, still cycles like single char
        let result = get_next_match(values.clone(), "aaa".into(), Some("Apple".into()));
        assert_eq!(result, Some("Avocado".into()));
    }

    #[test]
    fn next_match_multi_char_prefix() {
        let values = vec!["Apple".into(), "Application".into(), "Banana".into()];
        let result = get_next_match(values.clone(), "app".into(), None);
        assert_eq!(result, Some("Apple".into()));

        // Multi-char does NOT exclude current match
        let result = get_next_match(values.clone(), "app".into(), Some("Apple".into()));
        // "Apple" still matches and is the current, so returns None (no change)
        // Actually it wraps from Apple and finds Apple again, which equals current_match => None
        assert_eq!(result, None);
    }

    #[test]
    fn next_match_case_insensitive() {
        let values = vec!["Apple".into(), "Banana".into()];
        let result = get_next_match(values.clone(), "A".into(), None);
        assert_eq!(result, Some("Apple".into()));

        let result = get_next_match(values.clone(), "a".into(), None);
        assert_eq!(result, Some("Apple".into()));

        let result = get_next_match(values.clone(), "BANANA".into(), None);
        assert_eq!(result, Some("Banana".into()));
    }

    #[test]
    fn next_match_wraps_from_current_position() {
        let values = vec![
            "Alpha".into(),
            "Bravo".into(),
            "Charlie".into(),
            "Beta".into(),
        ];
        // Current match is "Charlie", search "b" should find "Beta" first (wraps around)
        let result = get_next_match(values.clone(), "b".into(), Some("Charlie".into()));
        assert_eq!(result, Some("Beta".into()));
    }

    #[test]
    fn next_match_returns_none_if_only_current_matches() {
        let values = vec!["Apple".into(), "Banana".into(), "Cherry".into()];
        // Only "Banana" starts with "b", and it's the current match
        let result = get_next_match(values.clone(), "b".into(), Some("Banana".into()));
        assert_eq!(result, None);
    }

    #[test]
    fn next_match_no_matching_values() {
        let values = vec!["Apple".into(), "Banana".into()];
        let result = get_next_match(values, "z".into(), None);
        assert_eq!(result, None);
    }
}
