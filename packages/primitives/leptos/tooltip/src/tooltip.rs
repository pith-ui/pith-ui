use std::cell::RefCell;
use std::rc::Rc;

use leptos::{
    attribute_interceptor::AttributeInterceptor, context::Provider, ev, html, prelude::*,
};
use leptos_node_ref::AnyNodeRef;
use radix_leptos_compose_refs::use_composed_refs;
use radix_leptos_dismissable_layer::DismissableLayer;
use radix_leptos_id::use_id;
use radix_leptos_popper::{
    Align, Padding, Popper, PopperAnchor, PopperArrow, PopperContent, Side, Sticky,
    UpdatePositionStrategy, provide_popper_scope, use_popper_scope,
};
use radix_leptos_portal::Portal;
use radix_leptos_presence::Presence;
use radix_leptos_primitive::{Primitive, compose_callbacks};
use radix_leptos_use_controllable_state::{UseControllableStateParams, use_controllable_state};
use radix_leptos_visually_hidden::VisuallyHidden;
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::closure::Closure;

/// Shared closure storage that outlives StoredValue disposal.
///
/// During Leptos scope disposal, `StoredValue` contents may be dropped before
/// `on_cleanup` callbacks run. If a `Closure` is dropped while a DOM listener
/// still references it, wasm_bindgen throws "closure invoked after being dropped".
///
/// By storing closures in `Rc<RefCell<...>>` (wrapped in SendWrapper for Send+Sync),
/// both the setup code and the cleanup callback hold Rc clones. Even if one clone is
/// dropped during disposal, the other keeps the Closure alive until cleanup removes
/// the listener.
type ClosureCell<T> = SendWrapper<Rc<RefCell<Option<Closure<T>>>>>;

fn closure_cell<T: ?Sized>() -> ClosureCell<T> {
    SendWrapper::new(Rc::new(RefCell::new(None)))
}

const DEFAULT_DELAY_DURATION: f64 = 700.0;
const TOOLTIP_OPEN: &str = "tooltip.open";

/* -------------------------------------------------------------------------------------------------
 * TooltipProvider
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy)]
struct TooltipProviderContextValue {
    is_open_delayed: RwSignal<bool>,
    delay_duration: Signal<f64>,
    on_open: Callback<()>,
    on_close: Callback<()>,
    on_pointer_in_transit_change: Callback<bool>,
    is_pointer_in_transit: RwSignal<bool>,
    disable_hoverable_content: Signal<bool>,
}

#[component]
pub fn TooltipProvider(
    #[prop(into, optional, default = MaybeProp::from(DEFAULT_DELAY_DURATION))]
    delay_duration: MaybeProp<f64>,
    #[prop(into, optional, default = MaybeProp::from(300.0))] skip_delay_duration: MaybeProp<f64>,
    #[prop(into, optional)] disable_hoverable_content: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let is_open_delayed = RwSignal::new(true);
    let is_pointer_in_transit = RwSignal::new(false);
    let skip_delay_timer_ref: StoredValue<Option<i32>> = StoredValue::new(None);

    let delay_duration_signal =
        Signal::derive(move || delay_duration.get().unwrap_or(DEFAULT_DELAY_DURATION));
    let disable_hoverable_content_signal =
        Signal::derive(move || disable_hoverable_content.get().unwrap_or(false));

    let on_open = Callback::new(move |_: ()| {
        clear_timeout(skip_delay_timer_ref);
        is_open_delayed.set(false);
    });

    let on_close = Callback::new(move |_: ()| {
        clear_timeout(skip_delay_timer_ref);
        let skip_delay = skip_delay_duration.get().unwrap_or(300.0);
        let timeout_id = set_timeout(
            move || {
                is_open_delayed.set(true);
            },
            skip_delay as i32,
        );
        skip_delay_timer_ref.set_value(Some(timeout_id));
    });

    let on_pointer_in_transit_change = Callback::new(move |in_transit: bool| {
        is_pointer_in_transit.set(in_transit);
    });

    on_cleanup(move || {
        clear_timeout(skip_delay_timer_ref);
    });

    let context = TooltipProviderContextValue {
        is_open_delayed,
        delay_duration: delay_duration_signal,
        on_open,
        on_close,
        on_pointer_in_transit_change,
        is_pointer_in_transit,
        disable_hoverable_content: disable_hoverable_content_signal,
    };

    view! {
        <Provider value=context>
            {children.with_value(|children| children())}
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * Tooltip
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy)]
struct TooltipContextValue {
    content_id: Signal<String>,
    open: Signal<bool>,
    state_attribute: Signal<&'static str>,
    trigger: RwSignal<Option<SendWrapper<web_sys::HtmlElement>>>,
    on_trigger_enter: Callback<()>,
    on_trigger_leave: Callback<()>,
    on_open: Callback<()>,
    on_close: Callback<()>,
    disable_hoverable_content: Signal<bool>,
}

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
        if open_derived.get() {
            if was_open_delayed_ref.get_value() {
                "delayed-open"
            } else {
                "instant-open"
            }
        } else {
            "closed"
        }
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

/* -------------------------------------------------------------------------------------------------
 * TooltipPortal
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone)]
struct TooltipPortalContextValue {
    force_mount: Signal<bool>,
}

#[component]
pub fn TooltipPortal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    #[prop(optional)] container_ref: AnyNodeRef,
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let force_mount_signal = Signal::derive(move || force_mount.get().unwrap_or(false));

    let portal_context = TooltipPortalContextValue {
        force_mount: force_mount_signal,
    };

    // Capture contexts before the portal boundary for re-provision inside mount_to.
    let tooltip_context = expect_context::<TooltipContextValue>();
    let provider_context = expect_context::<TooltipProviderContextValue>();
    let popper_scope = use_popper_scope();

    // Always render the Portal and let TooltipContent handle its own Presence wrapper.
    view! {
        <Provider value=portal_context>
            <Portal container=container container_ref=container_ref as_child=true>
                <Provider value=tooltip_context>
                    <Provider value=provider_context>
                        {
                            if let Some(scope) = popper_scope {
                                provide_popper_scope(scope);
                            }
                            children.with_value(|children| children())
                        }
                    </Provider>
                </Provider>
            </Portal>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * TooltipContent
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn TooltipContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<web_sys::CustomEvent>>,
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
    let portal_context = use_context::<TooltipPortalContextValue>();

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
                                &current_target.get_bounding_client_rect(),
                            );
                            let padded_exit_points =
                                get_padded_exit_points(&exit_point, &exit_side);
                            let hover_target_points = get_points_from_rect(
                                &content_for_trigger.get_bounding_client_rect(),
                            );
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
                                &current_target.get_bounding_client_rect(),
                            );
                            let padded_exit_points =
                                get_padded_exit_points(&exit_point, &exit_side);
                            let hover_target_points =
                                get_points_from_rect(&trigger_clone.get_bounding_client_rect());
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
                    let target: Option<web_sys::Node> = event.target().map(|t| t.unchecked_into());
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
    }});

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

fn cleanup_grace_area_listeners(
    trigger_el: &Option<SendWrapper<web_sys::HtmlElement>>,
    content_el: &Option<web_sys::HtmlElement>,
    trigger_leave_closure: &ClosureCell<dyn Fn(web_sys::PointerEvent)>,
    content_leave_closure: &ClosureCell<dyn Fn(web_sys::PointerEvent)>,
) {
    if let Some(closure) = trigger_leave_closure.borrow().as_ref() {
        if let Some(trigger) = trigger_el.as_ref() {
            trigger
                .remove_event_listener_with_callback(
                    "pointerleave",
                    closure.as_ref().unchecked_ref(),
                )
                .ok();
        }
    }
    if let Some(closure) = content_leave_closure.borrow().as_ref() {
        if let Some(content) = content_el.as_ref() {
            content
                .remove_event_listener_with_callback(
                    "pointerleave",
                    closure.as_ref().unchecked_ref(),
                )
                .ok();
        }
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

/* -------------------------------------------------------------------------------------------------
 * Geometry Utilities
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn get_exit_side_from_rect(point: &Point, rect: &web_sys::DomRect) -> Side {
    let top = (rect.top() - point.y).abs();
    let bottom = (rect.bottom() - point.y).abs();
    let right = (rect.right() - point.x).abs();
    let left = (rect.left() - point.x).abs();

    let min = top.min(bottom).min(right).min(left);

    if (min - left).abs() < f64::EPSILON {
        Side::Left
    } else if (min - right).abs() < f64::EPSILON {
        Side::Right
    } else if (min - top).abs() < f64::EPSILON {
        Side::Top
    } else {
        Side::Bottom
    }
}

fn get_padded_exit_points(exit_point: &Point, exit_side: &Side) -> Vec<Point> {
    let padding = 5.0;
    match exit_side {
        Side::Top => vec![
            Point {
                x: exit_point.x - padding,
                y: exit_point.y + padding,
            },
            Point {
                x: exit_point.x + padding,
                y: exit_point.y + padding,
            },
        ],
        Side::Bottom => vec![
            Point {
                x: exit_point.x - padding,
                y: exit_point.y - padding,
            },
            Point {
                x: exit_point.x + padding,
                y: exit_point.y - padding,
            },
        ],
        Side::Left => vec![
            Point {
                x: exit_point.x + padding,
                y: exit_point.y - padding,
            },
            Point {
                x: exit_point.x + padding,
                y: exit_point.y + padding,
            },
        ],
        Side::Right => vec![
            Point {
                x: exit_point.x - padding,
                y: exit_point.y - padding,
            },
            Point {
                x: exit_point.x - padding,
                y: exit_point.y + padding,
            },
        ],
    }
}

fn get_points_from_rect(rect: &web_sys::DomRect) -> Vec<Point> {
    vec![
        Point {
            x: rect.left(),
            y: rect.top(),
        },
        Point {
            x: rect.right(),
            y: rect.top(),
        },
        Point {
            x: rect.right(),
            y: rect.bottom(),
        },
        Point {
            x: rect.left(),
            y: rect.bottom(),
        },
    ]
}

/// Determine if a point is inside of a polygon.
/// Based on https://github.com/substack/point-in-polygon
fn is_point_in_polygon(point: &Point, polygon: &[Point]) -> bool {
    let (x, y) = (point.x, point.y);
    let mut inside = false;
    let len = polygon.len();
    let mut j = len - 1;
    for i in 0..len {
        let xi = polygon[i].x;
        let yi = polygon[i].y;
        let xj = polygon[j].x;
        let yj = polygon[j].y;

        let intersect = ((yi > y) != (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi) + xi);
        if intersect {
            inside = !inside;
        }
        j = i;
    }
    inside
}

/// Returns a new array of points representing the convex hull of the given set of points.
/// https://www.nayuki.io/page/convex-hull-algorithm
fn get_hull(points: &[Point]) -> Vec<Point> {
    let mut new_points: Vec<Point> = points.to_vec();
    new_points.sort_by(|a, b| {
        a.x.partial_cmp(&b.x)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.y.partial_cmp(&b.y).unwrap_or(std::cmp::Ordering::Equal))
    });
    get_hull_presorted(&new_points)
}

/// Returns the convex hull, assuming that each points[i] <= points[i + 1]. Runs in O(n) time.
fn get_hull_presorted(points: &[Point]) -> Vec<Point> {
    if points.len() <= 1 {
        return points.to_vec();
    }

    let mut upper_hull: Vec<Point> = Vec::new();
    for p in points {
        while upper_hull.len() >= 2 {
            let q = &upper_hull[upper_hull.len() - 1];
            let r = &upper_hull[upper_hull.len() - 2];
            if (q.x - r.x) * (p.y - r.y) >= (q.y - r.y) * (p.x - r.x) {
                upper_hull.pop();
            } else {
                break;
            }
        }
        upper_hull.push(p.clone());
    }
    upper_hull.pop();

    let mut lower_hull: Vec<Point> = Vec::new();
    for p in points.iter().rev() {
        while lower_hull.len() >= 2 {
            let q = &lower_hull[lower_hull.len() - 1];
            let r = &lower_hull[lower_hull.len() - 2];
            if (q.x - r.x) * (p.y - r.y) >= (q.y - r.y) * (p.x - r.x) {
                lower_hull.pop();
            } else {
                break;
            }
        }
        lower_hull.push(p.clone());
    }
    lower_hull.pop();

    if upper_hull.len() == 1
        && lower_hull.len() == 1
        && (upper_hull[0].x - lower_hull[0].x).abs() < f64::EPSILON
        && (upper_hull[0].y - lower_hull[0].y).abs() < f64::EPSILON
    {
        return upper_hull;
    }

    upper_hull.extend(lower_hull);
    upper_hull
}

/* -------------------------------------------------------------------------------------------------
 * Utils
 * -----------------------------------------------------------------------------------------------*/

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

/// Schedules a closure to run in a microtask (after the current synchronous execution).
/// This is used to defer state updates that would cause synchronous DOM mutations during
/// Leptos's delegated event dispatch, matching React 18+'s automatic batching behavior.
fn queue_microtask(f: impl FnOnce() + 'static) {
    let cb = Closure::once_into_js(f);
    web_sys::window()
        .expect("Window should exist.")
        .queue_microtask(cb.unchecked_ref());
}
