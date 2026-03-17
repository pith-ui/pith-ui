use super::*;

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
    view! {
        <MenuSubTrigger
            disabled=disabled
            text_value=text_value
            as_child=as_child
            node_ref=node_ref
            attr:data-radix-menubar-subtrigger=""
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
    #[prop(into, optional)] collision_boundary: MaybeProp<SendWrapper<Vec<web_sys::Element>>>,
    #[prop(into, optional)] collision_padding: MaybeProp<Padding>,
    #[prop(into, optional)] arrow_padding: MaybeProp<f64>,
    #[prop(into, optional)] sticky: MaybeProp<Sticky>,
    #[prop(into, optional)] hide_when_detached: MaybeProp<bool>,
    #[prop(into, optional)] r#loop: MaybeProp<bool>,
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
            attr:data-radix-menubar-content=""
            style:--radix-menubar-content-transform-origin="var(--radix-popper-transform-origin)"
            style:--radix-menubar-content-available-width="var(--radix-popper-available-width)"
            style:--radix-menubar-content-available-height="var(--radix-popper-available-height)"
            style:--radix-menubar-trigger-width="var(--radix-popper-anchor-width)"
            style:--radix-menubar-trigger-height="var(--radix-popper-anchor-height)"
            on_escape_key_down=on_escape_key_down
            on_focus_outside=on_focus_outside
        >
            {children()}
        </MenuSubContent>
    }
}
