use super::*;

/* -------------------------------------------------------------------------------------------------
 * PasswordToggleFieldSlot
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn PasswordToggleFieldSlot(
    #[prop(into, optional)] render: Option<Callback<bool, AnyView>>,
    #[prop(into, optional)] visible_content: Option<ChildrenFn>,
    #[prop(into, optional)] hidden_content: Option<ChildrenFn>,
) -> impl IntoView {
    let context = expect_context::<PasswordToggleFieldContextValue>();

    move || {
        let visible = context.visible.get();
        if let Some(render) = render {
            render.run(visible)
        } else if visible {
            visible_content
                .as_ref()
                .map(|children| children().into_any())
                .unwrap_or_else(|| ().into_any())
        } else {
            hidden_content
                .as_ref()
                .map(|children| children().into_any())
                .unwrap_or_else(|| ().into_any())
        }
    }
}

/* -------------------------------------------------------------------------------------------------
 * PasswordToggleFieldIcon
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn PasswordToggleFieldIcon(
    #[prop(into)] visible_icon: ViewFn,
    #[prop(into)] hidden_icon: ViewFn,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let context = expect_context::<PasswordToggleFieldContextValue>();
    let visible_icon = StoredValue::new(visible_icon);
    let hidden_icon = StoredValue::new(hidden_icon);

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=leptos::svg::svg
                as_child=true
                node_ref=node_ref
                attr:aria-hidden="true"
                {..attrs}
            >
                {move || {
                    if context.visible.get() {
                        visible_icon.with_value(|icon| icon.run())
                    } else {
                        hidden_icon.with_value(|icon| icon.run())
                    }
                }}
            </Primitive>
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * PasswordToggleFieldToggle
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn PasswordToggleFieldToggle(
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] on_pointer_down: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_cancel: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_up: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context = expect_context::<PasswordToggleFieldContextValue>();
    // Default to "Show password" / "Hide password" so aria-label is present during SSR.
    // The Effect below will refine this on the client (clearing it if children have text).
    let initial_label = if context.visible.get_untracked() {
        "Hide password"
    } else {
        "Show password"
    };
    let (internal_aria_label, set_internal_aria_label) =
        signal::<Option<String>>(Some(initial_label.to_string()));
    let element_ref = AnyNodeRef::new();
    let composed_ref = use_composed_refs(vec![node_ref, element_ref]);

    let focus_state = context.focus_state;

    // Auto aria-label via MutationObserver
    Effect::new(move |_| {
        let visible = context.visible.get();

        let Some(node) = element_ref.get() else {
            // During SSR or before mount, keep the default label.
            return;
        };

        if aria_label.get().is_some() {
            set_internal_aria_label.set(None);
            return;
        }

        let element: &web_sys::HtmlElement = node.unchecked_ref();
        let default_aria_label = if visible {
            "Hide password"
        } else {
            "Show password"
        };

        // Check current text content
        let text_content = element.text_content().unwrap_or_default();
        if !text_content.is_empty() {
            set_internal_aria_label.set(None);
        } else {
            set_internal_aria_label.set(Some(default_aria_label.to_string()));
        }

        // Set up MutationObserver for text changes
        let default_label = default_aria_label.to_string();
        let observer_callback = SendWrapper::new(Closure::<dyn Fn(web_sys::js_sys::Array)>::new(
            move |entries: web_sys::js_sys::Array| {
                let mut text_content: Option<String> = None;
                for i in 0..entries.length() {
                    let entry: web_sys::MutationRecord = entries.get(i).unchecked_into();
                    if entry.type_() == "characterData"
                        && let Some(target) = entry.target()
                    {
                        let el: &web_sys::Node = target.unchecked_ref();
                        if let Some(tc) = el.text_content()
                            && !tc.is_empty()
                        {
                            text_content = Some(tc);
                        }
                    }
                }
                if let Some(text) = text_content {
                    if !text.is_empty() {
                        set_internal_aria_label.set(None);
                    } else {
                        set_internal_aria_label.set(Some(default_label.clone()));
                    }
                }
            },
        ));

        let observer = SendWrapper::new(
            web_sys::MutationObserver::new(observer_callback.as_ref().unchecked_ref())
                .expect("MutationObserver should be created."),
        );

        let init = web_sys::MutationObserverInit::new();
        init.set_character_data(true);
        init.set_subtree(true);
        observer
            .observe_with_options(element, &init)
            .expect("MutationObserver should observe.");

        Owner::on_cleanup(move || {
            observer.disconnect();
            // prevent drop of the closure before the observer is disconnected
            drop(observer_callback);
        });
    });

    let resolved_aria_label =
        Signal::derive(move || aria_label.get().or_else(|| internal_aria_label.get()));

    // CSR-only: always provide aria-controls (no hydration gate needed)
    let aria_controls = Signal::derive(move || context.input_id.get());

    // Global pointerup listener for click_triggered reset
    Effect::new(move |_| {
        let window = web_sys::window().expect("Window should exist.");

        let focus_state = focus_state;
        let reset = SendWrapper::new(Closure::<dyn Fn()>::new(move || {
            focus_state.update_value(|state| state.click_triggered = false);
        }));

        let cleanup_handle: StoredValue<Option<i32>> = StoredValue::new(None);

        let reset_ref = SendWrapper::new(
            reset
                .as_ref()
                .unchecked_ref::<web_sys::js_sys::Function>()
                .clone(),
        );
        let handler = SendWrapper::new(Closure::<dyn Fn()>::new(move || {
            let window = web_sys::window().expect("Window should exist.");
            let handle = window
                .set_timeout_with_callback_and_timeout_and_arguments_0(&reset_ref, 1)
                .expect("setTimeout should succeed.");
            cleanup_handle.set_value(Some(handle));
        }));

        let window_sw = SendWrapper::new(window.clone());
        window_sw
            .add_event_listener_with_callback("pointerup", handler.as_ref().unchecked_ref())
            .expect("pointerup event listener should be added.");

        Owner::on_cleanup(move || {
            if let Some(handle) = cleanup_handle.get_value() {
                window_sw.clear_timeout_with_handle(handle);
            }
            window_sw
                .remove_event_listener_with_callback("pointerup", handler.as_ref().unchecked_ref())
                .expect("pointerup event listener should be removed.");
            // prevent drop of the reset closure before the handler is removed
            drop(reset);
        });
    });

    let set_visible = context.set_visible;
    let input_ref = context.input_ref;

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::button
                as_child=as_child
                node_ref=composed_ref
                attr:r#type="button"
                attr:aria-controls=move || aria_controls.get()
                attr:aria-label=move || resolved_aria_label.get()
                on:pointerdown=compose_callbacks(
                    on_pointer_down,
                    Some(Callback::new(move |_: ev::PointerEvent| {
                        focus_state.update_value(|state| state.click_triggered = true);
                    })),
                    None,
                )
                on:pointercancel=move |event: ev::PointerEvent| {
                    // Do not use compose_callbacks — always reset regardless of preventDefault
                    if let Some(on_pointer_cancel) = on_pointer_cancel {
                        on_pointer_cancel.run(event);
                    }
                    focus_state.set_value(INITIAL_FOCUS_STATE);
                }
                on:click=move |event: ev::MouseEvent| {
                    // Do not use compose_callbacks — always reset focus state
                    if let Some(on_click) = on_click {
                        on_click.run(event.clone());
                    }
                    if event.default_prevented() {
                        focus_state.set_value(INITIAL_FOCUS_STATE);
                        return;
                    }

                    // Toggle visibility (synchronous in Leptos CSR — no flushSync needed)
                    let current = context.visible.get_untracked();
                    set_visible.run(Some(!current));

                    if focus_state.with_value(|s| s.click_triggered)
                        && let Some(node) = input_ref.get()
                    {
                        let input: &web_sys::HtmlInputElement = node.unchecked_ref();
                        let selection_start = focus_state.with_value(|s| s.selection_start);
                        let selection_end = focus_state.with_value(|s| s.selection_end);
                        let _ = input.focus();

                        if selection_start.is_some() || selection_end.is_some() {
                            let input_clone = input.clone();
                            // Wait a tick so focus has settled, then restore selection
                            let cb = Closure::once_into_js(move || {
                                if let Some(doc) = input_clone.owner_document()
                                    && let Some(active) = doc.active_element()
                                    && active
                                        == *input_clone.unchecked_ref::<web_sys::Element>()
                                {
                                    let _ = input_clone.set_selection_start(selection_start);
                                    let _ = input_clone.set_selection_end(selection_end);
                                }
                            });
                            let _ = web_sys::window()
                                .expect("Window should exist.")
                                .request_animation_frame(cb.unchecked_ref());
                        }
                    }
                    focus_state.set_value(INITIAL_FOCUS_STATE);
                }
                on:pointerup=move |event: ev::PointerEvent| {
                    // Do not use compose_callbacks — always reset
                    if let Some(on_pointer_up) = on_pointer_up {
                        on_pointer_up.run(event);
                    }
                    // If click handler hasn't been called, reset after a short delay
                    let focus_state = focus_state;
                    let cb = Closure::once_into_js(move || {
                        focus_state.set_value(INITIAL_FOCUS_STATE);
                    });
                    let _ = web_sys::window()
                        .expect("Window should exist.")
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            cb.unchecked_ref(),
                            50,
                        );
                }
                {..attrs}
            >
                {children.with_value(|children| children())}
            </Primitive>
        </AttributeInterceptor>
    }
}
