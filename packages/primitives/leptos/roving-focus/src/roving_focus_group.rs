use std::fmt::Formatter;
use std::marker::PhantomData;
use std::{fmt::Display, ops::Deref};

use leptos::{
    attribute_interceptor::AttributeInterceptor, context::Provider, ev, html, prelude::*,
};
use leptos_node_ref::AnyNodeRef;
use radix_leptos_collection::{
    CollectionItemSlot, CollectionProvider, CollectionSlot, use_collection,
};
use radix_leptos_compose_refs::use_composed_refs;
use radix_leptos_direction::{Direction, use_direction};
use radix_leptos_id::use_id;
use radix_leptos_primitive::{Primitive, compose_callbacks, prop_or, prop_or_default};
use radix_leptos_use_controllable_state::{UseControllableStateParams, use_controllable_state};
use send_wrapper::SendWrapper;
use web_sys::{
    CustomEvent, CustomEventInit,
    wasm_bindgen::{JsCast, closure::Closure},
};

const ENTRY_FOCUS: &str = "rovingFocusGroup.onEntryFocus";

#[derive(Clone, Debug)]
struct ItemData {
    id: String,
    focusable: bool,
    active: bool,
}

const ITEM_DATA_PHANTHOM: PhantomData<ItemData> = PhantomData;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Orientation {
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

/// Public context provided by [`RovingFocusGroup`] for consumers that need
/// to know whether the group currently has a tab stop.
#[derive(Clone, Copy)]
pub struct RovingFocusGroupContext {
    pub has_tab_stop: Signal<bool>,
}

/// Public context provided by [`RovingFocusGroupItem`] for consumers that need
/// to know whether the item is the current tab stop.
#[derive(Clone, Copy)]
pub struct RovingFocusGroupItemContext {
    pub is_current_tab_stop: Signal<bool>,
}

#[derive(Clone, Debug)]
struct RovingContextValue {
    orientation: Signal<Option<Orientation>>,
    dir: Signal<Direction>,
    r#loop: Signal<bool>,
    current_tab_stop_id: Signal<Option<String>>,
    on_item_focus: Callback<String>,
    on_item_shift_tab: Callback<()>,
    on_focusable_item_add: Callback<()>,
    on_focusable_item_remove: Callback<()>,
}

#[component]
pub fn RovingFocusGroup(
    #[prop(into, optional)] orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] r#loop: MaybeProp<bool>,
    #[prop(into, optional)] current_tab_stop_id: MaybeProp<String>,
    #[prop(into, optional)] default_current_tab_stop_id: MaybeProp<String>,
    #[prop(into, optional)] on_current_tab_stop_id_change: Option<Callback<Option<String>>>,
    #[prop(into, optional)] on_entry_focus: Option<Callback<ev::Event>>,
    #[prop(into, optional)] prevent_scroll_on_entry_focus: MaybeProp<bool>,
    #[prop(into, optional)] on_mouse_down: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] on_focus: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] on_blur: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let current_tab_stop_id = StoredValue::new(current_tab_stop_id);
    let default_current_tab_stop_id = StoredValue::new(default_current_tab_stop_id);
    let children = StoredValue::new(children);

    view! {
        <CollectionProvider item_data_type=ITEM_DATA_PHANTHOM>
            <CollectionSlot item_data_type=ITEM_DATA_PHANTHOM>
                <RovingFocusGroupImpl
                    orientation=orientation
                    dir=dir
                    r#loop=r#loop
                    current_tab_stop_id=current_tab_stop_id.get_value()
                    default_current_tab_stop_id=default_current_tab_stop_id.get_value()
                    on_current_tab_stop_id_change=on_current_tab_stop_id_change
                    on_entry_focus=on_entry_focus
                    prevent_scroll_on_entry_focus=prevent_scroll_on_entry_focus
                    on_mouse_down=on_mouse_down
                    on_focus=on_focus
                    on_blur=on_blur
                    as_child=as_child
                    node_ref=node_ref
                >
                    {children.with_value(|children| children())}
                </RovingFocusGroupImpl>
            </CollectionSlot>
        </CollectionProvider>
    }
}

#[component]
fn RovingFocusGroupImpl(
    #[prop(into, optional)] orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] r#loop: MaybeProp<bool>,
    #[prop(into, optional)] current_tab_stop_id: MaybeProp<String>,
    #[prop(into, optional)] default_current_tab_stop_id: MaybeProp<String>,
    #[prop(into, optional)] on_current_tab_stop_id_change: Option<Option<Callback<Option<String>>>>,
    #[prop(into, optional)] on_entry_focus: Option<Option<Callback<ev::Event>>>,
    #[prop(into, optional)] prevent_scroll_on_entry_focus: MaybeProp<bool>,
    #[prop(into, optional)] on_mouse_down: Option<Option<Callback<ev::MouseEvent>>>,
    #[prop(into, optional)] on_focus: Option<Option<Callback<ev::FocusEvent>>>,
    #[prop(into, optional)] on_blur: Option<Option<Callback<ev::FocusEvent>>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let orientation = Signal::derive(move || orientation.get());
    let r#loop = prop_or_default(r#loop);

    let group_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, group_ref]);
    let direction = use_direction(dir);
    let (current_tab_stop_id, set_current_tab_stop_id) =
        use_controllable_state(UseControllableStateParams {
            prop: current_tab_stop_id,
            default_prop: default_current_tab_stop_id,
            on_change: on_current_tab_stop_id_change.flatten(),
        });
    let (is_tabbing_back_out, set_is_tabbing_back_out) = signal(false);
    let get_items = StoredValue::new(use_collection::<ItemData>());
    let is_click_focus = RwSignal::new(false);
    let (focusable_items_count, set_focusable_items_count) = signal(0);
    // Track whether any item has registered yet. In React, effects run
    // synchronously before paint so focusable_items_count is updated before the
    // user can interact. In Leptos WASM, effects run asynchronously after DOM
    // mount, creating a brief window where focusable_items_count is still 0 and
    // tabindex would be "-1", preventing Tab from focusing the group. We default
    // to tabindex="0" until at least one item effect has run.
    let items_initialized = RwSignal::new(false);

    let on_entry_focus = on_entry_focus.flatten();
    let handle_entry_focus: SendWrapper<Closure<dyn Fn(ev::Event)>> =
        SendWrapper::new(Closure::new(move |event: ev::Event| {
            if let Some(on_entry_focus) = on_entry_focus {
                on_entry_focus.run(event);
            }
        }));
    let handle_entry_focus = StoredValue::new(handle_entry_focus);

    Effect::new(move |_| {
        if let Some(node) = group_ref.get() {
            let el: &web_sys::HtmlElement = node.deref().unchecked_ref();
            let _ = handle_entry_focus.try_with_value(|closure| {
                el.add_event_listener_with_callback(ENTRY_FOCUS, closure.as_ref().unchecked_ref())
                    .expect("Entry focus event listener should be added.");
            });
        }
    });

    Owner::on_cleanup(move || {
        if let Some(node) = group_ref.get_untracked() {
            let el: &web_sys::HtmlElement = node.deref().unchecked_ref();
            let _ = handle_entry_focus.try_with_value(|closure| {
                el.remove_event_listener_with_callback(
                    ENTRY_FOCUS,
                    closure.as_ref().unchecked_ref(),
                )
                .expect("Entry focus event listener should be removed.");
            });
        }
    });

    let roving_context_value = RovingContextValue {
        orientation,
        dir: direction,
        r#loop,
        current_tab_stop_id,
        on_item_focus: Callback::new(move |tab_stop_id| {
            set_current_tab_stop_id.run(Some(tab_stop_id))
        }),
        on_item_shift_tab: Callback::new(move |_| {
            let _ = set_is_tabbing_back_out.try_set(true);
        }),
        on_focusable_item_add: Callback::new(move |_| {
            let _ = items_initialized.try_set(true);
            let _ = set_focusable_items_count
                .try_update(|focusable_items_count| *focusable_items_count += 1);
        }),
        on_focusable_item_remove: Callback::new(move |_| {
            let _ = items_initialized.try_set(true);
            let _ = set_focusable_items_count
                .try_update(|focusable_items_count| *focusable_items_count -= 1);
        }),
    };

    let public_group_context = RovingFocusGroupContext {
        has_tab_stop: Signal::derive(move || current_tab_stop_id.get().is_some()),
    };

    view! {
        <Provider value=roving_context_value>
            <Provider value=public_group_context>
            <AttributeInterceptor let:attrs>
                <Primitive
                    element=html::div
                    as_child=as_child
                    node_ref=composed_refs
                    attr:tabindex=move || match is_tabbing_back_out.get() || (items_initialized.get() && focusable_items_count.get() == 0) {
                        true => "-1",
                        false => "0",
                    }
                    attr:data-orientation=move || orientation.get().map(|o| o.to_string())
                    attr:style="outline: none"
                    on:mousedown=compose_callbacks(on_mouse_down.flatten(), Some(Callback::new(move |_: ev::MouseEvent| {
                        let _ = is_click_focus.try_set(true);
                    })), None)
                    on:mouseup=move |_: ev::MouseEvent| {
                        let _ = is_click_focus.try_set(false);
                    }
                    on:focus=compose_callbacks(on_focus.flatten(), Some(Callback::new(move |event: ev::FocusEvent| {
                        // We normally wouldn't need this check, because we already check
                        // that the focus is on the current target and not bubbling to it.
                        // We do this because Safari doesn't focus buttons when clicked, and
                        // instead, the wrapper will get focused and not through a bubbling event.
                        let is_keyboard_focus = !is_click_focus.try_get().unwrap_or(false);

                        if event.target() == event.current_target() && is_keyboard_focus && !is_tabbing_back_out.try_get().unwrap_or(false) {
                            let init = CustomEventInit::new();
                            init.set_bubbles(false);
                            init.set_cancelable(true);

                            let entry_focus_event = CustomEvent::new_with_event_init_dict(ENTRY_FOCUS, &init).expect("Entry focus event should be instantiated.");
                            event.current_target().expect("Event should have current target.").dispatch_event(&entry_focus_event).expect("Entry focus event should be dispatched.");

                            if !entry_focus_event.default_prevented() {
                                let items = get_items.try_with_value(|get_items| get_items()).unwrap_or_default();
                                let items: Vec<_> = items.iter().filter(|item| item.data.focusable).collect();
                                let active_item = items.iter().find(|item| item.data.active);
                                let current_item = items.iter().find(|item| current_tab_stop_id.get().is_some_and(|current_id| current_id == item.data.id));

                                let mut candidate_items = items.clone();
                                if let Some(active_item) = active_item {
                                    candidate_items.insert(0, active_item);
                                }
                                if let Some(current_item) = current_item {
                                    candidate_items.insert(0, current_item);
                                }
                                let candidate_nodes = candidate_items.iter().filter_map(|item| {
                                    item.r#ref.get().map(|el| {
                                        let html_el: &web_sys::HtmlElement = el.deref().unchecked_ref();
                                        html_el.clone()
                                    })
                                }).collect::<Vec<web_sys::HtmlElement>>();
                                focus_first(candidate_nodes, prevent_scroll_on_entry_focus.get());
                            }
                        }

                        let _ = is_click_focus.try_set(false);
                    })), None)
                    on:focusout=compose_callbacks(on_blur.flatten(), Some(Callback::new(move |_: ev::FocusEvent| {
                        let _ = set_is_tabbing_back_out.try_set(false);
                    })), None)
                    {..attrs}
                >
                    {children.with_value(|children| children())}
                </Primitive>
            </AttributeInterceptor>
            </Provider>
        </Provider>
    }
}

#[component]
pub fn RovingFocusGroupItem(
    #[prop(into, optional)] tab_stop_id: MaybeProp<String>,
    #[prop(into, optional)] focusable: MaybeProp<bool>,
    #[prop(into, optional)] active: MaybeProp<bool>,
    #[prop(into, optional)] on_mouse_down: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] on_focus: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let focusable = prop_or(focusable, true);
    let active = prop_or_default(active);

    let auto_id = use_id(None);
    let id = Signal::derive(move || tab_stop_id.get().unwrap_or(auto_id.get()));
    let context = expect_context::<RovingContextValue>();
    let is_current_tab_stop = Signal::derive(move || {
        context
            .current_tab_stop_id
            .get()
            .is_some_and(|current_tab_stop_id| current_tab_stop_id == id.get())
    });
    let get_items = StoredValue::new(use_collection::<ItemData>());

    Effect::new(move |was_focusable: Option<bool>| {
        let is_focusable = focusable.get();
        if is_focusable {
            context.on_focusable_item_add.run(());
        } else if was_focusable == Some(true) {
            // Was focusable, now isn't — mirrors React useEffect cleanup on re-run
            context.on_focusable_item_remove.run(());
        }
        is_focusable
    });
    Owner::on_cleanup(move || {
        if focusable.get_untracked() {
            // Use try_run because the parent RovingFocusGroup's StoredValues
            // may already be disposed during tree teardown.
            context.on_focusable_item_remove.try_run(());
        }
    });

    let public_item_context = RovingFocusGroupItemContext {
        is_current_tab_stop,
    };

    let item_data = Signal::derive(move || ItemData {
        id: id.get(),
        focusable: focusable.get(),
        active: active.get(),
    });

    view! {
        <CollectionItemSlot item_data_type=ITEM_DATA_PHANTHOM item_data=item_data>
            <Provider value=public_item_context>
            <AttributeInterceptor let:attrs>
                <Primitive
                    element=html::span
                    as_child=as_child
                    node_ref=node_ref
                    attr:tabindex=move || match is_current_tab_stop.get() {
                        true => "0",
                        false => "-1",
                    }
                    attr:data-orientation=move || context.orientation.get().map(|o| o.to_string())
                    on:mousedown=compose_callbacks(on_mouse_down, Some(Callback::new(move |event: ev::MouseEvent| {
                        // We prevent focusing non-focusable items on `mousedown`.
                        // Even though the item has `tabindex="-1"`, that only means take it out of the tab order.
                        if !focusable.get() {
                            event.prevent_default();
                        } else {
                            // Safari doesn't focus a button when clicked, so we run our logic on mousedown also.
                            context.on_item_focus.run(id.get());
                        }
                    })), None)
                    on:focus=compose_callbacks(on_focus, Some(Callback::new(move |_: ev::FocusEvent| {
                        // Defer the tab stop update to a macrotask to avoid WASM
                        // closure panics. When focus_first() is called synchronously
                        // from the keydown handler, .focus() triggers this handler.
                        // Updating set_current_tab_stop_id synchronously would trigger
                        // reactive effects while the keydown closure is on the stack.
                        //
                        // Use try_run because the setTimeout may fire after the component
                        // tree has been disposed (e.g., when a menu item click triggers
                        // focus and close in the same interaction — the close disposes
                        // components before the deferred callback runs).
                        let on_item_focus = context.on_item_focus;
                        let item_id = id.get();
                        let window = web_sys::window().expect("Window should exist.");
                        window
                            .set_timeout_with_callback_and_timeout_and_arguments_0(
                                Closure::once_into_js(move || {
                                    on_item_focus.try_run(item_id);
                                })
                                .unchecked_ref(),
                                0,
                            )
                            .expect("setTimeout should succeed.");
                    })), None)
                    on:keydown=compose_callbacks(on_key_down, Some(Callback::new(move |event: ev::KeyboardEvent| {
                        if event.key() == "Tab" && event.shift_key() {
                            context.on_item_shift_tab.run(());
                            return;
                        }

                        if event.target() != event.current_target() {
                            return;
                        }

                        let focus_intent = get_focus_intent(&event, context.orientation.get(), Some(context.dir.get()));
                        if let Some(focus_intent) = focus_intent {
                            if event.meta_key() || event.ctrl_key() || event.alt_key() || event.shift_key() {
                                return;
                            }

                            event.prevent_default();

                            let items = get_items.try_with_value(|get_items| get_items()).unwrap_or_default();
                            let items = items.into_iter().filter(|item| item.data.focusable);
                            let mut candidate_nodes = items.filter_map(|item| {
                                item.r#ref.get().map(|el| {
                                    let html_el: &web_sys::HtmlElement = el.deref().unchecked_ref();
                                    html_el.clone()
                                })
                            }).collect::<Vec<web_sys::HtmlElement>>();

                            if focus_intent == FocusIntent::Last {
                                candidate_nodes.reverse();
                            } else if focus_intent == FocusIntent::Prev || focus_intent == FocusIntent::Next {
                                if focus_intent == FocusIntent::Prev {
                                    candidate_nodes.reverse();
                                }

                                let current_index = candidate_nodes
                                    .iter()
                                    .position(|node| *node == event.current_target()
                                        .expect("Event should have current target.")
                                        .unchecked_into::<web_sys::HtmlElement>())
                                    .map(|index| index + 1)
                                    .unwrap_or(0);
                                candidate_nodes = match context.r#loop.get() {
                                    true => wrap_array(&mut candidate_nodes, current_index).to_vec(),
                                    false => candidate_nodes[current_index..].to_vec()
                                }
                            }

                            focus_first(candidate_nodes, None);
                        }
                    })), None)
                    {..attrs}
                >
                    {children.with_value(|children| children())}
                </Primitive>
            </AttributeInterceptor>
            </Provider>
        </CollectionItemSlot>
    }
}

fn get_direction_aware_key(key: String, dir: Option<Direction>) -> String {
    if dir != Some(Direction::Rtl) {
        return key;
    }

    (match key.as_str() {
        "ArrowLeft" => "ArrowRight",
        "ArrowRight" => "ArrowLeft",
        key => key,
    })
    .into()
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum FocusIntent {
    First,
    Last,
    Prev,
    Next,
}

fn get_focus_intent(
    event: &ev::KeyboardEvent,
    orientation: Option<Orientation>,
    dir: Option<Direction>,
) -> Option<FocusIntent> {
    focus_intent_for_key(&event.key(), orientation, dir)
}

/// Pure key-matching logic extracted from [`get_focus_intent`] for testability.
fn focus_intent_for_key(
    key: &str,
    orientation: Option<Orientation>,
    dir: Option<Direction>,
) -> Option<FocusIntent> {
    let key = get_direction_aware_key(key.to_string(), dir);

    if orientation == Some(Orientation::Vertical)
        && ["ArrowLeft", "ArrowRight"].contains(&key.as_str())
    {
        return None;
    }
    if orientation == Some(Orientation::Horizontal)
        && ["ArrowUp", "ArrowDown"].contains(&key.as_str())
    {
        return None;
    }

    match key.as_str() {
        "ArrowLeft" => Some(FocusIntent::Prev),
        "ArrowUp" => Some(FocusIntent::Prev),
        "ArrowRight" => Some(FocusIntent::Next),
        "ArrowDown" => Some(FocusIntent::Next),
        "PageUp" => Some(FocusIntent::First),
        "Home" => Some(FocusIntent::First),
        "PageDown" => Some(FocusIntent::Last),
        "End" => Some(FocusIntent::Last),
        _ => None,
    }
}

fn focus_first(candidates: Vec<web_sys::HtmlElement>, _prevent_scroll: Option<bool>) {
    let previously_focused_element = document().active_element();

    for candidate in candidates {
        // If focus is already where we want to go, we don't want to keep going through the candidates.
        if previously_focused_element.as_ref() == candidate.dyn_ref::<web_sys::Element>() {
            return;
        }

        // TODO: focus options with prevent_scroll
        candidate.focus().expect("Element should be focused.");

        if document().active_element() != previously_focused_element {
            return;
        }
    }
}

/// Wraps an array around itself at a given start index.
/// Example: `wrap_array(['a', 'b', 'c', 'd'], 2) == ['c', 'd', 'a', 'b']`
fn wrap_array<T: Clone>(array: &mut [T], start_index: usize) -> &[T] {
    array.rotate_left(start_index);
    array
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Orientation Display ──────────────────────────────────

    #[test]
    fn orientation_display_horizontal() {
        assert_eq!(Orientation::Horizontal.to_string(), "horizontal");
    }

    #[test]
    fn orientation_display_vertical() {
        assert_eq!(Orientation::Vertical.to_string(), "vertical");
    }

    // ── get_direction_aware_key ──────────────────────────────

    #[test]
    fn direction_aware_key_ltr_unchanged() {
        assert_eq!(get_direction_aware_key("ArrowLeft".into(), None), "ArrowLeft");
        assert_eq!(
            get_direction_aware_key("ArrowLeft".into(), Some(Direction::Ltr)),
            "ArrowLeft"
        );
        assert_eq!(
            get_direction_aware_key("ArrowRight".into(), Some(Direction::Ltr)),
            "ArrowRight"
        );
    }

    #[test]
    fn direction_aware_key_rtl_flips_horizontal() {
        assert_eq!(
            get_direction_aware_key("ArrowLeft".into(), Some(Direction::Rtl)),
            "ArrowRight"
        );
        assert_eq!(
            get_direction_aware_key("ArrowRight".into(), Some(Direction::Rtl)),
            "ArrowLeft"
        );
    }

    #[test]
    fn direction_aware_key_rtl_preserves_vertical() {
        assert_eq!(
            get_direction_aware_key("ArrowUp".into(), Some(Direction::Rtl)),
            "ArrowUp"
        );
        assert_eq!(
            get_direction_aware_key("ArrowDown".into(), Some(Direction::Rtl)),
            "ArrowDown"
        );
    }

    #[test]
    fn direction_aware_key_non_arrow_unchanged() {
        assert_eq!(
            get_direction_aware_key("Enter".into(), Some(Direction::Rtl)),
            "Enter"
        );
        assert_eq!(
            get_direction_aware_key("Tab".into(), Some(Direction::Rtl)),
            "Tab"
        );
    }

    // ── focus_intent_for_key ─────────────────────────────────

    #[test]
    fn focus_intent_no_orientation() {
        assert_eq!(focus_intent_for_key("ArrowLeft", None, None), Some(FocusIntent::Prev));
        assert_eq!(focus_intent_for_key("ArrowUp", None, None), Some(FocusIntent::Prev));
        assert_eq!(focus_intent_for_key("ArrowRight", None, None), Some(FocusIntent::Next));
        assert_eq!(focus_intent_for_key("ArrowDown", None, None), Some(FocusIntent::Next));
    }

    #[test]
    fn focus_intent_home_end_page_keys() {
        assert_eq!(focus_intent_for_key("Home", None, None), Some(FocusIntent::First));
        assert_eq!(focus_intent_for_key("PageUp", None, None), Some(FocusIntent::First));
        assert_eq!(focus_intent_for_key("End", None, None), Some(FocusIntent::Last));
        assert_eq!(focus_intent_for_key("PageDown", None, None), Some(FocusIntent::Last));
    }

    #[test]
    fn focus_intent_unknown_key_returns_none() {
        assert_eq!(focus_intent_for_key("Enter", None, None), None);
        assert_eq!(focus_intent_for_key("Tab", None, None), None);
        assert_eq!(focus_intent_for_key("Escape", None, None), None);
        assert_eq!(focus_intent_for_key("a", None, None), None);
    }

    #[test]
    fn focus_intent_vertical_ignores_horizontal_arrows() {
        let vert = Some(Orientation::Vertical);
        assert_eq!(focus_intent_for_key("ArrowLeft", vert, None), None);
        assert_eq!(focus_intent_for_key("ArrowRight", vert, None), None);
        // Vertical arrows still work
        assert_eq!(focus_intent_for_key("ArrowUp", vert, None), Some(FocusIntent::Prev));
        assert_eq!(focus_intent_for_key("ArrowDown", vert, None), Some(FocusIntent::Next));
    }

    #[test]
    fn focus_intent_horizontal_ignores_vertical_arrows() {
        let horiz = Some(Orientation::Horizontal);
        assert_eq!(focus_intent_for_key("ArrowUp", horiz, None), None);
        assert_eq!(focus_intent_for_key("ArrowDown", horiz, None), None);
        // Horizontal arrows still work
        assert_eq!(focus_intent_for_key("ArrowLeft", horiz, None), Some(FocusIntent::Prev));
        assert_eq!(focus_intent_for_key("ArrowRight", horiz, None), Some(FocusIntent::Next));
    }

    #[test]
    fn focus_intent_rtl_flips_arrows() {
        let rtl = Some(Direction::Rtl);
        // In RTL, ArrowLeft becomes ArrowRight (Next), ArrowRight becomes ArrowLeft (Prev)
        assert_eq!(focus_intent_for_key("ArrowLeft", None, rtl), Some(FocusIntent::Next));
        assert_eq!(focus_intent_for_key("ArrowRight", None, rtl), Some(FocusIntent::Prev));
        // Vertical arrows are unaffected by RTL
        assert_eq!(focus_intent_for_key("ArrowUp", None, rtl), Some(FocusIntent::Prev));
        assert_eq!(focus_intent_for_key("ArrowDown", None, rtl), Some(FocusIntent::Next));
    }

    #[test]
    fn focus_intent_rtl_vertical_orientation() {
        let rtl = Some(Direction::Rtl);
        let vert = Some(Orientation::Vertical);
        // Horizontal arrows ignored by vertical orientation (even after RTL flip)
        assert_eq!(focus_intent_for_key("ArrowLeft", vert, rtl), None);
        assert_eq!(focus_intent_for_key("ArrowRight", vert, rtl), None);
        // Vertical arrows work normally
        assert_eq!(focus_intent_for_key("ArrowUp", vert, rtl), Some(FocusIntent::Prev));
        assert_eq!(focus_intent_for_key("ArrowDown", vert, rtl), Some(FocusIntent::Next));
    }

    #[test]
    fn focus_intent_rtl_horizontal_orientation() {
        let rtl = Some(Direction::Rtl);
        let horiz = Some(Orientation::Horizontal);
        // RTL flips arrows, then horizontal orientation applies
        assert_eq!(focus_intent_for_key("ArrowLeft", horiz, rtl), Some(FocusIntent::Next));
        assert_eq!(focus_intent_for_key("ArrowRight", horiz, rtl), Some(FocusIntent::Prev));
        // Vertical arrows ignored by horizontal orientation
        assert_eq!(focus_intent_for_key("ArrowUp", horiz, rtl), None);
        assert_eq!(focus_intent_for_key("ArrowDown", horiz, rtl), None);
    }

    #[test]
    fn focus_intent_home_end_unaffected_by_orientation_and_dir() {
        for orientation in [None, Some(Orientation::Horizontal), Some(Orientation::Vertical)] {
            for dir in [None, Some(Direction::Ltr), Some(Direction::Rtl)] {
                assert_eq!(
                    focus_intent_for_key("Home", orientation, dir),
                    Some(FocusIntent::First),
                    "Home should be First with orientation={orientation:?}, dir={dir:?}"
                );
                assert_eq!(
                    focus_intent_for_key("End", orientation, dir),
                    Some(FocusIntent::Last),
                    "End should be Last with orientation={orientation:?}, dir={dir:?}"
                );
            }
        }
    }

    // ── wrap_array ───────────────────────────────────────────

    #[test]
    fn wrap_array_basic() {
        let mut arr = ['a', 'b', 'c', 'd'];
        assert_eq!(wrap_array(&mut arr, 2), &['c', 'd', 'a', 'b']);
    }

    #[test]
    fn wrap_array_at_zero() {
        let mut arr = ['a', 'b', 'c'];
        assert_eq!(wrap_array(&mut arr, 0), &['a', 'b', 'c']);
    }

    #[test]
    fn wrap_array_at_end() {
        let mut arr = ['a', 'b', 'c'];
        assert_eq!(wrap_array(&mut arr, 2), &['c', 'a', 'b']);
    }

    #[test]
    fn wrap_array_single_element() {
        let mut arr = [42];
        assert_eq!(wrap_array(&mut arr, 0), &[42]);
    }

    #[test]
    fn wrap_array_at_one() {
        let mut arr = [1, 2, 3, 4, 5];
        assert_eq!(wrap_array(&mut arr, 1), &[2, 3, 4, 5, 1]);
    }

    #[test]
    fn wrap_array_empty() {
        let mut arr: [i32; 0] = [];
        assert_eq!(wrap_array(&mut arr, 0), &[] as &[i32]);
    }
}
