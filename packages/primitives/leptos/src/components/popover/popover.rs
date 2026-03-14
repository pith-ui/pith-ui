use super::*;

/* -------------------------------------------------------------------------------------------------
 * Popover
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn Popover(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] modal: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let trigger_ref = AnyNodeRef::new();

    let (open_signal, set_open) = use_controllable_state(UseControllableStateParams {
        prop: open,
        default_prop: default_open,
        on_change: adapt_callback(on_open_change),
    });
    let open = Signal::derive(move || open_signal.get().unwrap_or(false));
    let modal = prop_or_default(modal);

    let has_custom_anchor = RwSignal::new(false);
    let content_id = use_id(None);

    let context = PopoverContextValue {
        trigger_ref,
        content_id,
        open,
        on_open_change: Callback::new(move |value: bool| {
            set_open.run(Some(value));
        }),
        on_open_toggle: Callback::new(move |_| {
            set_open.run(Some(!open.get()));
        }),
        has_custom_anchor,
        modal,
    };

    view! {
        <Provider value=context>
            <Popper>
                {children.with_value(|children| children())}
            </Popper>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * PopoverAnchor
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn PopoverAnchor(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<PopoverContextValue>();

    // Mark that we have a custom anchor on mount, unmark on cleanup.
    context.has_custom_anchor.set(true);
    on_cleanup(move || {
        context.has_custom_anchor.set(false);
    });

    view! {
        <PopperAnchor
            as_child=as_child
            node_ref=node_ref
        >
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </PopperAnchor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * PopoverTrigger
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn PopoverTrigger(
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<PopoverContextValue>();
    let composed_trigger_ref = use_composed_refs(vec![node_ref, context.trigger_ref]);

    let on_click = StoredValue::new(on_click);

    let on_click_composed = Callback::new(compose_callbacks(
        on_click.get_value(),
        Some(Callback::new(move |_: ev::MouseEvent| {
            context.on_open_toggle.run(());
        })),
        None,
    ));

    view! {
        <Show
            when=move || context.has_custom_anchor.get()
            fallback=move || view! {
                <PopperAnchor as_child=true>
                    <PopoverTriggerInner
                        on_click=on_click_composed
                        as_child=as_child
                        node_ref=composed_trigger_ref
                    >
                        {children.with_value(|children| children.as_ref().map(|children| children()))}
                    </PopoverTriggerInner>
                </PopperAnchor>
            }
        >
            <PopoverTriggerInner
                on_click=on_click_composed
                as_child=as_child
                node_ref=composed_trigger_ref
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </PopoverTriggerInner>
        </Show>
    }
}

#[component]
fn PopoverTriggerInner(
    on_click: Callback<ev::MouseEvent>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<PopoverContextValue>();

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::button
                as_child=as_child
                node_ref=node_ref
                attr:r#type="button"
                attr:aria-haspopup="dialog"
                attr:aria-expanded=move || context.open.get().to_string()
                attr:aria-controls=move || context.content_id.get()
                attr:data-state=move || open_closed_state(context.open.get())
                on:click=move |event: ev::MouseEvent| {
                    on_click.run(event);
                }
                {..attrs}
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </Primitive>
        </AttributeInterceptor>
    }
}
