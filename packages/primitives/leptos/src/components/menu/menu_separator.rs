use super::*;

#[component]
pub fn MenuSeparator(
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
                attr:role="separator"
                attr:aria-orientation="horizontal"
                {..attrs}
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </Primitive>
        </AttributeInterceptor>
    }
}

#[component]
pub fn MenuArrow(
    #[prop(into, optional)] width: MaybeProp<f64>,
    #[prop(into, optional)] height: MaybeProp<f64>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <PopperArrow
            width=width
            height=height
            as_child=as_child
            node_ref=node_ref
        >
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </PopperArrow>
    }
}
