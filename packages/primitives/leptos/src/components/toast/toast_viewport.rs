use super::*;

/* -------------------------------------------------------------------------------------------------
 * ToastViewport
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ToastViewport(
    /// The keys to use as the keyboard shortcut that will move focus to the toast viewport.
    #[prop(into, optional)]
    hotkey: Option<Vec<String>>,
    /// An author-localized label for the toast viewport to provide context for screen reader users
    /// when navigating page landmarks. The available `{hotkey}` placeholder will be replaced for you.
    #[prop(into, optional)]
    label: Option<String>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<ToastProviderContextValue>();
    let get_items = StoredValue::new(use_collection::<()>());

    let hotkey: Vec<String> = hotkey.unwrap_or_else(|| {
        VIEWPORT_DEFAULT_HOTKEY
            .iter()
            .map(|s| s.to_string())
            .collect()
    });
    let label = label.unwrap_or_else(|| "Notifications ({hotkey})".to_string());

    let hotkey_label = hotkey
        .iter()
        .map(|k| k.replace("Key", "").replace("Digit", ""))
        .collect::<Vec<_>>()
        .join("+");
    let aria_label = label.replace("{hotkey}", &hotkey_label);
    let aria_label = StoredValue::new(aria_label);

    let wrapper_ref = AnyNodeRef::new();
    let head_focus_proxy_ref = AnyNodeRef::new();
    let tail_focus_proxy_ref = AnyNodeRef::new();
    let viewport_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, viewport_ref]);

    let has_toasts = Signal::derive(move || context.toast_count.get() > 0);

    // F8 hotkey to focus the viewport
    let hotkey_stored = StoredValue::new(hotkey);
    Effect::new(move |_| {
        let hotkey = hotkey_stored.get_value();
        if hotkey.is_empty() {
            return;
        }
        let callback: Closure<dyn Fn(web_sys::KeyboardEvent)> =
            Closure::new(move |event: web_sys::KeyboardEvent| {
                let hotkey = hotkey_stored.get_value();
                let is_hotkey_pressed = !hotkey.is_empty()
                    && hotkey.iter().all(|key| {
                        event.code() == *key
                            || (key == "altKey" && event.alt_key())
                            || (key == "ctrlKey" && event.ctrl_key())
                            || (key == "metaKey" && event.meta_key())
                            || (key == "shiftKey" && event.shift_key())
                    });
                if is_hotkey_pressed && let Some(el) = viewport_ref.get_untracked() {
                    let el: &web_sys::HtmlElement = (*el).unchecked_ref();
                    let _ = el.focus();
                }
            });

        let _ = document()
            .add_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref());

        let callback_ref: SendWrapper<Closure<dyn Fn(web_sys::KeyboardEvent)>> =
            SendWrapper::new(callback);
        on_cleanup(move || {
            let _ = document().remove_event_listener_with_callback(
                "keydown",
                callback_ref.as_ref().unchecked_ref(),
            );
        });
    });

    // Register viewport element with provider
    Effect::new(move |_| {
        if let Some(el) = viewport_ref.get() {
            let el: web_sys::HtmlElement = (*el).clone().unchecked_into();
            context.on_viewport_change.set(Some(SendWrapper::new(el)));
        }
    });

    // Pause/resume on hover/focus
    Effect::new(move |_| {
        let has_toasts = has_toasts.get();
        let wrapper_el = wrapper_ref.get();
        let viewport_el = viewport_ref.get();

        if !has_toasts {
            return;
        }
        let Some(wrapper) = wrapper_el else { return };
        let Some(viewport) = viewport_el else { return };

        let wrapper: web_sys::HtmlElement = (*wrapper).clone().unchecked_into();
        let viewport: web_sys::HtmlElement = (*viewport).clone().unchecked_into();

        let viewport_for_pause = SendWrapper::new(viewport.clone());
        let handle_pause: Closure<dyn Fn()> = Closure::new(move || {
            if !context.is_close_paused_ref.get_value() {
                let pause_event = web_sys::CustomEvent::new(VIEWPORT_PAUSE)
                    .expect("CustomEvent should be created");
                let _ = viewport_for_pause.dispatch_event(&pause_event);
                context.is_close_paused_ref.set_value(true);
            }
        });

        let viewport_for_resume = SendWrapper::new(viewport.clone());
        let _handle_resume: Closure<dyn Fn()> = Closure::new(move || {
            if context.is_close_paused_ref.get_value() {
                let resume_event = web_sys::CustomEvent::new(VIEWPORT_RESUME)
                    .expect("CustomEvent should be created");
                let _ = viewport_for_resume.dispatch_event(&resume_event);
                context.is_close_paused_ref.set_value(false);
            }
        });

        let wrapper_for_focusout = SendWrapper::new(wrapper.clone());
        let handle_focus_out_resume: Closure<dyn Fn(web_sys::FocusEvent)> =
            Closure::new(move |event: web_sys::FocusEvent| {
                let is_focus_moving_outside = event
                    .related_target()
                    .and_then(|t| t.dyn_into::<web_sys::Node>().ok())
                    .map(|node| !wrapper_for_focusout.contains(Some(&node)))
                    .unwrap_or(true);
                if is_focus_moving_outside
                    && context.is_close_paused_ref.get_value()
                    && let Some(viewport) = context.viewport.get_untracked()
                {
                    let resume_event = web_sys::CustomEvent::new(VIEWPORT_RESUME)
                        .expect("CustomEvent should be created");
                    let _ = viewport.dispatch_event(&resume_event);
                    context.is_close_paused_ref.set_value(false);
                }
            });

        let wrapper_for_pointerleave = SendWrapper::new(wrapper.clone());
        let handle_pointer_leave_resume: Closure<dyn Fn()> = Closure::new(move || {
            let is_focus_inside = document()
                .active_element()
                .map(|el| {
                    let node: &web_sys::Node = el.unchecked_ref();
                    wrapper_for_pointerleave.contains(Some(node))
                })
                .unwrap_or(false);
            if !is_focus_inside
                && context.is_close_paused_ref.get_value()
                && let Some(viewport) = context.viewport.get_untracked()
            {
                let resume_event = web_sys::CustomEvent::new(VIEWPORT_RESUME)
                    .expect("CustomEvent should be created");
                let _ = viewport.dispatch_event(&resume_event);
                context.is_close_paused_ref.set_value(false);
            }
        });

        let handle_window_blur: Closure<dyn Fn()> = Closure::new(move || {
            if !context.is_close_paused_ref.get_value()
                && let Some(viewport) = context.viewport.get_untracked()
            {
                let pause_event = web_sys::CustomEvent::new(VIEWPORT_PAUSE)
                    .expect("CustomEvent should be created");
                let _ = viewport.dispatch_event(&pause_event);
                context.is_close_paused_ref.set_value(true);
            }
        });

        let handle_window_focus: Closure<dyn Fn()> = Closure::new(move || {
            if context.is_close_paused_ref.get_value()
                && let Some(viewport) = context.viewport.get_untracked()
            {
                let resume_event = web_sys::CustomEvent::new(VIEWPORT_RESUME)
                    .expect("CustomEvent should be created");
                let _ = viewport.dispatch_event(&resume_event);
                context.is_close_paused_ref.set_value(false);
            }
        });

        let _ = wrapper
            .add_event_listener_with_callback("focusin", handle_pause.as_ref().unchecked_ref());
        let _ = wrapper.add_event_listener_with_callback(
            "focusout",
            handle_focus_out_resume.as_ref().unchecked_ref(),
        );
        let _ = wrapper
            .add_event_listener_with_callback("pointermove", handle_pause.as_ref().unchecked_ref());
        let _ = wrapper.add_event_listener_with_callback(
            "pointerleave",
            handle_pointer_leave_resume.as_ref().unchecked_ref(),
        );
        let window = web_sys::window().expect("Window should exist.");
        let _ = window
            .add_event_listener_with_callback("blur", handle_window_blur.as_ref().unchecked_ref());
        let _ = window.add_event_listener_with_callback(
            "focus",
            handle_window_focus.as_ref().unchecked_ref(),
        );

        let wrapper_cleanup = SendWrapper::new(wrapper);
        let handle_pause = SendWrapper::new(handle_pause);
        let handle_focus_out_resume = SendWrapper::new(handle_focus_out_resume);
        let handle_pointer_leave_resume = SendWrapper::new(handle_pointer_leave_resume);
        let handle_window_blur = SendWrapper::new(handle_window_blur);
        let handle_window_focus = SendWrapper::new(handle_window_focus);

        on_cleanup(move || {
            let _ = wrapper_cleanup.remove_event_listener_with_callback(
                "focusin",
                handle_pause.as_ref().unchecked_ref(),
            );
            let _ = wrapper_cleanup.remove_event_listener_with_callback(
                "focusout",
                handle_focus_out_resume.as_ref().unchecked_ref(),
            );
            let _ = wrapper_cleanup.remove_event_listener_with_callback(
                "pointermove",
                handle_pause.as_ref().unchecked_ref(),
            );
            let _ = wrapper_cleanup.remove_event_listener_with_callback(
                "pointerleave",
                handle_pointer_leave_resume.as_ref().unchecked_ref(),
            );
            if let Some(window) = web_sys::window() {
                let _ = window.remove_event_listener_with_callback(
                    "blur",
                    handle_window_blur.as_ref().unchecked_ref(),
                );
                let _ = window.remove_event_listener_with_callback(
                    "focus",
                    handle_window_focus.as_ref().unchecked_ref(),
                );
            }
        });
    });

    // Custom tab management (reverse order, most-recent-first)

    Effect::new(move |_| {
        let Some(viewport) = viewport_ref.get() else {
            return;
        };
        let viewport: web_sys::HtmlElement = (*viewport).clone().unchecked_into();

        let callback: Closure<dyn Fn(web_sys::KeyboardEvent)> =
            Closure::new(move |event: web_sys::KeyboardEvent| {
                let is_meta_key = event.alt_key() || event.ctrl_key() || event.meta_key();
                let is_tab_key = event.key() == "Tab" && !is_meta_key;

                if !is_tab_key {
                    return;
                }

                let focused_element = document().active_element();
                let is_tabbing_backwards = event.shift_key();

                let target = event.target();
                let target_is_viewport = target
                    .as_ref()
                    .and_then(|t| t.dyn_ref::<web_sys::HtmlElement>())
                    .map(|t| {
                        viewport_ref
                            .get_untracked()
                            .map(|v| {
                                let v: &web_sys::HtmlElement = (*v).unchecked_ref();
                                t == v
                            })
                            .unwrap_or(false)
                    })
                    .unwrap_or(false);

                if target_is_viewport && is_tabbing_backwards {
                    if let Some(el) = head_focus_proxy_ref.get_untracked() {
                        let el: &web_sys::HtmlElement = (*el).unchecked_ref();
                        let _ = el.focus();
                    }
                    return;
                }

                let tabbing_direction = if is_tabbing_backwards {
                    "backwards"
                } else {
                    "forwards"
                };
                let sorted_candidates = compute_sorted_tabbable(get_items, tabbing_direction);
                let index = focused_element.and_then(|focused| {
                    sorted_candidates.iter().position(|candidate| {
                        let c: &web_sys::HtmlElement = candidate;
                        let f: &web_sys::Element = &focused;
                        c.unchecked_ref::<web_sys::Element>() == f
                    })
                });

                // Match React behavior: when focused element is not found
                // (JS findIndex returns -1, slice(-1+1) = slice(0) = full array),
                // try the full candidate list instead of skipping.
                let start = index.map(|i| i + 1).unwrap_or(0);
                let remaining = &sorted_candidates[start..];
                if focus_first_html(remaining) {
                    event.prevent_default();
                } else if is_tabbing_backwards {
                    if let Some(el) = head_focus_proxy_ref.get_untracked() {
                        let el: &web_sys::HtmlElement = (*el).unchecked_ref();
                        let _ = el.focus();
                    }
                } else if let Some(el) = tail_focus_proxy_ref.get_untracked() {
                    let el: &web_sys::HtmlElement = (*el).unchecked_ref();
                    let _ = el.focus();
                }
            });

        let _ =
            viewport.add_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref());

        let viewport_cleanup = SendWrapper::new(viewport);
        let callback = SendWrapper::new(callback);
        on_cleanup(move || {
            let _ = viewport_cleanup
                .remove_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref());
        });
    });

    view! {
        <DismissableLayerBranch
            node_ref=wrapper_ref
            attr:role="region"
            attr:aria-label=aria_label.get_value()
            attr:tabindex="-1"
            style:pointer-events=move || {
                if has_toasts.get() { None } else { Some("none") }
            }
        >
            <Show when=move || has_toasts.get()>
                <FocusProxy
                    node_ref=head_focus_proxy_ref
                    on_focus_from_outside_viewport=Callback::new(move |_: ()| {
                        let candidates = compute_sorted_tabbable(get_items, "forwards");
                        focus_first_html(&candidates);
                    })
                />
            </Show>

            <CollectionSlot item_data_type=ITEM_DATA_PHANTOM node_ref=composed_refs>
                <Primitive
                    element=html::ol
                    as_child=as_child
                    node_ref=composed_refs
                    attr:tabindex="-1"
                >
                    {children.with_value(|children| children.as_ref().map(|c| c()))}
                </Primitive>
            </CollectionSlot>

            <Show when=move || has_toasts.get()>
                <FocusProxy
                    node_ref=tail_focus_proxy_ref
                    on_focus_from_outside_viewport=Callback::new(move |_: ()| {
                        let candidates = compute_sorted_tabbable(get_items, "backwards");
                        focus_first_html(&candidates);
                    })
                />
            </Show>
        </DismissableLayerBranch>
    }
}

/* -------------------------------------------------------------------------------------------------
 * FocusProxy
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn FocusProxy(
    #[prop(into, optional)] node_ref: AnyNodeRef,
    on_focus_from_outside_viewport: Callback<()>,
) -> impl IntoView {
    let context = expect_context::<ToastProviderContextValue>();

    view! {
        <VisuallyHidden
            node_ref=node_ref
            attr:tabindex="0"
            style:position="fixed"
            on:focus=move |event: web_sys::FocusEvent| {
                let prev_focused_element = event
                    .related_target()
                    .and_then(|t| t.dyn_into::<web_sys::Node>().ok());
                let is_focus_from_outside_viewport = context
                    .viewport
                    .get_untracked()
                    .map(|viewport| {
                        let viewport_el: &web_sys::HtmlElement = &viewport;
                        !viewport_el.contains(prev_focused_element.as_ref())
                    })
                    .unwrap_or(true);
                if is_focus_from_outside_viewport {
                    on_focus_from_outside_viewport.run(());
                }
            }
        >
            {""}
        </VisuallyHidden>
    }
}
