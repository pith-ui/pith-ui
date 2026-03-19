//! Low-level element rendering primitive.
//!
//! Provides [`Primitive`] and [`VoidPrimitive`] components used by all
//! Radix components to render their root element. Supports the `as_child`
//! pattern for merging props into a consumer-provided child element.
//!
//! Also provides utility functions for prop defaults, callback composition,
//! and data attribute helpers used across the component library.

use leptos::{
    either::Either,
    ev::Event,
    html::{ElementType, HtmlElement},
    prelude::*,
    tachys::html::node_ref::NodeRefContainer,
    wasm_bindgen::JsCast,
};
use leptos_node_ref::{AnyNodeRef, any_node_ref};

/// Base rendering primitive for all Radix components.
///
/// When `as_child` is false (default), renders the specified HTML element.
/// When `as_child` is true, delegates rendering to the child, merging
/// attributes and refs onto it.
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

pub fn adapt_callback<T: 'static>(cb: Option<Callback<T>>) -> Option<Callback<Option<T>>> {
    cb.map(|cb| {
        Callback::new(move |value: Option<T>| {
            if let Some(value) = value {
                cb.run(value);
            }
        })
    })
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

#[cfg(test)]
mod tests {
    use super::*;

    use any_spawner::Executor;
    use leptos::reactive::owner::Owner;

    struct NoopExecutor;

    impl any_spawner::CustomExecutor for NoopExecutor {
        fn spawn(&self, _fut: any_spawner::PinnedFuture<()>) {}
        fn spawn_local(&self, _fut: any_spawner::PinnedLocalFuture<()>) {}
        fn poll_local(&self) {}
    }

    fn with_owner<T>(f: impl FnOnce() -> T) -> T {
        let _ = Executor::init_custom_executor(NoopExecutor);
        let owner = Owner::new_root(None);
        owner.with(f)
    }

    // ── open_closed_state ─────────────────────────────────

    #[test]
    fn open_closed_state_open() {
        assert_eq!(open_closed_state(true), "open");
    }

    #[test]
    fn open_closed_state_closed() {
        assert_eq!(open_closed_state(false), "closed");
    }

    // ── wrap_callback ──────────────────────────────────────

    #[test]
    fn wrap_callback_some_returns_original() {
        with_owner(|| {
            let cb = Callback::new(|v: i32| assert_eq!(v, 42));
            let wrapped = wrap_callback(Some(cb));
            wrapped.run(42);
        });
    }

    #[test]
    fn wrap_callback_none_returns_noop() {
        with_owner(|| {
            let wrapped: Callback<i32> = wrap_callback(None);
            // Should not panic
            wrapped.run(0);
        });
    }

    // ── adapt_callback ───────────────────────────────────────

    #[test]
    fn adapt_callback_some_forwards_value() {
        with_owner(|| {
            let cb = Callback::new(|v: i32| assert_eq!(v, 42));
            let adapted = adapt_callback(Some(cb)).unwrap();
            adapted.run(Some(42));
        });
    }

    #[test]
    fn adapt_callback_some_ignores_none() {
        with_owner(|| {
            let cb = Callback::new(|_: i32| panic!("should not be called"));
            let adapted = adapt_callback(Some(cb)).unwrap();
            // Should not panic
            adapted.run(None);
        });
    }

    #[test]
    fn adapt_callback_none_returns_none() {
        let result: Option<Callback<Option<i32>>> = adapt_callback(None);
        assert!(result.is_none());
    }

    // ── prop_or ─────────────────────────────────────────────

    #[test]
    fn prop_or_uses_value_when_present() {
        with_owner(|| {
            let sig = prop_or(MaybeProp::<i32>::from(Some(10)), 99);
            assert_eq!(sig.get_untracked(), 10);
        });
    }

    #[test]
    fn prop_or_uses_default_when_absent() {
        with_owner(|| {
            let none: MaybeProp<i32> = MaybeProp::from(None::<i32>);
            let sig = prop_or(none, 99);
            assert_eq!(sig.get_untracked(), 99);
        });
    }

    // ── prop_or_default ─────────────────────────────────────

    #[test]
    fn prop_or_default_uses_value() {
        with_owner(|| {
            let sig = prop_or_default(MaybeProp::<i32>::from(Some(7)));
            assert_eq!(sig.get_untracked(), 7);
        });
    }

    #[test]
    fn prop_or_default_uses_default() {
        with_owner(|| {
            let none: MaybeProp<i32> = MaybeProp::from(None::<i32>);
            let sig = prop_or_default(none);
            assert_eq!(sig.get_untracked(), 0);
        });
    }

    // ── data_attr ──────────────────────────────────────────

    #[test]
    fn data_attr_true() {
        with_owner(|| {
            let sig = Signal::derive(|| true);
            let f = data_attr(sig);
            assert_eq!(f(), Some(""));
        });
    }

    #[test]
    fn data_attr_false() {
        with_owner(|| {
            let sig = Signal::derive(|| false);
            let f = data_attr(sig);
            assert_eq!(f(), None);
        });
    }
}
