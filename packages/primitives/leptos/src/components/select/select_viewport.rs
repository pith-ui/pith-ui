use super::*;

/* -------------------------------------------------------------------------------------------------
 * SelectViewport
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn SelectViewport(
    #[prop(into, optional)] nonce: Option<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let nonce = StoredValue::new(nonce);

    let content_context = expect_context::<SelectContentContextValue>();
    let composed_ref = use_composed_refs(vec![node_ref, content_context.viewport_ref]);

    view! {
        <>
            // Hide scrollbars cross-browser
            <style nonce=nonce.get_value()>"[data-radix-select-viewport]{scrollbar-width:none;-ms-overflow-style:none;-webkit-overflow-scrolling:touch;}[data-radix-select-viewport]::-webkit-scrollbar{display:none}"</style>
            <CollectionSlot<SelectItemData> item_data_type=ITEM_DATA_PHANTOM>
                <AttributeInterceptor let:attrs>
                    <Primitive
                        element=html::div
                        as_child=as_child
                        attr:data-radix-select-viewport=""
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
            </CollectionSlot<SelectItemData>>
        </>
    }
}
