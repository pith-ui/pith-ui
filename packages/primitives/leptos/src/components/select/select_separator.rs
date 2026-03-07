use super::*;

/* -------------------------------------------------------------------------------------------------
 * SelectSeparator
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn SelectSeparator(
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
                attr:aria-hidden="true"
                {..attrs}
            >
                {children.try_with_value(|children| children.as_ref().map(|c| c()))}
            </Primitive>
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * SelectArrow
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn SelectArrow(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional, default = 10.0.into())] width: Signal<f64>,
    #[prop(into, optional, default = 5.0.into())] height: Signal<f64>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context = expect_context::<SelectContextValue>();
    let content_context = expect_context::<SelectContentContextValue>();

    let should_show = Signal::derive(move || {
        context.open.get()
            && content_context
                .position
                .try_get_value()
                .is_some_and(|p| p == "popper")
    });

    view! {
        <Show when=move || should_show.get()>
            <PopperArrow
                as_child=as_child
                node_ref=node_ref
                width=width
                height=height
            >
                {children.try_with_value(|children| children.as_ref().map(|c| c()))}
            </PopperArrow>
        </Show>
    }
}

/* -------------------------------------------------------------------------------------------------
 * SelectBubbleInput (internal)
 * -----------------------------------------------------------------------------------------------*/

/// Hidden native <select> element for form integration.
#[component]
pub(super) fn SelectBubbleInput(
    value: Signal<Option<String>>,
    name: Signal<Option<String>>,
    auto_complete: Signal<Option<String>>,
    form: Signal<Option<String>>,
    disabled: Signal<bool>,
    required: Signal<bool>,
) -> impl IntoView {
    let select_ref = AnyNodeRef::new();
    let prev_value: StoredValue<Option<String>> = StoredValue::new(None);

    // Bubble value change to parent forms
    Effect::new(move |_| {
        let current_value = value.get();
        let previous = prev_value.try_get_value().flatten();
        let _ = prev_value.try_set_value(current_value.clone());

        if previous != current_value
            && let Some(select_el) = select_ref.get()
        {
            let select_el: web_sys::HtmlSelectElement = (*select_el).clone().unchecked_into();
            select_el.set_value(&current_value.clone().unwrap_or_default());
            let event_init = web_sys::EventInit::new();
            event_init.set_bubbles(true);
            let event = web_sys::Event::new_with_event_init_dict("change", &event_init)
                .expect("Event should be created.");
            let _ = select_el.dispatch_event(&event);
        }
    });

    view! {
        <select
            node_ref=select_ref
            aria-hidden="true"
            tabindex="-1"
            name=move || name.get()
            autocomplete=move || auto_complete.get()
            form=move || form.get()
            disabled=move || disabled.get()
            required=move || required.get()
            style=VISUALLY_HIDDEN_STYLES_STR
            prop:value=move || value.get().unwrap_or_default()
        >
            <option value="">"" </option>
            {move || {
                value.get().filter(|v| !v.is_empty()).map(|v| {
                    let v2 = v.clone();
                    view! { <option value=v>{v2}</option> }
                })
            }}
        </select>
    }
}
