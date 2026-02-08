use leptos::{
    either::Either,
    ev::Event,
    html::{ElementType, HtmlElement},
    prelude::*,
    tachys::html::node_ref::NodeRefContainer,
    wasm_bindgen::JsCast,
};
use leptos_node_ref::{AnyNodeRef, any_node_ref};

#[component]
pub fn Primitive<E>(
    element: fn() -> HtmlElement<E, (), ()>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
where
    E: ElementType + 'static,
    HtmlElement<E, (), ()>: ElementChild<AnyView>,
    <HtmlElement<E, (), ()> as ElementChild<AnyView>>::Output: IntoView,
    <E as ElementType>::Output: JsCast,
    AnyNodeRef: NodeRefContainer<E>,
{
    let children = StoredValue::new(children);

    move || {
        if as_child.get().unwrap_or_default() {
            Either::Left(
                children
                    .with_value(|children| children())
                    .add_any_attr(any_node_ref(node_ref)),
            )
        } else {
            Either::Right(
                element()
                    .child(children.with_value(|children| children()))
                    .add_any_attr(any_node_ref(node_ref)),
            )
        }
    }
}

#[component]
pub fn VoidPrimitive<E>(
    element: fn() -> HtmlElement<E, (), ()>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
where
    E: ElementType + 'static,
    <E as ElementType>::Output: JsCast,
    AnyNodeRef: NodeRefContainer<E>,
{
    let children = StoredValue::new(children);

    move || {
        if as_child.get().unwrap_or_default() {
            Either::Left(
                children
                    .with_value(|children| children())
                    .add_any_attr(any_node_ref(node_ref)),
            )
        } else {
            Either::Right(element().add_any_attr(any_node_ref(node_ref)))
        }
    }
}

pub fn compose_callbacks<E>(
    original_handler: Option<Callback<E>>,
    our_handler: Option<Callback<E>>,
    check_default_prevented: Option<bool>,
) -> impl Fn(E)
where
    E: Clone + Into<Event> + 'static,
{
    let check_default_prevented = check_default_prevented.unwrap_or(true);

    move |event: E| {
        if let Some(original) = &original_handler {
            original.run(event.clone());
        }

        if (!check_default_prevented || !event.clone().into().default_prevented())
            && let Some(our) = &our_handler
        {
            our.run(event);
        }
    }
}
