// TODO: remove
#![expect(dead_code)]

use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

use leptos::{
    attribute_interceptor::AttributeInterceptor, context::Provider, ev, html, prelude::*,
};
use leptos_node_ref::AnyNodeRef;
use radix_leptos_aria_hidden::{hide_others, unhide_others};
use radix_leptos_collection::{
    CollectionItemSlot, CollectionProvider, CollectionSlot, use_collection,
};
use radix_leptos_compose_refs::use_composed_refs;
pub use radix_leptos_direction::Direction;
use radix_leptos_direction::use_direction;
use radix_leptos_dismissable_layer::DismissableLayer;
use radix_leptos_focus_guards::use_focus_guards;
use radix_leptos_focus_scope::FocusScope;
use radix_leptos_id::use_id;
pub use radix_leptos_popper::{
    Align, ClientRectObject, PopperVirtualElement, Side as PopperSide, set_popper_virtual_ref,
};
use radix_leptos_popper::{Popper, PopperAnchor, PopperArrow, PopperContent};
use radix_leptos_portal::ScopedPortal;
use radix_leptos_presence::Presence;
use radix_leptos_primitive::{
    Primitive, compose_callbacks, data_attr, open_closed_state, prop_or, prop_or_default,
    wrap_callback,
};
use radix_leptos_roving_focus::{Orientation, RovingFocusGroup, RovingFocusGroupItem};
use send_wrapper::SendWrapper;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{AddEventListenerOptions, CustomEvent, CustomEventInit, EventListenerOptions};

const SELECTION_KEYS: [&str; 2] = ["Enter", " "];
const FIRST_KEYS: [&str; 3] = ["ArrowDown", "PageUp", "Home"];
const LAST_KEYS: [&str; 3] = ["ArrowUp", "PageDown", "End"];
const FIRST_LAST_KEYS: [&str; 6] = ["ArrowDown", "PageUp", "Home", "ArrowUp", "PageDown", "End"];

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

#[component]
pub fn Menu(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] modal: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let open = prop_or_default(open);
    let modal = prop_or(modal, true);
    let on_open_change = on_open_change.unwrap_or(Callback::new(|_| {}));

    let content_ref = AnyNodeRef::new();
    let is_using_keyboard = RwSignal::new(false);
    let direction = use_direction(dir);

    let popper_anchor_ref = AnyNodeRef::new();
    let context_value = MenuContextValue {
        open,
        content_ref,
        on_open_change,
        popper_anchor_ref,
    };
    let root_context_value = MenuRootContextValue {
        is_using_keyboard: is_using_keyboard.into(),
        dir: direction,
        modal,
        on_close: Callback::new(move |_| on_open_change.run(false)),
    };

    let handle_pointer: SendWrapper<Rc<RefCell<Option<Closure<dyn Fn(ev::PointerEvent)>>>>> =
        SendWrapper::new(Rc::new(RefCell::new(Some(Closure::<
            dyn Fn(ev::PointerEvent),
        >::new(move |_| {
            is_using_keyboard.set(false);
        })))));

    let handle_key_down: SendWrapper<Rc<RefCell<Option<Closure<dyn Fn(ev::KeyboardEvent)>>>>> = {
        let handle_pointer = handle_pointer.clone();
        SendWrapper::new(Rc::new(RefCell::new(Some(Closure::<
            dyn Fn(ev::KeyboardEvent),
        >::new(move |_| {
            is_using_keyboard.set(true);

            let options = AddEventListenerOptions::new();
            options.set_capture(true);
            options.set_once(true);

            if let Some(hp) = handle_pointer.borrow().as_ref() {
                let cb: &wasm_bindgen::JsValue = hp.as_ref().unchecked_ref();
                document()
                    .add_event_listener_with_callback_and_add_event_listener_options(
                        "pointerdown",
                        cb.unchecked_ref(),
                        &options,
                    )
                    .expect("Pointer down event listener should be added.");
                document()
                    .add_event_listener_with_callback_and_add_event_listener_options(
                        "pointermove",
                        cb.unchecked_ref(),
                        &options,
                    )
                    .expect("Pointer move event listener should be added.");
            }
        })))))
    };

    Effect::new({
        let handle_key_down = handle_key_down.clone();
        move |_| {
            let options = AddEventListenerOptions::new();
            options.set_capture(true);

            // Capture phase ensures we set the boolean before any side effects execute
            // in response to the key or pointer event as they might depend on this value.
            if let Some(hk) = handle_key_down.borrow().as_ref() {
                let cb: &wasm_bindgen::JsValue = hk.as_ref().unchecked_ref();
                document()
                    .add_event_listener_with_callback_and_add_event_listener_options(
                        "keydown",
                        cb.unchecked_ref(),
                        &options,
                    )
                    .expect("Key down event listener should be added.");
            }
        }
    });

    on_cleanup(move || {
        let options = EventListenerOptions::new();
        options.set_capture(true);

        if let Some(hk) = handle_key_down.borrow().as_ref() {
            let cb: &wasm_bindgen::JsValue = hk.as_ref().unchecked_ref();
            document()
                .remove_event_listener_with_callback_and_event_listener_options(
                    "keydown",
                    cb.unchecked_ref(),
                    &options,
                )
                .expect("Key down event listener should be removed.");
        }

        if let Some(hp) = handle_pointer.borrow().as_ref() {
            let cb: &wasm_bindgen::JsValue = hp.as_ref().unchecked_ref();
            document()
                .remove_event_listener_with_callback_and_event_listener_options(
                    "pointerdown",
                    cb.unchecked_ref(),
                    &options,
                )
                .expect("Pointer down event listener should be removed.");
            document()
                .remove_event_listener_with_callback_and_event_listener_options(
                    "pointermove",
                    cb.unchecked_ref(),
                    &options,
                )
                .expect("Pointer move event listener should be removed.");
        }
    });

    view! {
        <Popper anchor_ref=popper_anchor_ref>
            <Provider value=context_value>
                <Provider value=root_context_value>
                    {children.with_value(|children| children())}
                </Provider>
            </Provider>
        </Popper>
    }
}

#[component]
pub fn MenuAnchor(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let menu_context = expect_context::<MenuContextValue>();
    // Compose the user-provided node_ref with the Menu's own popper anchor ref.
    // This ensures the Menu's Popper context gets the anchor element directly,
    // even when a closer Popper context (e.g., from a Tooltip wrapping the trigger)
    // would shadow it during PopperAnchor's normal expect_context lookup.
    let composed_refs = use_composed_refs(vec![node_ref, menu_context.popper_anchor_ref]);

    view! {
        <PopperAnchor as_child=as_child node_ref=composed_refs>
            {children()}
        </PopperAnchor>
    }
}

#[component]
pub fn MenuPortal(
    /// Specify a container element to portal the content into.
    #[prop(into, optional)]
    container: MaybeProp<SendWrapper<web_sys::Element>>,
    /// Optional ref for the container element.
    #[prop(optional)]
    container_ref: AnyNodeRef,
    /// Used to force mounting when more control is needed. Useful when controlling animation with animation libraries.
    #[prop(into, optional)]
    force_mount: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <ScopedPortal container=container container_ref=container_ref force_mount=force_mount>
            {children.with_value(|children| children())}
        </ScopedPortal>
    }
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

#[component]
pub fn MenuContent(
    /// Used to force mounting when more control is needed. Useful when controlling animation with animation libraries.
    #[prop(into, optional)]
    force_mount: MaybeProp<bool>,
    /// CSS class applied directly to the inner content element (same element as data-state).
    #[prop(into, optional)]
    class: MaybeProp<String>,
    /// Event handler called when auto-focusing on close. Can be prevented.
    #[prop(into, optional)]
    on_close_auto_focus: Option<Callback<ev::Event>>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<CustomEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<CustomEvent>>,
    #[prop(into, optional)] on_interact_outside: Option<Callback<CustomEvent>>,
    /// Event handler called when the content receives initial focus. Can be prevented.
    #[prop(into, optional)]
    on_entry_focus: Option<Callback<ev::Event>>,
    /// Event handler called on keydown events on the content element.
    #[prop(into, optional)]
    on_key_down: Option<Callback<ev::KeyboardEvent>>,
    /// The preferred side of the trigger to render against when open.
    #[prop(into, optional)]
    side: MaybeProp<PopperSide>,
    /// The distance in pixels from the trigger.
    #[prop(into, optional)]
    side_offset: MaybeProp<f64>,
    /// The preferred alignment against the trigger.
    #[prop(into, optional)]
    align: MaybeProp<Align>,
    /// An offset in pixels from the "start" or "end" alignment options.
    #[prop(into, optional)]
    align_offset: MaybeProp<f64>,
    /// When `true`, overrides the `side` and `align` preferences to prevent collisions with boundary edges.
    #[prop(into, optional)]
    avoid_collisions: MaybeProp<bool>,
    /// The id of the content element.
    #[prop(into, optional)]
    id: MaybeProp<String>,
    /// The id of the element that labels the content.
    #[prop(into, optional)]
    aria_labelledby: MaybeProp<String>,
    /// Additional inline styles to apply to the content element. Used by wrapper components
    /// (e.g., ContextMenuContent) to set CSS custom property aliases on the final rendered element.
    #[prop(into, optional)]
    content_style: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<MenuContextValue>();
    let root_context = expect_context::<MenuRootContextValue>();

    let present = Signal::derive(move || force_mount.get().unwrap_or(false) || context.open.get());

    // Wrap Option<Callback<T>> → Callback<T> for forwarding through view! macro.
    let on_close_auto_focus = wrap_callback(on_close_auto_focus);
    let on_escape_key_down = wrap_callback(on_escape_key_down);
    let on_pointer_down_outside = wrap_callback(on_pointer_down_outside);
    let on_focus_outside = wrap_callback(on_focus_outside);
    let on_interact_outside = wrap_callback(on_interact_outside);
    let on_entry_focus = wrap_callback(on_entry_focus);
    let on_key_down = wrap_callback(on_key_down);

    view! {
        <CollectionProvider item_data_type=ITEM_DATA_PHANTHOM>
            <Presence present=present node_ref=node_ref>
                <CollectionSlot item_data_type=ITEM_DATA_PHANTHOM>
                    <Show
                        when=move || root_context.modal.get()
                        fallback=move || view! {
                            <MenuRootContentNonModal
                                class=class
                                content_style=content_style
                                on_close_auto_focus=on_close_auto_focus
                                on_escape_key_down=on_escape_key_down
                                on_pointer_down_outside=on_pointer_down_outside
                                on_focus_outside=on_focus_outside
                                on_interact_outside=on_interact_outside
                                on_entry_focus=on_entry_focus
                                on_key_down=on_key_down
                                side=side
                                side_offset=side_offset
                                align=align
                                align_offset=align_offset
                                avoid_collisions=avoid_collisions
                                id=id
                                aria_labelledby=aria_labelledby
                                as_child=as_child
                                node_ref=node_ref
                            >
                                {children.with_value(|children| children())}
                            </MenuRootContentNonModal>
                        }
                    >
                        <MenuRootContentModal
                            class=class
                            content_style=content_style
                            on_close_auto_focus=on_close_auto_focus
                            on_escape_key_down=on_escape_key_down
                            on_pointer_down_outside=on_pointer_down_outside
                            on_focus_outside=on_focus_outside
                            on_interact_outside=on_interact_outside
                            on_entry_focus=on_entry_focus
                            on_key_down=on_key_down
                            side=side
                            side_offset=side_offset
                            align=align
                            align_offset=align_offset
                            avoid_collisions=avoid_collisions
                            id=id
                            aria_labelledby=aria_labelledby
                            as_child=as_child
                            node_ref=node_ref
                        >
                            {children.with_value(|children| children())}
                        </MenuRootContentModal>
                    </Show>
                </CollectionSlot>
            </Presence>
        </CollectionProvider>
    }
}

#[component]
fn MenuRootContentModal(
    #[prop(into, optional)] on_close_auto_focus: Option<Callback<ev::Event>>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<CustomEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<CustomEvent>>,
    #[prop(into, optional)] on_interact_outside: Option<Callback<CustomEvent>>,
    #[prop(into, optional)] on_entry_focus: Option<Callback<ev::Event>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] side: MaybeProp<PopperSide>,
    #[prop(into, optional)] side_offset: MaybeProp<f64>,
    #[prop(into, optional)] align: MaybeProp<Align>,
    #[prop(into, optional)] align_offset: MaybeProp<f64>,
    #[prop(into, optional)] avoid_collisions: MaybeProp<bool>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] content_style: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let context = expect_context::<MenuContextValue>();
    let content_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, content_ref]);

    // Hide everything from ARIA except the `MenuContent`.
    let hidden_elements: RwSignal<Vec<SendWrapper<web_sys::Element>>> = RwSignal::new(Vec::new());

    Effect::new(move |_| {
        if let Some(content) = content_ref.get() {
            let content: web_sys::HtmlElement = content.unchecked_into();
            hide_others(&content, hidden_elements);
        }
    });

    on_cleanup(move || {
        unhide_others(hidden_elements);
    });

    // Wrap for forwarding through view! macro.
    let on_close_auto_focus = wrap_callback(on_close_auto_focus);
    let on_escape_key_down = wrap_callback(on_escape_key_down);
    let on_pointer_down_outside = wrap_callback(on_pointer_down_outside);
    let on_interact_outside = wrap_callback(on_interact_outside);
    let on_entry_focus = wrap_callback(on_entry_focus);
    let on_key_down = wrap_callback(on_key_down);

    view! {
        <MenuContentImpl
            // We make sure we're not trapping once it's been closed (closed != unmounted when animating out).
            trap_focus=context.open
            // Make sure to only disable pointer events when open. This avoids blocking interactions while animating out.
            disable_outside_pointer_events=context.open
            disable_outside_scroll=true
            // When focus is trapped, a `focusout` event may still happen. We make sure we don't trigger our `on_dismiss` in such case.
            on_focus_outside=compose_callbacks(on_focus_outside, Some(Callback::new(move |event: CustomEvent| {
                event.prevent_default();
            })), Some(false))
            on_close_auto_focus=on_close_auto_focus
            on_escape_key_down=on_escape_key_down
            on_pointer_down_outside=on_pointer_down_outside
            on_interact_outside=on_interact_outside
            on_entry_focus=on_entry_focus
            on_key_down=on_key_down
            on_dismiss=move |_| context.on_open_change.run(false)
            side=side
            side_offset=side_offset
            align=align
            align_offset=align_offset
            avoid_collisions=avoid_collisions
            id=id
            aria_labelledby=aria_labelledby
            class=class
            content_style=content_style
            as_child=as_child
            node_ref=composed_refs
        >
            {children()}
        </MenuContentImpl>
    }
}

#[component]
fn MenuRootContentNonModal(
    #[prop(into, optional)] on_close_auto_focus: Option<Callback<ev::Event>>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<CustomEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<CustomEvent>>,
    #[prop(into, optional)] on_interact_outside: Option<Callback<CustomEvent>>,
    #[prop(into, optional)] on_entry_focus: Option<Callback<ev::Event>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] side: MaybeProp<PopperSide>,
    #[prop(into, optional)] side_offset: MaybeProp<f64>,
    #[prop(into, optional)] align: MaybeProp<Align>,
    #[prop(into, optional)] align_offset: MaybeProp<f64>,
    #[prop(into, optional)] avoid_collisions: MaybeProp<bool>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] content_style: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let context = expect_context::<MenuContextValue>();

    // Wrap for forwarding through view! macro.
    let on_close_auto_focus = wrap_callback(on_close_auto_focus);
    let on_escape_key_down = wrap_callback(on_escape_key_down);
    let on_pointer_down_outside = wrap_callback(on_pointer_down_outside);
    let on_focus_outside = wrap_callback(on_focus_outside);
    let on_interact_outside = wrap_callback(on_interact_outside);
    let on_entry_focus = wrap_callback(on_entry_focus);
    let on_key_down = wrap_callback(on_key_down);

    view! {
        <MenuContentImpl
            trap_focus=false
            disable_outside_pointer_events=false
            disable_outside_scroll=false
            on_close_auto_focus=on_close_auto_focus
            on_escape_key_down=on_escape_key_down
            on_pointer_down_outside=on_pointer_down_outside
            on_focus_outside=on_focus_outside
            on_interact_outside=on_interact_outside
            on_entry_focus=on_entry_focus
            on_key_down=on_key_down
            on_dismiss=move |_| context.on_open_change.run(false)
            side=side
            side_offset=side_offset
            align=align
            align_offset=align_offset
            avoid_collisions=avoid_collisions
            id=id
            aria_labelledby=aria_labelledby
            class=class
            content_style=content_style
            as_child=as_child
            node_ref=node_ref
        >
            {children()}
        </MenuContentImpl>
    }
}

#[component]
fn MenuContentImpl(
    /// Event handler called when auto-focusing on open. Can be prevented.
    #[prop(into, optional)]
    on_open_auto_focus: Option<Callback<ev::Event>>,
    /// Event handler called when auto-focusing on close. Can be prevented.
    #[prop(into, optional)]
    on_close_auto_focus: Option<Callback<ev::Event>>,
    #[prop(into, optional)] disable_outside_pointer_events: MaybeProp<bool>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<CustomEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<CustomEvent>>,
    #[prop(into, optional)] on_interact_outside: Option<Callback<CustomEvent>>,
    #[prop(into, optional)] on_dismiss: Option<Callback<()>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_blur: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] on_pointer_move: Option<Callback<ev::PointerEvent>>,
    /// Whether scrolling outside the `MenuContent` should be prevented. Defaults to `false`.
    #[prop(into, optional)]
    disable_outside_scroll: MaybeProp<bool>,
    /// Whether focus should be trapped within the `MenuContent`. Defaults to `false`.
    #[prop(into, optional)]
    trap_focus: MaybeProp<bool>,
    /// Whether keyboard navigation should loop around. Defaults to `false`.
    #[prop(into, optional)]
    r#loop: MaybeProp<bool>,
    #[prop(into, optional)] on_entry_focus: Option<Callback<ev::Event>>,
    /// The preferred side of the trigger to render against when open. Forwarded to PopperContent.
    #[prop(into, optional)]
    side: MaybeProp<PopperSide>,
    /// The distance in pixels from the trigger. Forwarded to PopperContent.
    #[prop(into, optional)]
    side_offset: MaybeProp<f64>,
    /// The preferred alignment against the trigger. Forwarded to PopperContent.
    #[prop(into, optional)]
    align: MaybeProp<Align>,
    /// An offset in pixels from the "start" or "end" alignment options. Forwarded to PopperContent.
    #[prop(into, optional)]
    align_offset: MaybeProp<f64>,
    /// When `true`, overrides the `side` and `align` preferences to prevent collisions with boundary edges.
    #[prop(into, optional)]
    avoid_collisions: MaybeProp<bool>,
    /// The id of the content element.
    #[prop(into, optional)]
    id: MaybeProp<String>,
    /// The id of the element that labels the content.
    #[prop(into, optional)]
    aria_labelledby: MaybeProp<String>,
    /// CSS class applied directly to the inner content element (same element as data-state).
    /// Use this instead of `attr:class` for reliable reactive class updates.
    #[prop(into, optional)]
    class: MaybeProp<String>,
    /// Additional inline styles for the content element (e.g., CSS custom property aliases).
    #[prop(into, optional)]
    content_style: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let r#loop = prop_or_default(r#loop);

    let context = expect_context::<MenuContextValue>();
    let root_context = expect_context::<MenuRootContextValue>();
    let get_items = StoredValue::new(use_collection::<ItemData>());
    let (current_item_id, set_current_item_id) = signal::<Option<String>>(None);
    let content_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, content_ref]);
    let timer = RwSignal::new(0);
    let search = RwSignal::new("".to_string());
    let pointer_grace_timer = RwSignal::new(0);
    let pointer_grace_intent: RwSignal<Option<GraceIntent>> = RwSignal::new(None);
    let pointer_dir = RwSignal::new(Side::Right);
    let last_pointer_x = RwSignal::new(0);

    let clear_search = StoredValue::new(SendWrapper::new(Closure::<dyn Fn()>::new(move || {
        let _ = search.try_set("".into());
        window().clear_timeout_with_handle(timer.try_get_untracked().unwrap_or(0));
    })));

    let handle_typeahead_search = Callback::new(move |key: String| {
        let search_value = search.try_get_untracked().unwrap_or_default() + &key;
        let items = get_items
            .try_with_value(|get_items| get_items())
            .unwrap_or_default();
        let items = items
            .iter()
            .filter(|item| !item.data.disabled)
            .collect::<Vec<_>>();
        let current_item = document().active_element();
        let current_match = items
            .iter()
            .find(|item| {
                item.r#ref
                    .get_untracked()
                    .map(|node| {
                        let element: web_sys::Element = node.unchecked_into();
                        element
                    })
                    .as_ref()
                    == current_item.as_ref()
            })
            .map(|item| item.data.text_value.clone());
        let values = items
            .iter()
            .map(|item| item.data.text_value.clone())
            .collect::<Vec<_>>();
        let next_match = get_next_match(values, search_value.clone(), current_match);
        let new_item = items
            .iter()
            .find(|item| {
                next_match
                    .as_ref()
                    .is_some_and(|next_match| item.data.text_value == *next_match)
            })
            .and_then(|item| item.r#ref.get_untracked());

        let _ = search.try_set(search_value.clone());
        window().clear_timeout_with_handle(timer.try_get_untracked().unwrap_or(0));
        if !search_value.is_empty() {
            // Reset search 1 second after it was last updated.
            let _ = clear_search.try_with_value(|cs| {
                let cb: &wasm_bindgen::JsValue = cs.as_ref().unchecked_ref();
                let _ = timer.try_set(
                    window()
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            cb.unchecked_ref(),
                            1000,
                        )
                        .expect("Timeout should be set"),
                );
            });
        }

        if let Some(new_item) = new_item {
            let new_item: web_sys::HtmlElement = new_item.unchecked_into();
            let cb = Closure::once_into_js(move || {
                new_item.focus().ok();
            });
            window()
                .set_timeout_with_callback(cb.unchecked_ref())
                .expect("Timeout should be set.");
        }
    });

    on_cleanup(move || {
        window().clear_timeout_with_handle(timer.try_get_untracked().unwrap_or(0));
    });

    // Make sure the whole tree has focus guards as our `MenuContent` may be the last element in the DOM (because of the `Portal`).
    use_focus_guards();

    let is_pointer_moving_to_submenu = move |event: &ev::PointerEvent| -> bool {
        let Some(dir) = pointer_dir.try_get_untracked() else {
            return false;
        };
        let is_moving_towards = Some(dir)
            == pointer_grace_intent
                .try_get_untracked()
                .flatten()
                .map(|intent| intent.side);
        is_moving_towards
            && is_pointer_in_grace_area(
                event,
                pointer_grace_intent
                    .try_get_untracked()
                    .flatten()
                    .map(|intent| intent.area),
            )
    };

    let content_context_value = MenuContentContextValue {
        search,
        on_item_enter: Callback::new(move |event| {
            if is_pointer_moving_to_submenu(&event) {
                event.prevent_default();
            }
        }),
        on_item_leave: Callback::new(move |event| {
            if is_pointer_moving_to_submenu(&event) {
                return;
            }
            if let Some(content) = content_ref.get_untracked() {
                let content: web_sys::HtmlElement = content.unchecked_into();
                content.focus().ok();
            }
            let _ = set_current_item_id.try_set(None);
        }),
        on_trigger_leave: Callback::new(move |event| {
            if is_pointer_moving_to_submenu(&event) {
                event.prevent_default();
            }
        }),
        pointer_grace_timer,
        on_pointer_grace_intent_change: Callback::new(move |intent| {
            let _ = pointer_grace_intent.try_set(intent);
        }),
    };

    let disable_outside = prop_or_default(disable_outside_pointer_events);

    let trapped = prop_or_default(trap_focus);

    let current_tab_stop_id_signal =
        Signal::derive(move || current_item_id.get().unwrap_or_default());

    let children = StoredValue::new(children);

    // Unwrap optional callbacks for DismissableLayer (its props are #[prop(into, optional)]
    // so they need concrete Callback values, not Option<Callback>).
    let on_escape_key_down = on_escape_key_down.unwrap_or(Callback::new(|_| {}));
    let on_pointer_down_outside = on_pointer_down_outside.unwrap_or(Callback::new(|_| {}));
    let on_focus_outside_cb = on_focus_outside.unwrap_or(Callback::new(|_| {}));
    let on_interact_outside = on_interact_outside.unwrap_or(Callback::new(|_| {}));
    let on_dismiss = on_dismiss.unwrap_or(Callback::new(|_| {}));

    // Event handlers for keydown, blur, and pointermove must be attached directly to the
    // content element (inner Primitive) via addEventListener, not via on: attributes on
    // <PopperContent>. PopperContent renders a wrapper div as its first DOM element for
    // positioning, and on: handlers set on <PopperContent> land on that wrapper div.
    // This causes event.current_target() to be the wrapper div instead of the content
    // element that has [data-radix-menu-content], breaking is_key_down_inside checks
    // and preventing typeahead from working.
    let keydown_handler = compose_callbacks(
        on_key_down,
        Some(Callback::new(move |event: ev::KeyboardEvent| {
            // Submenu key events bubble through portals. We only care about keys in this menu.
            let target = event
                .target()
                .map(|target| target.unchecked_into::<web_sys::HtmlElement>())
                .expect("Event should have target.");
            let is_key_down_inside = target
                .closest("[data-radix-menu-content]")
                .expect("Element should be able to query closest.")
                == event
                    .current_target()
                    .and_then(|current_target| current_target.dyn_into::<web_sys::Element>().ok());
            let is_modifier_key = event.ctrl_key() || event.alt_key() || event.meta_key();
            let is_character_key = event.key().len() == 1;

            if is_key_down_inside {
                // Menus should not be navigated using tab key so we prevent it.
                if event.key() == "Tab" {
                    event.prevent_default();
                }
                if !is_modifier_key && is_character_key {
                    handle_typeahead_search.run(event.key());
                }
            }

            // Focus first/last item based on key pressed.
            if let Some(content) = content_ref.get_untracked() {
                let content_el: &web_sys::Element = content.unchecked_ref();
                if *content_el == target.unchecked_into::<web_sys::Element>() {
                    if !FIRST_LAST_KEYS.contains(&event.key().as_str()) {
                        return;
                    }

                    event.prevent_default();

                    let items = get_items
                        .try_with_value(|get_items| get_items())
                        .unwrap_or_default();
                    let items = items.iter().filter(|item| !item.data.disabled);
                    let mut candidate_nodes: Vec<web_sys::HtmlElement> = items
                        .filter_map(|item| {
                            item.r#ref.get_untracked().map(|node| node.unchecked_into())
                        })
                        .collect();
                    if LAST_KEYS.contains(&event.key().as_str()) {
                        candidate_nodes.reverse();
                    }
                    focus_first(candidate_nodes);
                }
            }
        })),
        None,
    );
    let blur_handler = compose_callbacks(
        on_blur,
        Some(Callback::new(move |event: ev::FocusEvent| {
            // Clear search buffer when leaving the menu.
            let target = event
                .target()
                .map(|target| target.unchecked_into::<web_sys::Node>())
                .expect("Event should have target.");
            let current_target = event
                .current_target()
                .map(|current_target| current_target.unchecked_into::<web_sys::Node>())
                .expect("Event should have current target.");
            if !current_target.contains(Some(&target)) {
                window().clear_timeout_with_handle(timer.try_get_untracked().unwrap_or(0));
                let _ = search.try_set("".into());
            }
        })),
        None,
    );
    let pointermove_handler = compose_callbacks(
        on_pointer_move,
        Some(when_mouse(move |event: ev::PointerEvent| {
            let target = event
                .target()
                .map(|target| target.unchecked_into::<web_sys::HtmlElement>())
                .expect("Event should have target.");
            let current_target = event
                .current_target()
                .map(|current_target| current_target.unchecked_into::<web_sys::Node>())
                .expect("Event should have current target.");
            let pointer_x_has_changed =
                last_pointer_x.try_get_untracked().unwrap_or(0) != event.client_x();

            // We don't use `event.movementX` for this check because Safari will always return `0` on a pointer event.
            if current_target.contains(Some(&target)) && pointer_x_has_changed {
                let new_dir =
                    match event.client_x() > last_pointer_x.try_get_untracked().unwrap_or(0) {
                        true => Side::Right,
                        false => Side::Left,
                    };
                let _ = pointer_dir.try_set(new_dir);
                let _ = last_pointer_x.try_set(event.client_x());
            }
        })),
        None,
    );

    let keydown_closure: SendWrapper<Rc<RefCell<Option<Closure<dyn Fn(ev::KeyboardEvent)>>>>> =
        SendWrapper::new(Rc::new(RefCell::new(Some(Closure::<
            dyn Fn(ev::KeyboardEvent),
        >::new(keydown_handler)))));
    let blur_closure: SendWrapper<Rc<RefCell<Option<Closure<dyn Fn(ev::FocusEvent)>>>>> =
        SendWrapper::new(Rc::new(RefCell::new(Some(
            Closure::<dyn Fn(ev::FocusEvent)>::new(blur_handler),
        ))));
    let pointermove_closure: SendWrapper<Rc<RefCell<Option<Closure<dyn Fn(ev::PointerEvent)>>>>> =
        SendWrapper::new(Rc::new(RefCell::new(Some(Closure::<
            dyn Fn(ev::PointerEvent),
        >::new(
            pointermove_handler
        )))));

    // Attach event handlers to the content element after mount.
    Effect::new({
        let keydown_closure = keydown_closure.clone();
        let blur_closure = blur_closure.clone();
        let pointermove_closure = pointermove_closure.clone();
        move |_| {
            if let Some(node) = content_ref.get() {
                let el: web_sys::HtmlElement = node.unchecked_into();
                if let Some(c) = keydown_closure.borrow().as_ref() {
                    el.add_event_listener_with_callback("keydown", c.as_ref().unchecked_ref())
                        .ok();
                }
                if let Some(c) = blur_closure.borrow().as_ref() {
                    el.add_event_listener_with_callback("blur", c.as_ref().unchecked_ref())
                        .ok();
                }
                if let Some(c) = pointermove_closure.borrow().as_ref() {
                    el.add_event_listener_with_callback("pointermove", c.as_ref().unchecked_ref())
                        .ok();
                }
            }
        }
    });

    on_cleanup(move || {
        if let Some(node) = content_ref.get_untracked() {
            let el: web_sys::HtmlElement = node.unchecked_into();
            if let Some(c) = keydown_closure.borrow().as_ref() {
                el.remove_event_listener_with_callback("keydown", c.as_ref().unchecked_ref())
                    .ok();
            }
            if let Some(c) = blur_closure.borrow().as_ref() {
                el.remove_event_listener_with_callback("blur", c.as_ref().unchecked_ref())
                    .ok();
            }
            if let Some(c) = pointermove_closure.borrow().as_ref() {
                el.remove_event_listener_with_callback("pointermove", c.as_ref().unchecked_ref())
                    .ok();
            }
        }
    });

    // Scroll lock: prevent scrolling outside the menu when disableOutsideScroll is true.
    // React uses `react-remove-scroll`; we use a simple body overflow approach.
    Effect::new(move |_| {
        if disable_outside_scroll.get().unwrap_or(false)
            && let Some(body) = document().body()
        {
            let style = body.style();
            let prev_overflow = style.get_property_value("overflow").unwrap_or_default();
            style.set_property("overflow", "hidden").ok();

            let style = SendWrapper::new(style);
            on_cleanup(move || {
                if prev_overflow.is_empty() {
                    style.remove_property("overflow").ok();
                } else {
                    style.set_property("overflow", &prev_overflow).ok();
                }
            });
        }
    });

    view! {
        <Provider value=content_context_value>
            <FocusScope
                as_child=true
                trapped=trapped
                on_mount_auto_focus=compose_callbacks(
                    on_open_auto_focus,
                    Some(Callback::new(move |event: ev::Event| {
                        // Always prevent default to take control of focusing.
                        event.prevent_default();

                        if root_context.is_using_keyboard.get_untracked() {
                            // For keyboard users, defer focus to a RAF so that collection items
                            // have time to register (Leptos effects are async, unlike React's
                            // synchronous useEffect). We focus the first menu item directly,
                            // bypassing RovingFocusGroup's entry focus which depends on
                            // collection items being registered.
                            let content_ref = content_ref;
                            let cb = Closure::once_into_js(move || {
                                // Use try_read_untracked to avoid panicking if the
                                // reactive scope has already been disposed by the time
                                // this RAF callback fires.
                                if let Some(guard) = content_ref.try_read_untracked() {
                                    if let Some(ref content) = *guard {
                                        let el: &web_sys::HtmlElement = content.unchecked_ref();
                                        let selector = "[role=menuitem]:not([data-disabled]), \
                                                         [role=menuitemcheckbox]:not([data-disabled]), \
                                                         [role=menuitemradio]:not([data-disabled])";
                                        if let Ok(Some(first_item)) = el.query_selector(selector) {
                                            let first: web_sys::HtmlElement = first_item.unchecked_into();
                                            first.focus().ok();
                                        } else {
                                            el.focus().ok();
                                        }
                                    }
                                }
                            });
                            window().request_animation_frame(cb.unchecked_ref()).ok();
                        } else {
                            // For pointer users, focus the content element so DismissableLayer
                            // works correctly. Don't focus a specific item.
                            // Defer to RAF because PopperContent's attribute transfer Effect
                            // (which moves tabindex from the wrapper div to the inner content
                            // div) may not have run yet. Without tabindex, focus() is a no-op.
                            let content_ref = content_ref;
                            let cb = Closure::once_into_js(move || {
                                // Use try_read_untracked to avoid panicking if the
                                // reactive scope has already been disposed by the time
                                // this RAF callback fires.
                                if let Some(guard) = content_ref.try_read_untracked() {
                                    if let Some(ref content) = *guard {
                                        let content: web_sys::HtmlElement = content.clone().unchecked_into();
                                        content.focus().ok();
                                    }
                                }
                            });
                            window().request_animation_frame(cb.unchecked_ref()).ok();
                        }
                    })),
                    None,
                )
                on_unmount_auto_focus=on_close_auto_focus
            >
                <DismissableLayer
                    as_child=true
                    disable_outside_pointer_events=disable_outside
                    on_escape_key_down=on_escape_key_down
                    on_pointer_down_outside=on_pointer_down_outside
                    on_focus_outside=on_focus_outside_cb
                    on_interact_outside=on_interact_outside
                    on_dismiss=on_dismiss
                >
                    <RovingFocusGroup
                        as_child=true
                        dir=root_context.dir
                        orientation=Orientation::Vertical
                        r#loop=r#loop
                        current_tab_stop_id=current_tab_stop_id_signal
                        on_current_tab_stop_id_change=Callback::new(move |value: Option<String>| {
                            let _ = set_current_item_id.try_set(value);
                        })
                        on_entry_focus=compose_callbacks(on_entry_focus, Some(Callback::new(move |event: ev::Event| {
                            if !root_context.is_using_keyboard.get_untracked() {
                                event.prevent_default();
                            }
                        })), None)
                        prevent_scroll_on_entry_focus=true
                    >
                        <PopperContent
                            side=Signal::derive(move || side.get().unwrap_or(PopperSide::Bottom))
                            side_offset=Signal::derive(move || side_offset.get().unwrap_or(0.0))
                            align=Signal::derive(move || align.get().unwrap_or(Align::Center))
                            align_offset=Signal::derive(move || align_offset.get().unwrap_or(0.0))
                            avoid_collisions=Signal::derive(move || avoid_collisions.get().unwrap_or(true))
                            dir=Signal::derive(move || Some(root_context.dir.get().to_string()))
                            as_child=as_child
                            node_ref=composed_refs
                            attr:class=move || class.get().unwrap_or_default()
                            attr:style=move || {
                                let extra = content_style.get().unwrap_or_default();
                                if extra.is_empty() {
                                    "outline: none;".to_string()
                                } else {
                                    format!("outline: none; {extra}")
                                }
                            }
                            attr:role="menu"
                            attr:aria-orientation="vertical"
                            attr:data-state=move || open_closed_state(context.open.get())
                            attr:data-radix-menu-content=""
                            attr:dir=move || root_context.dir.get()
                            attr:id=move || id.get()
                            attr:aria-labelledby=move || aria_labelledby.get()
                        >
                            {children.with_value(|children| children())}
                        </PopperContent>
                    </RovingFocusGroup>
                </DismissableLayer>
            </FocusScope>
        </Provider>
    }
}

#[component]
pub fn MenuGroup(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attr:role="group"
                {..attrs}
            >
                {children.with_value(|children| children())}
            </Primitive>
        </AttributeInterceptor>
    }
}

#[component]
pub fn MenuLabel(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref=node_ref
        >
            {children()}
        </Primitive>
    }
}

const ITEM_SELECT: &str = "menu.itemSelect";

#[component]
pub fn MenuItem(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_select: Option<Callback<ev::Event>>,
    #[prop(into, optional)] text_value: MaybeProp<String>,
    #[prop(into, optional)] role: MaybeProp<String>,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] on_pointer_down: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_up: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let disabled = prop_or_default(disabled);

    let item_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, item_ref]);
    let root_context = expect_context::<MenuRootContextValue>();
    let content_context = expect_context::<MenuContentContextValue>();
    let is_pointer_down = RwSignal::new(false);

    let handle_select = Callback::new(move |_: ev::MouseEvent| {
        if disabled.get_untracked() {
            return;
        }

        if item_ref.get_untracked().is_none() {
            return;
        }

        // React uses dispatchDiscreteCustomEvent (which wraps in flushSync) to
        // dispatch a custom event and attach on_select as a one-time listener.
        // In Leptos we don't need flushSync, so we call on_select directly with
        // a cancelable CustomEvent — preserving the preventDefault() contract.
        let init = CustomEventInit::new();
        init.set_bubbles(true);
        init.set_cancelable(true);

        let item_select_event = CustomEvent::new_with_event_init_dict(ITEM_SELECT, &init)
            .expect("Item select event should be instantiated.");

        if let Some(on_select) = on_select {
            on_select.run(item_select_event.clone().unchecked_into());
        }

        if item_select_event.default_prevented() {
            let _ = is_pointer_down.try_set(false);
        } else {
            root_context.on_close.run(());
        }
    });

    view! {
        <MenuItemImpl
            disabled={disabled}
            text_value=text_value
            role=role
            as_child=as_child
            node_ref=composed_refs
            on:click=compose_callbacks(on_click, Some(handle_select), None)
            on:pointerdown=move |event| {
                if let Some(on_pointer_down) = on_pointer_down {
                    on_pointer_down.run(event);
                }
                let _ = is_pointer_down.try_set(true);
            }
            on:pointerup=compose_callbacks(on_pointer_up, Some(Callback::new(move |event: ev::PointerEvent| {
                // Pointer down can move to a different menu item which should activate it on pointer up.
                // We dispatch a click for selection to allow composition with click based triggers and to
                // prevent Firefox from getting stuck in text selection mode when the menu closes.
                if !is_pointer_down.try_get_untracked().unwrap_or(false)
                    && let Some(current_target) = event.current_target().map(|current_target| current_target.unchecked_into::<web_sys::HtmlElement>())
                {
                    current_target.click();
                }
            })), None)
            on:keydown=compose_callbacks(on_key_down, Some(Callback::new(move |event: ev::KeyboardEvent| {
                let is_typing_ahead = !content_context.search.try_get_untracked().unwrap_or_default().is_empty();
                if disabled.get_untracked() || (is_typing_ahead && event.key() == " ") {
                    return;
                }
                if SELECTION_KEYS.contains(&event.key().as_str()) {
                    let current_target = event.current_target().map(|current_target| current_target.unchecked_into::<web_sys::HtmlElement>()).expect("Event should have current target.");
                    current_target.click();

                    // We prevent default browser behaviour for selection keys as they should trigger a selection only:
                    // - prevents space from scrolling the page.
                    // - if keydown causes focus to move, prevents keydown from firing on the new target.
                    event.prevent_default();
                }
            })), None)
        >
            {children()}
        </MenuItemImpl>
    }
}

#[component]
fn MenuItemImpl(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] text_value: MaybeProp<String>,
    #[prop(into, optional)] role: MaybeProp<String>,
    #[prop(into, optional)] on_pointer_move: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_leave: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_focus: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] on_blur: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let disabled = prop_or_default(disabled);

    let content_context = expect_context::<MenuContentContextValue>();
    let item_ref = AnyNodeRef::new();
    let composed_ref = use_composed_refs(vec![node_ref, item_ref]);
    let (is_focused, set_is_focused) = signal(false);

    // Get the item's `.textContent` as default strategy for typeahead `textValue`.
    let (text_content, set_text_content) = signal("".to_string());
    Effect::new(move |_| {
        if let Some(item) = item_ref.get() {
            let item: web_sys::HtmlElement = item.unchecked_into();
            set_text_content.set(item.text_content().unwrap_or("".into()).trim().to_string());
        }
    });

    let item_data = Signal::derive(move || ItemData {
        disabled: disabled.get(),
        text_value: text_value.get().unwrap_or(text_content.get()),
    });

    let children = StoredValue::new(children);

    view! {
        <CollectionItemSlot item_data_type=ITEM_DATA_PHANTHOM item_data=item_data>
            <RovingFocusGroupItem as_child=true focusable=Signal::derive(move || !disabled.get())>
                <AttributeInterceptor let:attrs>
                    <Primitive
                        element=html::div
                        as_child=as_child
                        node_ref=composed_ref
                        attr:role=move || role.get().unwrap_or("menuitem".into())
                        attr:data-highlighted=data_attr(is_focused.into())
                        attr:aria-disabled=move || disabled.get().then_some("true")
                        attr:data-disabled=data_attr(disabled)
                        /*
                        * We focus items on `pointermove` to achieve the following:
                        *
                        * - Mouse over an item (it focuses)
                        * - Leave mouse where it is and use keyboard to focus a different item
                        * - Wiggle mouse without it leaving previously focused item
                        * - Previously focused item should re-focus
                        *
                        * If we used `mouseover`/`mouseenter` it would not re-focus when the mouse
                        * wiggles. This is to match native menu implementation.
                        */
                        on:pointermove=compose_callbacks(on_pointer_move, Some(when_mouse(move |event| {
                            if disabled.get_untracked() {
                                content_context.on_item_leave.run(event);
                            } else {
                                content_context.on_item_enter.run(event.clone());
                                if !event.default_prevented() {
                                    let item = event.current_target().map(|target| target.unchecked_into::<web_sys::HtmlElement>()).expect("Current target should exist.");
                                    // TODO: focus options
                                    item.focus().expect("Element should be focused.");
                                }
                            }
                        })), None)
                        on:pointerleave=compose_callbacks(on_pointer_leave, Some(when_mouse(move |event| {
                            content_context.on_item_leave.run(event);
                        })), None)
                        on:focus=compose_callbacks(on_focus, Some(Callback::new(move |_| {
                            let _ = set_is_focused.try_set(true);
                        })), None)
                        on:blur=compose_callbacks(on_blur, Some(Callback::new(move |_| {
                            let _ = set_is_focused.try_set(false);
                        })), None)
                        {..attrs}
                    >
                        {children.with_value(|children| children())}
                    </Primitive>
                </AttributeInterceptor>
            </RovingFocusGroupItem>
        </CollectionItemSlot>
    }
}

#[component]
pub fn MenuCheckboxItem(
    #[prop(into, optional)] checked: MaybeProp<CheckedState>,
    #[prop(into, optional)] on_checked_change: Option<Callback<bool>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_select: Option<Callback<ev::Event>>,
    #[prop(into, optional)] text_value: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let checked = prop_or(checked, CheckedState::False);

    let indicator_context = ItemIndicatorContextValue { checked };

    // Compose on_select: always run our toggle regardless of preventDefault (checkForDefaultPrevented: false).
    let composed_select = Callback::new(move |event: ev::Event| {
        if let Some(on_select) = on_select {
            on_select.run(event);
        }
        if let Some(on_checked_change) = on_checked_change {
            on_checked_change.run(if is_indeterminate(checked.get_untracked()) {
                true
            } else {
                checked.get_untracked() != CheckedState::True
            });
        }
    });

    view! {
        <Provider value=indicator_context>
            <MenuItem
                role="menuitemcheckbox"
                disabled=disabled
                text_value=text_value
                as_child=as_child
                node_ref=node_ref
                attr:aria-checked=move || {
                    if is_indeterminate(checked.get()) {
                        "mixed".to_string()
                    } else {
                        (checked.get() == CheckedState::True).to_string()
                    }
                }
                attr:data-state=move || get_checked_state(checked.get())
                on_select=composed_select
            >
                {children()}
            </MenuItem>
        </Provider>
    }
}

#[component]
pub fn MenuRadioGroup(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let radio_group_context = RadioGroupContextValue {
        value: Signal::derive(move || value.get()),
        on_value_change: on_value_change.unwrap_or(Callback::new(|_| {})),
    };

    view! {
        <Provider value=radio_group_context>
            <MenuGroup as_child=as_child node_ref=node_ref>
                {children()}
            </MenuGroup>
        </Provider>
    }
}

#[component]
pub fn MenuRadioItem(
    #[prop(into)] value: MaybeProp<String>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_select: Option<Callback<ev::Event>>,
    #[prop(into, optional)] text_value: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let radio_context = expect_context::<RadioGroupContextValue>();
    let checked = Signal::derive(move || {
        let v = value.get().unwrap_or_default();
        radio_context
            .value
            .get()
            .is_some_and(|ctx_val| ctx_val == v)
    });
    let checked_state = Signal::derive(move || CheckedState::from(checked.get()));

    let indicator_context = ItemIndicatorContextValue {
        checked: checked_state,
    };

    let composed_select = Callback::new(move |event: ev::Event| {
        if let Some(on_select) = on_select {
            on_select.run(event);
        }
        if let Some(v) = value.get_untracked() {
            radio_context.on_value_change.run(v);
        }
    });

    view! {
        <Provider value=indicator_context>
            <MenuItem
                role="menuitemradio"
                disabled=disabled
                text_value=text_value
                as_child=as_child
                node_ref=node_ref
                attr:aria-checked=move || checked.get().to_string()
                attr:data-state=move || get_checked_state(checked_state.get())
                on_select=composed_select
            >
                {children()}
            </MenuItem>
        </Provider>
    }
}

#[component]
pub fn MenuItemIndicator(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let context = expect_context::<ItemIndicatorContextValue>();

    let present = Signal::derive(move || {
        force_mount.get().unwrap_or(false)
            || is_indeterminate(context.checked.get())
            || context.checked.get() == CheckedState::True
    });

    let children = StoredValue::new(children);

    view! {
        <Presence present=present node_ref=node_ref>
            <AttributeInterceptor let:attrs>
                <Primitive
                    element=html::span
                    as_child=as_child
                    node_ref=node_ref
                    attr:data-state=move || get_checked_state(context.checked.get())
                    {..attrs}
                >
                    {children.with_value(|children| children.as_ref().map(|children| children()))}
                </Primitive>
            </AttributeInterceptor>
        </Presence>
    }
}

#[component]
pub fn MenuSeparator(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attr:role="separator"
                attr:aria-orientation="horizontal"
                {..attrs}
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </Primitive>
        </AttributeInterceptor>
    }
}

#[component]
pub fn MenuArrow(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <PopperArrow
            as_child=as_child
            node_ref=node_ref
        >
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </PopperArrow>
    }
}

#[component]
pub fn MenuSub(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let parent_context = expect_context::<MenuContextValue>();
    let on_open_change = on_open_change.unwrap_or(Callback::new(|_| {}));
    let open = prop_or_default(open);

    let content_ref = AnyNodeRef::new();
    let trigger_ref = AnyNodeRef::new();
    let content_id = use_id(None);
    let trigger_id = use_id(None);

    let menu_context = MenuContextValue {
        open,
        content_ref,
        on_open_change,
        // Sub-menus don't have their own Popper (they use the parent's), but
        // MenuAnchor still expects this field. Use an independent ref for the
        // sub-trigger anchor.
        popper_anchor_ref: AnyNodeRef::new(),
    };

    let sub_context = MenuSubContextValue {
        content_id,
        trigger_id,
        trigger_ref,
    };

    // Close submenu when parent closes.
    Effect::new(move |_| {
        if !parent_context.open.get() {
            on_open_change.run(false);
        }
    });

    on_cleanup(move || {
        on_open_change.run(false);
    });

    view! {
        <Popper>
            <Provider value=menu_context>
                <Provider value=sub_context>
                    {children.with_value(|children| children())}
                </Provider>
            </Provider>
        </Popper>
    }
}

#[component]
pub fn MenuSubTrigger(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] text_value: MaybeProp<String>,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] on_pointer_move: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_leave: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<MenuContextValue>();
    let root_context = expect_context::<MenuRootContextValue>();
    let sub_context = expect_context::<MenuSubContextValue>();
    let content_context = expect_context::<MenuContentContextValue>();
    let open_timer: RwSignal<Option<i32>> = RwSignal::new(None);
    let disabled = prop_or_default(disabled);

    let composed_refs = use_composed_refs(vec![node_ref, sub_context.trigger_ref]);

    let clear_open_timer = move || {
        if let Some(timer_id) = open_timer.try_get_untracked().flatten() {
            window().clear_timeout_with_handle(timer_id);
            let _ = open_timer.try_set(None);
        }
    };

    on_cleanup(move || {
        clear_open_timer();
    });

    // Clean up grace area on unmount.
    on_cleanup(move || {
        window().clear_timeout_with_handle(
            content_context
                .pointer_grace_timer
                .try_get_untracked()
                .unwrap_or(0) as i32,
        );
        content_context.on_pointer_grace_intent_change.run(None);
    });

    view! {
        <MenuAnchor as_child=true>
            <MenuItemImpl
                disabled=disabled
                text_value=text_value
                as_child=as_child
                node_ref=composed_refs
                attr:id=move || sub_context.trigger_id.get()
                attr:aria-haspopup="menu"
                attr:aria-expanded=move || context.open.get().to_string()
                attr:aria-controls=move || sub_context.content_id.get()
                attr:data-state=move || open_closed_state(context.open.get())
                on:click=compose_callbacks(on_click, Some(Callback::new(move |event: ev::MouseEvent| {
                    if disabled.get_untracked() || event.default_prevented() {
                        return;
                    }
                    // We manually focus because iOS Safari doesn't always focus on click (e.g. buttons)
                    // and we rely heavily on `onFocusOutside` for submenus to close when switching
                    // between separate submenus.
                    if let Some(current_target) = event.current_target() {
                        let el: web_sys::HtmlElement = current_target.unchecked_into();
                        el.focus().ok();
                    }
                    if !context.open.get_untracked() {
                        context.on_open_change.run(true);
                    }
                })), None)
                // Pass pointer handlers as props (not `on:` directives) so they compose
                // with MenuItemImpl's internal handlers via compose_callbacks. This ensures
                // SubTrigger's grace area logic runs BEFORE MenuItemImpl's on_item_leave
                // checks it (compose_callbacks runs the prop handler first, then internal).
                on_pointer_move=Callback::new(compose_callbacks(on_pointer_move, Some(when_mouse(move |event: ev::PointerEvent| {
                    content_context.on_item_enter.run(event.clone());
                    if event.default_prevented() {
                        return;
                    }
                    if !disabled.get_untracked() && !context.open.get_untracked() && open_timer.try_get_untracked().flatten().is_none() {
                        content_context.on_pointer_grace_intent_change.run(None);
                        let timer_id = window()
                            .set_timeout_with_callback_and_timeout_and_arguments_0(
                                Closure::once_into_js(move || {
                                    context.on_open_change.run(true);
                                    let _ = open_timer.try_set(None);
                                })
                                .unchecked_ref(),
                                100,
                            )
                            .expect("Timeout should be set.");
                        let _ = open_timer.try_set(Some(timer_id));
                    }
                })), None))
                on_pointer_leave=Callback::new(compose_callbacks(on_pointer_leave, Some(when_mouse(move |event: ev::PointerEvent| {
                    clear_open_timer();

                    if let Some(content) = context.content_ref.get_untracked() {
                        let content_el: web_sys::HtmlElement = content.unchecked_into();
                        let content_rect = content_el.get_bounding_client_rect();
                        let data_side = content_el
                            .unchecked_ref::<web_sys::Element>()
                            .get_attribute("data-side");
                        let right_side = data_side.as_deref() == Some("right");
                        let bleed = if right_side { -5.0 } else { 5.0 };
                        let content_near_edge = if right_side {
                            content_rect.left()
                        } else {
                            content_rect.right()
                        };
                        let content_far_edge = if right_side {
                            content_rect.right()
                        } else {
                            content_rect.left()
                        };

                        content_context.on_pointer_grace_intent_change.run(Some(GraceIntent {
                            area: vec![
                                Point { x: event.client_x() as f64 + bleed, y: event.client_y() as f64 },
                                Point { x: content_near_edge, y: content_rect.top() },
                                Point { x: content_far_edge, y: content_rect.top() },
                                Point { x: content_far_edge, y: content_rect.bottom() },
                                Point { x: content_near_edge, y: content_rect.bottom() },
                            ],
                            side: if right_side { Side::Right } else { Side::Left },
                        }));

                        window().clear_timeout_with_handle(content_context.pointer_grace_timer.try_get_untracked().unwrap_or(0) as i32);
                        let timer_id = window()
                            .set_timeout_with_callback_and_timeout_and_arguments_0(
                                Closure::once_into_js(move || {
                                    content_context.on_pointer_grace_intent_change.run(None);
                                })
                                .unchecked_ref(),
                                300,
                            )
                            .expect("Timeout should be set.");
                        let _ = content_context.pointer_grace_timer.try_set(timer_id as u64);
                    } else {
                        content_context.on_trigger_leave.run(event.clone());
                        if event.default_prevented() {
                            return;
                        }
                        // There's 100ms where the user may leave an item before the submenu was opened.
                        content_context.on_pointer_grace_intent_change.run(None);
                    }
                })), None))
                on:keydown=compose_callbacks(on_key_down, Some(Callback::new(move |event: ev::KeyboardEvent| {
                    let is_typing_ahead = !content_context.search.try_get_untracked().unwrap_or_default().is_empty();
                    if disabled.get_untracked() || (is_typing_ahead && event.key() == " ") {
                        return;
                    }
                    if sub_open_keys(root_context.dir.get_untracked()).contains(&event.key().as_str()) {
                        context.on_open_change.run(true);
                        // Focus the first menu item in the submenu via RAF because:
                        // 1. The content element may not be fully mounted/registered yet.
                        // 2. When Presence keeps the element in DOM during exit animation
                        //    and the submenu is reopened, FocusScope's on_mount_auto_focus
                        //    Effect won't re-fire (container ref unchanged), so we must
                        //    handle first-item focus ourselves.
                        if let Some(content) = context.content_ref.get_untracked() {
                            let content: web_sys::HtmlElement = content.unchecked_into();
                            let cb = Closure::once_into_js(move || {
                                let selector = "[role=menuitem]:not([data-disabled]), \
                                                 [role=menuitemcheckbox]:not([data-disabled]), \
                                                 [role=menuitemradio]:not([data-disabled])";
                                if let Ok(Some(first_item)) = content.query_selector(selector) {
                                    let first: web_sys::HtmlElement = first_item.unchecked_into();
                                    first.focus().ok();
                                } else {
                                    content.focus().ok();
                                }
                            });
                            window().request_animation_frame(cb.unchecked_ref()).ok();
                        }
                        event.prevent_default();
                    }
                })), None)
            >
                {children.with_value(|children| children())}
            </MenuItemImpl>
        </MenuAnchor>
    }
}

#[component]
pub fn MenuSubContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] side_offset: MaybeProp<f64>,
    #[prop(into, optional)] align_offset: MaybeProp<f64>,
    #[prop(into, optional)] avoid_collisions: MaybeProp<bool>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<CustomEvent>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    /// CSS class applied directly to the inner content element (same element as data-state).
    #[prop(into, optional)]
    class: MaybeProp<String>,
    /// Additional inline styles for the content element (e.g., CSS custom property aliases).
    #[prop(into, optional)]
    content_style: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<MenuContextValue>();
    let root_context = expect_context::<MenuRootContextValue>();
    let sub_context = expect_context::<MenuSubContextValue>();
    let content_ref = AnyNodeRef::new();
    // Include context.content_ref so MenuSubTrigger can focus the content element via context.
    let composed_refs = use_composed_refs(vec![node_ref, content_ref, context.content_ref]);

    let present = Signal::derive(move || force_mount.get().unwrap_or(false) || context.open.get());

    let sub_side = Signal::derive(move || match root_context.dir.get() {
        Direction::Rtl => PopperSide::Left,
        Direction::Ltr => PopperSide::Right,
    });

    view! {
        <CollectionProvider item_data_type=ITEM_DATA_PHANTHOM>
            <Presence present=present node_ref=composed_refs>
                <CollectionSlot item_data_type=ITEM_DATA_PHANTHOM>
                    <MenuContentImpl
                        id=Signal::derive(move || Some(sub_context.content_id.get()))
                        aria_labelledby=Signal::derive(move || Some(sub_context.trigger_id.get()))
                        side=sub_side
                        side_offset=side_offset
                        align=Align::Start
                        align_offset=align_offset
                        avoid_collisions=avoid_collisions
                        disable_outside_pointer_events=false
                        disable_outside_scroll=false
                        trap_focus=false
                        class=class
                        content_style=content_style
                        as_child=as_child
                        node_ref=composed_refs
                        on_open_auto_focus=Callback::new(move |event: ev::Event| {
                            // When opening a submenu, focus content for keyboard users only.
                            // We defer to RAF because the content element and its children may not
                            // be fully mounted/registered when this callback fires (Leptos Effect
                            // timing differs from React's useEffect). We focus the first focusable
                            // menu item directly rather than the content element, bypassing
                            // RovingFocusGroup's entry focus which depends on collection items
                            // being registered (a timing-sensitive operation in Leptos).
                            if root_context.is_using_keyboard.get_untracked() {
                                let content_ref = content_ref;
                                let cb = Closure::once_into_js(move || {
                                    if let Some(content) = content_ref.get_untracked() {
                                        let el: &web_sys::HtmlElement = content.unchecked_ref();
                                        // Focus first non-disabled item directly.
                                        let selector = "[role=menuitem]:not([data-disabled]), \
                                                         [role=menuitemcheckbox]:not([data-disabled]), \
                                                         [role=menuitemradio]:not([data-disabled])";
                                        if let Ok(Some(first_item)) = el.query_selector(selector) {
                                            let first: web_sys::HtmlElement = first_item.unchecked_into();
                                            first.focus().ok();
                                        } else {
                                            // Fallback: focus the content element itself.
                                            el.focus().ok();
                                        }
                                    }
                                });
                                window().request_animation_frame(cb.unchecked_ref()).ok();
                            }
                            event.prevent_default();
                        })
                        on_close_auto_focus=Callback::new(move |event: ev::Event| {
                            // The menu might close because of focusing another menu item in the parent menu. We
                            // don't want it to refocus the trigger in that case so we handle trigger focus ourselves.
                            event.prevent_default();
                        })
                        on_focus_outside=compose_callbacks(on_focus_outside, Some(Callback::new(move |event: CustomEvent| {
                            // We prevent closing when the trigger is focused to avoid triggering a re-open animation
                            // on pointer interaction.
                            let target = event.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok());
                            let trigger = sub_context.trigger_ref.get_untracked().map(|n| {
                                let el: web_sys::Element = n.unchecked_into();
                                el
                            });
                            if target.as_ref() != trigger.as_ref() {
                                context.on_open_change.run(false);
                            }
                        })), Some(false))
                        on_escape_key_down=compose_callbacks(on_escape_key_down, Some(Callback::new(move |event: ev::KeyboardEvent| {
                            root_context.on_close.run(());
                            // Ensure pressing escape in submenu doesn't escape full screen mode.
                            event.prevent_default();
                        })), None)
                        on_key_down=compose_callbacks(on_key_down, Some(Callback::new(move |event: ev::KeyboardEvent| {
                            // Submenu key events bubble through portals. We only care about keys in this menu.
                            let is_key_down_inside = event.current_target()
                                .and_then(|ct| ct.dyn_into::<web_sys::Node>().ok())
                                .zip(event.target().and_then(|t| t.dyn_into::<web_sys::Node>().ok()))
                                .is_some_and(|(ct, target)| ct.contains(Some(&target)));
                            let is_close_key = sub_close_keys(root_context.dir.get_untracked()).contains(&event.key().as_str());
                            if is_key_down_inside && is_close_key {
                                context.on_open_change.run(false);
                                // We focus manually because we prevented it in `on_close_auto_focus`.
                                if let Some(trigger) = sub_context.trigger_ref.get_untracked() {
                                    let trigger: web_sys::HtmlElement = trigger.unchecked_into();
                                    trigger.focus().ok();
                                }
                                event.prevent_default();
                            }
                        })), None)
                    >
                        {children.with_value(|children| children())}
                    </MenuContentImpl>
                </CollectionSlot>
            </Presence>
        </CollectionProvider>
    }
}

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

/// Wraps an array around itself at a given start index.
fn wrap_array<T: Clone>(array: &mut [T], start_index: usize) -> &[T] {
    array.rotate_left(start_index);
    array
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

#[derive(Clone, Debug)]
struct Point {
    x: f64,
    y: f64,
}

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

/// Determine if a point is inside of a polygon.
fn is_point_in_polygon(point: Point, polygon: Polygon) -> bool {
    let Point { x, y } = point;
    let mut inside = false;

    let mut i = 0;
    let mut j = polygon.len() - 1;
    while i < polygon.len() {
        let xi = polygon[i].x;
        let yi = polygon[i].y;
        let xj = polygon[j].x;
        let yj = polygon[j].y;

        let intersect = ((yi > y) != (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi) + xi);
        if intersect {
            inside = !inside;
        }

        j = i;
        i += 1;
    }

    inside
}

fn is_pointer_in_grace_area(event: &ev::PointerEvent, area: Option<Polygon>) -> bool {
    if let Some(area) = area {
        let cursor_pos = Point {
            x: event.client_x() as f64,
            y: event.client_y() as f64,
        };
        is_point_in_polygon(cursor_pos, area)
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
