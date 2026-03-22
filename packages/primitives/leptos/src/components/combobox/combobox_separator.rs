use super::*;

/* -------------------------------------------------------------------------------------------------
 * ComboboxSeparator
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ComboboxSeparator(
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
 * ComboboxArrow
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ComboboxArrow(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional, default = 10.0.into())] width: Signal<f64>,
    #[prop(into, optional, default = 5.0.into())] height: Signal<f64>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context = expect_context::<ComboboxContextValue>();

    view! {
        <Show when=move || context.open.get()>
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
 * ComboboxEmpty
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ComboboxEmpty(
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
                attr:role="status"
                {..attrs}
            >
                {children.try_with_value(|children| children.as_ref().map(|c| c()))}
            </Primitive>
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ComboboxViewport
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ComboboxViewport(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let content_context = expect_context::<ComboboxContentContextValue>();
    let composed_ref = use_composed_refs(vec![node_ref, content_context.viewport_ref]);

    view! {
        <CollectionSlot<ComboboxItemData> item_data_type=ITEM_DATA_PHANTOM>
            <AttributeInterceptor let:attrs>
                <Primitive
                    element=html::div
                    as_child=as_child
                    attr:data-radix-combobox-viewport=""
                    attr:role="presentation"
                    {..attrs}
                    node_ref=composed_ref
                    style:position="relative"
                    style:flex="1"
                    style:overflow="hidden auto"
                >
                    {children.try_with_value(|children| children.as_ref().map(|c| c()))}
                </Primitive>
            </AttributeInterceptor>
        </CollectionSlot<ComboboxItemData>>
    }
}
