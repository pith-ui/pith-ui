use super::*;

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
    #[prop(into, optional)] align_offset: MaybeProp<f64>,
    #[prop(into, optional)] avoid_collisions: MaybeProp<bool>,
    #[prop(into, optional)] collision_boundary: MaybeProp<SendWrapper<Vec<web_sys::Element>>>,
    #[prop(into, optional)] collision_padding: MaybeProp<Padding>,
    #[prop(into, optional)] arrow_padding: MaybeProp<f64>,
    #[prop(into, optional)] sticky: MaybeProp<Sticky>,
    #[prop(into, optional)] hide_when_detached: MaybeProp<bool>,
    #[prop(into, optional)] r#loop: MaybeProp<bool>,
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
            as_child=as_child
            node_ref=node_ref
            attr:class=move || class.get().unwrap_or_default()
            side=PopperSide::Right
            side_offset=2.0
            align=Align::Start
            align_offset=align_offset
            avoid_collisions=avoid_collisions
            collision_boundary=collision_boundary
            collision_padding=collision_padding
            arrow_padding=arrow_padding
            sticky=sticky
            hide_when_detached=hide_when_detached
            r#loop=r#loop
            style:--radix-context-menu-content-transform-origin="var(--radix-popper-transform-origin)"
            style:--radix-context-menu-content-available-width="var(--radix-popper-available-width)"
            style:--radix-context-menu-content-available-height="var(--radix-popper-available-height)"
            style:--radix-context-menu-trigger-width="var(--radix-popper-anchor-width)"
            style:--radix-context-menu-trigger-height="var(--radix-popper-anchor-height)"
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
        on_change: adapt_callback(on_open_change),
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
    #[prop(into, optional)] align_offset: MaybeProp<f64>,
    #[prop(into, optional)] avoid_collisions: MaybeProp<bool>,
    #[prop(into, optional)] collision_boundary: MaybeProp<SendWrapper<Vec<web_sys::Element>>>,
    #[prop(into, optional)] collision_padding: MaybeProp<Padding>,
    #[prop(into, optional)] arrow_padding: MaybeProp<f64>,
    #[prop(into, optional)] sticky: MaybeProp<Sticky>,
    #[prop(into, optional)] hide_when_detached: MaybeProp<bool>,
    #[prop(into, optional)] r#loop: MaybeProp<bool>,
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
            align_offset=align_offset
            avoid_collisions=avoid_collisions
            collision_boundary=collision_boundary
            collision_padding=collision_padding
            arrow_padding=arrow_padding
            sticky=sticky
            hide_when_detached=hide_when_detached
            r#loop=r#loop
            as_child=as_child
            node_ref=node_ref
            attr:class=move || class.get().unwrap_or_default()
            style:--radix-context-menu-content-transform-origin="var(--radix-popper-transform-origin)"
            style:--radix-context-menu-content-available-width="var(--radix-popper-available-width)"
            style:--radix-context-menu-content-available-height="var(--radix-popper-available-height)"
            style:--radix-context-menu-trigger-width="var(--radix-popper-anchor-width)"
            style:--radix-context-menu-trigger-height="var(--radix-popper-anchor-height)"
            on_escape_key_down=on_escape_key_down
            on_focus_outside=on_focus_outside
        >
            {children()}
        </MenuSubContent>
    }
}
