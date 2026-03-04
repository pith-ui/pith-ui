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

pub fn wrap_callback<T: 'static>(cb: Option<Callback<T>>) -> Callback<T> {
    match cb {
        Some(cb) => cb,
        None => Callback::new(|_| {}),
    }
}

pub fn open_closed_state(open: bool) -> &'static str {
    match open {
        true => "open",
        false => "closed",
    }
}

pub fn prop_or<T: Clone + Send + Sync + 'static>(prop: MaybeProp<T>, default: T) -> Signal<T> {
    Signal::derive(move || prop.get().unwrap_or(default.clone()))
}

pub fn prop_or_default<T: Clone + Default + Send + Sync + 'static>(
    prop: MaybeProp<T>,
) -> Signal<T> {
    Signal::derive(move || prop.get().unwrap_or_default())
}

pub fn data_attr(signal: Signal<bool>) -> impl Fn() -> Option<&'static str> + Send + Sync {
    move || signal.get().then_some("")
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
            // Use try_run to avoid panicking if the callback's StoredValue
            // has been disposed during synchronous unmount.
            original.try_run(event.clone());
        }

        if (!check_default_prevented || !event.clone().into().default_prevented())
            && let Some(our) = &our_handler
        {
            our.try_run(event);
        }
    }
}
