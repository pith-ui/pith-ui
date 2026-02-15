use leptos::{attribute_interceptor::AttributeInterceptor, context::Provider, ev, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use radix_leptos_compose_refs::use_composed_refs;
use radix_leptos_dismissable_layer::DismissableLayer;
use radix_leptos_popper::{
    Align, Padding, Popper, PopperAnchor, PopperArrow, PopperContent, Side, Sticky,
    UpdatePositionStrategy, provide_popper_scope, use_popper_scope,
};
use radix_leptos_portal::Portal;
use radix_leptos_presence::Presence;
use radix_leptos_primitive::{Primitive, compose_callbacks};
use radix_leptos_use_controllable_state::{UseControllableStateParams, use_controllable_state};
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::closure::Closure;

/* -------------------------------------------------------------------------------------------------
 * HoverCard
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy)]
struct HoverCardContextValue {
    open: Signal<bool>,
    #[allow(dead_code)]
    on_open_change: Callback<bool>,
    on_open: Callback<()>,
    on_close: Callback<()>,
    on_dismiss: Callback<()>,
    has_selection_ref: RwSignal<bool>,
    is_pointer_down_on_content_ref: RwSignal<bool>,
}

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
        on_change: on_open_change.map(|on_open_change| {
            Callback::new(move |value: Option<bool>| {
                if let Some(value) = value {
                    on_open_change.run(value);
                }
            })
        }),
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
            // orphaned — its ID is overwritten and it can no longer be cancelled.
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
        // closes — the parent's close timer was cancelled by the child's on_open chain
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
                    attr:data-state=move || get_state(context.open.get())
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

/* -------------------------------------------------------------------------------------------------
 * HoverCardPortal
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone)]
struct HoverCardPortalContextValue {
    force_mount: Signal<bool>,
}

#[component]
pub fn HoverCardPortal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    #[prop(optional)] container_ref: AnyNodeRef,
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let force_mount_signal = Signal::derive(move || force_mount.get().unwrap_or(false));

    let portal_context = HoverCardPortalContextValue {
        force_mount: force_mount_signal,
    };

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
        <Provider value=portal_context>
            <Portal container=container container_ref=container_ref as_child=true>
                <Provider value=hover_card_context>
                    {
                        if let Some(scope) = popper_scope {
                            provide_popper_scope(scope);
                        }
                        children.with_value(|children| children())
                    }
                </Provider>
            </Portal>
        </Provider>
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
    #[prop(into, optional, default = SendWrapper::new(vec![]).into())]
    collision_boundary: Signal<SendWrapper<Vec<web_sys::Element>>>,
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
    let portal_context = use_context::<HoverCardPortalContextValue>();

    let force_mount = Signal::derive(move || {
        force_mount
            .get()
            .or_else(|| portal_context.as_ref().map(|pc| pc.force_mount.get()))
            .unwrap_or(false)
    });

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

#[derive(Clone, Copy)]
struct ContentCallbacks {
    on_escape_key_down: StoredValue<Option<Callback<web_sys::KeyboardEvent>>>,
    on_pointer_down_outside: StoredValue<Option<Callback<web_sys::CustomEvent>>>,
    on_focus_outside: StoredValue<Option<Callback<web_sys::CustomEvent>>>,
    on_interact_outside: StoredValue<Option<Callback<web_sys::CustomEvent>>>,
}

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
    #[prop(into, optional, default = SendWrapper::new(vec![]).into())]
    collision_boundary: Signal<SendWrapper<Vec<web_sys::Element>>>,
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

    // Apply custom CSS properties and user-select via Effect rather than attr:style.
    // Caller attributes (including attr:style from stories) are forwarded through the
    // component chain to the inner Primitive. Using attr:style here would conflict with
    // the caller's attr:style (last one wins). An Effect uses setProperty() which sets
    // individual CSS properties without affecting the style attribute string.
    Effect::new(move |_| {
        if let Some(el) = content_ref.get() {
            let el: web_sys::HtmlElement = el.unchecked_into();
            let style = el.style();
            // Re-namespace exposed content custom properties
            let _ = style.set_property(
                "--radix-hover-card-content-transform-origin",
                "var(--radix-popper-transform-origin)",
            );
            let _ = style.set_property(
                "--radix-hover-card-content-available-width",
                "var(--radix-popper-available-width)",
            );
            let _ = style.set_property(
                "--radix-hover-card-content-available-height",
                "var(--radix-popper-available-height)",
            );
            let _ = style.set_property(
                "--radix-hover-card-trigger-width",
                "var(--radix-popper-anchor-width)",
            );
            let _ = style.set_property(
                "--radix-hover-card-trigger-height",
                "var(--radix-popper-anchor-height)",
            );
            if contain_selection.get() {
                let _ = style.set_property("user-select", "text");
                let _ = style.set_property("-webkit-user-select", "text");
            } else {
                let _ = style.remove_property("user-select");
                let _ = style.remove_property("-webkit-user-select");
            }
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
                attr:data-state=move || get_state(context.open.get())
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

/* -------------------------------------------------------------------------------------------------
 * Utils
 * -----------------------------------------------------------------------------------------------*/

fn get_state(open: bool) -> &'static str {
    match open {
        true => "open",
        false => "closed",
    }
}

/// Returns a list of nodes that can be in the tab sequence.
/// See: https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker
fn get_tabbable_nodes(container: &web_sys::HtmlElement) -> Vec<web_sys::HtmlElement> {
    let mut nodes = Vec::new();

    let accept_node_closure: Closure<dyn Fn(web_sys::Node) -> u32> =
        Closure::new(move |node: web_sys::Node| -> u32 {
            if let Some(html_element) = node.dyn_ref::<web_sys::HtmlElement>()
                && html_element.tab_index() >= 0
            {
                // NodeFilter.FILTER_ACCEPT
                return 1;
            }
            // NodeFilter.FILTER_SKIP
            3
        });

    let node_filter = web_sys::NodeFilter::new();
    node_filter.set_accept_node(accept_node_closure.as_ref().unchecked_ref());

    let walker = document()
        // 0x01 is NodeFilter.SHOW_ELEMENT
        .create_tree_walker_with_what_to_show_and_filter(container, 0x1, Some(&node_filter))
        .expect("Tree walker should be created.");

    while let Some(node) = walker
        .next_node()
        .expect("Tree walker should return a next node.")
    {
        if let Ok(element) = node.dyn_into::<web_sys::HtmlElement>() {
            nodes.push(element);
        }
    }

    nodes
}

fn document() -> web_sys::Document {
    web_sys::window()
        .expect("Window should exist.")
        .document()
        .expect("Document should exist.")
}

fn set_timeout(f: impl Fn() + 'static, delay: i32) -> i32 {
    let closure = Closure::once_into_js(f);
    web_sys::window()
        .expect("Window should exist.")
        .set_timeout_with_callback_and_timeout_and_arguments_0(closure.unchecked_ref(), delay)
        .expect("setTimeout should succeed.")
}

fn clear_timeout(handle: StoredValue<Option<i32>>) {
    if let Some(id) = handle.get_value() {
        web_sys::window()
            .expect("Window should exist.")
            .clear_timeout_with_handle(id);
        handle.set_value(None);
    }
}
