use super::*;

/* -------------------------------------------------------------------------------------------------
 * Toast (Root) -- merged with ToastImpl
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy)]
struct ToastInteractiveContextValue {
    on_close: Callback<()>,
}

#[component]
pub fn Toast(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    /// Used to force mounting when more control is needed. Useful when
    /// controlling animation with animation libraries.
    #[prop(into, optional)]
    force_mount: Option<bool>,
    /// The type of toast. `Foreground` toasts are announced as `assertive`,
    /// `Background` toasts as `polite`.
    #[prop(into, optional)]
    r#type: MaybeProp<ToastType>,
    /// Time in milliseconds that toast should remain visible for. Overrides value
    /// given to `ToastProvider`.
    #[prop(into, optional)]
    duration: MaybeProp<i32>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    #[prop(into, optional)] on_pause: Option<Callback<()>>,
    #[prop(into, optional)] on_resume: Option<Callback<()>>,
    #[prop(into, optional)] on_swipe_start: Option<Callback<SwipeEvent>>,
    #[prop(into, optional)] on_swipe_move: Option<Callback<SwipeEvent>>,
    #[prop(into, optional)] on_swipe_cancel: Option<Callback<SwipeEvent>>,
    #[prop(into, optional)] on_swipe_end: Option<Callback<SwipeEvent>>,
    /// Explicit class forwarding -- `attr:class` on `<Toast>` cannot cross the Portal
    /// boundary (Portal uses `mount_to` which creates a separate rendering context).
    /// Use `attr:class` as usual; this prop is for internal forwarding to the `<li>`.
    #[prop(into, optional)]
    class: MaybeProp<String>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let (open_signal, set_open) = use_controllable_state(UseControllableStateParams {
        prop: open,
        default_prop: match default_open.get_untracked() {
            Some(_) => default_open,
            None => MaybeProp::from(Some(true)),
        },
        on_change: adapt_callback(on_open_change),
    });
    let is_open = Signal::derive(move || open_signal.get().unwrap_or(true));

    let force_mount = force_mount.unwrap_or(false);
    let toast_type = Signal::derive(move || r#type.get().unwrap_or_default());

    let context = expect_context::<ToastProviderContextValue>();
    let toast_node_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, toast_node_ref]);
    let pointer_start_ref: StoredValue<Option<(f64, f64)>> = StoredValue::new(None);
    let swipe_delta_ref: StoredValue<Option<(f64, f64)>> = StoredValue::new(None);
    let duration_val =
        Signal::derive(move || duration.get().unwrap_or_else(|| context.duration.get()));
    let close_timer_start_time_ref: StoredValue<f64> = StoredValue::new(0.0);
    let close_timer_remaining_time_ref: StoredValue<f64> =
        StoredValue::new(duration_val.get_untracked() as f64);
    let close_timer_ref: StoredValue<Option<i32>> = StoredValue::new(None);

    let handle_close = Callback::new(move |_: ()| {
        let is_focus_in_toast = toast_node_ref
            .get_untracked()
            .and_then(|node| {
                let node: &web_sys::HtmlElement = (*node).unchecked_ref();
                document()
                    .active_element()
                    .map(|active| node.contains(Some(active.unchecked_ref())))
            })
            .unwrap_or(false);

        if is_focus_in_toast && let Some(viewport) = context.viewport.get_untracked() {
            let _ = viewport.focus();
        }
        set_open.run(Some(false));
    });

    let start_timer = Callback::new(move |dur: f64| {
        if dur <= 0.0 || dur == f64::INFINITY {
            return;
        }
        clear_timeout(close_timer_ref);
        close_timer_start_time_ref.set_value(js_sys::Date::now());
        let timeout_id = set_timeout(
            move || {
                handle_close.run(());
            },
            dur as i32,
        );
        close_timer_ref.set_value(Some(timeout_id));
    });

    // Listen for viewport pause/resume events
    Effect::new(move |_| {
        let Some(viewport) = context.viewport.get() else {
            return;
        };
        let viewport_el: web_sys::HtmlElement = (*viewport).clone();

        let handle_resume_fn: Closure<dyn Fn()> = Closure::new(move || {
            start_timer.run(close_timer_remaining_time_ref.get_value());
            if let Some(cb) = on_resume {
                cb.run(());
            }
        });

        let handle_pause_fn: Closure<dyn Fn()> = Closure::new(move || {
            let elapsed_time = js_sys::Date::now() - close_timer_start_time_ref.get_value();
            close_timer_remaining_time_ref
                .set_value(close_timer_remaining_time_ref.get_value() - elapsed_time);
            clear_timeout(close_timer_ref);
            if let Some(cb) = on_pause {
                cb.run(());
            }
        });

        let _ = viewport_el.add_event_listener_with_callback(
            VIEWPORT_PAUSE,
            handle_pause_fn.as_ref().unchecked_ref(),
        );
        let _ = viewport_el.add_event_listener_with_callback(
            VIEWPORT_RESUME,
            handle_resume_fn.as_ref().unchecked_ref(),
        );

        let viewport_cleanup = SendWrapper::new(viewport_el);
        let handle_pause_fn = SendWrapper::new(handle_pause_fn);
        let handle_resume_fn = SendWrapper::new(handle_resume_fn);
        on_cleanup(move || {
            let _ = viewport_cleanup.remove_event_listener_with_callback(
                VIEWPORT_PAUSE,
                handle_pause_fn.as_ref().unchecked_ref(),
            );
            let _ = viewport_cleanup.remove_event_listener_with_callback(
                VIEWPORT_RESUME,
                handle_resume_fn.as_ref().unchecked_ref(),
            );
        });
    });

    // Start timer when toast opens or duration changes
    Effect::new(move |_| {
        let is_open_val = is_open.get();
        let dur = duration_val.get() as f64;
        if is_open_val && !context.is_close_paused_ref.get_value() {
            close_timer_remaining_time_ref.set_value(dur);
            start_timer.run(dur);
        }
    });

    // Track toast count
    Effect::new(move |_| {
        context.set_toast_count.update(|c| *c += 1);
        on_cleanup(move || {
            context.set_toast_count.update(|c| *c -= 1);
        });
    });

    on_cleanup(move || {
        clear_timeout(close_timer_ref);
    });

    // Announce text content
    let announce_text = RwSignal::new(Vec::<String>::new());
    Effect::new(move |_| {
        // Track open state to re-read text when content changes
        let _ = is_open.get();
        if let Some(node) = toast_node_ref.get() {
            let node: &web_sys::HtmlElement = (*node).unchecked_ref();
            announce_text.set(get_announce_text_content(node));
        }
    });

    let swipe_direction = context.swipe_direction;
    let swipe_threshold = context.swipe_threshold;

    let has_viewport = Signal::derive(move || context.viewport.get().is_some());

    // Compose swipe handlers with data-attribute / CSS variable management
    let on_swipe_start_composed = on_swipe_start;
    let on_swipe_move_composed = on_swipe_move;
    let on_swipe_cancel_composed = on_swipe_cancel;
    let on_swipe_end_composed = on_swipe_end;
    let on_escape_key_down_stored = on_escape_key_down;

    view! {
        <Presence present=Signal::derive(move || force_mount || is_open.get())>
            <Show when=move || has_viewport.get()>
                <Provider value=ToastInteractiveContextValue { on_close: handle_close }>
                    <Portal
                        as_child=true
                        container=Signal::derive(move || {
                            context.viewport.get().map(|v| {
                                let el: web_sys::Element = (*v).clone().unchecked_into();
                                SendWrapper::new(el)
                            })
                        })
                    >
                        <Provider value=context>
                            <Provider value=ToastInteractiveContextValue { on_close: handle_close }>
                                <CollectionItemSlot item_data_type=ITEM_DATA_PHANTOM item_data=()>
                                    <Primitive
                                        element=html::li
                                        as_child=as_child
                                        node_ref=composed_refs
                                        attr:class=move || class.get().unwrap_or_default()
                                        attr:role="status"
                                        attr:aria-live=move || {
                                            match toast_type.get() {
                                                ToastType::Foreground => "assertive",
                                                ToastType::Background => "polite",
                                            }
                                        }
                                        attr:aria-atomic="true"
                                        attr:tabindex="0"
                                        attr:data-state=move || if is_open.get() { "open" } else { "closed" }
                                        attr:data-swipe-direction=move || swipe_direction.get().as_str()
                                        style:user-select="none"
                                        style:touch-action="none"
                                        on:keydown=move |event: web_sys::KeyboardEvent| {
                                            if event.key() != "Escape" {
                                                return;
                                            }
                                            if let Some(cb) = on_escape_key_down_stored {
                                                cb.run(event.clone());
                                            }
                                            if !event.default_prevented() {
                                                context.is_focused_toast_escape_key_down_ref.set_value(true);
                                                handle_close.run(());
                                            }
                                        }
                                        on:pointerdown=move |event: web_sys::PointerEvent| {
                                            if event.button() != 0 {
                                                return;
                                            }
                                            pointer_start_ref.set_value(Some((event.client_x() as f64, event.client_y() as f64)));
                                        }
                                        on:pointermove=move |event: web_sys::PointerEvent| {
                                            let Some(start) = pointer_start_ref.get_value() else {
                                                return;
                                            };
                                            let x = event.client_x() as f64 - start.0;
                                            let y = event.client_y() as f64 - start.1;
                                            let has_swipe_move_started = swipe_delta_ref.get_value().is_some();
                                            let direction = swipe_direction.get_untracked();
                                            let is_horizontal_swipe = matches!(direction, SwipeDirection::Left | SwipeDirection::Right);
                                            let clamp_fn: fn(f64, f64) -> f64 = match direction {
                                                SwipeDirection::Left | SwipeDirection::Up => f64::min,
                                                SwipeDirection::Right | SwipeDirection::Down => f64::max,
                                            };
                                            let clamped_x = if is_horizontal_swipe { clamp_fn(0.0, x) } else { 0.0 };
                                            let clamped_y = if !is_horizontal_swipe { clamp_fn(0.0, y) } else { 0.0 };
                                            let move_start_buffer = if event.pointer_type() == "touch" { 10.0 } else { 2.0 };
                                            let delta = (clamped_x, clamped_y);
                                            let current_target = event.current_target()
                                                .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
                                                .map(SwipeEventTarget::new);

                                            if has_swipe_move_started {
                                                swipe_delta_ref.set_value(Some(delta));
                                                if let Some(ref ct) = current_target {
                                                    let _ = ct.set_attribute("data-swipe", "move");
                                                    let style = ct.style();
                                                    let _ = style.set_property("--radix-toast-swipe-move-x", &format!("{}px", delta.0));
                                                    let _ = style.set_property("--radix-toast-swipe-move-y", &format!("{}px", delta.1));
                                                }
                                                if let Some(cb) = on_swipe_move_composed {
                                                    cb.run(SwipeEvent { current_target, delta });
                                                }
                                            } else if is_delta_in_direction(delta, direction, move_start_buffer) {
                                                swipe_delta_ref.set_value(Some(delta));
                                                if let Some(ref ct) = current_target {
                                                    let _ = ct.set_attribute("data-swipe", "start");
                                                }
                                                if let Some(cb) = on_swipe_start_composed {
                                                    cb.run(SwipeEvent { current_target, delta });
                                                }
                                                let _ = event.target()
                                                    .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
                                                    .map(|t| t.set_pointer_capture(event.pointer_id()));
                                            } else if x.abs() > move_start_buffer || y.abs() > move_start_buffer {
                                                pointer_start_ref.set_value(None);
                                            }
                                        }
                                        on:pointerup=move |event: web_sys::PointerEvent| {
                                            let delta = swipe_delta_ref.get_value();
                                            if let Some(target) = event.target()
                                                .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
                                                && target.has_pointer_capture(event.pointer_id()) {
                                                    let _ = target.release_pointer_capture(event.pointer_id());
                                                }
                                            swipe_delta_ref.set_value(None);
                                            pointer_start_ref.set_value(None);
                                            if let Some(delta) = delta {
                                                let direction = swipe_direction.get_untracked();
                                                let threshold = swipe_threshold.get_untracked();
                                                let current_target = event.current_target()
                                                    .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
                                                    .map(SwipeEventTarget::new);
                                                if is_delta_in_direction(delta, direction, threshold) {
                                                    if let Some(ref ct) = current_target {
                                                        let _ = ct.set_attribute("data-swipe", "end");
                                                        let style = ct.style();
                                                        let _ = style.remove_property("--radix-toast-swipe-move-x");
                                                        let _ = style.remove_property("--radix-toast-swipe-move-y");
                                                        let _ = style.set_property("--radix-toast-swipe-end-x", &format!("{}px", delta.0));
                                                        let _ = style.set_property("--radix-toast-swipe-end-y", &format!("{}px", delta.1));
                                                    }
                                                    if let Some(cb) = on_swipe_end_composed {
                                                        cb.run(SwipeEvent { current_target, delta });
                                                    }
                                                    set_open.run(Some(false));
                                                } else {
                                                    if let Some(ref ct) = current_target {
                                                        let _ = ct.set_attribute("data-swipe", "cancel");
                                                        let style = ct.style();
                                                        let _ = style.remove_property("--radix-toast-swipe-move-x");
                                                        let _ = style.remove_property("--radix-toast-swipe-move-y");
                                                        let _ = style.remove_property("--radix-toast-swipe-end-x");
                                                        let _ = style.remove_property("--radix-toast-swipe-end-y");
                                                    }
                                                    if let Some(cb) = on_swipe_cancel_composed {
                                                        cb.run(SwipeEvent { current_target, delta });
                                                    }
                                                }
                                                // Prevent click event from triggering on items within the toast
                                                if let Some(toast) = event.current_target()
                                                    .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
                                                {
                                                    let prevent_click: Closure<dyn FnMut(web_sys::Event)> =
                                                        Closure::once(move |event: web_sys::Event| {
                                                            event.prevent_default();
                                                        });
                                                    let mut options = web_sys::AddEventListenerOptions::new();
                                                    options.set_once(true);
                                                    let _ = toast.add_event_listener_with_callback_and_add_event_listener_options(
                                                        "click",
                                                        prevent_click.as_ref().unchecked_ref(),
                                                        &options,
                                                    );
                                                    prevent_click.forget();
                                                }
                                            }
                                        }
                                    >
                                        {children.with_value(|children| children())}
                                    </Primitive>
                                </CollectionItemSlot>
                            </Provider>
                        </Provider>
                    </Portal>
                </Provider>
            </Show>
        </Presence>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ToastTitle
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ToastTitle(
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <Primitive element=html::div as_child=as_child node_ref=node_ref>
            {children.with_value(|children| children())}
        </Primitive>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ToastDescription
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ToastDescription(
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <Primitive element=html::div as_child=as_child node_ref=node_ref>
            {children.with_value(|children| children())}
        </Primitive>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ToastAction
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ToastAction(
    /// A short description for an alternate way to carry out the action. For screen reader users
    /// who will not be able to navigate to the button easily/quickly.
    #[prop(into)]
    alt_text: String,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let alt_text = StoredValue::new(alt_text);

    view! {
        <ToastAnnounceExclude alt_text=alt_text.get_value()>
            <ToastClose node_ref=node_ref as_child=as_child>
                {children.with_value(|children| children())}
            </ToastClose>
        </ToastAnnounceExclude>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ToastClose
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ToastClose(
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let interactive_context = expect_context::<ToastInteractiveContextValue>();

    view! {
        <ToastAnnounceExclude>
            <Primitive
                element=html::button
                as_child=as_child
                node_ref=node_ref
                attr:r#type="button"
                on:click=move |_: web_sys::MouseEvent| {
                    interactive_context.on_close.run(());
                }
            >
                {children.with_value(|children| children())}
            </Primitive>
        </ToastAnnounceExclude>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ToastAnnounceExclude
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn ToastAnnounceExclude(
    #[prop(into, optional)] alt_text: Option<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let alt_text = StoredValue::new(alt_text);

    // React always uses asChild here -- the data attributes are merged onto the child
    // element rather than rendering a wrapper <div>.
    view! {
        <Primitive
            element=html::div
            as_child=true
            attr:data-radix-toast-announce-exclude=""
            attr:data-radix-toast-announce-alt=move || alt_text.get_value()
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}
