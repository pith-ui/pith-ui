use super::*;

/* -------------------------------------------------------------------------------------------------
 * OneTimePasswordFieldInput
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn OneTimePasswordFieldInput(
    #[prop(into, optional)] on_invalid_change: Option<Callback<String>>,
    #[prop(into, optional)] index: MaybeProp<usize>,
    #[prop(into, optional)] on_focus: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] on_cut: Option<Callback<ev::ClipboardEvent>>,
    #[prop(into, optional)] on_input: Option<Callback<ev::Event>>,
    #[prop(into, optional)] on_change: Option<Callback<ev::Event>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let _children = children;

    let context = expect_context::<OneTimePasswordFieldContextValue>();

    let input_ref = AnyNodeRef::new();
    let composed_input_ref = use_composed_refs(vec![node_ref, input_ref]);

    // Assign a sequential index from the counter during component creation.
    // This avoids relying on the collection (populated asynchronously via Effects)
    // for index resolution.
    let auto_index = context.index_counter.with_value(|counter| {
        let idx = counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        context.input_count.set(idx + 1);
        idx
    });

    let resolved_index = Signal::derive(move || index.get().unwrap_or(auto_index));

    let char_value = Signal::derive(move || {
        let idx = resolved_index.get();
        context.value.get().get(idx).cloned().unwrap_or_default()
    });

    let total_value = Signal::derive(move || context.value.get().join("").trim().to_string());

    let last_selectable_index = Signal::derive(move || {
        let size = context.input_count.get();
        let len = total_value.get().len();
        len.min(size.saturating_sub(1))
    });

    let is_focusable = Signal::derive(move || resolved_index.get() <= last_selectable_index.get());

    let validation = Signal::derive(move || get_validation(context.validation_type.get()));

    let placeholder_value = Signal::derive(move || {
        let idx = resolved_index.get();
        if let Some(ph) = context.placeholder.get() {
            if context.value.get().is_empty() {
                ph.chars().nth(idx).map(|c| c.to_string())
            } else {
                None
            }
        } else {
            None
        }
    });

    let collection_size = Signal::derive(move || context.input_count.get());

    let aria_label = Signal::derive(move || {
        format!(
            "Character {} of {}",
            resolved_index.get() + 1,
            collection_size.get()
        )
    });

    let item_data = Signal::derive(move || ItemData);

    // Read roving focus contexts to determine supports_auto_complete
    let has_tab_stop = Signal::derive(move || {
        use_context::<RovingFocusGroupContext>()
            .map(|ctx| ctx.has_tab_stop.get())
            .unwrap_or(false)
    });
    view! {
        <CollectionItemSlot item_data_type=ITEM_DATA_PHANTOM item_data=item_data node_ref=input_ref>
            <RovingFocusGroupItem
                as_child=true
                focusable=Signal::derive(move || !context.disabled.get() && is_focusable.get())
                active=Signal::derive(move || resolved_index.get() == last_selectable_index.get())
            >
                <OneTimePasswordFieldInputInner
                    context=context
                    composed_input_ref=composed_input_ref
                    resolved_index=resolved_index
                    char_value=char_value
                    last_selectable_index=last_selectable_index
                    validation=validation
                    placeholder_value=placeholder_value
                    collection_size=collection_size
                    aria_label=aria_label
                    has_tab_stop=has_tab_stop
                    on_invalid_change=on_invalid_change
                    on_focus=on_focus
                    on_cut=on_cut
                    on_input=on_input
                    on_change=on_change
                    on_key_down=on_key_down
                    on_pointer_down=on_pointer_down
                    as_child=as_child
                />
            </RovingFocusGroupItem>
        </CollectionItemSlot>
    }
}

/// Inner component that reads `RovingFocusGroupItemContext` after
/// `RovingFocusGroupItem` provides it.
#[component]
fn OneTimePasswordFieldInputInner(
    context: OneTimePasswordFieldContextValue,
    composed_input_ref: AnyNodeRef,
    resolved_index: Signal<usize>,
    char_value: Signal<String>,
    last_selectable_index: Signal<usize>,
    validation: Signal<Option<ValidationInfo>>,
    placeholder_value: Signal<Option<String>>,
    collection_size: Signal<usize>,
    aria_label: Signal<String>,
    has_tab_stop: Signal<bool>,
    on_invalid_change: Option<Callback<String>>,
    on_focus: Option<Callback<ev::FocusEvent>>,
    on_cut: Option<Callback<ev::ClipboardEvent>>,
    on_input: Option<Callback<ev::Event>>,
    on_change: Option<Callback<ev::Event>>,
    on_key_down: Option<Callback<ev::KeyboardEvent>>,
    on_pointer_down: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
) -> impl IntoView {
    let is_current_tab_stop = Signal::derive(move || {
        use_context::<RovingFocusGroupItemContext>()
            .map(|ctx| ctx.is_current_tab_stop.get())
            .unwrap_or(false)
    });

    let supports_auto_complete = Signal::derive(move || {
        if has_tab_stop.get() {
            is_current_tab_stop.get()
        } else {
            resolved_index.get() == 0
        }
    });

    let auto_complete_attr = Signal::derive(move || {
        if supports_auto_complete.get() {
            context.auto_complete.get().as_str().to_string()
        } else {
            "off".to_string()
        }
    });

    let max_length_attr = Signal::derive(move || {
        if supports_auto_complete.get() {
            collection_size.get()
        } else {
            1
        }
    });

    let pw_manager_ignore = Signal::derive(move || {
        if supports_auto_complete.get() {
            None
        } else {
            Some("true".to_string())
        }
    });

    let input_mode_attr =
        Signal::derive(move || validation.get().map(|v| v.input_mode.to_string()));

    let pattern_attr = Signal::derive(move || validation.get().map(|v| v.pattern.to_string()));

    let dispatch = context.dispatch;
    let user_action = context.user_action;

    view! {
        <VoidPrimitive
            element=html::input
            as_child=as_child
            node_ref=composed_input_ref
            attr:r#type=move || context.r#type.get().as_str()
            attr:disabled=data_attr(context.disabled)
            attr:aria-label=move || aria_label.get()
            attr:autocomplete=move || auto_complete_attr.get()
            attr:data-1p-ignore=move || pw_manager_ignore.get()
            attr:data-lpignore=move || pw_manager_ignore.get()
            attr:data-protonpass-ignore=move || pw_manager_ignore.get()
            attr:data-bwignore=move || pw_manager_ignore.get()
            attr:inputmode=move || input_mode_attr.get()
            attr:maxlength=move || max_length_attr.get().to_string()
            attr:pattern=move || pattern_attr.get()
            attr:readonly=data_attr(context.read_only)
            prop:value=move || char_value.get()
            attr:placeholder=move || placeholder_value.get()
            attr:data-radix-otp-input=""
            attr:data-radix-index=move || resolved_index.get().to_string()
            on:focus=compose_callbacks(on_focus, Some(Callback::new(move |event: ev::FocusEvent| {
                let target: web_sys::HtmlInputElement = event.current_target()
                    .expect("Event should have current target")
                    .unchecked_into();
                target.select();
            })), None)
            on:cut=compose_callbacks(on_cut, Some(Callback::new(move |event: ev::ClipboardEvent| {
                let target: web_sys::HtmlInputElement = event.current_target()
                    .expect("Event should have current target")
                    .unchecked_into();
                if !target.value().is_empty() {
                    user_action.set(Some(KeyboardActionDetails::Cut));
                    // Short timeout to clear after change handler completes
                    let user_action = user_action;
                    let cb = web_sys::wasm_bindgen::closure::Closure::once_into_js(move || {
                        user_action.set(None);
                    });
                    let _ = web_sys::window()
                        .expect("Window should exist")
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            cb.unchecked_ref(),
                            10,
                        );
                }
            })), None)
            on:input=compose_callbacks(on_input, Some(Callback::new(move |event: ev::Event| {
                let target: web_sys::HtmlInputElement = event.current_target()
                    .expect("Event should have current target")
                    .unchecked_into();
                let value = target.value();
                let window = web_sys::window().expect("Window should exist");

                // Password manager auto-fill: value has multiple characters
                if value.len() > 1 {
                    event.prevent_default();
                    user_action.set(Some(KeyboardActionDetails::AutocompletePaste));
                    dispatch.with_value(|d| d(UpdateAction::Paste { value }));
                    let user_action = user_action;
                    let cb = web_sys::wasm_bindgen::closure::Closure::once_into_js(move || {
                        user_action.set(None);
                    });
                    let _ = window
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            cb.unchecked_ref(),
                            10,
                        );
                    return;
                }

                // Check for pending user action from keydown/cut
                let action = user_action.get_untracked();
                user_action.set(None);
                let index = resolved_index.get_untracked();

                if let Some(action) = action {
                    match action {
                        KeyboardActionDetails::Cut => {
                            dispatch.with_value(|d| d(UpdateAction::ClearChar {
                                index,
                                reason: ClearCharReason::Cut,
                            }));
                            return;
                        }
                        KeyboardActionDetails::AutocompletePaste => {
                            // Already handled above
                            return;
                        }
                        KeyboardActionDetails::Keydown { key, meta_key, ctrl_key } => {
                            if key == KeydownKey::Char {
                                // Character was dispatched directly in keydown; ignore
                                return;
                            }
                            let is_clearing = key == KeydownKey::Backspace && (meta_key || ctrl_key);
                            if key == KeydownKey::Clear || is_clearing {
                                dispatch.with_value(|d| d(UpdateAction::Clear {
                                    reason: ClearReason::Backspace,
                                }));
                            } else {
                                let reason = match key {
                                    KeydownKey::Delete => ClearCharReason::Delete,
                                    _ => ClearCharReason::Backspace,
                                };
                                dispatch.with_value(|d| d(UpdateAction::ClearChar {
                                    index,
                                    reason,
                                }));
                            }
                            return;
                        }
                    }
                }

                // Default path: process based on validity
                event.prevent_default();
                if target.validity().valid() {
                    if value.is_empty() {
                        let mut reason = ClearCharReason::Backspace;
                        if let Some(input_event) = event.dyn_ref::<web_sys::InputEvent>() {
                            let input_type = input_event.input_type();
                            match input_type.as_str() {
                                "deleteContentBackward" => reason = ClearCharReason::Backspace,
                                "deleteByCut" => reason = ClearCharReason::Cut,
                                _ => {}
                            }
                        }
                        dispatch.with_value(|d| d(UpdateAction::ClearChar {
                            index,
                            reason,
                        }));
                    } else {
                        dispatch.with_value(|d| d(UpdateAction::SetChar {
                            char: value,
                            index,
                        }));
                    }
                } else {
                    if let Some(on_invalid_change) = on_invalid_change {
                        on_invalid_change.run(target.value());
                    }
                    let target_clone = target.clone();
                    let cb = web_sys::wasm_bindgen::closure::Closure::once_into_js(move || {
                        if let Some(active) = target_clone.owner_document()
                            .and_then(|doc| doc.active_element())
                            && active == *target_clone.unchecked_ref::<web_sys::Element>() {
                                target_clone.select();
                            }
                    });
                    let _ = window.request_animation_frame(cb.unchecked_ref());
                }
            })), None)
            on:change=move |event: ev::Event| {
                if let Some(on_change) = on_change {
                    on_change.run(event);
                }
            }
            on:keydown=compose_callbacks(on_key_down, Some(Callback::new(move |event: ev::KeyboardEvent| {
                let index = resolved_index.get_untracked();
                let inputs = context.get_items.with_value(|g| collection_inputs(&g()));
                let key = event.key();
                let window = web_sys::window().expect("Window should exist");

                match key.as_str() {
                    "Clear" | "Delete" | "Backspace" => {
                        let current_value = event.current_target()
                            .map(|ct| ct.unchecked_into::<web_sys::HtmlInputElement>().value())
                            .unwrap_or_default();

                        let keydown_key = match key.as_str() {
                            "Clear" => KeydownKey::Clear,
                            "Delete" => KeydownKey::Delete,
                            _ => KeydownKey::Backspace,
                        };

                        if current_value.is_empty() {
                            // Empty input: no change event will fire, handle directly
                            if keydown_key == KeydownKey::Delete {
                                return;
                            }
                            let is_clearing = keydown_key == KeydownKey::Clear || event.meta_key() || event.ctrl_key();
                            if is_clearing {
                                dispatch.with_value(|d| d(UpdateAction::Clear {
                                    reason: ClearReason::Backspace,
                                }));
                            } else {
                                // Focus previous input via rAF
                                let element = event.current_target()
                                    .map(|ct| ct.unchecked_into::<web_sys::HtmlElement>());
                                if let Some(el) = element {
                                    let prev = otp_input_from(&inputs, &el, -1).cloned();
                                    let cb = web_sys::wasm_bindgen::closure::Closure::once_into_js(move || {
                                        focus_input(prev.as_ref());
                                    });
                                    let _ = window.request_animation_frame(cb.unchecked_ref());
                                }
                            }
                        } else {
                            // Filled input: let browser clear the value, on:input handles dispatch
                            user_action.set(Some(KeyboardActionDetails::Keydown {
                                key: keydown_key,
                                meta_key: event.meta_key(),
                                ctrl_key: event.ctrl_key(),
                            }));
                            let user_action = user_action;
                            let cb = web_sys::wasm_bindgen::closure::Closure::once_into_js(move || {
                                user_action.set(None);
                            });
                            let _ = window
                                .set_timeout_with_callback_and_timeout_and_arguments_0(
                                    cb.unchecked_ref(),
                                    10,
                                );
                        }
                    }
                    "Enter" => {
                        event.prevent_default();
                        context.attempt_submit.with_value(|f| f());
                    }
                    "ArrowDown" | "ArrowUp" => {
                        if context.orientation.get() == Orientation::Horizontal {
                            // Prevent up/down from deselecting the input value
                            event.prevent_default();
                        }
                        // In vertical orientation, RovingFocusGroup handles navigation
                    }
                    _ => {
                        // Only handle single printable characters without modifier keys.
                        let is_single_char = key.chars().count() == 1 && key != " ";
                        if !is_single_char || event.meta_key() || event.ctrl_key() || event.alt_key() {
                            return;
                        }

                        let current_value = event.current_target()
                            .map(|ct| ct.unchecked_into::<web_sys::HtmlInputElement>().value())
                            .unwrap_or_default();

                        if current_value == key {
                            // Same value as key press — no change event will fire.
                            // Focus the next input.
                            event.prevent_default();
                            let element = event.current_target()
                                .map(|ct| ct.unchecked_into::<web_sys::HtmlElement>());
                            if let Some(el) = element {
                                let next = otp_input_from(&inputs, &el, 1);
                                focus_input(next);
                            }
                        } else if !current_value.is_empty() {
                            // Filled with different value and not selected — overflow
                            let ct = event.current_target()
                                .map(|ct| ct.unchecked_into::<web_sys::HtmlInputElement>());
                            if let Some(input) = ct {
                                let sel_start = input.selection_start().ok().flatten().unwrap_or(0);
                                let sel_end = input.selection_end().ok().flatten().unwrap_or(0);
                                let is_selected = sel_start == 0 && sel_end > 0;

                                if !is_selected {
                                    let html_el: web_sys::HtmlElement = input.clone().unchecked_into();
                                    let next_input = otp_input_from(&inputs, &html_el, 1);
                                    let last_input = otp_input_at(&inputs, -1);

                                    if next_input != last_input
                                        && last_input.map(|e| e.unchecked_ref::<web_sys::HtmlElement>() != &html_el).unwrap_or(true)
                                    {
                                        if sel_start == 0 {
                                            dispatch.with_value(|d| d(UpdateAction::SetChar {
                                                char: key,
                                                index,
                                            }));
                                        } else {
                                            dispatch.with_value(|d| d(UpdateAction::SetChar {
                                                char: key,
                                                index: index + 1,
                                            }));
                                        }

                                        user_action.set(Some(KeyboardActionDetails::Keydown {
                                            key: KeydownKey::Char,
                                            meta_key: event.meta_key(),
                                            ctrl_key: event.ctrl_key(),
                                        }));
                                        let user_action = user_action;
                                        let cb = web_sys::wasm_bindgen::closure::Closure::once_into_js(move || {
                                            user_action.set(None);
                                        });
                                        let _ = window
                                            .set_timeout_with_callback_and_timeout_and_arguments_0(
                                                cb.unchecked_ref(),
                                                10,
                                            );
                                    } else {
                                        // On the last input: replace current value in place
                                        event.prevent_default();
                                        dispatch.with_value(|d| d(UpdateAction::SetChar {
                                            char: key,
                                            index,
                                        }));
                                    }
                                }
                                // If selected, browser will replace the selected text
                                // and on:input will handle the SET_CHAR dispatch
                            }
                        }
                        // If empty, let browser insert the character.
                        // on:input will handle the SET_CHAR dispatch.
                    }
                }
            })), None)
            on:pointerdown=compose_callbacks(on_pointer_down, Some(Callback::new(move |event: ev::PointerEvent| {
                event.prevent_default();
                let last = last_selectable_index.get_untracked();
                let inputs = context.get_items.with_value(|g| collection_inputs(&g()));
                let element = otp_input_at(&inputs, last as isize);
                focus_input(element);
            })), None)
        >
            {()}
        </VoidPrimitive>
    }
}

/* -------------------------------------------------------------------------------------------------
 * OneTimePasswordFieldHiddenInput
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn OneTimePasswordFieldHiddenInput(
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let context = expect_context::<OneTimePasswordFieldContextValue>();
    let composed_ref = use_composed_refs(vec![context.hidden_input_ref, node_ref]);

    let value = Signal::derive(move || context.value.get().join("").trim().to_string());
    let name_sig = Signal::derive(move || name.get().or_else(|| context.name.get()));

    view! {
        <input
            node_ref=composed_ref
            name=move || name_sig.get().unwrap_or_default()
            value=move || value.get()
            autocomplete="off"
            autocapitalize="off"
            spellcheck="false"
            type="hidden"
            readonly
        />
    }
}
