use std::cell::Cell;
use std::marker::PhantomData;
use std::rc::Rc;

use crate::collection::{
    CollectionItemSlot, CollectionItemValue, CollectionProvider, CollectionSlot, use_collection,
};
use crate::compose_refs::use_composed_refs;
use crate::direction::{Direction, use_direction};
use crate::id::use_id;
pub use crate::menu::CheckedState;
use crate::menu::*;
use crate::primitive::{
    Primitive, compose_callbacks, data_attr, prop_or, prop_or_default, wrap_callback,
};
use crate::roving_focus::{Orientation, RovingFocusGroup, RovingFocusGroupItem};
use crate::use_controllable_state::{UseControllableStateParams, use_controllable_state};
use leptos::{context::Provider, ev, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use radix_utils::wrap_array;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::JsCast;

/* -------------------------------------------------------------------------------------------------
 * Menubar
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Debug)]
struct ItemData {
    value: String,
    disabled: bool,
}

const ITEM_DATA_PHANTOM: PhantomData<ItemData> = PhantomData;

#[derive(Clone)]
struct MenubarContextValue {
    value: Signal<String>,
    dir: Signal<Direction>,
    r#loop: Signal<bool>,
    on_menu_open: Callback<String>,
    on_menu_close: Callback<()>,
    on_menu_toggle: Callback<String>,
}

#[component]
pub fn Menubar(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] r#loop: MaybeProp<bool>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let direction = use_direction(dir);
    let r#loop = prop_or(r#loop, true);

    let (value_state, set_value_state) = use_controllable_state(UseControllableStateParams {
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

    let value_signal = Signal::derive(move || value_state.get().unwrap_or_default());

    // We need to manage tab stop id manually as `RovingFocusGroup` updates the stop
    // based on focus, and in some situations our triggers won't ever be given focus
    // (e.g. click to open and then outside to close)
    let current_tab_stop_id = RwSignal::new(None::<String>);

    let context = MenubarContextValue {
        value: value_signal,
        dir: direction,
        r#loop,
        on_menu_open: Callback::new(move |value: String| {
            set_value_state.run(Some(value.clone()));
            current_tab_stop_id.set(Some(value));
        }),
        on_menu_close: Callback::new(move |_: ()| {
            set_value_state.run(Some(String::new()));
        }),
        on_menu_toggle: Callback::new(move |value: String| {
            let prev = value_signal.get_untracked();
            let new_val = if !prev.is_empty() {
                String::new()
            } else {
                value.clone()
            };
            set_value_state.run(Some(new_val));
            // `onMenuOpen` and `onMenuToggle` are called exclusively so we
            // need to update the id in either case.
            current_tab_stop_id.set(Some(value));
        }),
    };

    view! {
        <Provider value=context>
            <CollectionProvider item_data_type=ITEM_DATA_PHANTOM>
                <CollectionSlot item_data_type=ITEM_DATA_PHANTOM>
                    <RovingFocusGroup
                        as_child=true
                        orientation=Orientation::Horizontal
                        r#loop=r#loop
                        dir=direction
                        current_tab_stop_id=Signal::derive(move || current_tab_stop_id.get())
                        on_current_tab_stop_id_change=Callback::new(move |id: Option<String>| {
                            current_tab_stop_id.set(id);
                        })
                    >
                        <Primitive
                            element=html::div
                            as_child=as_child
                            node_ref=node_ref
                            attr:role="menubar"
                        >
                            {children.with_value(|children| children())}
                        </Primitive>
                    </RovingFocusGroup>
                </CollectionSlot>
            </CollectionProvider>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * MenubarMenu
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone)]
struct MenubarMenuContextValue {
    value: String,
    trigger_id: ReadSignal<String>,
    trigger_ref: AnyNodeRef,
    content_id: ReadSignal<String>,
    was_keyboard_trigger_open_ref: SendWrapper<Rc<Cell<bool>>>,
}

#[component]
pub fn MenubarMenu(
    #[prop(into, optional)] value: Option<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let auto_value = use_id(None);
    // We need to provide an initial deterministic value as `use_id` will return
    // empty string on the first render and we don't want to match our internal "closed" value.
    let value = value.unwrap_or_else(|| auto_value.get_untracked());
    let context = expect_context::<MenubarContextValue>();
    let trigger_ref = AnyNodeRef::new();
    let was_keyboard_trigger_open_ref = SendWrapper::new(Rc::new(Cell::new(false)));
    let open = {
        let value = value.clone();
        Signal::derive(move || context.value.get() == value)
    };

    let trigger_id = use_id(None);
    let content_id = use_id(None);

    {
        let was_keyboard_trigger_open_ref = was_keyboard_trigger_open_ref.clone();
        Effect::new(move |_| {
            if !open.get() {
                was_keyboard_trigger_open_ref.set(false);
            }
        });
    }

    let menu_context = MenubarMenuContextValue {
        value: value.clone(),
        trigger_id,
        trigger_ref,
        content_id,
        was_keyboard_trigger_open_ref,
    };

    view! {
        <Provider value=menu_context>
            <Menu
                open=open
                on_open_change=Callback::new(move |open: bool| {
                    // Menu only calls `onOpenChange` when dismissing so we
                    // want to close our MenuBar based on the same events.
                    if !open {
                        context.on_menu_close.run(());
                    }
                })
                modal=false
                dir=context.dir
            >
                {children.with_value(|children| children())}
            </Menu>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * MenubarTrigger
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn MenubarTrigger(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context = expect_context::<MenubarContextValue>();
    let menu_context = expect_context::<MenubarMenuContextValue>();
    let trigger_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, trigger_ref, menu_context.trigger_ref]);

    let disabled = prop_or_default(disabled);
    let is_focused = RwSignal::new(false);

    let value = StoredValue::new(menu_context.value.clone());
    let open = Signal::derive(move || context.value.get() == value.get_value());

    let was_keyboard = StoredValue::new(menu_context.was_keyboard_trigger_open_ref.clone());

    view! {
        <CollectionItemSlot
            item_data_type=ITEM_DATA_PHANTOM
            item_data=Signal::derive(move || ItemData { value: value.get_value(), disabled: disabled.get() })
        >
            <RovingFocusGroupItem
                as_child=true
                focusable=Signal::derive(move || !disabled.get())
                tab_stop_id=Signal::derive(move || value.get_value())
            >
                <MenuAnchor as_child=true>
                    <Primitive
                        element=html::button
                        as_child=as_child
                        node_ref=composed_refs
                        attr:r#type="button"
                        attr:role="menuitem"
                        attr:id=move || menu_context.trigger_id.get()
                        attr:aria-haspopup="menu"
                        attr:aria-expanded=move || open.get().to_string()
                        attr:aria-controls=move || open.get().then(|| menu_context.content_id.get())
                        attr:data-highlighted=data_attr(is_focused.into())
                        attr:data-state=move || if open.get() { "open" } else { "closed" }
                        attr:data-disabled=data_attr(disabled)
                        attr:disabled=data_attr(disabled)
                        on:pointerdown=move |event: ev::PointerEvent| {
                            // only call handler if it's the left button (mousedown gets triggered by all mouse buttons)
                            // but not when the control key is pressed (avoiding MacOS right click)
                            if !disabled.get_untracked() && event.button() == 0 && !event.ctrl_key() {
                                context.on_menu_open.run(value.get_value());
                                // prevent trigger focusing when opening
                                // this allows the content to be given focus without competition
                                if !open.get_untracked() {
                                    event.prevent_default();
                                }
                            }
                        }
                        on:pointerenter=move |_event: ev::PointerEvent| {
                            let menubar_open = !context.value.get_untracked().is_empty();
                            if menubar_open && !open.get_untracked() {
                                context.on_menu_open.run(value.get_value());
                                if let Some(el) = trigger_ref.get() {
                                    let el: web_sys::HtmlElement = el.unchecked_into();
                                    el.focus().ok();
                                }
                            }
                        }
                        on:keydown=move |event: ev::KeyboardEvent| {
                            if disabled.get_untracked() {
                                return;
                            }
                            if event.key() == "Enter" || event.key() == " " {
                                context.on_menu_toggle.run(value.get_value());
                            }
                            if event.key() == "ArrowDown" {
                                context.on_menu_open.run(value.get_value());
                            }
                            // prevent keydown from scrolling window / first focused item to execute
                            // that keydown (inadvertently closing the menu)
                            if event.key() == "Enter" || event.key() == " " || event.key() == "ArrowDown" {
                                was_keyboard.with_value(|w| w.set(true));
                                event.prevent_default();
                            }
                        }
                        on:focus=move |_| is_focused.set(true)
                        on:blur=move |_| is_focused.set(false)
                    >
                        {children.with_value(|children| children())}
                    </Primitive>
                </MenuAnchor>
            </RovingFocusGroupItem>
        </CollectionItemSlot>
    }
}

/* -------------------------------------------------------------------------------------------------
 * MenubarPortal
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn MenubarPortal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    #[prop(optional)] container_ref: AnyNodeRef,
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <MenuPortal container=container container_ref=container_ref force_mount=force_mount>
            {children.with_value(|children| children())}
        </MenuPortal>
    }
}

/* -------------------------------------------------------------------------------------------------
 * MenubarContent
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn MenubarContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] align: MaybeProp<Align>,
    #[prop(into, optional)] align_offset: MaybeProp<f64>,
    #[prop(into, optional)] avoid_collisions: MaybeProp<bool>,
    #[prop(into, optional)] side: MaybeProp<PopperSide>,
    #[prop(into, optional)] side_offset: MaybeProp<f64>,
    #[prop(into, optional)] on_close_auto_focus: Option<Callback<ev::Event>>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] on_interact_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let context = expect_context::<MenubarContextValue>();
    let menu_context = expect_context::<MenubarMenuContextValue>();
    // Get the collection items at this scope level (before MenuContent provides its own).
    let get_items = StoredValue::new(use_collection::<ItemData>());

    let has_interacted_outside_ref = SendWrapper::new(Rc::new(Cell::new(false)));

    let content_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, content_ref]);

    // Set data-radix-menubar-content via Effect so it lands on the inner content
    // element (where the keydown handler is attached), not the outer PopperContent
    // wrapper. This matches the pattern used by MenubarSubContent.
    Effect::new(move |_| {
        if let Some(el) = content_ref.get() {
            let el: &web_sys::Element = (*el).unchecked_ref();
            el.set_attribute("data-radix-menubar-content", "").ok();
        }
    });

    let align = prop_or(align, Align::Start);

    // Wrap pass-through callbacks for view! macro forwarding.
    let on_escape_key_down = wrap_callback(on_escape_key_down);
    let on_pointer_down_outside = wrap_callback(on_pointer_down_outside);

    let was_keyboard = menu_context.was_keyboard_trigger_open_ref.clone();
    let menu_value = menu_context.value.clone();

    view! {
        <MenuContent
            force_mount=force_mount
            class=class
            as_child=as_child
            node_ref=composed_refs
            id=Signal::derive(move || Some(menu_context.content_id.get()))
            aria_labelledby=Signal::derive(move || Some(menu_context.trigger_id.get()))
            align=align
            align_offset=align_offset
            avoid_collisions=avoid_collisions
            side=side
            side_offset=side_offset
            content_style="--radix-menubar-content-transform-origin: var(--radix-popper-transform-origin); --radix-menubar-content-available-width: var(--radix-popper-available-width); --radix-menubar-content-available-height: var(--radix-popper-available-height); --radix-menubar-trigger-width: var(--radix-popper-anchor-width); --radix-menubar-trigger-height: var(--radix-popper-anchor-height);"
            on_close_auto_focus=compose_callbacks(
                on_close_auto_focus,
                Some(Callback::new({
                    let has_interacted_outside_ref = has_interacted_outside_ref.clone();
                    let trigger_ref = menu_context.trigger_ref;
                    move |event: ev::Event| {
                        let menubar_open = !context.value.get_untracked().is_empty();
                        if !menubar_open
                            && !has_interacted_outside_ref.get()
                            && let Some(trigger) = trigger_ref.get_untracked()
                        {
                            let el: web_sys::HtmlElement = trigger.unchecked_into();
                            el.focus().ok();
                        }
                        has_interacted_outside_ref.set(false);
                        // Always prevent auto focus because we either focus manually or want user agent focus
                        event.prevent_default();
                    }
                })),
                None,
            )
            on_focus_outside=compose_callbacks(
                on_focus_outside,
                Some(Callback::new(move |event: web_sys::CustomEvent| {
                    if let Some(target) = event.target() {
                        let target: web_sys::Element = target.unchecked_into();
                        let is_menubar_trigger = get_items.with_value(|get_items| {
                            get_items().iter().any(|item| {
                                item.r#ref
                                    .get_untracked()
                                    .map(|el| {
                                        let node: &web_sys::Node = (*el).unchecked_ref();
                                        node.contains(Some(&target))
                                    })
                                    .unwrap_or(false)
                            })
                        });
                        if is_menubar_trigger {
                            event.prevent_default();
                        }
                    }
                })),
                Some(false),
            )
            on_interact_outside=compose_callbacks(
                on_interact_outside,
                Some(Callback::new({
                    let has_interacted_outside_ref = has_interacted_outside_ref.clone();
                    move |_event: web_sys::CustomEvent| {
                        has_interacted_outside_ref.set(true);
                    }
                })),
                None,
            )
            on_entry_focus=Callback::new({
                let was_keyboard = was_keyboard.clone();
                move |event: ev::Event| {
                    if !was_keyboard.get() {
                        event.prevent_default();
                    }
                }
            })
            on_key_down=compose_callbacks(
                on_key_down,
                Some(Callback::new({
                    let menu_value = menu_value.clone();
                    move |event: ev::KeyboardEvent| {
                        if event.key() == "ArrowRight" || event.key() == "ArrowLeft" {
                            let Some(target) = event.target() else { return; };
                            let target: web_sys::Element = target.unchecked_into();
                            let target_is_sub_trigger = target.has_attribute("data-radix-menubar-subtrigger");
                            let Some(current_target) = event.current_target() else { return; };
                            let current_target: web_sys::Element = current_target.unchecked_into();
                            let is_key_down_inside_sub_menu = target
                                .closest("[data-radix-menubar-content]")
                                .ok()
                                .flatten()
                                .map(|el| el != current_target)
                                .unwrap_or(false);

                            let prev_menu_key = if context.dir.get_untracked() == Direction::Rtl {
                                "ArrowRight"
                            } else {
                                "ArrowLeft"
                            };
                            let is_prev_key = prev_menu_key == event.key();
                            let is_next_key = !is_prev_key;

                            // Prevent navigation when we're opening a submenu
                            if is_next_key && target_is_sub_trigger {
                                return;
                            }
                            // or we're inside a submenu and are moving backwards to close it
                            if is_key_down_inside_sub_menu && is_prev_key {
                                return;
                            }

                            let candidate_values = get_items.with_value(|get_items| {
                                let items = get_items();
                                let items: Vec<&CollectionItemValue<ItemData>> =
                                    items.iter().filter(|item| !item.data.disabled).collect();
                                let mut candidate_values: Vec<String> =
                                    items.iter().map(|item| item.data.value.clone()).collect();
                                if is_prev_key {
                                    candidate_values.reverse();
                                }

                                let current_index = candidate_values
                                    .iter()
                                    .position(|v| *v == menu_value)
                                    .unwrap_or(0);

                                if context.r#loop.get_untracked() {
                                    wrap_array(&mut candidate_values, current_index + 1).to_vec()
                                } else {
                                    candidate_values.split_off(current_index + 1)
                                }
                            });

                            if let Some(next_value) = candidate_values.first() {
                                context.on_menu_open.run(next_value.clone());
                            }
                        }
                    }
                })),
                Some(false),
            )
            on_escape_key_down=on_escape_key_down
            on_pointer_down_outside=on_pointer_down_outside
        >
            {children()}
        </MenuContent>
    }
}

/* -------------------------------------------------------------------------------------------------
 * MenubarGroup
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn MenubarGroup(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <MenuGroup as_child=as_child node_ref=node_ref>
            {children()}
        </MenuGroup>
    }
}

/* -------------------------------------------------------------------------------------------------
 * MenubarLabel
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn MenubarLabel(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <MenuLabel as_child=as_child node_ref=node_ref>
            {children()}
        </MenuLabel>
    }
}

/* -------------------------------------------------------------------------------------------------
 * MenubarItem
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn MenubarItem(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_select: Option<Callback<ev::Event>>,
    #[prop(into, optional)] text_value: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let on_select = wrap_callback(on_select);

    view! {
        <MenuItem
            disabled=disabled
            on_select=on_select
            text_value=text_value
            as_child=as_child
            node_ref=node_ref
        >
            {children()}
        </MenuItem>
    }
}

/* -------------------------------------------------------------------------------------------------
 * MenubarCheckboxItem
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn MenubarCheckboxItem(
    #[prop(into, optional)] checked: MaybeProp<CheckedState>,
    #[prop(into, optional)] on_checked_change: Option<Callback<bool>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_select: Option<Callback<ev::Event>>,
    #[prop(into, optional)] text_value: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let on_checked_change = wrap_callback(on_checked_change);
    let on_select = wrap_callback(on_select);

    view! {
        <MenuCheckboxItem
            checked=checked
            on_checked_change=on_checked_change
            disabled=disabled
            on_select=on_select
            text_value=text_value
            as_child=as_child
            node_ref=node_ref
        >
            {children()}
        </MenuCheckboxItem>
    }
}

/* -------------------------------------------------------------------------------------------------
 * MenubarRadioGroup
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn MenubarRadioGroup(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let on_value_change = wrap_callback(on_value_change);

    view! {
        <MenuRadioGroup
            value=value
            on_value_change=on_value_change
            as_child=as_child
            node_ref=node_ref
        >
            {children()}
        </MenuRadioGroup>
    }
}

/* -------------------------------------------------------------------------------------------------
 * MenubarRadioItem
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn MenubarRadioItem(
    #[prop(into)] value: MaybeProp<String>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_select: Option<Callback<ev::Event>>,
    #[prop(into, optional)] text_value: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let on_select = wrap_callback(on_select);

    view! {
        <MenuRadioItem
            value=value
            disabled=disabled
            on_select=on_select
            text_value=text_value
            as_child=as_child
            node_ref=node_ref
        >
            {children()}
        </MenuRadioItem>
    }
}

/* -------------------------------------------------------------------------------------------------
 * MenubarItemIndicator
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn MenubarItemIndicator(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <MenuItemIndicator force_mount=force_mount as_child=as_child node_ref=node_ref>
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </MenuItemIndicator>
    }
}

/* -------------------------------------------------------------------------------------------------
 * MenubarSeparator
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn MenubarSeparator(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <MenuSeparator as_child=as_child node_ref=node_ref>
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </MenuSeparator>
    }
}

/* -------------------------------------------------------------------------------------------------
 * MenubarArrow
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn MenubarArrow(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <MenuArrow as_child=as_child node_ref=node_ref>
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </MenuArrow>
    }
}

/* -------------------------------------------------------------------------------------------------
 * MenubarSub
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn MenubarSub(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let (open_state, set_open_state) = use_controllable_state(UseControllableStateParams {
        prop: open,
        default_prop: default_open,
        on_change: on_open_change.map(|on_open_change| {
            Callback::new(move |value: Option<bool>| {
                if let Some(value) = value {
                    on_open_change.run(value);
                }
            })
        }),
    });

    let open_signal = Signal::derive(move || open_state.get().unwrap_or(false));

    view! {
        <MenuSub
            open=open_signal
            on_open_change=Callback::new(move |value: bool| {
                set_open_state.run(Some(value));
            })
        >
            {children.with_value(|children| children())}
        </MenuSub>
    }
}

/* -------------------------------------------------------------------------------------------------
 * MenubarSubTrigger
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn MenubarSubTrigger(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] text_value: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let sub_trigger_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, sub_trigger_ref]);

    // Set data-radix-menubar-subtrigger via Effect since MenuSubTrigger doesn't support
    // arbitrary data attributes passthrough.
    Effect::new(move |_| {
        if let Some(el) = sub_trigger_ref.get() {
            let el: &web_sys::Element = (*el).unchecked_ref();
            el.set_attribute("data-radix-menubar-subtrigger", "").ok();
        }
    });

    view! {
        <MenuSubTrigger
            disabled=disabled
            text_value=text_value
            as_child=as_child
            node_ref=composed_refs
        >
            {children()}
        </MenuSubTrigger>
    }
}

/* -------------------------------------------------------------------------------------------------
 * MenubarSubContent
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn MenubarSubContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] side_offset: MaybeProp<f64>,
    #[prop(into, optional)] align_offset: MaybeProp<f64>,
    #[prop(into, optional)] avoid_collisions: MaybeProp<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let sub_content_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, sub_content_ref]);

    let on_escape_key_down = wrap_callback(on_escape_key_down);
    let on_focus_outside = wrap_callback(on_focus_outside);

    // Set data-radix-menubar-content via Effect.
    Effect::new(move |_| {
        if let Some(el) = sub_content_ref.get() {
            let el: &web_sys::Element = (*el).unchecked_ref();
            el.set_attribute("data-radix-menubar-content", "").ok();
        }
    });

    view! {
        <MenuSubContent
            force_mount=force_mount
            side_offset=side_offset
            align_offset=align_offset
            avoid_collisions=avoid_collisions
            class=class
            as_child=as_child
            node_ref=composed_refs
            on_escape_key_down=on_escape_key_down
            on_focus_outside=on_focus_outside
            content_style="--radix-menubar-content-transform-origin: var(--radix-popper-transform-origin); --radix-menubar-content-available-width: var(--radix-popper-available-width); --radix-menubar-content-available-height: var(--radix-popper-available-height); --radix-menubar-trigger-width: var(--radix-popper-anchor-width); --radix-menubar-trigger-height: var(--radix-popper-anchor-height);"
        >
            {children()}
        </MenuSubContent>
    }
}
