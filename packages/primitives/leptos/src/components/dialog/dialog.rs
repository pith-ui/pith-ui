use super::*;

/* -------------------------------------------------------------------------------------------------
 * Dialog
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn Dialog(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] modal: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let trigger_ref = AnyNodeRef::new();
    let content_ref = AnyNodeRef::new();

    let (open_signal, set_open) = use_controllable_state(UseControllableStateParams {
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
    let open = Signal::derive(move || open_signal.get().unwrap_or(false));
    let modal = prop_or(modal, true);

    let content_id = use_id(None);
    let title_id = use_id(None);
    let description_id = use_id(None);

    let context = DialogContextValue {
        trigger_ref,
        content_ref,
        content_id,
        title_id,
        description_id,
        open,
        on_open_change: Callback::new(move |value: bool| {
            set_open.run(Some(value));
        }),
        on_open_toggle: Callback::new(move |_| {
            set_open.run(Some(!open.get()));
        }),
        modal,
    };

    view! {
        <Provider value=context>
            {children.with_value(|children| children())}
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * DialogTrigger
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn DialogTrigger(
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<DialogContextValue>();

    let composed_trigger_ref = use_composed_refs(vec![node_ref, context.trigger_ref]);

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::button
                as_child=as_child
                node_ref=composed_trigger_ref
                attr:r#type="button"
                attr:aria-haspopup="dialog"
                attr:aria-expanded=move || context.open.get().to_string()
                attr:aria-controls=move || context.content_id.get()
                attr:data-state=move || open_closed_state(context.open.get())
                on:click=compose_callbacks(
                    on_click,
                    Some(Callback::new(move |_: ev::MouseEvent| {
                        context.on_open_toggle.run(());
                    })),
                    None,
                )
                {..attrs}
            >
                {children.with_value(|children| children())}
            </Primitive>
        </AttributeInterceptor>
    }
}
