use std::marker::PhantomData;

use crate::support::collection::{
    CollectionItemSlot, CollectionItemValue, CollectionProvider, CollectionSlot, use_collection,
};
use crate::support::compose_refs::use_composed_refs;
use crate::support::dismissable_layer::{DismissableLayer, DismissableLayerBranch};
use crate::support::forwarded_attrs::ForwardedAttrs;
use crate::support::portal::Portal;
use crate::support::presence::Presence;
use crate::support::primitive::{Primitive, adapt_callback};
use crate::support::use_controllable_state::{UseControllableStateParams, use_controllable_state};
use crate::support::visually_hidden::VisuallyHidden;
use leptos::attribute_interceptor::AttributeInterceptor;
use leptos::{context::Provider, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::closure::Closure;

mod toast;
mod toast_provider;
mod toast_viewport;

pub use toast::*;
pub use toast_provider::*;
pub use toast_viewport::*;

/* -------------------------------------------------------------------------------------------------
 * Constants
 * -----------------------------------------------------------------------------------------------*/

pub(super) const VIEWPORT_DEFAULT_HOTKEY: &[&str] = &["F8"];
pub(super) const VIEWPORT_PAUSE: &str = "toast.viewportPause";
pub(super) const VIEWPORT_RESUME: &str = "toast.viewportResume";

pub(super) const ITEM_DATA_PHANTOM: PhantomData<()> = PhantomData;

pub(super) type GetCollectionItems =
    StoredValue<SendWrapper<Box<dyn Fn() -> Vec<CollectionItemValue<()>>>>>;

/* -------------------------------------------------------------------------------------------------
 * Types
 * -----------------------------------------------------------------------------------------------*/

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SwipeDirection {
    Up,
    Down,
    Left,
    #[default]
    Right,
}

impl SwipeDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            SwipeDirection::Up => "up",
            SwipeDirection::Down => "down",
            SwipeDirection::Left => "left",
            SwipeDirection::Right => "right",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ToastType {
    #[default]
    Foreground,
    Background,
}

#[derive(Clone, Debug)]
pub struct SwipeEvent {
    pub current_target: Option<SwipeEventTarget>,
    pub delta: (f64, f64),
}

#[derive(Clone, Debug)]
pub struct SwipeEventTarget(SendWrapper<web_sys::HtmlElement>);

impl SwipeEventTarget {
    pub(super) fn new(el: web_sys::HtmlElement) -> Self {
        Self(SendWrapper::new(el))
    }

    pub(super) fn set_attribute(
        &self,
        name: &str,
        value: &str,
    ) -> Result<(), wasm_bindgen::JsValue> {
        self.0.set_attribute(name, value)
    }

    pub(super) fn style(&self) -> web_sys::CssStyleDeclaration {
        self.0.style()
    }
}

/* -------------------------------------------------------------------------------------------------
 * ToastProviderContextValue
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy)]
pub(super) struct ToastProviderContextValue {
    pub(super) label: StoredValue<String>,
    pub(super) duration: Signal<i32>,
    pub(super) swipe_direction: Signal<SwipeDirection>,
    pub(super) swipe_threshold: Signal<f64>,
    pub(super) toast_count: ReadSignal<i32>,
    pub(super) set_toast_count: WriteSignal<i32>,
    pub(super) viewport: ReadSignal<Option<SendWrapper<web_sys::HtmlElement>>>,
    pub(super) on_viewport_change: WriteSignal<Option<SendWrapper<web_sys::HtmlElement>>>,
    pub(super) is_focused_toast_escape_key_down_ref: StoredValue<bool>,
    pub(super) is_close_paused_ref: StoredValue<bool>,
}

/* -------------------------------------------------------------------------------------------------
 * Utility functions
 * -----------------------------------------------------------------------------------------------*/

pub(super) fn get_announce_text_content(container: &web_sys::HtmlElement) -> Vec<String> {
    let mut text_content = Vec::new();
    let child_nodes = container.child_nodes();

    for i in 0..child_nodes.length() {
        if let Some(node) = child_nodes.item(i) {
            if node.node_type() == web_sys::Node::TEXT_NODE {
                if let Some(text) = node.text_content()
                    && !text.is_empty()
                {
                    text_content.push(text);
                }
            } else if let Ok(element) = node.dyn_into::<web_sys::HtmlElement>() {
                let is_hidden = element
                    .get_attribute("aria-hidden")
                    .is_some_and(|v| v == "true")
                    || element.hidden()
                    || element
                        .style()
                        .get_property_value("display")
                        .ok()
                        .is_some_and(|v| v == "none");

                let is_excluded = element
                    .get_attribute("data-radix-toast-announce-exclude")
                    .is_some();

                if !is_hidden {
                    if is_excluded {
                        if let Some(alt_text) =
                            element.get_attribute("data-radix-toast-announce-alt")
                            && !alt_text.is_empty()
                        {
                            text_content.push(alt_text);
                        }
                    } else {
                        text_content.extend(get_announce_text_content(&element));
                    }
                }
            }
        }
    }

    text_content
}

pub(super) fn compute_sorted_tabbable(
    get_items: GetCollectionItems,
    tabbing_direction: &str,
) -> Vec<SendWrapper<web_sys::HtmlElement>> {
    let toast_items = get_items.with_value(|f| f());
    let tabbable_candidates: Vec<Vec<SendWrapper<web_sys::HtmlElement>>> = toast_items
        .iter()
        .map(|toast_item| {
            let Some(node) = toast_item.r#ref.get_untracked() else {
                return vec![];
            };
            let toast_node: web_sys::HtmlElement = (*node).clone().unchecked_into();

            let mut candidates = vec![SendWrapper::new(toast_node.clone())];
            candidates.extend(
                get_tabbable_candidates(&toast_node)
                    .into_iter()
                    .map(SendWrapper::new),
            );

            if tabbing_direction == "backwards" {
                candidates.reverse();
            }
            candidates
        })
        .collect();

    let ordered: Vec<Vec<SendWrapper<web_sys::HtmlElement>>> = if tabbing_direction == "forwards" {
        tabbable_candidates.into_iter().rev().collect()
    } else {
        tabbable_candidates
    };

    ordered.into_iter().flatten().collect()
}

pub(super) fn is_delta_in_direction(
    delta: (f64, f64),
    direction: SwipeDirection,
    threshold: f64,
) -> bool {
    let delta_x = delta.0.abs();
    let delta_y = delta.1.abs();
    let is_delta_x = delta_x > delta_y;
    match direction {
        SwipeDirection::Left | SwipeDirection::Right => is_delta_x && delta_x > threshold,
        SwipeDirection::Up | SwipeDirection::Down => !is_delta_x && delta_y > threshold,
    }
}

/// Returns a list of potential tabbable candidates.
fn get_tabbable_candidates(container: &web_sys::HtmlElement) -> Vec<web_sys::HtmlElement> {
    let mut nodes = Vec::new();

    let accept_node_closure: Closure<dyn Fn(web_sys::Node) -> u32> =
        Closure::new(move |node: web_sys::Node| -> u32 {
            if let Some(html_element) = node.dyn_ref::<web_sys::HtmlElement>() {
                if let Some(input) = node.dyn_ref::<web_sys::HtmlInputElement>()
                    && input.type_() == "hidden"
                {
                    return 3; // FILTER_SKIP
                }
                if html_element.hidden() {
                    return 3; // FILTER_SKIP
                }
                if html_element.tab_index() >= 0 {
                    return 1; // FILTER_ACCEPT
                }
            }
            3 // FILTER_SKIP
        });

    let node_filter = web_sys::NodeFilter::new();
    node_filter.set_accept_node(accept_node_closure.as_ref().unchecked_ref());

    if let Ok(walker) = document().create_tree_walker_with_what_to_show_and_filter(
        container,
        0x1,
        Some(&node_filter),
    ) {
        while let Ok(Some(node)) = walker.next_node() {
            if let Ok(element) = node.dyn_into::<web_sys::HtmlElement>() {
                nodes.push(element);
            }
        }
    }

    drop(accept_node_closure);
    nodes
}

pub(super) fn focus_first_html(candidates: &[SendWrapper<web_sys::HtmlElement>]) -> bool {
    let previously_focused = document().active_element();
    for candidate in candidates {
        let c: &web_sys::HtmlElement = candidate;
        if previously_focused
            .as_ref()
            .is_some_and(|f| c.unchecked_ref::<web_sys::Element>() == f)
        {
            return true;
        }
        let _ = c.focus();
        if document()
            .active_element()
            .is_some_and(|active| previously_focused.as_ref().is_none_or(|f| active != *f))
        {
            return true;
        }
    }
    false
}

pub(super) fn document() -> web_sys::Document {
    web_sys::window()
        .expect("Window should exist.")
        .document()
        .expect("Document should exist.")
}

pub(super) fn set_timeout(f: impl Fn() + 'static, delay: i32) -> i32 {
    let closure = Closure::once_into_js(f);
    web_sys::window()
        .expect("Window should exist.")
        .set_timeout_with_callback_and_timeout_and_arguments_0(closure.unchecked_ref(), delay)
        .expect("setTimeout should succeed.")
}

pub(super) fn clear_timeout(handle: StoredValue<Option<i32>>) {
    if let Some(id) = handle.get_value() {
        web_sys::window()
            .expect("Window should exist.")
            .clear_timeout_with_handle(id);
        handle.set_value(None);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── SwipeDirection ──────────────────────────────────────

    #[test]
    fn swipe_direction_as_str() {
        assert_eq!(SwipeDirection::Up.as_str(), "up");
        assert_eq!(SwipeDirection::Down.as_str(), "down");
        assert_eq!(SwipeDirection::Left.as_str(), "left");
        assert_eq!(SwipeDirection::Right.as_str(), "right");
    }

    #[test]
    fn swipe_direction_default_is_right() {
        assert_eq!(SwipeDirection::default(), SwipeDirection::Right);
    }

    // ── is_delta_in_direction ───────────────────────────────

    #[test]
    fn horizontal_swipe_right_above_threshold() {
        assert!(is_delta_in_direction(
            (20.0, 5.0),
            SwipeDirection::Right,
            10.0
        ));
    }

    #[test]
    fn horizontal_swipe_left_above_threshold() {
        // Negative x delta, but abs is used — direction variant only checks axis
        assert!(is_delta_in_direction(
            (-20.0, 5.0),
            SwipeDirection::Left,
            10.0
        ));
    }

    #[test]
    fn horizontal_swipe_below_threshold() {
        assert!(!is_delta_in_direction(
            (5.0, 2.0),
            SwipeDirection::Right,
            10.0
        ));
    }

    #[test]
    fn horizontal_swipe_at_threshold_not_exceeded() {
        // delta_x == threshold, but we need > threshold
        assert!(!is_delta_in_direction(
            (10.0, 2.0),
            SwipeDirection::Right,
            10.0
        ));
    }

    #[test]
    fn vertical_swipe_down_above_threshold() {
        assert!(is_delta_in_direction(
            (5.0, 20.0),
            SwipeDirection::Down,
            10.0
        ));
    }

    #[test]
    fn vertical_swipe_up_above_threshold() {
        assert!(is_delta_in_direction(
            (5.0, -20.0),
            SwipeDirection::Up,
            10.0
        ));
    }

    #[test]
    fn vertical_swipe_below_threshold() {
        assert!(!is_delta_in_direction((2.0, 5.0), SwipeDirection::Up, 10.0));
    }

    #[test]
    fn diagonal_equal_is_not_horizontal() {
        // delta_x == delta_y → is_delta_x is false → horizontal direction fails
        assert!(!is_delta_in_direction(
            (15.0, 15.0),
            SwipeDirection::Right,
            10.0
        ));
    }

    #[test]
    fn diagonal_equal_is_not_vertical() {
        // delta_x == delta_y → is_delta_x is false, but !is_delta_x is true
        // however delta_y must > threshold
        assert!(is_delta_in_direction(
            (15.0, 15.0),
            SwipeDirection::Down,
            10.0
        ));
    }

    #[test]
    fn wrong_axis_rejects() {
        // Mostly vertical movement, but asking about horizontal direction
        assert!(!is_delta_in_direction(
            (2.0, 50.0),
            SwipeDirection::Right,
            10.0
        ));
        // Mostly horizontal movement, but asking about vertical direction
        assert!(!is_delta_in_direction(
            (50.0, 2.0),
            SwipeDirection::Down,
            10.0
        ));
    }

    #[test]
    fn zero_delta() {
        assert!(!is_delta_in_direction(
            (0.0, 0.0),
            SwipeDirection::Right,
            0.0
        ));
        assert!(!is_delta_in_direction((0.0, 0.0), SwipeDirection::Up, 0.0));
    }
}
