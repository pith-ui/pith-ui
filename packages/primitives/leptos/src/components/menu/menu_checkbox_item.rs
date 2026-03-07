use super::*;

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
