use leptos::{context::Provider, ev, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
pub use radix_leptos_menu::CheckedState;
use radix_leptos_menu::*;
use radix_leptos_primitive::{
    Primitive, compose_callbacks, data_attr, prop_or, prop_or_default, wrap_callback,
};
use radix_leptos_use_controllable_state::{UseControllableStateParams, use_controllable_state};
use send_wrapper::SendWrapper;
use wasm_bindgen::{JsCast, closure::Closure};

/* -------------------------------------------------------------------------------------------------
 * ContextMenu
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy)]
struct ContextMenuContextValue {
    open: Signal<bool>,
    on_open_change: Callback<bool>,
    modal: Signal<bool>,
}

#[component]
pub fn ContextMenu(
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] modal: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let modal = prop_or(modal, true);
    let (open, set_open) = signal(false);
    let open_signal = Signal::derive(move || open.get());

    let handle_open_change = Callback::new(move |value: bool| {
        set_open.set(value);
        if let Some(on_open_change) = on_open_change {
            on_open_change.run(value);
        }
    });

    let context = ContextMenuContextValue {
        open: open_signal,
        on_open_change: handle_open_change,
        modal,
    };

    view! {
        <Provider value=context>
            <Menu
                open=open_signal
                on_open_change=handle_open_change
                dir=dir
                modal=modal
            >
                {children.with_value(|children| children())}
            </Menu>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ContextMenuTrigger
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy, Default)]
struct Point {
    x: f64,
    y: f64,
}

/// A virtual element for floating-ui that returns a zero-size rect at a given point.
/// Stores raw coordinates so `PartialEq` detects changes when the point moves,
/// which triggers floating-ui to re-compute the position.
#[derive(Clone, Debug, PartialEq)]
struct PointVirtualElement {
    x: f64,
    y: f64,
}

impl PopperVirtualElement<web_sys::Element> for PointVirtualElement {
    fn get_bounding_client_rect(&self) -> ClientRectObject {
        ClientRectObject {
            x: self.x,
            y: self.y,
            width: 0.0,
            height: 0.0,
            top: self.y,
            right: self.x,
            bottom: self.y,
            left: self.x,
        }
    }

    fn get_client_rects(&self) -> Option<Vec<ClientRectObject>> {
        None
    }

    fn context_element(&self) -> Option<web_sys::Element> {
        None
    }
}

#[component]
pub fn ContextMenuTrigger(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context = expect_context::<ContextMenuContextValue>();
    let disabled = prop_or_default(disabled);
    let point = RwSignal::new(Point::default());
    let long_press_timer = RwSignal::new(0i32);

    let clear_long_press = move || {
        web_sys::window()
            .expect("Window should exist.")
            .clear_timeout_with_handle(long_press_timer.get_untracked());
    };

    // Clear timer on unmount.
    on_cleanup(move || {
        clear_long_press();
    });

    // Clear timer when disabled.
    Effect::new(move |_| {
        if disabled.get() {
            clear_long_press();
        }
    });

    // Set a virtual element as the Popper anchor so no extra DOM element is needed.
    // This mirrors React's virtualRef pattern. The Effect tracks point changes and
    // creates a new PointVirtualElement with the updated coordinates, which floating-ui
    // detects via PartialEq and uses to re-position.
    Effect::new(move |_| {
        let p = point.get();
        set_popper_virtual_ref(Box::new(PointVirtualElement { x: p.x, y: p.y }));
    });

    view! {
        <Primitive
            element=html::span
            as_child=as_child
            node_ref=node_ref
            attr:data-state=move || if context.open.get() { "open" } else { "closed" }
            attr:data-disabled=data_attr(disabled)
            attr:style="-webkit-touch-callout: none;"
            on:contextmenu=move |event: ev::MouseEvent| {
                if disabled.get_untracked() {
                    return;
                }
                // Clear the long press here because some platforms already support
                // long press to trigger a contextmenu event.
                clear_long_press();
                point.set(Point {
                    x: event.client_x() as f64,
                    y: event.client_y() as f64,
                });
                context.on_open_change.run(true);
                event.prevent_default();
            }
            on:pointerdown=move |event: ev::PointerEvent| {
                if disabled.get_untracked() || event.pointer_type() == "mouse" {
                    return;
                }
                // Clear the long press here in case there's multiple touch points.
                clear_long_press();
                let client_x = event.client_x();
                let client_y = event.client_y();
                let on_open_change = context.on_open_change;
                let timer_cb = Closure::once_into_js(move || {
                    point.set(Point {
                        x: client_x as f64,
                        y: client_y as f64,
                    });
                    on_open_change.run(true);
                });
                let timer_id = web_sys::window()
                    .expect("Window should exist.")
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        timer_cb.unchecked_ref(),
                        700,
                    )
                    .expect("Timeout should be set.");
                long_press_timer.set(timer_id);
            }
            on:pointermove=move |event: ev::PointerEvent| {
                if disabled.get_untracked() || event.pointer_type() == "mouse" {
                    return;
                }
                clear_long_press();
            }
            on:pointercancel=move |event: ev::PointerEvent| {
                if disabled.get_untracked() || event.pointer_type() == "mouse" {
                    return;
                }
                clear_long_press();
            }
            on:pointerup=move |event: ev::PointerEvent| {
                if disabled.get_untracked() || event.pointer_type() == "mouse" {
                    return;
                }
                clear_long_press();
            }
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ContextMenuPortal
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ContextMenuPortal(
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
 * ContextMenuContent
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ContextMenuContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] on_close_auto_focus: Option<Callback<ev::Event>>,
    #[prop(into, optional)] on_interact_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let context = expect_context::<ContextMenuContextValue>();
    let has_interacted_outside = RwSignal::new(false);

    // Wrap pass-through callbacks for view! macro forwarding.
    let on_escape_key_down = wrap_callback(on_escape_key_down);
    let on_pointer_down_outside = wrap_callback(on_pointer_down_outside);
    let on_focus_outside = wrap_callback(on_focus_outside);

    view! {
        <MenuContent
            force_mount=force_mount
            class=class
            as_child=as_child
            node_ref=node_ref
            side=PopperSide::Right
            side_offset=2.0
            align=Align::Start
            content_style="--radix-context-menu-content-transform-origin: var(--radix-popper-transform-origin); --radix-context-menu-content-available-width: var(--radix-popper-available-width); --radix-context-menu-content-available-height: var(--radix-popper-available-height); --radix-context-menu-trigger-width: var(--radix-popper-anchor-width); --radix-context-menu-trigger-height: var(--radix-popper-anchor-height);"
            on_close_auto_focus=compose_callbacks(
                on_close_auto_focus,
                Some(Callback::new(move |event: ev::Event| {
                    if !event.default_prevented() && has_interacted_outside.get_untracked() {
                        event.prevent_default();
                    }
                    has_interacted_outside.set(false);
                })),
                None,
            )
            on_interact_outside=compose_callbacks(
                on_interact_outside,
                Some(Callback::new(move |event: web_sys::CustomEvent| {
                    if !event.default_prevented() && !context.modal.get_untracked() {
                        has_interacted_outside.set(true);
                    }
                })),
                None,
            )
            on_escape_key_down=on_escape_key_down
            on_pointer_down_outside=on_pointer_down_outside
            on_focus_outside=on_focus_outside
        >
            {children()}
        </MenuContent>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ContextMenuGroup
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ContextMenuGroup(
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
 * ContextMenuLabel
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ContextMenuLabel(
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
 * ContextMenuItem
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ContextMenuItem(
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
 * ContextMenuCheckboxItem
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ContextMenuCheckboxItem(
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
 * ContextMenuRadioGroup
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ContextMenuRadioGroup(
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
 * ContextMenuRadioItem
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ContextMenuRadioItem(
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
 * ContextMenuItemIndicator
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ContextMenuItemIndicator(
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
 * ContextMenuSeparator
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ContextMenuSeparator(
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
 * ContextMenuArrow
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ContextMenuArrow(
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
 * ContextMenuSub
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ContextMenuSub(
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
 * ContextMenuSubTrigger
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ContextMenuSubTrigger(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] text_value: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <MenuSubTrigger
            disabled=disabled
            text_value=text_value
            as_child=as_child
            node_ref=node_ref
        >
            {children()}
        </MenuSubTrigger>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ContextMenuSubContent
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ContextMenuSubContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] side_offset: MaybeProp<f64>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let on_escape_key_down = wrap_callback(on_escape_key_down);
    let on_focus_outside = wrap_callback(on_focus_outside);

    view! {
        <MenuSubContent
            force_mount=force_mount
            side_offset=side_offset
            class=class
            as_child=as_child
            node_ref=node_ref
            on_escape_key_down=on_escape_key_down
            on_focus_outside=on_focus_outside
            content_style="--radix-context-menu-content-transform-origin: var(--radix-popper-transform-origin); --radix-context-menu-content-available-width: var(--radix-popper-available-width); --radix-context-menu-content-available-height: var(--radix-popper-available-height); --radix-context-menu-trigger-width: var(--radix-popper-anchor-width); --radix-context-menu-trigger-height: var(--radix-popper-anchor-height);"
        >
            {children()}
        </MenuSubContent>
    }
}
