use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};

use crate::support::aria_hidden::{hide_others, unhide_others};
use crate::support::collection::{
    CollectionItemSlot, CollectionItemValue, CollectionProvider, CollectionSlot,
    provide_collection_scope, use_collection, use_collection_scope,
};
use crate::support::compose_refs::use_composed_refs;
use crate::support::direction::{Direction, use_direction};
use crate::support::dismissable_layer::DismissableLayer;
use crate::support::focus_guards::use_focus_guards;
use crate::support::focus_scope::FocusScope;
use crate::support::id::use_id;
use crate::support::popper::{
    Align, Padding, Popper, PopperAnchor, PopperArrow, PopperContent, Side, Sticky,
    UpdatePositionStrategy, provide_popper_scope, use_popper_scope,
};
use crate::support::portal::ScopedPortal;
use crate::support::primitive::{Primitive, adapt_callback, compose_callbacks, data_attr, prop_or_default};
use crate::support::use_controllable_state::{UseControllableStateParams, use_controllable_state};
use crate::support::use_internal_styles::{use_forced_styles, use_internal_styles};
use leptos::{
    attribute_interceptor::AttributeInterceptor, context::Provider, ev, html, prelude::*,
};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::closure::Closure;

mod select;
mod select_content;
mod select_item;
mod select_portal;
mod select_scroll_button;
mod select_separator;
mod select_value;
mod select_viewport;

pub use select::*;
pub use select_content::*;
pub use select_item::*;
pub use select_portal::*;
pub use select_scroll_button::*;
pub use select_separator::*;
pub use select_value::*;
pub use select_viewport::*;

/* -------------------------------------------------------------------------------------------------
 * Constants
 * -----------------------------------------------------------------------------------------------*/

const OPEN_KEYS: &[&str] = &[" ", "Enter", "ArrowUp", "ArrowDown"];
const SELECTION_KEYS: &[&str] = &[" ", "Enter"];

/* -------------------------------------------------------------------------------------------------
 * Collection ItemData
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Debug)]
pub struct SelectItemData {
    pub value: String,
    pub disabled: bool,
    pub text_value: String,
}

const ITEM_DATA_PHANTOM: PhantomData<SelectItemData> = PhantomData;

/* -------------------------------------------------------------------------------------------------
 * Contexts
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy)]
struct SelectContextValue {
    trigger_ref: AnyNodeRef,
    value_node_ref: AnyNodeRef,
    value_node_has_children: ReadSignal<bool>,
    content_id: ReadSignal<String>,
    value: Signal<Option<String>>,
    on_value_change: Callback<String>,
    open: Signal<bool>,
    required: Signal<bool>,
    on_open_change: Callback<bool>,
    dir: Signal<Direction>,
    trigger_pointer_down_pos_ref: StoredValue<Option<(f64, f64)>>,
    disabled: Signal<bool>,
}

#[derive(Clone, Copy)]
struct SelectContentContextValue {
    #[allow(dead_code)]
    content_ref: AnyNodeRef,
    viewport_ref: AnyNodeRef,
    on_item_leave: Callback<()>,
    position: StoredValue<String>,
    is_positioned: ReadSignal<bool>,
    search_ref: StoredValue<String>,
}

#[derive(Clone, Copy)]
struct SelectViewportContextValue {
    #[allow(dead_code)]
    content_wrapper_ref: AnyNodeRef,
}

#[derive(Clone)]
struct SelectItemContextValue {
    #[allow(dead_code)]
    value: String,
    #[allow(dead_code)]
    disabled: bool,
    text_id: ReadSignal<String>,
    is_selected: Signal<bool>,
    on_item_text_change: Callback<Option<SendWrapper<web_sys::HtmlElement>>>,
}

#[derive(Clone, Copy)]
struct SelectGroupContextValue {
    id: ReadSignal<String>,
}

/* -------------------------------------------------------------------------------------------------
 * Utilities
 * -----------------------------------------------------------------------------------------------*/

fn should_show_placeholder(value: &Option<String>) -> bool {
    match value {
        None => true,
        Some(v) => v.is_empty(),
    }
}

fn parse_px_value(css_value: &str) -> f64 {
    css_value.replace("px", "").parse::<f64>().unwrap_or(0.0)
}

fn is_scrollable_down(scroll_top: i32, scroll_height: i32, client_height: i32) -> bool {
    (scroll_top as f64).ceil() < (scroll_height - client_height) as f64
}

/// Typeahead search hook that returns (search_ref, handle_typeahead_search, reset_typeahead)
fn use_typeahead_search(
    on_search_change: Callback<String>,
) -> (StoredValue<String>, Callback<String>, Callback<()>) {
    let search_ref: StoredValue<String> = StoredValue::new(String::new());
    let timer_ref: StoredValue<Option<i32>> = StoredValue::new(None);

    let handle_typeahead_search = Callback::new(move |key: String| {
        let current = search_ref.try_get_value().unwrap_or_default();
        let search = format!("{}{}", current, key);
        on_search_change.run(search.clone());

        let _ = search_ref.try_set_value(search.clone());
        if let Some(timer) = timer_ref.try_get_value().flatten() {
            web_sys::window()
                .expect("Window should exist.")
                .clear_timeout_with_handle(timer);
        }
        if !search.is_empty() {
            let timer = web_sys::window()
                .expect("Window should exist.")
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    Closure::once_into_js(move || {
                        let _ = search_ref.try_set_value(String::new());
                    })
                    .unchecked_ref(),
                    1000,
                )
                .expect("setTimeout should succeed.");
            let _ = timer_ref.try_set_value(Some(timer));
        }
    });

    let reset_typeahead = Callback::new(move |_: ()| {
        let _ = search_ref.try_set_value(String::new());
        if let Some(timer) = timer_ref.try_get_value().flatten() {
            web_sys::window()
                .expect("Window should exist.")
                .clear_timeout_with_handle(timer);
        }
    });

    on_cleanup(move || {
        if let Some(timer) = timer_ref.try_get_value().flatten()
            && let Some(window) = web_sys::window()
        {
            window.clear_timeout_with_handle(timer);
        }
    });

    (search_ref, handle_typeahead_search, reset_typeahead)
}

/// Variant without reset (for content-level typeahead)
fn use_typeahead_search_no_reset(
    on_search_change: Callback<String>,
) -> (StoredValue<String>, Callback<String>) {
    let (search_ref, handle, _) = use_typeahead_search(on_search_change);
    (search_ref, handle)
}

/// Find the next item matching the typeahead search
fn find_next_item<'a>(
    items: &'a [&'a CollectionItemValue<SelectItemData>],
    search: &str,
    current_item: Option<&'a CollectionItemValue<SelectItemData>>,
) -> Option<&'a CollectionItemValue<SelectItemData>> {
    if search.is_empty() {
        return None;
    }

    // Normalize repeated characters (e.g., "aaa" → "a")
    let is_repeated =
        search.len() > 1 && search.chars().all(|c| c == search.chars().next().unwrap());
    let normalized_search = if is_repeated {
        search.chars().next().unwrap().to_string()
    } else {
        search.to_string()
    };

    let current_index = current_item
        .and_then(|current| items.iter().position(|item| std::ptr::eq(*item, current)))
        .unwrap_or(0);

    // Wrap array starting from current position
    let mut wrapped_items: Vec<&CollectionItemValue<SelectItemData>> = items.to_vec();
    let start = current_index.min(wrapped_items.len());
    let (a, b) = wrapped_items.split_at(start);
    wrapped_items = [b, a].concat();

    // For single character search, exclude current item to enable cycling
    let exclude_current = normalized_search.len() == 1;
    if exclude_current && let Some(current) = current_item {
        wrapped_items.retain(|item| !std::ptr::eq(*item, current));
    }

    let normalized_lower = normalized_search.to_lowercase();
    let next = wrapped_items.iter().find(|item| {
        item.data
            .text_value
            .to_lowercase()
            .starts_with(&normalized_lower)
    });

    // Don't return current item if it's the same
    next.and_then(|item| {
        if current_item.is_some_and(|current| std::ptr::eq(*item, current)) {
            None
        } else {
            Some(*item)
        }
    })
}


/// Margin around the select content for item-aligned positioning.
const CONTENT_MARGIN: f64 = 10.0;

/// Position the select content so the selected item aligns with the trigger.
/// This is a port of React's `SelectItemAlignedPosition` positioning logic.
#[allow(clippy::too_many_arguments)]
fn position_item_aligned(
    wrapper: &web_sys::HtmlElement,
    content: &web_sys::HtmlElement,
    trigger: &web_sys::HtmlElement,
    value_node: &web_sys::HtmlElement,
    viewport: &web_sys::HtmlElement,
    selected_item: &web_sys::HtmlElement,
    dir: Direction,
    is_first_item: bool,
    is_last_item: bool,
) {
    let window = web_sys::window().expect("Window should exist.");
    let inner_width = window.inner_width().unwrap().as_f64().unwrap();
    let inner_height = window.inner_height().unwrap().as_f64().unwrap();

    let trigger_rect = trigger.get_bounding_client_rect();
    let content_rect = content.get_bounding_client_rect();
    let value_node_rect = value_node.get_bounding_client_rect();

    // Find the text span within the selected item for horizontal alignment
    let item_text_rect = selected_item
        .query_selector("span[id]")
        .ok()
        .flatten()
        .map(|el| el.get_bounding_client_rect())
        .unwrap_or_else(|| selected_item.get_bounding_client_rect());

    let wrapper_style = wrapper.style();

    // ── Horizontal positioning ──────────────────────────────
    if dir != Direction::Rtl {
        let item_text_offset = item_text_rect.left() - content_rect.left();
        let left = value_node_rect.left() - item_text_offset;
        let left_delta = trigger_rect.left() - left;
        let min_content_width = trigger_rect.width() + left_delta;
        let content_width = min_content_width.max(content_rect.width());
        let right_edge = inner_width - CONTENT_MARGIN;
        let clamped_left = left
            .max(CONTENT_MARGIN)
            .min((right_edge - content_width).max(CONTENT_MARGIN));

        let _ = wrapper_style.set_property("min-width", &format!("{}px", min_content_width));
        let _ = wrapper_style.set_property("left", &format!("{}px", clamped_left));
    } else {
        let item_text_offset = content_rect.right() - item_text_rect.right();
        let right = inner_width - value_node_rect.right() - item_text_offset;
        let right_delta = inner_width - trigger_rect.right() - right;
        let min_content_width = trigger_rect.width() + right_delta;
        let content_width = min_content_width.max(content_rect.width());
        let left_edge = inner_width - CONTENT_MARGIN;
        let clamped_right = right
            .max(CONTENT_MARGIN)
            .min((left_edge - content_width).max(CONTENT_MARGIN));

        let _ = wrapper_style.set_property("min-width", &format!("{}px", min_content_width));
        let _ = wrapper_style.set_property("right", &format!("{}px", clamped_right));
    }

    // ── Vertical positioning ────────────────────────────────
    let available_height = inner_height - CONTENT_MARGIN * 2.0;
    let items_height = viewport.scroll_height() as f64;

    let Some(content_styles) = window.get_computed_style(content).ok().flatten() else {
        return;
    };
    let parse_px = |prop: &str| -> f64 {
        parse_px_value(&content_styles.get_property_value(prop).unwrap_or_default())
    };
    let content_border_top = parse_px("border-top-width");
    let content_padding_top = parse_px("padding-top");
    let content_border_bottom = parse_px("border-bottom-width");
    let content_padding_bottom = parse_px("padding-bottom");

    let full_content_height = content_border_top
        + content_padding_top
        + items_height
        + content_padding_bottom
        + content_border_bottom;
    let min_content_height = (selected_item.offset_height() as f64 * 5.0).min(full_content_height);

    let Some(viewport_styles) = window.get_computed_style(viewport).ok().flatten() else {
        return;
    };
    let viewport_padding_top = parse_px_value(
        &viewport_styles
            .get_property_value("padding-top")
            .unwrap_or_default(),
    );
    let viewport_padding_bottom = parse_px_value(
        &viewport_styles
            .get_property_value("padding-bottom")
            .unwrap_or_default(),
    );

    let top_edge_to_trigger_middle =
        trigger_rect.top() + trigger_rect.height() / 2.0 - CONTENT_MARGIN;
    let trigger_middle_to_bottom_edge = available_height - top_edge_to_trigger_middle;

    let selected_item_half_height = selected_item.offset_height() as f64 / 2.0;
    let item_offset_middle = selected_item.offset_top() as f64 + selected_item_half_height;
    let content_top_to_item_middle = content_border_top + content_padding_top + item_offset_middle;
    let item_middle_to_content_bottom = full_content_height - content_top_to_item_middle;

    let will_align_without_top_overflow = content_top_to_item_middle <= top_edge_to_trigger_middle;

    if will_align_without_top_overflow {
        let _ = wrapper_style.set_property("bottom", "0px");
        let viewport_offset_bottom = content.client_height() as f64
            - viewport.offset_top() as f64
            - viewport.offset_height() as f64;
        let clamped_trigger_middle_to_bottom_edge = trigger_middle_to_bottom_edge.max(
            selected_item_half_height
                + if is_last_item {
                    viewport_padding_bottom
                } else {
                    0.0
                }
                + viewport_offset_bottom
                + content_border_bottom,
        );
        let height = content_top_to_item_middle + clamped_trigger_middle_to_bottom_edge;
        let _ = wrapper_style.set_property("height", &format!("{}px", height));
    } else {
        let _ = wrapper_style.set_property("top", "0px");
        let clamped_top_edge_to_trigger_middle = top_edge_to_trigger_middle.max(
            content_border_top
                + viewport.offset_top() as f64
                + if is_first_item {
                    viewport_padding_top
                } else {
                    0.0
                }
                + selected_item_half_height,
        );
        let height = clamped_top_edge_to_trigger_middle + item_middle_to_content_bottom;
        let _ = wrapper_style.set_property("height", &format!("{}px", height));
        viewport.set_scroll_top(
            (content_top_to_item_middle - top_edge_to_trigger_middle + viewport.offset_top() as f64)
                as i32,
        );
    }

    let _ = wrapper_style.set_property("margin", &format!("{}px 0", CONTENT_MARGIN));
    let _ = wrapper_style.set_property("min-height", &format!("{}px", min_content_height));
    let _ = wrapper_style.set_property("max-height", &format!("{}px", available_height));
}

/// Visually hidden styles for the bubble select element
const VISUALLY_HIDDEN_STYLES_STR: &str = "position: absolute; border: 0; width: 1px; height: 1px; padding: 0; margin: -1px; overflow: hidden; clip: rect(0, 0, 0, 0); white-space: nowrap; word-wrap: normal;";

#[cfg(test)]
mod tests {
    use super::*;

    // ── should_show_placeholder ─────────────────────────────

    #[test]
    fn placeholder_shown_for_none() {
        assert!(should_show_placeholder(&None));
    }

    #[test]
    fn placeholder_shown_for_empty_string() {
        assert!(should_show_placeholder(&Some(String::new())));
    }

    #[test]
    fn placeholder_hidden_for_value() {
        assert!(!should_show_placeholder(&Some("apple".into())));
    }

    // ── parse_px_value ──────────────────────────────────────

    #[test]
    fn parse_px_normal() {
        assert!((parse_px_value("10px") - 10.0).abs() < f64::EPSILON);
    }

    #[test]
    fn parse_px_float() {
        assert!((parse_px_value("3.5px") - 3.5).abs() < f64::EPSILON);
    }

    #[test]
    fn parse_px_empty() {
        assert!((parse_px_value("") - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn parse_px_no_unit() {
        assert!((parse_px_value("42") - 42.0).abs() < f64::EPSILON);
    }

    #[test]
    fn parse_px_garbage() {
        assert!((parse_px_value("auto") - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn parse_px_zero() {
        assert!((parse_px_value("0px") - 0.0).abs() < f64::EPSILON);
    }

    // ── is_scrollable_down ────────────────────────────────────

    #[test]
    fn can_scroll_when_not_at_bottom() {
        assert!(is_scrollable_down(0, 500, 200));
    }

    #[test]
    fn cannot_scroll_when_at_bottom() {
        assert!(!is_scrollable_down(300, 500, 200));
    }

    #[test]
    fn cannot_scroll_when_content_fits() {
        assert!(!is_scrollable_down(0, 200, 200));
    }

    #[test]
    fn can_scroll_just_below_bottom() {
        // scroll_top=299, max_scroll=300 → ceil(299)=299 < 300
        assert!(is_scrollable_down(299, 500, 200));
    }

    #[test]
    fn cannot_scroll_at_exact_bottom() {
        // scroll_top=300, max_scroll=300 → ceil(300)=300 < 300 is false
        assert!(!is_scrollable_down(300, 500, 200));
    }
}
