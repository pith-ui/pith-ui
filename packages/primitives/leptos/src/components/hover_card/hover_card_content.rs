use super::*;

/* -------------------------------------------------------------------------------------------------
 * HoverCardPortal
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn HoverCardPortal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    #[prop(optional)] container_ref: AnyNodeRef,
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    // Capture contexts before the portal boundary for re-provision inside mount_to.
    // React uses createContextScope with scope composition to automatically isolate
    // contexts per component instance across portals. Leptos has no equivalent, so
    // we explicitly capture and re-provide contexts to maintain the chain through
    // the portal's mount_to owner boundary.
    let hover_card_context = expect_context::<HoverCardContextValue>();
    let popper_scope = use_popper_scope();

    // Always render the Portal and let HoverCardContent handle its own Presence wrapper.
    // A portal-level Presence would have no node_ref to observe, causing it to unmount
    // immediately before exit animations can complete (same pattern as DialogPortal).
    view! {
        <ScopedPortal container=container container_ref=container_ref force_mount=force_mount>
            <Provider value=hover_card_context>
                {
                    if let Some(scope) = popper_scope {
                        provide_popper_scope(scope);
                    }
                    children.with_value(|children| children())
                }
            </Provider>
        </ScopedPortal>
    }
}

/* -------------------------------------------------------------------------------------------------
 * HoverCardContent
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn HoverCardContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] on_interact_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] on_pointer_enter: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_leave: Option<Callback<ev::PointerEvent>>,
    // PopperContent forwarded props
    #[prop(into, optional, default = Side::Bottom.into())] side: Signal<Side>,
    #[prop(into, optional, default = 0.0.into())] side_offset: Signal<f64>,
    #[prop(into, optional, default = Align::Center.into())] align: Signal<Align>,
    #[prop(into, optional, default = 0.0.into())] align_offset: Signal<f64>,
    #[prop(into, optional, default = 0.0.into())] arrow_padding: Signal<f64>,
    #[prop(into, optional, default = true.into())] avoid_collisions: Signal<bool>,
    #[prop(into, optional, default = SendWrapper::new(vec![]).into())] collision_boundary: Signal<
        SendWrapper<Vec<web_sys::Element>>,
    >,
    #[prop(into, optional, default = Padding::All(0.0).into())] collision_padding: Signal<Padding>,
    #[prop(into, optional, default = Sticky::Partial.into())] sticky: Signal<Sticky>,
    #[prop(into, optional, default = false.into())] hide_when_detached: Signal<bool>,
    #[prop(into, optional, default = UpdatePositionStrategy::Optimized.into())]
    update_position_strategy: Signal<UpdatePositionStrategy>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<HoverCardContextValue>();

    let force_mount = resolve_force_mount(force_mount);

    let present = Signal::derive(move || force_mount.get() || context.open.get());

    let presence_ref = AnyNodeRef::new();

    let callbacks = ContentCallbacks {
        on_escape_key_down: StoredValue::new(on_escape_key_down),
        on_pointer_down_outside: StoredValue::new(on_pointer_down_outside),
        on_focus_outside: StoredValue::new(on_focus_outside),
        on_interact_outside: StoredValue::new(on_interact_outside),
    };

    let on_open = context.on_open;
    let on_close = context.on_close;

    let composed_pointer_enter = Callback::new(compose_callbacks(
        on_pointer_enter,
        Some(Callback::new(move |event: ev::PointerEvent| {
            if event.pointer_type() != "touch" {
                on_open.run(());
            }
        })),
        None,
    ));

    let composed_pointer_leave = Callback::new(compose_callbacks(
        on_pointer_leave,
        Some(Callback::new(move |event: ev::PointerEvent| {
            if event.pointer_type() != "touch" {
                on_close.run(());
            }
        })),
        None,
    ));

    view! {
        <Presence present=present node_ref=presence_ref>
            <HoverCardContentImpl
                callbacks=callbacks
                on_pointer_enter=composed_pointer_enter
                on_pointer_leave=composed_pointer_leave
                side=side
                side_offset=side_offset
                align=align
                align_offset=align_offset
                arrow_padding=arrow_padding
                avoid_collisions=avoid_collisions
                collision_boundary=collision_boundary
                collision_padding=collision_padding
                sticky=sticky
                hide_when_detached=hide_when_detached
                update_position_strategy=update_position_strategy
                as_child=as_child
                node_ref=node_ref
                presence_ref=presence_ref
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </HoverCardContentImpl>
        </Presence>
    }
}

/* -------------------------------------------------------------------------------------------------
 * HoverCardContentImpl
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn HoverCardContentImpl(
    callbacks: ContentCallbacks,
    #[prop(into)] on_pointer_enter: Callback<ev::PointerEvent>,
    #[prop(into)] on_pointer_leave: Callback<ev::PointerEvent>,
    // PopperContent forwarded props
    #[prop(into, optional, default = Side::Bottom.into())] side: Signal<Side>,
    #[prop(into, optional, default = 0.0.into())] side_offset: Signal<f64>,
    #[prop(into, optional, default = Align::Center.into())] align: Signal<Align>,
    #[prop(into, optional, default = 0.0.into())] align_offset: Signal<f64>,
    #[prop(into, optional, default = 0.0.into())] arrow_padding: Signal<f64>,
    #[prop(into, optional, default = true.into())] avoid_collisions: Signal<bool>,
    #[prop(into, optional, default = SendWrapper::new(vec![]).into())] collision_boundary: Signal<
        SendWrapper<Vec<web_sys::Element>>,
    >,
    #[prop(into, optional, default = Padding::All(0.0).into())] collision_padding: Signal<Padding>,
    #[prop(into, optional, default = Sticky::Partial.into())] sticky: Signal<Sticky>,
    #[prop(into, optional, default = false.into())] hide_when_detached: Signal<bool>,
    #[prop(into, optional, default = UpdatePositionStrategy::Optimized.into())]
    update_position_strategy: Signal<UpdatePositionStrategy>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    presence_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<HoverCardContextValue>();
    let content_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, content_ref, presence_ref]);

    let contain_selection = RwSignal::new(false);

    // User-select management on body
    let original_body_user_select: StoredValue<Option<String>> = StoredValue::new(None);
    Effect::new(move |_| {
        if contain_selection.get() {
            let body = document().body().expect("Document should have body.");
            let style = body.style();
            let current = style.get_property_value("user-select").unwrap_or_default();
            original_body_user_select.set_value(Some(current));
            style
                .set_property("user-select", "none")
                .expect("Style should be set.");
            // Safari requires prefix
            style
                .set_property("-webkit-user-select", "none")
                .expect("Style should be set.");
        } else if let Some(original) = original_body_user_select.get_value() {
            let body = document().body().expect("Document should have body.");
            let style = body.style();
            style
                .set_property("user-select", &original)
                .expect("Style should be set.");
            style
                .set_property("-webkit-user-select", &original)
                .expect("Style should be set.");
        }
    });

    // Pointer-up listener for text selection tracking.
    // We use StoredValue (not RwSignal) since Closure is not Clone.
    #[allow(clippy::type_complexity)]
    let pointerup_closure: StoredValue<Option<SendWrapper<Closure<dyn Fn(web_sys::Event)>>>> =
        StoredValue::new(None);

    Effect::new({
        move |_| {
            if content_ref.get().is_some() {
                // Remove any existing listener before adding a new one
                pointerup_closure.with_value(|existing| {
                    if let Some(closure) = existing {
                        document()
                            .remove_event_listener_with_callback(
                                "pointerup",
                                closure.as_ref().unchecked_ref(),
                            )
                            .ok();
                    }
                });

                let has_selection_ref = context.has_selection_ref;
                let is_pointer_down_on_content_ref = context.is_pointer_down_on_content_ref;
                let closure =
                    Closure::<dyn Fn(web_sys::Event)>::new(move |_event: web_sys::Event| {
                        contain_selection.set(false);
                        is_pointer_down_on_content_ref.set(false);

                        // Delay a frame to ensure we always access the latest selection
                        let cb = Closure::once_into_js(move || {
                            let has_selection = document()
                                .get_selection()
                                .ok()
                                .flatten()
                                .and_then(|s| s.to_string().as_string())
                                .is_some_and(|s| !s.is_empty());
                            if has_selection {
                                has_selection_ref.set(true);
                            }
                        });
                        web_sys::window()
                            .expect("Window should exist.")
                            .set_timeout_with_callback(cb.unchecked_ref())
                            .ok();
                    });

                document()
                    .add_event_listener_with_callback("pointerup", closure.as_ref().unchecked_ref())
                    .expect("Event listener should be added.");

                pointerup_closure.set_value(Some(SendWrapper::new(closure)));
            }
        }
    });

    on_cleanup({
        move || {
            pointerup_closure.with_value(|closure| {
                if let Some(closure) = closure {
                    document()
                        .remove_event_listener_with_callback(
                            "pointerup",
                            closure.as_ref().unchecked_ref(),
                        )
                        .ok();
                }
            });
            context.has_selection_ref.set(false);
            context.is_pointer_down_on_content_ref.set(false);
        }
    });

    // Suppress tabbable nodes
    Effect::new(move |_| {
        if let Some(content) = content_ref.get() {
            let content: web_sys::HtmlElement = content.unchecked_into();
            let tabbables = get_tabbable_nodes(&content);
            for tabbable in tabbables {
                tabbable
                    .set_attribute("tabindex", "-1")
                    .expect("Attribute should be set.");
            }
        }
    });

    let on_escape_key_down = callbacks
        .on_escape_key_down
        .get_value()
        .unwrap_or(Callback::new(|_| {}));
    let on_pointer_down_outside = callbacks
        .on_pointer_down_outside
        .get_value()
        .unwrap_or(Callback::new(|_| {}));
    let on_interact_outside = callbacks
        .on_interact_outside
        .get_value()
        .unwrap_or(Callback::new(|_| {}));

    let on_focus_outside = Callback::new(move |event: web_sys::CustomEvent| {
        callbacks.on_focus_outside.with_value(|cb| {
            if let Some(cb) = cb {
                cb.run(event.clone());
            }
        });
        event.prevent_default();
    });

    let on_dismiss = context.on_dismiss;

    let composed_refs = use_internal_styles_effect(composed_refs, move |style| {
        for (name, value) in [
            (
                "--radix-hover-card-content-transform-origin",
                "var(--radix-popper-transform-origin)",
            ),
            (
                "--radix-hover-card-content-available-width",
                "var(--radix-popper-available-width)",
            ),
            (
                "--radix-hover-card-content-available-height",
                "var(--radix-popper-available-height)",
            ),
            (
                "--radix-hover-card-trigger-width",
                "var(--radix-popper-anchor-width)",
            ),
            (
                "--radix-hover-card-trigger-height",
                "var(--radix-popper-anchor-height)",
            ),
        ] {
            if style.get_property_value(name).unwrap_or_default().is_empty() {
                let _ = style.set_property(name, value);
            }
        }
        if contain_selection.get() {
            let _ = style.set_property("user-select", "text");
            let _ = style.set_property("-webkit-user-select", "text");
        } else {
            let _ = style.remove_property("user-select");
            let _ = style.remove_property("-webkit-user-select");
        }
    });

    view! {
        <DismissableLayer
            as_child=true
            disable_outside_pointer_events=false
            on_interact_outside=on_interact_outside
            on_escape_key_down=on_escape_key_down
            on_pointer_down_outside=on_pointer_down_outside
            on_focus_outside=on_focus_outside
            on_dismiss=Callback::new(move |_| {
                on_dismiss.run(());
            })
        >
            <PopperContent
                side=side
                side_offset=side_offset
                align=align
                align_offset=align_offset
                arrow_padding=arrow_padding
                avoid_collisions=avoid_collisions
                collision_boundary=collision_boundary
                collision_padding=collision_padding
                sticky=sticky
                hide_when_detached=hide_when_detached
                update_position_strategy=update_position_strategy
                as_child=as_child
                node_ref=composed_refs
                attr:data-state=move || open_closed_state(context.open.get())
                on:pointerenter=move |event: ev::PointerEvent| {
                    on_pointer_enter.run(event);
                }
                on:pointerleave=move |event: ev::PointerEvent| {
                    on_pointer_leave.run(event);
                }
                on:pointerdown=move |event: ev::PointerEvent| {
                    // Contain selection to current layer
                    if let Some(current_target) = event.current_target() {
                        let current_target: web_sys::HtmlElement = current_target.unchecked_into();
                        if let Some(target) = event.target() {
                            let target: web_sys::Node = target.unchecked_into();
                            if current_target.contains(Some(&target)) {
                                contain_selection.set(true);
                            }
                        }
                    }
                    context.has_selection_ref.set(false);
                    context.is_pointer_down_on_content_ref.set(true);
                }
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </PopperContent>
        </DismissableLayer>
    }
}

/* -------------------------------------------------------------------------------------------------
 * HoverCardArrow
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn HoverCardArrow(
    #[prop(into, optional)] width: MaybeProp<f64>,
    #[prop(into, optional)] height: MaybeProp<f64>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <AttributeInterceptor let:attrs>
            <PopperArrow
                width=width
                height=height
                as_child=as_child
                node_ref={node_ref}
                {..attrs}
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </PopperArrow>
        </AttributeInterceptor>
    }
}
