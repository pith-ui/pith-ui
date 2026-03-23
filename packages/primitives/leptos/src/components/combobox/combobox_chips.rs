use super::*;

/* -------------------------------------------------------------------------------------------------
 * ComboboxChips
 * -----------------------------------------------------------------------------------------------*/

/// Container for chips representing multi-select values.
#[component]
pub fn ComboboxChips(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref=node_ref
        >
            {children.try_with_value(|children| children.as_ref().map(|c| c()))}
        </Primitive>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ComboboxChip
 * -----------------------------------------------------------------------------------------------*/

/// Individual chip representing a selected value in multi-select mode.
#[component]
pub fn ComboboxChip(
    #[prop(into)] value: String,
    /// The index of this chip in the selected values list (for highlight tracking).
    #[prop(into, optional)] index: MaybeProp<usize>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<ComboboxContextValue>();
    let _value = StoredValue::new(value);
    let is_highlighted = Signal::derive(move || {
        index.get().is_some_and(|i| {
            context.highlighted_chip_index.get().is_some_and(|hi| hi == i)
        })
    });

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::span
                as_child=as_child
                node_ref=node_ref
                attr:data-disabled=data_attr(context.disabled)
                attr:data-highlighted=move || is_highlighted.get().then_some("")
                {..attrs}
            >
                {children.try_with_value(|children| children.as_ref().map(|c| c()))}
            </Primitive>
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ComboboxChipRemove
 * -----------------------------------------------------------------------------------------------*/

/// Button to remove a selected value from the multi-select list.
#[component]
pub fn ComboboxChipRemove(
    #[prop(into)] value: String,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<ComboboxContextValue>();
    let value = StoredValue::new(value);
    let on_click_stored = StoredValue::new(on_click);

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::button
                as_child=as_child
                node_ref=node_ref
                attr:r#type="button"
                attr:tabindex="-1"
                attr:aria-label="Remove"
                attr:data-disabled=data_attr(context.disabled)
                attr:disabled=data_attr(context.disabled)
                on:click=move |event: ev::MouseEvent| {
                    if let Some(Some(cb)) = on_click_stored.try_get_value() {
                        cb.run(event.clone());
                    }
                    if !event.default_prevented() && !context.disabled.get_untracked() {
                        if let Some(val) = value.try_get_value() {
                            context.on_value_change.run(val);
                        }
                        // Focus the input
                        if let Some(input_el) = context.input_ref.get_untracked() {
                            let el: &web_sys::HtmlElement = (*input_el).unchecked_ref();
                            let _ = el.focus();
                        }
                    }
                }
                {..attrs}
            >
                {children.try_with_value(|children| {
                    children.as_ref().map(|c| c()).unwrap_or_else(|| "\u{2715}".into_any())
                })}
            </Primitive>
        </AttributeInterceptor>
    }
}
