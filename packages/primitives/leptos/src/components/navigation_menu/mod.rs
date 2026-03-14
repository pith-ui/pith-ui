use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

use std::sync::Arc;

use crate::support::collection::{
    CollectionItemSlot, CollectionProvider, CollectionSlot, provide_collection_scope,
    use_collection, use_collection_scope,
};
use crate::support::compose_refs::use_composed_refs;
use crate::support::direction::{Direction, use_direction};
use crate::support::dismissable_layer::DismissableLayer;
use crate::support::id::use_id;
use crate::support::presence::Presence;
use crate::support::primitive::{Primitive, adapt_callback, compose_callbacks, open_closed_state, prop_or};
use crate::support::use_controllable_state::{UseControllableStateParams, use_controllable_state};
use crate::support::use_previous::use_previous;
use crate::support::visually_hidden::VisuallyHidden;
use leptos::{
    attr::Attribute as _, attribute_interceptor::AttributeInterceptor, context::Provider, ev, html,
    prelude::*,
};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::closure::Closure;

/* -------------------------------------------------------------------------------------------------
 * Sub-modules
 * -----------------------------------------------------------------------------------------------*/

mod navigation_menu;
mod navigation_menu_content;
mod navigation_menu_indicator;
mod navigation_menu_item;
mod navigation_menu_list;
mod navigation_menu_trigger;
mod navigation_menu_viewport;

pub use self::navigation_menu::*;
pub use navigation_menu_content::*;
pub use navigation_menu_indicator::*;
pub use navigation_menu_item::*;
pub use navigation_menu_list::*;
pub use navigation_menu_trigger::*;
pub use navigation_menu_viewport::*;

// Re-export pub(super) items so sibling sub-modules can access them via `use super::*;`
use navigation_menu_content::NavigationMenuContentImpl;
use navigation_menu_list::NavigationMenuProvider;
use navigation_menu_viewport::FocusGroup;
use navigation_menu_viewport::FocusGroupItem;

/* -------------------------------------------------------------------------------------------------
 * Types and constants
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Orientation {
    #[default]
    Horizontal,
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

const LINK_SELECT: &str = "navigationMenu.linkSelect";
const ROOT_CONTENT_DISMISS: &str = "navigationMenu.rootContentDismiss";

const ARROW_KEYS: &[&str] = &["ArrowRight", "ArrowLeft", "ArrowUp", "ArrowDown"];

/* -------------------------------------------------------------------------------------------------
 * Collection item data
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Debug)]
pub struct NavigationMenuItemData {
    pub value: String,
}

/// Empty item data for the FocusGroup collection.
#[derive(Clone, Debug)]
struct FocusGroupItemData;

/* -------------------------------------------------------------------------------------------------
 * Context types
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone)]
struct NavigationMenuContextValue {
    is_root_menu: bool,
    value: Signal<String>,
    previous_value: Memo<String>,
    base_id: ReadSignal<String>,
    dir: Signal<Direction>,
    orientation: Signal<Orientation>,
    root_navigation_menu: AnyNodeRef,
    indicator_track: RwSignal<Option<SendWrapper<web_sys::HtmlElement>>>,
    viewport: RwSignal<Option<SendWrapper<web_sys::HtmlElement>>>,
    /// Set synchronously during NavigationMenuViewport construction (not via Effect),
    /// so Content components know a viewport exists before any menu open interaction.
    has_viewport_component: RwSignal<bool>,
    on_trigger_enter: Callback<String>,
    on_trigger_leave: Callback<()>,
    on_content_enter: Callback<()>,
    on_content_leave: Callback<()>,
    on_item_select: Callback<String>,
    on_item_dismiss: Callback<()>,
    on_viewport_content_change: Callback<(String, ContentData)>,
    on_viewport_content_remove: Callback<String>,
}

#[derive(Clone)]
struct NavigationMenuItemContextValue {
    value: String,
    trigger_ref: AnyNodeRef,
    content_ref: AnyNodeRef,
    focus_proxy_ref: AnyNodeRef,
    was_escape_close_ref: RwSignal<bool>,
    on_entry_key_down: Callback<()>,
    on_focus_proxy_enter: Callback<&'static str>,
    on_root_content_close: Callback<()>,
    on_content_focus_outside: Callback<()>,
}

#[derive(Clone)]
struct ViewportContentContextValue {
    items: RwSignal<HashMap<String, ContentData>>,
}

#[derive(Clone)]
pub struct ContentData {
    pub value: String,
    pub trigger_ref: AnyNodeRef,
    pub focus_proxy_ref: AnyNodeRef,
    pub was_escape_close_ref: RwSignal<bool>,
    pub on_content_focus_outside: Callback<()>,
    pub on_root_content_close: Callback<()>,
    pub force_mount: bool,
    pub children: Option<ChildrenFn>,
    pub on_pointer_enter: Option<Callback<ev::PointerEvent>>,
    pub on_pointer_leave: Option<Callback<ev::PointerEvent>>,
    pub on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    pub on_focus_outside: Option<Callback<web_sys::CustomEvent>>,
    pub on_pointer_down_outside: Option<Callback<web_sys::CustomEvent>>,
    pub on_interact_outside: Option<Callback<web_sys::CustomEvent>>,
    pub content_ref: AnyNodeRef,
    /// The NavigationMenuItem's internal content ref, used by handle_content_entry to focus
    /// into content on ArrowDown. Must be included in the viewport's ref chain so it gets
    /// set when the content element mounts.
    pub item_content_ref: AnyNodeRef,
    /// User attributes (e.g., data-testid, class) captured from the component and forwarded
    /// to the viewport rendering path. In React, these are spread via {...contentProps}; in
    /// Leptos, we capture them via AttributeInterceptor and transfer them explicitly.
    pub extra_attrs: Vec<(String, String)>,
}

/* -------------------------------------------------------------------------------------------------
 * Helper functions
 * -----------------------------------------------------------------------------------------------*/

fn compute_motion_attribute(
    values: &[String],
    current_value: &str,
    previous_value: &str,
    this_value: &str,
    prev_attribute: Option<&'static str>,
) -> Option<&'static str> {
    let index = values.iter().position(|v| v == current_value);
    let prev_index = values.iter().position(|v| v == previous_value);
    let is_selected = this_value == current_value;
    let was_selected = prev_index == values.iter().position(|v| v == this_value);

    if !is_selected && !was_selected {
        return prev_attribute;
    }

    if let (Some(idx), Some(prev_idx)) = (index, prev_index) {
        if idx != prev_idx {
            if is_selected && prev_idx != usize::MAX {
                Some(if idx > prev_idx {
                    "from-end"
                } else {
                    "from-start"
                })
            } else if was_selected {
                Some(if idx > prev_idx { "to-start" } else { "to-end" })
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn make_trigger_id(base_id: &str, value: &str) -> String {
    format!("{base_id}-trigger-{value}")
}

fn make_content_id(base_id: &str, value: &str) -> String {
    format!("{base_id}-content-{value}")
}

fn document() -> web_sys::Document {
    web_sys::window()
        .expect("Window should exist.")
        .document()
        .expect("Document should exist.")
}

fn set_timeout(f: impl FnOnce() + 'static, delay: i32) -> i32 {
    let closure = Closure::once_into_js(f);
    web_sys::window()
        .expect("Window should exist.")
        .set_timeout_with_callback_and_timeout_and_arguments_0(closure.unchecked_ref(), delay)
        .expect("setTimeout should succeed.")
}

fn clear_timeout(handle: StoredValue<Option<i32>>) {
    if let Some(id) = handle.get_value() {
        web_sys::window()
            .expect("Window should exist.")
            .clear_timeout_with_handle(id);
        handle.set_value(None);
    }
}

/// Returns a list of potential tabbable candidates.
fn get_tabbable_candidates(container: &web_sys::HtmlElement) -> Vec<web_sys::HtmlElement> {
    let mut nodes = Vec::new();

    let accept_node_closure: Closure<dyn Fn(web_sys::Node) -> u32> =
        Closure::new(move |node: web_sys::Node| -> u32 {
            if let Some(html_el) = node.dyn_ref::<web_sys::HtmlElement>() {
                // Check for hidden input
                if let Some(input) = node.dyn_ref::<web_sys::HtmlInputElement>()
                    && input.type_() == "hidden"
                {
                    return 3; // FILTER_SKIP
                }
                if html_el.hidden() {
                    return 3; // FILTER_SKIP
                }
                if html_el.tab_index() >= 0 { 1 } else { 3 } // FILTER_ACCEPT / FILTER_SKIP
            } else {
                3 // FILTER_SKIP
            }
        });

    let node_filter = web_sys::NodeFilter::new();
    node_filter.set_accept_node(accept_node_closure.as_ref().unchecked_ref());

    let walker = document()
        .create_tree_walker_with_what_to_show_and_filter(container, 0x1, Some(&node_filter))
        .expect("Tree walker should be created.");

    while let Some(node) = walker
        .next_node()
        .expect("Tree walker should return a next node.")
    {
        if let Ok(element) = node.dyn_into::<web_sys::HtmlElement>() {
            nodes.push(element);
        }
    }

    nodes
}

fn focus_first(candidates: &[web_sys::HtmlElement]) -> bool {
    let previously_focused = document().active_element();
    candidates.iter().any(|candidate| {
        if previously_focused
            .as_ref()
            .is_some_and(|el| el == candidate.unchecked_ref::<web_sys::Element>())
        {
            return true;
        }
        candidate.focus().ok();
        document().active_element().as_ref() != previously_focused.as_ref()
    })
}

fn remove_from_tab_order(
    candidates: &[web_sys::HtmlElement],
) -> Vec<(web_sys::HtmlElement, String)> {
    candidates
        .iter()
        .map(|candidate| {
            let prev = candidate.get_attribute("tabindex").unwrap_or_default();
            candidate
                .set_attribute("tabindex", "-1")
                .expect("Attribute should be set.");
            (candidate.clone(), prev)
        })
        .collect()
}

fn restore_tab_order(saved: &[(web_sys::HtmlElement, String)]) {
    for (el, prev) in saved {
        el.set_attribute("tabindex", prev)
            .expect("Attribute should be set.");
    }
}

fn use_resize_observer(
    element: Signal<Option<SendWrapper<web_sys::HtmlElement>>>,
    on_resize: Callback<()>,
) {
    #[allow(clippy::type_complexity)]
    let observer: StoredValue<Option<SendWrapper<web_sys::ResizeObserver>>> =
        StoredValue::new(None);
    let raf_id: StoredValue<Option<i32>> = StoredValue::new(None);

    Effect::new(move |_| {
        // Clean up previous observer
        observer.with_value(|obs| {
            if let Some(obs) = obs {
                obs.disconnect();
            }
        });
        if let Some(raf) = raf_id.get_value() {
            web_sys::window()
                .expect("Window should exist.")
                .cancel_animation_frame(raf)
                .ok();
        }

        if let Some(el) = element.get() {
            let closure = Closure::<dyn Fn(web_sys::js_sys::Array)>::new(
                move |_entries: web_sys::js_sys::Array| {
                    if let Some(raf) = raf_id.get_value() {
                        web_sys::window()
                            .expect("Window should exist.")
                            .cancel_animation_frame(raf)
                            .ok();
                    }
                    let id = web_sys::window()
                        .expect("Window should exist.")
                        .request_animation_frame(
                            Closure::once_into_js(move || {
                                on_resize.run(());
                            })
                            .unchecked_ref(),
                        )
                        .expect("rAF should succeed.");
                    raf_id.set_value(Some(id));
                },
            );
            let ro = web_sys::ResizeObserver::new(closure.as_ref().unchecked_ref())
                .expect("ResizeObserver should be created.");
            ro.observe(&el);
            // Leak the closure so it lives as long as the observer
            closure.forget();
            observer.set_value(Some(SendWrapper::new(ro)));
        }
    });

    Owner::on_cleanup(move || {
        observer.with_value(|obs| {
            if let Some(obs) = obs {
                obs.disconnect();
            }
        });
        if let Some(raf) = raf_id.get_value() {
            web_sys::window()
                .expect("Window should exist.")
                .cancel_animation_frame(raf)
                .ok();
        }
    });
}

/// Extracts `(name, value)` pairs from an `AnyAttribute` by building it on a temporary
/// detached DOM element. The temp element is never inserted into the document, so it cannot
/// be found by `querySelector` / `findByTestId`.
fn extract_attrs(attrs: leptos::attr::any_attribute::AnyAttribute) -> Vec<(String, String)> {
    let tmp = document()
        .create_element("div")
        .expect("Element should be created.");
    let _state = attrs.build(&tmp);
    let named = tmp.attributes();
    let mut pairs = vec![];
    for i in 0..named.length() {
        if let Some(attr) = named.item(i) {
            pairs.push((attr.name(), attr.value()));
        }
    }
    pairs
}

/* -------------------------------------------------------------------------------------------------
 * Tests
 * -----------------------------------------------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use super::*;

    fn vals(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    // ── open_closed_state ──────────────────────────────────────

    #[test]
    fn open_state_open() {
        assert_eq!(open_closed_state(true), "open");
    }

    #[test]
    fn open_state_closed() {
        assert_eq!(open_closed_state(false), "closed");
    }

    // ── make_trigger_id / make_content_id ───────────────────

    #[test]
    fn trigger_id_format() {
        assert_eq!(make_trigger_id("nav-1", "home"), "nav-1-trigger-home");
    }

    #[test]
    fn content_id_format() {
        assert_eq!(make_content_id("nav-1", "home"), "nav-1-content-home");
    }

    // ── compute_motion_attribute ────────────────────────────

    #[test]
    fn motion_from_end_when_selected_and_index_increased() {
        // items: [a, b, c], prev=a(0), current=c(2), this=c → is_selected, idx>prev_idx → from-end
        let items = vals(&["a", "b", "c"]);
        assert_eq!(
            compute_motion_attribute(&items, "c", "a", "c", None),
            Some("from-end")
        );
    }

    #[test]
    fn motion_from_start_when_selected_and_index_decreased() {
        // items: [a, b, c], prev=c(2), current=a(0), this=a → is_selected, idx<prev_idx → from-start
        let items = vals(&["a", "b", "c"]);
        assert_eq!(
            compute_motion_attribute(&items, "a", "c", "a", None),
            Some("from-start")
        );
    }

    #[test]
    fn motion_to_start_when_was_selected_and_index_increased() {
        // items: [a, b, c], prev=a(0), current=c(2), this=a → was_selected, idx>prev_idx → to-start
        let items = vals(&["a", "b", "c"]);
        assert_eq!(
            compute_motion_attribute(&items, "c", "a", "a", None),
            Some("to-start")
        );
    }

    #[test]
    fn motion_to_end_when_was_selected_and_index_decreased() {
        // items: [a, b, c], prev=c(2), current=a(0), this=c → was_selected, idx<prev_idx → to-end
        let items = vals(&["a", "b", "c"]);
        assert_eq!(
            compute_motion_attribute(&items, "a", "c", "c", None),
            Some("to-end")
        );
    }

    #[test]
    fn motion_none_when_neither_selected_nor_was_selected() {
        // this=b, current=a, prev=c → b is neither selected nor was_selected
        let items = vals(&["a", "b", "c"]);
        assert_eq!(
            compute_motion_attribute(&items, "a", "c", "b", Some("from-end")),
            Some("from-end") // returns prev_attribute
        );
    }

    #[test]
    fn motion_none_when_same_index() {
        // current==previous → idx == prev_idx → None
        let items = vals(&["a", "b"]);
        assert_eq!(compute_motion_attribute(&items, "a", "a", "a", None), None);
    }

    #[test]
    fn motion_none_when_current_not_in_list() {
        let items = vals(&["a", "b"]);
        assert_eq!(compute_motion_attribute(&items, "z", "a", "z", None), None);
    }
}
