use super::*;

/* -------------------------------------------------------------------------------------------------
 * PopoverPortal
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn PopoverPortal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    #[prop(optional)] container_ref: AnyNodeRef,
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let popover_context = expect_context::<PopoverContextValue>();
    let popper_scope = use_popper_scope();

    view! {
        <ScopedPortal
            container=container
            container_ref=container_ref
            force_mount=force_mount
            context_bridge=Callback::new(move |_| {
                provide_context(popover_context);
                if let Some(scope) = popper_scope {
                    provide_popper_scope(scope);
                }
            })
        >
            {children.with_value(|children| children())}
        </ScopedPortal>
    }
}

/* -------------------------------------------------------------------------------------------------
 * PopoverContent
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn PopoverContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] on_open_auto_focus: Option<Callback<ev::Event>>,
    #[prop(into, optional)] on_close_auto_focus: Option<Callback<ev::Event>>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<ev::CustomEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<ev::CustomEvent>>,
    #[prop(into, optional)] on_interact_outside: Option<Callback<ev::CustomEvent>>,
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

    let context = expect_context::<PopoverContextValue>();

    let force_mount = resolve_force_mount(force_mount);

    let present = Signal::derive(move || force_mount.get() || context.open.get());
    let is_modal = context.modal;

    let callbacks = ContentCallbacks {
        on_open_auto_focus: StoredValue::new(on_open_auto_focus),
        on_close_auto_focus: StoredValue::new(on_close_auto_focus),
        on_escape_key_down: StoredValue::new(on_escape_key_down),
        on_pointer_down_outside: StoredValue::new(on_pointer_down_outside),
        on_focus_outside: StoredValue::new(on_focus_outside),
        on_interact_outside: StoredValue::new(on_interact_outside),
    };

    let presence_ref = AnyNodeRef::new();

    view! {
        <Presence present=present node_ref=presence_ref>
            <Show
                when=move || is_modal.get()
                fallback=move || view! {
                    <PopoverContentNonModal
                        callbacks=callbacks
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
                    </PopoverContentNonModal>
                }
            >
                <PopoverContentModal
                    callbacks=callbacks
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
                </PopoverContentModal>
            </Show>
        </Presence>
    }
}

/* -------------------------------------------------------------------------------------------------
 * PopoverContentModal
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn PopoverContentModal(
    callbacks: ContentCallbacks,
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

    let context = expect_context::<PopoverContextValue>();
    let content_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, content_ref, presence_ref]);

    let is_right_click_outside_ref: StoredValue<bool> = StoredValue::new(false);

    // aria-hide everything except the content (better supported equivalent to setting aria-modal).
    // Deferred to requestAnimationFrame so that FocusScope's deferred auto-focus moves focus
    // into the popover content before hide_others sets aria-hidden on outside elements.
    let hidden_elements: RwSignal<Vec<SendWrapper<web_sys::Element>>> = RwSignal::new(Vec::new());

    Effect::new(move |_| {
        if let Some(content) = content_ref.get() {
            let content: web_sys::HtmlElement = content.unchecked_into();
            let cb = Closure::once_into_js(move || {
                hide_others(&content, hidden_elements);
            });
            web_sys::window()
                .expect("Window should exist.")
                .request_animation_frame(cb.unchecked_ref())
                .ok();
        }
    });

    on_cleanup(move || {
        unhide_others(hidden_elements);
    });

    let on_close_auto_focus = Callback::new(move |event: web_sys::Event| {
        callbacks.on_close_auto_focus.with_value(|cb| {
            if let Some(cb) = cb {
                cb.run(event.clone());
            }
        });
        // Remove aria-hidden from outside elements BEFORE focusing the trigger.
        unhide_others(hidden_elements);
        event.prevent_default();
        if !is_right_click_outside_ref.get_value()
            && let Some(trigger) = context.trigger_ref.get_untracked()
        {
            let trigger: &web_sys::HtmlElement = trigger.unchecked_ref();
            trigger.focus().ok();
        }
    });

    let on_pointer_down_outside = Callback::new(move |event: web_sys::CustomEvent| {
        callbacks.on_pointer_down_outside.with_value(|cb| {
            if let Some(cb) = cb {
                cb.run(event.clone());
            }
        });
        let original_event = js_sys::Reflect::get(&event.detail(), &"originalEvent".into())
            .ok()
            .and_then(|v| v.dyn_into::<web_sys::PointerEvent>().ok());
        if let Some(original_event) = original_event {
            let ctrl_left_click = original_event.button() == 0 && original_event.ctrl_key();
            let is_right_click = original_event.button() == 2 || ctrl_left_click;
            is_right_click_outside_ref.set_value(is_right_click);
        }
    });

    let on_focus_outside = Callback::new(move |event: web_sys::CustomEvent| {
        callbacks.on_focus_outside.with_value(|cb| {
            if let Some(cb) = cb {
                cb.run(event.clone());
            }
        });
        // When focus is trapped, a `focusout` event may still happen.
        // We make sure we don't trigger our `onDismiss` in such case.
        event.prevent_default();
    });

    let on_open_auto_focus = callbacks
        .on_open_auto_focus
        .get_value()
        .unwrap_or(Callback::new(|_| {}));
    let on_escape_key_down = callbacks
        .on_escape_key_down
        .get_value()
        .unwrap_or(Callback::new(|_| {}));
    let on_interact_outside = callbacks
        .on_interact_outside
        .get_value()
        .unwrap_or(Callback::new(|_| {}));

    // Body scroll lock for modal popover
    use_body_scroll_lock();

    view! {
        <PopoverContentImpl
            trap_focus=context.open
            disable_outside_pointer_events=true
            on_open_auto_focus=on_open_auto_focus
            on_close_auto_focus=on_close_auto_focus
            on_escape_key_down=on_escape_key_down
            on_pointer_down_outside=on_pointer_down_outside
            on_focus_outside=on_focus_outside
            on_interact_outside=on_interact_outside
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
        >
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </PopoverContentImpl>
    }
}

/* -------------------------------------------------------------------------------------------------
 * PopoverContentNonModal
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn PopoverContentNonModal(
    callbacks: ContentCallbacks,
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

    let context = expect_context::<PopoverContextValue>();
    let composed_refs = use_composed_refs(vec![node_ref, presence_ref]);

    let has_interacted_outside = RwSignal::new(false);
    let has_pointer_down_outside = RwSignal::new(false);

    let on_close_auto_focus = Callback::new(move |event: web_sys::Event| {
        callbacks.on_close_auto_focus.with_value(|cb| {
            if let Some(cb) = cb {
                cb.run(event.clone());
            }
        });

        if !event.default_prevented() {
            if !has_interacted_outside.get_untracked()
                && let Some(trigger) = context.trigger_ref.get_untracked()
            {
                let trigger: &web_sys::HtmlElement = trigger.unchecked_ref();
                trigger.focus().ok();
            }
            // Always prevent auto focus because we either focus manually or want user agent focus
            event.prevent_default();
        }

        has_interacted_outside.set(false);
        has_pointer_down_outside.set(false);
    });

    let on_interact_outside = Callback::new(move |event: web_sys::CustomEvent| {
        callbacks.on_interact_outside.with_value(|cb| {
            if let Some(cb) = cb {
                cb.run(event.clone());
            }
        });

        if !event.default_prevented() {
            has_interacted_outside.set(true);
            let original_event_type =
                js_sys::Reflect::get(&event.detail(), &"originalEvent".into())
                    .ok()
                    .and_then(|v| v.dyn_into::<web_sys::Event>().ok())
                    .map(|e| e.type_());

            if original_event_type.as_deref() == Some("pointerdown") {
                has_pointer_down_outside.set(true);
            }
        }

        // Prevent dismissing when clicking the trigger.
        // As the trigger is already setup to close, without doing so would
        // cause it to close and immediately open.
        if let Some(target) = event.target() {
            let target: web_sys::Node = target.unchecked_into();
            if let Some(trigger) = context.trigger_ref.get_untracked() {
                let trigger: &web_sys::Node = trigger.unchecked_ref();
                if trigger.contains(Some(&target)) {
                    event.prevent_default();
                }
            }
        }

        // On Safari if the trigger is inside a container with tabIndex={0}, when clicked
        // we will get the pointer down outside event on the trigger, but then a subsequent
        // focus outside event on the container, we ignore any focus outside event when we've
        // already had a pointer down outside event.
        let original_event_type = js_sys::Reflect::get(&event.detail(), &"originalEvent".into())
            .ok()
            .and_then(|v| v.dyn_into::<web_sys::Event>().ok())
            .map(|e| e.type_());
        if original_event_type.as_deref() == Some("focusin")
            && has_pointer_down_outside.get_untracked()
        {
            event.prevent_default();
        }
    });

    let on_open_auto_focus = callbacks
        .on_open_auto_focus
        .get_value()
        .unwrap_or(Callback::new(|_| {}));
    let on_escape_key_down = callbacks
        .on_escape_key_down
        .get_value()
        .unwrap_or(Callback::new(|_| {}));
    let on_pointer_down_outside = callbacks
        .on_pointer_down_outside
        .get_value()
        .unwrap_or(Callback::new(|_| {}));
    let on_focus_outside = callbacks
        .on_focus_outside
        .get_value()
        .unwrap_or(Callback::new(|_| {}));

    view! {
        <PopoverContentImpl
            trap_focus=false
            disable_outside_pointer_events=false
            on_open_auto_focus=on_open_auto_focus
            on_close_auto_focus=on_close_auto_focus
            on_escape_key_down=on_escape_key_down
            on_pointer_down_outside=on_pointer_down_outside
            on_focus_outside=on_focus_outside
            on_interact_outside=on_interact_outside
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
        >
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </PopoverContentImpl>
    }
}

/* -------------------------------------------------------------------------------------------------
 * PopoverContentImpl
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn PopoverContentImpl(
    #[prop(into, optional)] trap_focus: MaybeProp<bool>,
    #[prop(into, optional)] disable_outside_pointer_events: MaybeProp<bool>,
    #[prop(into)] on_open_auto_focus: Callback<web_sys::Event>,
    #[prop(into)] on_close_auto_focus: Callback<web_sys::Event>,
    #[prop(into)] on_escape_key_down: Callback<web_sys::KeyboardEvent>,
    #[prop(into)] on_pointer_down_outside: Callback<web_sys::CustomEvent>,
    #[prop(into)] on_focus_outside: Callback<web_sys::CustomEvent>,
    #[prop(into)] on_interact_outside: Callback<web_sys::CustomEvent>,
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

    let context = expect_context::<PopoverContextValue>();

    // Make sure the whole tree has focus guards as our `Popover` may be
    // the last element in the DOM (because of the `Portal`)
    use_focus_guards();

    let trapped = prop_or_default(trap_focus);
    let disable_outside = prop_or_default(disable_outside_pointer_events);

    view! {
        <FocusScope
            as_child=true
            r#loop=true
            trapped=trapped
            on_mount_auto_focus=on_open_auto_focus
            on_unmount_auto_focus=Some(on_close_auto_focus)
        >
            <DismissableLayer
                as_child=true
                disable_outside_pointer_events=disable_outside
                on_escape_key_down=on_escape_key_down
                on_pointer_down_outside=on_pointer_down_outside
                on_focus_outside=on_focus_outside
                on_interact_outside=on_interact_outside
                on_dismiss=Callback::new(move |_| {
                    context.on_open_change.run(false);
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
                    node_ref=node_ref
                    style:--popover-content-transform-origin="var(--popper-transform-origin)"
                    style:--popover-content-available-width="var(--popper-available-width)"
                    style:--popover-content-available-height="var(--popper-available-height)"
                    style:--popover-trigger-width="var(--popper-anchor-width)"
                    style:--popover-trigger-height="var(--popper-anchor-height)"
                    attr:data-state=move || open_closed_state(context.open.get())
                    attr:role="dialog"
                    attr:id=move || context.content_id.get()
                >
                    {children.with_value(|children| children.as_ref().map(|children| children()))}
                </PopperContent>
            </DismissableLayer>
        </FocusScope>
    }
}

/* -------------------------------------------------------------------------------------------------
 * PopoverClose
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn PopoverClose(
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<PopoverContextValue>();

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

/* -------------------------------------------------------------------------------------------------
 * PopoverArrow
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn PopoverArrow(
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
