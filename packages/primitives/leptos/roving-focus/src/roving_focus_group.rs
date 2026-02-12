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
use radix_leptos_primitive::{Primitive, compose_callbacks};
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
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let current_tab_stop_id = StoredValue::new(current_tab_stop_id);
    let default_current_tab_stop_id = StoredValue::new(default_current_tab_stop_id);
    let children = StoredValue::new(children.into_inner());

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
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    let orientation = Signal::derive(move || orientation.get());
    let r#loop = Signal::derive(move || r#loop.get().unwrap_or(false));

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
            handle_entry_focus.with_value(|closure| {
                el.add_event_listener_with_callback(ENTRY_FOCUS, closure.as_ref().unchecked_ref())
                    .expect("Entry focus event listener should be added.");
            });
        }
    });

    Owner::on_cleanup(move || {
        if let Some(node) = group_ref.get() {
            let el: &web_sys::HtmlElement = node.deref().unchecked_ref();
            handle_entry_focus.with_value(|closure| {
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
        on_item_shift_tab: Callback::new(move |_| set_is_tabbing_back_out.set(true)),
        on_focusable_item_add: Callback::new(move |_| {
            set_focusable_items_count.update(|focusable_items_count| *focusable_items_count += 1)
        }),
        on_focusable_item_remove: Callback::new(move |_| {
            set_focusable_items_count.update(|focusable_items_count| *focusable_items_count -= 1)
        }),
    };

    view! {
        <Provider value=roving_context_value>
            <AttributeInterceptor let:attrs>
                <Primitive
                    element=html::div
                    as_child=as_child
                    node_ref=composed_refs
                    attr:tabindex=move || match is_tabbing_back_out.get() || focusable_items_count.get() == 0 {
                        true => "-1",
                        false => "0",
                    }
                    attr:data-orientation=move || orientation.get().map(|o| o.to_string())
                    attr:style="outline: none"
                    on:mousedown=compose_callbacks(on_mouse_down.flatten(), Some(Callback::new(move |_: ev::MouseEvent| {
                        is_click_focus.set(true);
                    })), None)
                    on:mouseup=move |_: ev::MouseEvent| {
                        is_click_focus.set(false);
                    }
                    on:focus=compose_callbacks(on_focus.flatten(), Some(Callback::new(move |event: ev::FocusEvent| {
                        // We normally wouldn't need this check, because we already check
                        // that the focus is on the current target and not bubbling to it.
                        // We do this because Safari doesn't focus buttons when clicked, and
                        // instead, the wrapper will get focused and not through a bubbling event.
                        let is_keyboard_focus = !is_click_focus.get();

                        if event.target() == event.current_target() && is_keyboard_focus && !is_tabbing_back_out.get() {
                            let init = CustomEventInit::new();
                            init.set_bubbles(false);
                            init.set_cancelable(true);

                            let entry_focus_event = CustomEvent::new_with_event_init_dict(ENTRY_FOCUS, &init).expect("Entry focus event should be instantiated.");
                            event.current_target().expect("Event should have current target.").dispatch_event(&entry_focus_event).expect("Entry focus event should be dispatched.");

                            if !entry_focus_event.default_prevented() {
                                let items = get_items.with_value(|get_items| get_items());
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

                        is_click_focus.set(false);
                    })), None)
                    on:focusout=compose_callbacks(on_blur.flatten(), Some(Callback::new(move |_: ev::FocusEvent| {
                        set_is_tabbing_back_out.set(false);
                    })), None)
                    {..attrs}
                >
                    {children.with_value(|children| children())}
                </Primitive>
            </AttributeInterceptor>
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
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    let focusable = Signal::derive(move || focusable.get().unwrap_or(true));
    let active = Signal::derive(move || active.get().unwrap_or(false));

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
            context.on_focusable_item_remove.run(());
        }
    });

    let item_data = Signal::derive(move || ItemData {
        id: id.get(),
        focusable: focusable.get(),
        active: active.get(),
    });

    view! {
        <CollectionItemSlot item_data_type=ITEM_DATA_PHANTHOM item_data=item_data>
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
                        context.on_item_focus.run(id.get());
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

                            let items = get_items.with_value(|get_items| get_items());
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
    let key = get_direction_aware_key(event.key(), dir);

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
