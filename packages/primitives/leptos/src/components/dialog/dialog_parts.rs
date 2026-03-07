use super::*;

/* -------------------------------------------------------------------------------------------------
 * DialogPortal
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn DialogPortal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    #[prop(optional)] container_ref: AnyNodeRef,
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    // React wraps each child individually in <Presence><Portal>, allowing each child
    // to observe its own animation events. We cannot map over children the same way,
    // so we always render the Portal and let each child (DialogOverlay, DialogContent)
    // handle mount/unmount via their own Presence wrappers. This avoids the portal-level
    // Presence (which has no node_ref to observe) from prematurely unmounting children
    // before their exit animations complete.
    view! {
        <ScopedPortal container=container container_ref=container_ref force_mount=force_mount>
            {children.with_value(|children| children())}
        </ScopedPortal>
    }
}

/* -------------------------------------------------------------------------------------------------
 * DialogOverlay
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn DialogOverlay(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<DialogContextValue>();

    let force_mount = resolve_force_mount(force_mount);

    let present = Signal::derive(move || force_mount.get() || context.open.get());
    let is_modal = context.modal;

    let presence_ref = AnyNodeRef::new();

    view! {
        <Show when=move || is_modal.get()>
            <Presence present=present node_ref=presence_ref>
                <DialogOverlayImpl
                    as_child=as_child
                    node_ref=node_ref
                    presence_ref=presence_ref
                >
                    {children.with_value(|children| children.as_ref().map(|children| children()))}
                </DialogOverlayImpl>
            </Presence>
        </Show>
    }
}

#[component]
fn DialogOverlayImpl(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    presence_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<DialogContextValue>();
    let composed_ref = use_composed_refs(vec![node_ref, presence_ref]);

    // Body scroll lock: set overflow hidden on body while overlay is mounted
    use_body_scroll_lock();

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=composed_ref
                attr:data-state=move || open_closed_state(context.open.get())
                attr:style="pointer-events: auto;"
                {..attrs}
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </Primitive>
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * DialogTitle
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn DialogTitle(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<DialogContextValue>();

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::h2
                as_child=as_child
                node_ref=node_ref
                attr:id=move || context.title_id.get()
                {..attrs}
            >
                {children.with_value(|children| children())}
            </Primitive>
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * DialogDescription
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn DialogDescription(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<DialogContextValue>();

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::p
                as_child=as_child
                node_ref=node_ref
                attr:id=move || context.description_id.get()
                {..attrs}
            >
                {children.with_value(|children| children())}
            </Primitive>
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * DialogClose
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn DialogClose(
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<DialogContextValue>();

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::button
                as_child=as_child
                node_ref=node_ref
                attr:r#type="button"
                on:click=compose_callbacks(
                    on_click,
                    Some(Callback::new(move |_: ev::MouseEvent| {
                        context.on_open_change.run(false);
                    })),
                    None,
                )
                {..attrs}
            >
                {children.with_value(|children| children())}
            </Primitive>
        </AttributeInterceptor>
    }
}
