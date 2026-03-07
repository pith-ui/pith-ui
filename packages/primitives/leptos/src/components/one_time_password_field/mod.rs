use std::marker::PhantomData;
use std::ops::Deref;

use crate::support::collection::{
    CollectionItemSlot, CollectionItemValue, CollectionProvider, CollectionSlot, use_collection,
};
use crate::support::compose_refs::use_composed_refs;
use crate::support::direction::{Direction, use_direction};
use crate::support::primitive::{
    Primitive, VoidPrimitive, compose_callbacks, data_attr, prop_or, prop_or_default,
};
use crate::support::roving_focus::{
    Orientation, RovingFocusGroup, RovingFocusGroupContext, RovingFocusGroupItem,
    RovingFocusGroupItemContext,
};
use crate::support::use_controllable_state::{UseControllableStateParams, use_controllable_state};
use leptos::{context::Provider, ev, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::JsCast;

mod one_time_password_field;
mod one_time_password_field_input;

pub use one_time_password_field::*;
pub use one_time_password_field_input::*;

/* -------------------------------------------------------------------------------------------------
 * Types
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum InputValidationType {
    Alpha,
    #[default]
    Numeric,
    Alphanumeric,
    None,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum InputType {
    Password,
    #[default]
    Text,
}

impl InputType {
    fn as_str(&self) -> &'static str {
        match self {
            InputType::Password => "password",
            InputType::Text => "text",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum AutoComplete {
    Off,
    #[default]
    OneTimeCode,
}

impl AutoComplete {
    fn as_str(&self) -> &'static str {
        match self {
            AutoComplete::Off => "off",
            AutoComplete::OneTimeCode => "one-time-code",
        }
    }
}

#[derive(Clone)]
struct ValidationInfo {
    regexp: &'static str,
    pattern: &'static str,
    input_mode: &'static str,
}

fn get_validation(validation_type: InputValidationType) -> Option<ValidationInfo> {
    match validation_type {
        InputValidationType::Numeric => Some(ValidationInfo {
            regexp: r"[^\d]",
            pattern: r"\d{1}",
            input_mode: "numeric",
        }),
        InputValidationType::Alpha => Some(ValidationInfo {
            regexp: r"[^a-zA-Z]",
            pattern: "[a-zA-Z]{1}",
            input_mode: "text",
        }),
        InputValidationType::Alphanumeric => Some(ValidationInfo {
            regexp: r"[^a-zA-Z0-9]",
            pattern: "[a-zA-Z0-9]{1}",
            input_mode: "text",
        }),
        InputValidationType::None => None,
    }
}

/* -------------------------------------------------------------------------------------------------
 * Actions
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Debug)]
enum UpdateAction {
    SetChar {
        index: usize,
        char: String,
    },
    ClearChar {
        index: usize,
        reason: ClearCharReason,
    },
    Clear {
        reason: ClearReason,
    },
    Paste {
        value: String,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ClearCharReason {
    Backspace,
    Delete,
    Cut,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(dead_code)]
enum ClearReason {
    Reset,
    Backspace,
    Delete,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
enum KeyboardActionDetails {
    Keydown {
        key: KeydownKey,
        meta_key: bool,
        ctrl_key: bool,
    },
    Cut,
    AutocompletePaste,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum KeydownKey {
    Backspace,
    Delete,
    Clear,
    Char,
}

/* -------------------------------------------------------------------------------------------------
 * Collection item data
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Debug)]
struct ItemData;

const ITEM_DATA_PHANTOM: PhantomData<ItemData> = PhantomData;

/* -------------------------------------------------------------------------------------------------
 * Collection helpers
 * -----------------------------------------------------------------------------------------------*/

fn collection_at(
    items: &[CollectionItemValue<ItemData>],
    index: isize,
) -> Option<&CollectionItemValue<ItemData>> {
    if index < 0 {
        let actual = items.len() as isize + index;
        if actual >= 0 {
            items.get(actual as usize)
        } else {
            None
        }
    } else {
        items.get(index as usize)
    }
}

fn collection_index_of(
    items: &[CollectionItemValue<ItemData>],
    element: &web_sys::HtmlElement,
) -> Option<usize> {
    items.iter().position(|item| {
        item.r#ref
            .get()
            .is_some_and(|el| *el.deref().unchecked_ref::<web_sys::HtmlElement>() == *element)
    })
}

fn collection_from<'a>(
    items: &'a [CollectionItemValue<ItemData>],
    element: &web_sys::HtmlElement,
    direction: isize,
) -> Option<&'a CollectionItemValue<ItemData>> {
    let index = collection_index_of(items, element)?;
    let new_index = index as isize + direction;
    if new_index >= 0 && (new_index as usize) < items.len() {
        Some(&items[new_index as usize])
    } else {
        None
    }
}

fn collection_element(item: &CollectionItemValue<ItemData>) -> Option<web_sys::HtmlInputElement> {
    item.r#ref.get().map(|el| {
        el.deref()
            .unchecked_ref::<web_sys::HtmlInputElement>()
            .clone()
    })
}

/* -------------------------------------------------------------------------------------------------
 * Context
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy)]
#[allow(dead_code)]
struct OneTimePasswordFieldContextValue {
    value: Signal<Vec<String>>,
    dispatch: StoredValue<SendWrapper<Box<dyn Fn(UpdateAction)>>>,
    attempt_submit: StoredValue<SendWrapper<Box<dyn Fn()>>>,
    auto_complete: Signal<AutoComplete>,
    auto_focus: Signal<bool>,
    disabled: Signal<bool>,
    read_only: Signal<bool>,
    form: Signal<Option<String>>,
    name: Signal<Option<String>>,
    placeholder: Signal<Option<String>>,
    r#type: Signal<InputType>,
    orientation: Signal<Orientation>,
    validation_type: Signal<InputValidationType>,
    user_action: RwSignal<Option<KeyboardActionDetails>>,
    sanitize_value: StoredValue<SendWrapper<Box<dyn Fn(String) -> Vec<String>>>>,
    hidden_input_ref: AnyNodeRef,
}

/* -------------------------------------------------------------------------------------------------
 * Utility functions
 * -----------------------------------------------------------------------------------------------*/

fn remove_whitespace(value: &str) -> String {
    value.chars().filter(|c| !c.is_whitespace()).collect()
}

fn focus_input(element: Option<&web_sys::HtmlInputElement>) {
    let Some(element) = element else {
        return;
    };

    if let Some(active) = element
        .owner_document()
        .and_then(|doc| doc.active_element())
        && active == *element.unchecked_ref::<web_sys::Element>()
    {
        // Already focused — select in next frame
        let el = element.clone();
        let cb = web_sys::wasm_bindgen::closure::Closure::once_into_js(move || {
            el.select();
        });
        let _ = web_sys::window()
            .expect("Window should exist")
            .request_animation_frame(cb.unchecked_ref());
        return;
    }

    let _ = element.focus();
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Helper to build a collection of N items ─────────────

    fn make_items(n: usize) -> Vec<CollectionItemValue<ItemData>> {
        let owner = Owner::new();
        owner.with(|| {
            (0..n)
                .map(|_| CollectionItemValue {
                    r#ref: AnyNodeRef::new(),
                    data: ItemData,
                })
                .collect()
        })
    }

    // ── get_validation ──────────────────────────────────────

    #[test]
    fn validation_numeric() {
        let info = get_validation(InputValidationType::Numeric).unwrap();
        assert_eq!(info.regexp, r"[^\d]");
        assert_eq!(info.pattern, r"\d{1}");
        assert_eq!(info.input_mode, "numeric");
    }

    #[test]
    fn validation_alpha() {
        let info = get_validation(InputValidationType::Alpha).unwrap();
        assert_eq!(info.regexp, r"[^a-zA-Z]");
        assert_eq!(info.pattern, "[a-zA-Z]{1}");
        assert_eq!(info.input_mode, "text");
    }

    #[test]
    fn validation_alphanumeric() {
        let info = get_validation(InputValidationType::Alphanumeric).unwrap();
        assert_eq!(info.regexp, r"[^a-zA-Z0-9]");
        assert_eq!(info.pattern, "[a-zA-Z0-9]{1}");
        assert_eq!(info.input_mode, "text");
    }

    #[test]
    fn validation_none() {
        assert!(get_validation(InputValidationType::None).is_none());
    }

    // ── remove_whitespace ───────────────────────────────────

    #[test]
    fn remove_whitespace_empty() {
        assert_eq!(remove_whitespace(""), "");
    }

    #[test]
    fn remove_whitespace_only_spaces() {
        assert_eq!(remove_whitespace("   "), "");
    }

    #[test]
    fn remove_whitespace_mixed() {
        assert_eq!(remove_whitespace("1 2 3"), "123");
    }

    #[test]
    fn remove_whitespace_tabs_and_newlines() {
        assert_eq!(remove_whitespace("a\tb\nc"), "abc");
    }

    #[test]
    fn remove_whitespace_no_whitespace() {
        assert_eq!(remove_whitespace("abc123"), "abc123");
    }

    #[test]
    fn remove_whitespace_carriage_return() {
        assert_eq!(remove_whitespace("a\r\nb"), "ab");
    }

    #[test]
    fn remove_whitespace_unicode_spaces() {
        // \u{00A0} = non-breaking space, \u{2003} = em space
        assert_eq!(remove_whitespace("1\u{00A0}2\u{2003}3"), "123");
    }

    // ── collection_at ───────────────────────────────────────

    #[test]
    fn collection_at_positive_in_bounds() {
        let items = make_items(4);
        for i in 0..4 {
            assert!(collection_at(&items, i).is_some());
        }
    }

    #[test]
    fn collection_at_positive_out_of_bounds() {
        let items = make_items(3);
        assert!(collection_at(&items, 3).is_none());
        assert!(collection_at(&items, 100).is_none());
    }

    #[test]
    fn collection_at_negative_wraps_from_end() {
        let items = make_items(4);
        // -1 => last, -4 => first
        assert!(collection_at(&items, -1).is_some());
        assert!(collection_at(&items, -4).is_some());
    }

    #[test]
    fn collection_at_negative_out_of_bounds() {
        let items = make_items(3);
        assert!(collection_at(&items, -4).is_none());
        assert!(collection_at(&items, -100).is_none());
    }

    #[test]
    fn collection_at_empty() {
        let items: Vec<CollectionItemValue<ItemData>> = vec![];
        assert!(collection_at(&items, 0).is_none());
        assert!(collection_at(&items, -1).is_none());
    }

    #[test]
    fn collection_at_single_item() {
        let items = make_items(1);
        assert!(collection_at(&items, 0).is_some());
        assert!(collection_at(&items, -1).is_some());
        assert!(collection_at(&items, 1).is_none());
        assert!(collection_at(&items, -2).is_none());
    }

    #[test]
    fn collection_at_negative_maps_to_correct_positive() {
        let items = make_items(6);
        for i in 0..6 {
            let positive = collection_at(&items, i as isize).unwrap();
            let negative = collection_at(&items, i as isize - 6).unwrap();
            assert!(std::ptr::eq(positive, negative));
        }
    }

    // ── InputType / AutoComplete as_str ─────────────────────

    #[test]
    fn input_type_as_str() {
        assert_eq!(InputType::Password.as_str(), "password");
        assert_eq!(InputType::Text.as_str(), "text");
    }

    #[test]
    fn autocomplete_as_str() {
        assert_eq!(AutoComplete::Off.as_str(), "off");
        assert_eq!(AutoComplete::OneTimeCode.as_str(), "one-time-code");
    }

    // ── Defaults ────────────────────────────────────────────

    #[test]
    fn input_type_default_is_text() {
        assert_eq!(InputType::default(), InputType::Text);
    }

    #[test]
    fn autocomplete_default_is_one_time_code() {
        assert_eq!(AutoComplete::default(), AutoComplete::OneTimeCode);
    }

    #[test]
    fn input_validation_type_default_is_numeric() {
        assert_eq!(InputValidationType::default(), InputValidationType::Numeric);
    }
}
