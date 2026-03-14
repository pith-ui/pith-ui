use super::*;

/* -------------------------------------------------------------------------------------------------
 * HoverCard
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn HoverCard(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional, default = MaybeProp::from(700.0))] open_delay: MaybeProp<f64>,
    #[prop(into, optional, default = MaybeProp::from(300.0))] close_delay: MaybeProp<f64>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    // Capture parent hover card context (if nested) before providing our own.
    // React's synthetic events bubble through the React component tree even across
    // portals, so a parent HoverCardContent's onPointerLeave doesn't fire when the
    // mouse moves to a child's portalled content. Leptos uses native DOM events which
    // DO fire pointerleave when the mouse crosses a portal boundary. To compensate,
    // we chain on_open calls up the parent hierarchy: when a child hover card becomes
    // active, it cancels all ancestor close timers.
    let parent_context = use_context::<HoverCardContextValue>();

    let (open_signal, set_open) = use_controllable_state(UseControllableStateParams {
        prop: open,
        default_prop: default_open,
        on_change: adapt_callback(on_open_change),
    });
    let open = Signal::derive(move || open_signal.get().unwrap_or(false));

    let open_timer_ref: StoredValue<Option<i32>> = StoredValue::new(None);
    let close_timer_ref: StoredValue<Option<i32>> = StoredValue::new(None);
    let has_selection_ref = RwSignal::new(false);
    let is_pointer_down_on_content_ref = RwSignal::new(false);

    let handle_open = Callback::new(move |_: ()| {
        clear_timeout(close_timer_ref);
        // Clear any existing open timer before scheduling a new one to prevent
        // orphaned timers. In React, orphaned timers are harmless (setState on
        // unmounted components is a no-op). In Leptos, orphaned timer callbacks
        // can panic when accessing disposed reactive values.
        clear_timeout(open_timer_ref);
        let delay = open_delay.get().unwrap_or(700.0);
        let timeout_id = set_timeout(
            move || {
                set_open.run(Some(true));
            },
            delay as i32,
        );
        open_timer_ref.set_value(Some(timeout_id));

        // Keep ancestor hover cards open. Each parent's on_open clears its close
        // timer, and itself chains to its own parent, so this propagates all the
        // way up through any number of nesting levels.
        if let Some(parent) = parent_context {
            parent.on_open.run(());
        }
    });

    let handle_close = Callback::new(move |_: ()| {
        clear_timeout(open_timer_ref);
        if !has_selection_ref.get_untracked() && !is_pointer_down_on_content_ref.get_untracked() {
            // Clear any existing close timer before scheduling a new one. Without
            // this, when handle_close is called twice (e.g., once from the on_close
            // chain and once from a native pointerleave), the first timer becomes
            // orphaned -- its ID is overwritten and it can no longer be cancelled.
            // The orphaned timer fires and closes the hover card prematurely.
            clear_timeout(close_timer_ref);
            let delay = close_delay.get().unwrap_or(300.0);
            let timeout_id = set_timeout(
                move || {
                    set_open.run(Some(false));
                },
                delay as i32,
            );
            close_timer_ref.set_value(Some(timeout_id));
        }

        // Propagate close to ancestor hover cards. Each parent's on_close starts its
        // own close timer (with its own delay and selection/pointer-down checks), and
        // itself chains to its own parent, so this propagates all the way up. Without
        // this, when the mouse leaves all nested hover cards at once, only the innermost
        // closes -- the parent's close timer was cancelled by the child's on_open chain
        // and never restarted.
        if let Some(parent) = parent_context {
            parent.on_close.run(());
        }
    });

    let handle_dismiss = Callback::new(move |_: ()| {
        set_open.run(Some(false));
    });

    on_cleanup(move || {
        clear_timeout(open_timer_ref);
        clear_timeout(close_timer_ref);
    });

    let context = HoverCardContextValue {
        open,
        on_open_change: Callback::new(move |value: bool| {
            set_open.run(Some(value));
        }),
        on_open: handle_open,
        on_close: handle_close,
        on_dismiss: handle_dismiss,
        has_selection_ref,
        is_pointer_down_on_content_ref,
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
 * HoverCardTrigger
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn HoverCardTrigger(
    #[prop(into, optional)] on_pointer_enter: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_leave: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_focus: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] on_blur: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] on_touch_start: Option<Callback<ev::TouchEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<HoverCardContextValue>();

    let on_open = context.on_open;
    let on_close = context.on_close;

    view! {
        <PopperAnchor as_child=true>
            <AttributeInterceptor let:attrs>
                <Primitive
                    element=html::a
                    as_child=as_child
                    node_ref=node_ref
                    attr:data-state=move || open_closed_state(context.open.get())
                    on:pointerenter=compose_callbacks(
                        on_pointer_enter,
                        Some(Callback::new(move |event: ev::PointerEvent| {
                            if event.pointer_type() != "touch" {
                                on_open.run(());
                            }
                        })),
                        None,
                    )
                    on:pointerleave=compose_callbacks(
                        on_pointer_leave,
                        Some(Callback::new(move |event: ev::PointerEvent| {
                            if event.pointer_type() != "touch" {
                                on_close.run(());
                            }
                        })),
                        None,
                    )
                    on:focus=compose_callbacks(
                        on_focus,
                        Some(Callback::new(move |_: ev::FocusEvent| {
                            on_open.run(());
                        })),
                        None,
                    )
                    on:blur=compose_callbacks(
                        on_blur,
                        Some(Callback::new(move |_: ev::FocusEvent| {
                            on_close.run(());
                        })),
                        None,
                    )
                    // Prevent focus event on touch devices
                    on:touchstart=compose_callbacks(
                        on_touch_start,
                        Some(Callback::new(|event: ev::TouchEvent| {
                            event.prevent_default();
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
