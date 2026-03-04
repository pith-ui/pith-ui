use leptos::{context::Provider, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use leptos_portal::LeptosPortal;
use radix_leptos_primitive::{Primitive, prop_or_default};
use send_wrapper::SendWrapper;

/// Shared context provided by all `ScopedPortal` instances.
/// Component-specific portals wrap `ScopedPortal` and add their own context re-provision.
#[derive(Clone, Copy)]
pub struct PortalContextValue {
    pub force_mount: Signal<bool>,
}

/// Portal wrapper that provides a shared [`PortalContextValue`] with `force_mount`.
/// Component-specific portals wrap this and add their own context re-provision.
#[component]
pub fn ScopedPortal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    #[prop(optional)] container_ref: AnyNodeRef,
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let portal_context = PortalContextValue {
        force_mount: prop_or_default(force_mount),
    };

    view! {
        <Provider value=portal_context>
            <Portal container=container container_ref=container_ref as_child=true>
                {children.with_value(|children| children())}
            </Portal>
        </Provider>
    }
}

/// Resolves `force_mount` from a component's own prop OR the portal context.
/// Extracts the repeated 5-line pattern from every Content component.
pub fn resolve_force_mount(prop: MaybeProp<bool>) -> Signal<bool> {
    let portal_context = use_context::<PortalContextValue>();
    Signal::derive(move || {
        prop.get()
            .or_else(|| portal_context.as_ref().map(|pc| pc.force_mount.get()))
            .unwrap_or(false)
    })
}

#[component]
pub fn Portal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    #[prop(optional)] container_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    // TODO: pass attrs to primitive
    view! {
        // <AttributeInterceptor let:attrs>
            <LeptosPortal mount=container mount_ref=container_ref>
                <Primitive
                    element=html::div
                    as_child=as_child
                    node_ref={node_ref}
                    // {..attrs}
                >
                    {children.with_value(|children| children())}
                </Primitive>
            </LeptosPortal>
        // </AttributeInterceptor>
    }
}

/// Based on [`leptos::Portal`].
mod leptos_portal {
    use std::sync::Arc;

    use leptos::prelude::{
        Effect, Get, IntoView, MaybeProp, Owner, RwSignal, Set, Signal, TypedChildrenFn, component,
        mount_to, untrack,
    };
    use leptos_dom::helpers::document;
    use leptos_node_ref::AnyNodeRef;
    use send_wrapper::SendWrapper;

    /// Renders components somewhere else in the DOM.
    ///
    /// Useful for inserting modals and tooltips outside of a cropping layout.
    /// If no mount point is given, the portal is inserted in `document.body`.
    #[component]
    pub fn LeptosPortal<V>(
        /// Target element where the children will be appended
        #[prop(into, optional)]
        mount: MaybeProp<SendWrapper<web_sys::Element>>,
        #[prop(optional)] mount_ref: AnyNodeRef,
        /// The children to teleport into the `mount` element
        children: TypedChildrenFn<V>,
    ) -> impl IntoView
    where
        V: IntoView + 'static,
    {
        if cfg!(target_arch = "wasm32")
            && Owner::current_shared_context()
                .map(|sc| sc.is_browser())
                .unwrap_or(true)
        {
            use web_sys::wasm_bindgen::JsCast;

            let mount = Signal::derive(move || {
                mount_ref
                    .get()
                    .map(|mount| SendWrapper::new(mount.unchecked_into::<web_sys::Element>()))
                    .or_else(|| mount.get())
                    .unwrap_or_else(|| {
                        SendWrapper::new(document().body().expect("body to exist").into())
                    })
            });
            let children = children.into_inner();

            let current_mount: RwSignal<Option<SendWrapper<web_sys::Element>>> =
                RwSignal::new(None);

            Effect::new(move |_| {
                let mount = mount.get();

                if current_mount.get().as_deref() != Some(&*mount) {
                    current_mount.set(Some(mount));
                }
            });

            Effect::new(move |_| {
                if let Some(current_mount) = current_mount.get() {
                    let handle =
                        SendWrapper::new(mount_to((*current_mount).clone().unchecked_into(), {
                            let children = Arc::clone(&children);
                            move || untrack(|| children())
                        }));

                    Owner::on_cleanup({
                        move || {
                            let handle = handle.take();
                            drop(handle);
                        }
                    })
                }
            });
        }
    }
}
