use super::*;

/* -------------------------------------------------------------------------------------------------
 * Tooltip
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn Tooltip(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] delay_duration: MaybeProp<f64>,
    #[prop(into, optional)] disable_hoverable_content: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let provider_context = expect_context::<TooltipProviderContextValue>();

    let trigger: RwSignal<Option<SendWrapper<web_sys::HtmlElement>>> = RwSignal::new(None);
    let content_id = use_id(None);
    let content_id_signal = Signal::derive(move || content_id.get());
    let open_timer_ref: StoredValue<Option<i32>> = StoredValue::new(None);
    let was_open_delayed_ref: StoredValue<bool> = StoredValue::new(false);

    let resolved_disable_hoverable_content = Signal::derive(move || {
        disable_hoverable_content
            .get()
            .unwrap_or_else(|| provider_context.disable_hoverable_content.get())
    });
    let resolved_delay_duration = Signal::derive(move || {
        delay_duration
            .get()
            .unwrap_or_else(|| provider_context.delay_duration.get())
    });

    let on_open_change_stored = StoredValue::new(on_open_change);

    let (open_signal, set_open) = use_controllable_state(UseControllableStateParams {
        prop: open,
        default_prop: default_open,
        on_change: Some(Callback::new(move |value: Option<bool>| {
            if let Some(open_val) = value {
                if open_val {
                    provider_context.on_open.run(());
                    // Dispatch tooltip.open custom event for cross-tooltip coordination
                    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                        let init = web_sys::CustomEventInit::new();
                        init.set_bubbles(false);
                        if let Ok(event) =
                            web_sys::CustomEvent::new_with_event_init_dict(TOOLTIP_OPEN, &init)
                        {
                            let _ = document.dispatch_event(&event);
                        }
                    }
                } else {
                    provider_context.on_close.run(());
                }
                on_open_change_stored.with_value(|cb| {
                    if let Some(cb) = cb {
                        cb.run(open_val);
                    }
                });
            }
        })),
    });

    let open_derived = Signal::derive(move || open_signal.get().unwrap_or(false));

    let state_attribute = Signal::derive(move || {
        tooltip_state_attribute(open_derived.get(), was_open_delayed_ref.get_value())
    });

    let handle_open = Callback::new(move |_: ()| {
        clear_timeout(open_timer_ref);
        was_open_delayed_ref.set_value(false);
        set_open.run(Some(true));
    });

    let handle_close = Callback::new(move |_: ()| {
        clear_timeout(open_timer_ref);
        set_open.run(Some(false));
    });

    let handle_delayed_open = Callback::new(move |_: ()| {
        clear_timeout(open_timer_ref);
        let delay = resolved_delay_duration.get_untracked();
        let timeout_id = set_timeout(
            move || {
                was_open_delayed_ref.set_value(true);
                set_open.run(Some(true));
            },
            delay as i32,
        );
        open_timer_ref.set_value(Some(timeout_id));
    });

    on_cleanup(move || {
        clear_timeout(open_timer_ref);
    });

    let on_trigger_enter = Callback::new(move |_: ()| {
        if provider_context.is_open_delayed.get_untracked() {
            handle_delayed_open.run(());
        } else {
            handle_open.run(());
        }
    });

    let on_trigger_leave = Callback::new(move |_: ()| {
        if resolved_disable_hoverable_content.get_untracked() {
            handle_close.run(());
        } else {
            // Clear the timer in case the pointer leaves the trigger before the tooltip is opened.
            clear_timeout(open_timer_ref);
        }
    });

    let context = TooltipContextValue {
        content_id: content_id_signal,
        open: open_derived,
        state_attribute,
        trigger,
        on_trigger_enter,
        on_trigger_leave,
        on_open: handle_open,
        on_close: handle_close,
        disable_hoverable_content: resolved_disable_hoverable_content,
    };

    view! {
        <Provider value=context>
            <Popper>
                {children.with_value(|children| children())}
            </Popper>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * TooltipTrigger
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn TooltipTrigger(
    #[prop(into, optional)] on_pointer_move: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_leave: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_down: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_focus: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] on_blur: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<TooltipContextValue>();
    let provider_context = expect_context::<TooltipProviderContextValue>();

    let trigger_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, trigger_ref]);
    let is_pointer_down_ref: StoredValue<bool> = StoredValue::new(false);
    let has_pointer_move_opened_ref: StoredValue<bool> = StoredValue::new(false);

    // Sync trigger element ref to context
    Effect::new(move |_| {
        if let Some(el) = trigger_ref.get() {
            let html_el: web_sys::HtmlElement = el.unchecked_into();
            context.trigger.set(Some(SendWrapper::new(html_el)));
        }
    });

    // We use Closure::once_into_js for the document-level pointerup listener because:
    // 1. The listener uses { once: true } so the browser auto-removes it after firing.
    // 2. once_into_js leaks the closure to JS, preventing premature Rust-side drops.
    // This avoids "closure invoked after being dropped" errors that occur when a
    // StoredValue holding the Closure is disposed during reactive scope cleanup
    // while the document listener still references it.

    view! {
        <PopperAnchor as_child=true>
            <AttributeInterceptor let:attrs>
                <Primitive
                    element=html::button
                    as_child=as_child
                    node_ref=composed_refs
                    attr:aria-describedby=move || {
                        if context.open.get() {
                            Some(context.content_id.get())
                        } else {
                            None
                        }
                    }
                    attr:data-state=move || context.state_attribute.get()
                    on:pointermove=compose_callbacks(
                        on_pointer_move,
                        Some(Callback::new(move |event: ev::PointerEvent| {
                            if event.pointer_type() == "touch" {
                                return;
                            }
                            if !has_pointer_move_opened_ref.get_value()
                                && !provider_context.is_pointer_in_transit.get_untracked()
                            {
                                context.on_trigger_enter.run(());
                                has_pointer_move_opened_ref.set_value(true);
                            }
                        })),
                        None,
                    )
                    on:pointerleave=compose_callbacks(
                        on_pointer_leave,
                        Some(Callback::new(move |_: ev::PointerEvent| {
                            context.on_trigger_leave.run(());
                            has_pointer_move_opened_ref.set_value(false);
                        })),
                        None,
                    )
                    on:pointerdown=compose_callbacks(
                        on_pointer_down,
                        Some(Callback::new(move |_: ev::PointerEvent| {
                            // Defer the close to a microtask to avoid synchronous DOM mutations
                            // during Leptos's delegated event dispatch. Unlike React 18+ which
                            // batches state updates during event handlers, Leptos signal updates
                            // trigger immediate re-renders. If the tooltip content unmounts
                            // synchronously during a pointerdown handler while the delegated
                            // event walk is still in progress, closures stored on DOM elements
                            // can be invalidated, causing "closure invoked after being dropped".
                            if context.open.get_untracked() {
                                let on_close = context.on_close;
                                queue_microtask(move || {
                                    on_close.run(());
                                });
                            }
                            is_pointer_down_ref.set_value(true);

                            // Register a one-time pointerup handler on document.
                            // Using once_into_js so the closure is leaked to JS and won't
                            // be dropped when Leptos disposes the reactive scope. The
                            // { once: true } option ensures the browser removes the listener
                            // after it fires. We use try_set_value so that if the scope has
                            // been disposed by the time pointerup fires, it's a no-op.
                            let cb = Closure::once_into_js(move || {
                                let _ = is_pointer_down_ref.try_set_value(false);
                            });
                            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                                let opts = web_sys::AddEventListenerOptions::new();
                                opts.set_once(true);
                                document
                                    .add_event_listener_with_callback_and_add_event_listener_options(
                                        "pointerup",
                                        cb.unchecked_ref(),
                                        &opts,
                                    )
                                    .ok();
                            }
                        })),
                        None,
                    )
                    on:focus=compose_callbacks(
                        on_focus,
                        Some(Callback::new(move |_: ev::FocusEvent| {
                            if !is_pointer_down_ref.get_value() {
                                context.on_open.run(());
                            }
                        })),
                        None,
                    )
                    on:blur=compose_callbacks(
                        on_blur,
                        Some(Callback::new(move |_: ev::FocusEvent| {
                            context.on_close.run(());
                        })),
                        None,
                    )
                    on:click=compose_callbacks(
                        on_click,
                        Some(Callback::new(move |_: ev::MouseEvent| {
                            context.on_close.run(());
                        })),
                        None,
                    )
                    {..attrs}
                >
                    {children.with_value(|children| children.as_ref().map(|children| children()))}
                </Primitive>
            </AttributeInterceptor>
        </PopperAnchor>
    }
}
