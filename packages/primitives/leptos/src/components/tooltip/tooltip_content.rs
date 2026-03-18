use super::*;

/* -------------------------------------------------------------------------------------------------
 * TooltipPortal
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn TooltipPortal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    #[prop(optional)] container_ref: AnyNodeRef,
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let tooltip_context = expect_context::<TooltipContextValue>();
    let provider_context = expect_context::<TooltipProviderContextValue>();
    let popper_scope = use_popper_scope();

    view! {
        <ScopedPortal
            container=container
            container_ref=container_ref
            force_mount=force_mount
            context_bridge=Callback::new(move |_| {
                provide_context(tooltip_context);
                provide_context(provider_context);
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
 * TooltipContent
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn TooltipContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<ev::CustomEvent>>,
    // PopperContent forwarded props
    #[prop(into, optional, default = Side::Top.into())] side: Signal<Side>,
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

    let context = expect_context::<TooltipContextValue>();

    let force_mount = resolve_force_mount(force_mount);

    let present = Signal::derive(move || force_mount.get() || context.open.get());

    let presence_ref = AnyNodeRef::new();

    let callbacks = ContentCallbacks {
        on_escape_key_down: StoredValue::new(on_escape_key_down),
        on_pointer_down_outside: StoredValue::new(on_pointer_down_outside),
    };

    let disable_hoverable = context.disable_hoverable_content;

    view! {
        <Presence present=present node_ref=presence_ref>
            {move || {
                if disable_hoverable.get() {
                    view! {
                        <TooltipContentImpl
                            callbacks=callbacks
                            aria_label=aria_label
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
                        </TooltipContentImpl>
                    }.into_any()
                } else {
                    view! {
                        <TooltipContentHoverable
                            callbacks=callbacks
                            aria_label=aria_label
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
                        </TooltipContentHoverable>
                    }.into_any()
                }
            }}
        </Presence>
    }
}

/* -------------------------------------------------------------------------------------------------
 * TooltipContentHoverable
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn TooltipContentHoverable(
    callbacks: ContentCallbacks,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    // PopperContent forwarded props
    #[prop(into, optional, default = Side::Top.into())] side: Signal<Side>,
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

    let context = expect_context::<TooltipContextValue>();
    let provider_context = expect_context::<TooltipProviderContextValue>();
    let content_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, content_ref]);
    let pointer_grace_area: RwSignal<Option<Vec<Point>>> = RwSignal::new(None);

    let on_close = context.on_close;
    let on_pointer_in_transit_change = provider_context.on_pointer_in_transit_change;

    let handle_remove_grace_area = Callback::new(move |_: ()| {
        pointer_grace_area.set(None);
        on_pointer_in_transit_change.run(false);
    });

    on_cleanup(move || {
        handle_remove_grace_area.run(());
    });

    // Set up pointerleave listeners on trigger and content to create grace areas
    let trigger_leave_closure: ClosureCell<dyn Fn(web_sys::PointerEvent)> = closure_cell();
    let content_leave_closure: ClosureCell<dyn Fn(web_sys::PointerEvent)> = closure_cell();

    Effect::new({
        let trigger_leave_closure = trigger_leave_closure.clone();
        let content_leave_closure = content_leave_closure.clone();
        move |_| {
            let trigger_el = context.trigger.get();
            let content_el = content_ref.get();

            // Clean up previous listeners
            {
                let content_html: Option<web_sys::HtmlElement> =
                    content_el.as_ref().map(|n| (*n).clone().unchecked_into());
                cleanup_grace_area_listeners(
                    &trigger_el,
                    &content_html,
                    &trigger_leave_closure,
                    &content_leave_closure,
                );
            }

            if let (Some(trigger_sw), Some(content_node)) = (trigger_el.as_ref(), content_el) {
                let trigger: &web_sys::HtmlElement = trigger_sw;
                let content: web_sys::HtmlElement = content_node.unchecked_into();

                // When pointer leaves trigger, create grace area toward content
                let content_for_trigger = content.clone();
                let trigger_leave = Closure::<dyn Fn(web_sys::PointerEvent)>::new(
                    move |event: web_sys::PointerEvent| {
                        let current_target = event.current_target();
                        if let Some(current_target) = current_target {
                            let current_target: web_sys::HtmlElement =
                                current_target.unchecked_into();
                            let exit_point = Point {
                                x: event.client_x() as f64,
                                y: event.client_y() as f64,
                            };
                            let exit_side = get_exit_side_from_rect(
                                &exit_point,
                                &Rect::from(&current_target.get_bounding_client_rect()),
                            );
                            let padded_exit_points =
                                get_padded_exit_points(&exit_point, &exit_side);
                            let hover_target_points = get_points_from_rect(&Rect::from(
                                &content_for_trigger.get_bounding_client_rect(),
                            ));
                            let mut all_points = padded_exit_points;
                            all_points.extend(hover_target_points);
                            let grace_area = get_hull(&all_points);
                            pointer_grace_area.set(Some(grace_area));
                            on_pointer_in_transit_change.run(true);
                        }
                    },
                );
                trigger
                    .add_event_listener_with_callback(
                        "pointerleave",
                        trigger_leave.as_ref().unchecked_ref(),
                    )
                    .ok();
                *trigger_leave_closure.borrow_mut() = Some(trigger_leave);

                // When pointer leaves content, create grace area toward trigger
                let trigger_clone: web_sys::HtmlElement = trigger.clone();
                let content_leave = Closure::<dyn Fn(web_sys::PointerEvent)>::new(
                    move |event: web_sys::PointerEvent| {
                        let current_target = event.current_target();
                        if let Some(current_target) = current_target {
                            let current_target: web_sys::HtmlElement =
                                current_target.unchecked_into();
                            let exit_point = Point {
                                x: event.client_x() as f64,
                                y: event.client_y() as f64,
                            };
                            let exit_side = get_exit_side_from_rect(
                                &exit_point,
                                &Rect::from(&current_target.get_bounding_client_rect()),
                            );
                            let padded_exit_points =
                                get_padded_exit_points(&exit_point, &exit_side);
                            let hover_target_points = get_points_from_rect(&Rect::from(
                                &trigger_clone.get_bounding_client_rect(),
                            ));
                            let mut all_points = padded_exit_points;
                            all_points.extend(hover_target_points);
                            let grace_area = get_hull(&all_points);
                            pointer_grace_area.set(Some(grace_area));
                            on_pointer_in_transit_change.run(true);
                        }
                    },
                );
                content
                    .add_event_listener_with_callback(
                        "pointerleave",
                        content_leave.as_ref().unchecked_ref(),
                    )
                    .ok();
                *content_leave_closure.borrow_mut() = Some(content_leave);
            }
        }
    });

    // Track pointer movement to check if pointer is within grace area
    let pointermove_closure: ClosureCell<dyn Fn(web_sys::PointerEvent)> = closure_cell();

    Effect::new({
        let pointermove_closure = pointermove_closure.clone();
        move |_| {
            let grace_area = pointer_grace_area.get();

            // Remove previous listener
            if let Some(closure) = pointermove_closure.borrow().as_ref() {
                if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                    document
                        .remove_event_listener_with_callback(
                            "pointermove",
                            closure.as_ref().unchecked_ref(),
                        )
                        .ok();
                }
            }
            *pointermove_closure.borrow_mut() = None;

            if let Some(grace_area) = grace_area {
                let trigger_el = context.trigger.get_untracked();
                let content_el: Option<web_sys::HtmlElement> =
                    content_ref.get_untracked().map(|n| n.unchecked_into());

                let closure = Closure::<dyn Fn(web_sys::PointerEvent)>::new(
                    move |event: web_sys::PointerEvent| {
                        let target: Option<web_sys::Node> =
                            event.target().map(|t| t.unchecked_into());
                        let pointer_position = Point {
                            x: event.client_x() as f64,
                            y: event.client_y() as f64,
                        };

                        let has_entered_target = target.as_ref().is_some_and(|target| {
                            trigger_el
                                .as_ref()
                                .is_some_and(|t| t.contains(Some(target)))
                                || content_el
                                    .as_ref()
                                    .is_some_and(|c| c.contains(Some(target)))
                        });

                        let is_pointer_outside_grace_area =
                            !is_point_in_polygon(&pointer_position, &grace_area);

                        if has_entered_target {
                            handle_remove_grace_area.run(());
                        } else if is_pointer_outside_grace_area {
                            handle_remove_grace_area.run(());
                            on_close.run(());
                        }
                    },
                );

                if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                    document
                        .add_event_listener_with_callback(
                            "pointermove",
                            closure.as_ref().unchecked_ref(),
                        )
                        .ok();
                }
                *pointermove_closure.borrow_mut() = Some(closure);
            }
        }
    });

    on_cleanup(move || {
        // Clean up grace area listeners
        let trigger_el = context.trigger.get_untracked();
        let content_el: Option<web_sys::HtmlElement> =
            content_ref.get_untracked().map(|n| n.unchecked_into());
        cleanup_grace_area_listeners(
            &trigger_el,
            &content_el,
            &trigger_leave_closure,
            &content_leave_closure,
        );

        // Clean up pointermove listener
        if let Some(closure) = pointermove_closure.borrow().as_ref() {
            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                document
                    .remove_event_listener_with_callback(
                        "pointermove",
                        closure.as_ref().unchecked_ref(),
                    )
                    .ok();
            }
        }
    });

    view! {
        <TooltipContentImpl
            callbacks=callbacks
            aria_label=aria_label
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
            presence_ref=presence_ref
        >
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </TooltipContentImpl>
    }
}

/* -------------------------------------------------------------------------------------------------
 * TooltipContentImpl
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy)]
struct ContentCallbacks {
    on_escape_key_down: StoredValue<Option<Callback<web_sys::KeyboardEvent>>>,
    on_pointer_down_outside: StoredValue<Option<Callback<web_sys::CustomEvent>>>,
}

/// Context for VisuallyHidden content — prevents TooltipArrow from rendering
/// inside the VisuallyHidden copy of children.
#[derive(Clone, Copy)]
struct VisuallyHiddenContentContextValue {
    is_inside: bool,
}

#[component]
fn TooltipContentImpl(
    callbacks: ContentCallbacks,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    // PopperContent forwarded props
    #[prop(into, optional, default = Side::Top.into())] side: Signal<Side>,
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

    let context = expect_context::<TooltipContextValue>();
    let content_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, content_ref, presence_ref]);

    // Provide default VisuallyHidden context (is_inside: false) so TooltipArrow
    // rendered in the visible children finds the correct value. The inner Provider
    // around VisuallyHidden overrides with is_inside: true for the duplicate copy.
    provide_context(VisuallyHiddenContentContextValue { is_inside: false });

    let on_close = context.on_close;

    // Close this tooltip if another one opens
    let tooltip_open_closure: ClosureCell<dyn Fn(web_sys::Event)> = closure_cell();

    Effect::new({
        let tooltip_open_closure = tooltip_open_closure.clone();
        move |_| {
            // Clean up previous listener
            if let Some(closure) = tooltip_open_closure.borrow().as_ref() {
                if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                    document
                        .remove_event_listener_with_callback(
                            TOOLTIP_OPEN,
                            closure.as_ref().unchecked_ref(),
                        )
                        .ok();
                }
            }

            let closure = Closure::<dyn Fn(web_sys::Event)>::new(move |_: web_sys::Event| {
                on_close.run(());
            });

            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                document
                    .add_event_listener_with_callback(
                        TOOLTIP_OPEN,
                        closure.as_ref().unchecked_ref(),
                    )
                    .ok();
            }
            *tooltip_open_closure.borrow_mut() = Some(closure);
        }
    });

    // Close the tooltip if the trigger is scrolled
    let scroll_closure: ClosureCell<dyn Fn(web_sys::Event)> = closure_cell();

    Effect::new({
        let scroll_closure = scroll_closure.clone();
        move |_| {
            let trigger = context.trigger.get();

            // Clean up previous listener
            if let Some(closure) = scroll_closure.borrow().as_ref() {
                if let Some(window) = web_sys::window() {
                    let opts = web_sys::EventListenerOptions::new();
                    opts.set_capture(true);
                    window
                        .remove_event_listener_with_callback_and_event_listener_options(
                            "scroll",
                            closure.as_ref().unchecked_ref(),
                            &opts,
                        )
                        .ok();
                }
            }
            *scroll_closure.borrow_mut() = None;

            if let Some(trigger_el) = trigger {
                let closure =
                    Closure::<dyn Fn(web_sys::Event)>::new(move |event: web_sys::Event| {
                        if let Some(target) = event.target() {
                            let target: web_sys::HtmlElement = target.unchecked_into();
                            if target.contains(Some(&trigger_el)) {
                                on_close.run(());
                            }
                        }
                    });

                if let Some(window) = web_sys::window() {
                    let opts = web_sys::AddEventListenerOptions::new();
                    opts.set_capture(true);
                    window
                        .add_event_listener_with_callback_and_add_event_listener_options(
                            "scroll",
                            closure.as_ref().unchecked_ref(),
                            &opts,
                        )
                        .ok();
                }
                *scroll_closure.borrow_mut() = Some(closure);
            }
        }
    });

    on_cleanup(move || {
        if let Some(closure) = tooltip_open_closure.borrow().as_ref() {
            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                document
                    .remove_event_listener_with_callback(
                        TOOLTIP_OPEN,
                        closure.as_ref().unchecked_ref(),
                    )
                    .ok();
            }
        }
        if let Some(closure) = scroll_closure.borrow().as_ref() {
            if let Some(window) = web_sys::window() {
                let opts = web_sys::EventListenerOptions::new();
                opts.set_capture(true);
                window
                    .remove_event_listener_with_callback_and_event_listener_options(
                        "scroll",
                        closure.as_ref().unchecked_ref(),
                        &opts,
                    )
                    .ok();
            }
        }
    });

    // Apply custom CSS properties via Effect
    Effect::new(move |_| {
        if let Some(el) = content_ref.get() {
            let el: web_sys::HtmlElement = el.unchecked_into();
            let style = el.style();
            let _ = style.set_property(
                "--radix-tooltip-content-transform-origin",
                "var(--radix-popper-transform-origin)",
            );
            let _ = style.set_property(
                "--radix-tooltip-content-available-width",
                "var(--radix-popper-available-width)",
            );
            let _ = style.set_property(
                "--radix-tooltip-content-available-height",
                "var(--radix-popper-available-height)",
            );
            let _ = style.set_property(
                "--radix-tooltip-trigger-width",
                "var(--radix-popper-anchor-width)",
            );
            let _ = style.set_property(
                "--radix-tooltip-trigger-height",
                "var(--radix-popper-anchor-height)",
            );
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

    let on_focus_outside = Callback::new(move |event: web_sys::CustomEvent| {
        event.prevent_default();
    });

    let on_dismiss = context.on_close;

    view! {
        <DismissableLayer
            as_child=true
            disable_outside_pointer_events=false
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
                attr:data-state=move || context.state_attribute.get()
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
                <Provider value=VisuallyHiddenContentContextValue { is_inside: true }>
                    <VisuallyHidden
                        attr:id=move || context.content_id.get()
                        attr:role="tooltip"
                    >
                        {move || {
                            let label = aria_label.get();
                            if label.is_some() {
                                label.into_any()
                            } else {
                                children.with_value(|children| children.as_ref().map(|children| children())).into_any()
                            }
                        }}
                    </VisuallyHidden>
                </Provider>
            </PopperContent>
        </DismissableLayer>
    }
}

/* -------------------------------------------------------------------------------------------------
 * TooltipArrow
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn TooltipArrow(
    #[prop(into, optional)] width: MaybeProp<f64>,
    #[prop(into, optional)] height: MaybeProp<f64>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let visually_hidden_context = use_context::<VisuallyHiddenContentContextValue>();
    // If the arrow is inside the VisuallyHidden, don't render it to prevent
    // positioning issues due to the duplicate
    if visually_hidden_context.is_some_and(|ctx| ctx.is_inside) {
        return None::<AnyView>.into_any();
    }

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
    .into_any()
}
