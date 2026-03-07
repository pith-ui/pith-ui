use super::*;

/* -------------------------------------------------------------------------------------------------
 * SelectValue
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn SelectValue(
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<SelectContextValue>();
    let (set_value_node_has_children,) = expect_context::<(WriteSignal<bool>,)>();

    let has_children = children.try_with_value(|c| c.is_some()).unwrap_or(false);
    set_value_node_has_children.set(has_children);

    let composed_ref = use_composed_refs(vec![node_ref, context.value_node_ref]);

    // The selected item's text will render its content via a portal into this span
    // when it is selected AND this component has no static children.
    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::span
                as_child=as_child
                node_ref=composed_ref
                style:pointer-events="none"
                // Empty attr to prevent view! macro parse ambiguity with {..attrs}
                attr:data-radix-select-value=""
                {..attrs}
            >
                {move || {
                    if should_show_placeholder(&context.value.get()) {
                        let ph = placeholder.get().unwrap_or_default();
                        Some(ph.into_any())
                    } else {
                        children.try_with_value(|c| c.as_ref().map(|c| c().into_any())).flatten()
                    }
                }}
            </Primitive>
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * SelectIcon
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn SelectIcon(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::span
                as_child=as_child
                node_ref=node_ref
                attr:aria-hidden="true"
                {..attrs}
            >
                {children.try_with_value(|children| {
                    children.as_ref().map(|children| children()).unwrap_or_else(|| "▼".into_any())
                })}
            </Primitive>
        </AttributeInterceptor>
    }
}
