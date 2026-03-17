use super::*;

/* -------------------------------------------------------------------------------------------------
 * SelectPortal
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn SelectPortal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    #[prop(optional)] container_ref: AnyNodeRef,
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let select_context = expect_context::<SelectContextValue>();
    let popper_scope = use_popper_scope();
    let collection_scope = use_collection_scope::<SelectItemData>();

    view! {
        <ScopedPortal
            container=container
            container_ref=container_ref
            force_mount=force_mount
            context_bridge=Callback::new(move |_| {
                provide_context(select_context);
                if let Some(scope) = popper_scope {
                    provide_popper_scope(scope);
                }
                if let Some(scope) = collection_scope {
                    provide_collection_scope(scope);
                }
            })
        >
            {children.try_with_value(|children| children())}
        </ScopedPortal>
    }
}
