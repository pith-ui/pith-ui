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
    let get_items: StoredValue<SendWrapper<Box<dyn Fn() -> Vec<CollectionItemValue<ItemData>>>>> =
        StoredValue::new(use_collection::<ItemData>());

    let input_ref = AnyNodeRef::new();
    let (element, set_element) = signal::<Option<SendWrapper<web_sys::HtmlInputElement>>>(None);
    let composed_input_ref = use_composed_refs(vec![node_ref, input_ref]);

    // Track element from ref
    Effect::new(move |_| {
        if let Some(node) = input_ref.get() {
            let el: web_sys::HtmlInputElement = node.deref().clone().unchecked_into();
            set_element.set(Some(SendWrapper::new(el)));
        }
    });

    let resolved_index = Signal::derive(move || {
        if let Some(idx) = index.get() {
            idx
        } else if let Some(el) = element.get() {
            let items = get_items.with_value(|f| f());
            let html_el: &web_sys::HtmlElement = el.unchecked_ref();
            collection_index_of(&items, html_el).unwrap_or(0)
        } else {
            0
        }
    });

    let char_value = Signal::derive(move || {
        let idx = resolved_index.get();
        context.value.get().get(idx).cloned().unwrap_or_default()
    });

    let total_value = Signal::derive(move || context.value.get().join("").trim().to_string());

    let last_selectable_index = Signal::derive(move || {
        let items = get_items.with_value(|f| f());
        let size = items.len();
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

    let collection_size = Signal::derive(move || get_items.with_value(|f| f()).len());

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
    let is_current_tab_stop_ctx = Signal::derive(move || {
        use_context::<RovingFocusGroupItemContext>()
            .map(|ctx| ctx.is_current_tab_stop.get())
            .unwrap_or(false)
    });

    view! {
        <CollectionItemSlot item_data_type=ITEM_DATA_PHANTOM item_data=item_data>
            <RovingFocusGroupItem
                as_child=true
                focusable=Signal::derive(move || !context.disabled.get() && is_focusable.get())
                active=Signal::derive(move || resolved_index.get() == last_selectable_index.get())
            >
                <OneTimePasswordFieldInputInner
                    context=context
                    get_items=get_items
                    composed_input_ref=composed_input_ref
                    resolved_index=resolved_index
                    char_value=char_value
                    last_selectable_index=last_selectable_index
                    validation=validation
                    placeholder_value=placeholder_value
                    collection_size=collection_size
                    aria_label=aria_label
                    has_tab_stop=has_tab_stop
                    is_current_tab_stop_ctx=is_current_tab_stop_ctx
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
    get_items: StoredValue<SendWrapper<Box<dyn Fn() -> Vec<CollectionItemValue<ItemData>>>>>,
    composed_input_ref: AnyNodeRef,
    resolved_index: Signal<usize>,
    char_value: Signal<String>,
    last_selectable_index: Signal<usize>,
    validation: Signal<Option<ValidationInfo>>,
    placeholder_value: Signal<Option<String>>,
    collection_size: Signal<usize>,
    aria_label: Signal<String>,
    has_tab_stop: Signal<bool>,
    is_current_tab_stop_ctx: Signal<bool>,
    on_invalid_change: Option<Callback<String>>,
    on_focus: Option<Callback<ev::FocusEvent>>,
    on_cut: Option<Callback<ev::ClipboardEvent>>,
    on_input: Option<Callback<ev::Event>>,
    on_change: Option<Callback<ev::Event>>,
    on_key_down: Option<Callback<ev::KeyboardEvent>>,
    on_pointer_down: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
) -> impl IntoView {
    let supports_auto_complete = Signal::derive(move || {
        if has_tab_stop.get() {
            is_current_tab_stop_ctx.get()
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
                if value.len() > 1 {
                    // Password manager auto-fill
                    event.prevent_default();
                    user_action.set(Some(KeyboardActionDetails::AutocompletePaste));
                    dispatch.with_value(|d| d(UpdateAction::Paste { value }));
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
            on:change=compose_callbacks(on_change, Some(Callback::new(move |event: ev::Event| {
                let target: web_sys::HtmlInputElement = event.target()
                    .expect("Event should have target")
                    .unchecked_into();
                let value = target.value();
                event.prevent_default();
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
                            // Already handled in input handler
                            return;
                        }
                        KeyboardActionDetails::Keydown { key, meta_key, ctrl_key } => {
                            if key == KeydownKey::Char {
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

                // Skip if the DOM value already matches the signal value.
                // This prevents spurious dispatches when `prop:value` marks the
                // input as dirty and `change` fires on blur (e.g., Tab key).
                if value == char_value.get_untracked() {
                    return;
                }

                // Only update if valid
                if target.validity().valid() {
                    if value.is_empty() {
                        let mut reason = ClearCharReason::Backspace;
                        // Check native InputEvent inputType
                        if let Ok(input_event) = event.clone().dyn_into::<web_sys::InputEvent>()
                            && let Some(input_type) = input_event.input_type().into() {
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
                    let _ = web_sys::window()
                        .expect("Window should exist")
                        .request_animation_frame(cb.unchecked_ref());
                }
            })), None)
            on:keydown=compose_callbacks(on_key_down, Some(Callback::new(move |event: ev::KeyboardEvent| {
                let index = resolved_index.get_untracked();
                let items = get_items.with_value(|f| f());
                let key = event.key();

                match key.as_str() {
                    "Clear" | "Delete" | "Backspace" => {
                        event.prevent_default();
                        let current_value = event.current_target()
                            .map(|ct| ct.unchecked_into::<web_sys::HtmlInputElement>().value())
                            .unwrap_or_default();

                        let keydown_key = match key.as_str() {
                            "Clear" => KeydownKey::Clear,
                            "Delete" => KeydownKey::Delete,
                            _ => KeydownKey::Backspace,
                        };

                        if current_value.is_empty() {
                            if keydown_key == KeydownKey::Delete {
                                return;
                            }
                            let is_clearing = keydown_key == KeydownKey::Clear || event.meta_key() || event.ctrl_key();
                            if is_clearing {
                                dispatch.with_value(|d| d(UpdateAction::Clear {
                                    reason: ClearReason::Backspace,
                                }));
                            } else {
                                let element = event.current_target()
                                    .map(|ct| ct.unchecked_into::<web_sys::HtmlElement>());
                                if let Some(el) = element {
                                    let prev = collection_from(&items, &el, -1)
                                        .and_then(collection_element);
                                    focus_input(prev.as_ref());
                                }
                            }
                        } else {
                            let is_clearing = (keydown_key == KeydownKey::Backspace && (event.meta_key() || event.ctrl_key()))
                                || keydown_key == KeydownKey::Clear;
                            if is_clearing {
                                dispatch.with_value(|d| d(UpdateAction::Clear {
                                    reason: ClearReason::Backspace,
                                }));
                            } else {
                                let reason = match keydown_key {
                                    KeydownKey::Delete => ClearCharReason::Delete,
                                    _ => ClearCharReason::Backspace,
                                };
                                dispatch.with_value(|d| d(UpdateAction::ClearChar {
                                    index,
                                    reason,
                                }));
                            }
                        }
                    }
                    "Enter" => {
                        event.prevent_default();
                        context.attempt_submit.with_value(|f| f());
                    }
                    "ArrowRight" | "ArrowLeft" => {
                        if context.orientation.get() == Orientation::Horizontal {
                            event.prevent_default();
                            // Stop the RovingFocusGroup from also handling arrow keys,
                            // since its collection data may be stale.
                            event.stop_immediate_propagation();
                            let direction: isize = if key == "ArrowRight" { 1 } else { -1 };
                            let element = event.current_target()
                                .map(|ct| ct.unchecked_into::<web_sys::HtmlElement>());
                            if let Some(el) = element {
                                let next = collection_from(&items, &el, direction)
                                    .and_then(collection_element);
                                if let Some(next_input) = &next {
                                    let next_idx = collection_index_of(
                                        &items,
                                        next_input.unchecked_ref(),
                                    )
                                    .unwrap_or(usize::MAX);
                                    if next_idx <= last_selectable_index.get_untracked() {
                                        // Defer focus to a microtask so it runs after all
                                        // synchronous handlers (including RovingFocusGroup).
                                        let next_el = next_input.clone();
                                        let window = web_sys::window().expect("Window should exist");
                                        let cb = web_sys::wasm_bindgen::closure::Closure::once_into_js(move || {
                                            focus_input(Some(&next_el));
                                        });
                                        window.queue_microtask(cb.unchecked_ref());
                                    }
                                }
                            }
                        } else {
                            // In vertical orientation, prevent ArrowLeft/ArrowRight
                            // from deselecting the input value.
                            event.prevent_default();
                        }
                    }
                    "ArrowDown" | "ArrowUp" => {
                        if context.orientation.get() == Orientation::Horizontal {
                            // In horizontal orientation, prevent ArrowUp/ArrowDown
                            // from deselecting the input value.
                            event.prevent_default();
                        } else {
                            event.prevent_default();
                            event.stop_immediate_propagation();
                            let direction: isize = if key == "ArrowDown" { 1 } else { -1 };
                            let element = event.current_target()
                                .map(|ct| ct.unchecked_into::<web_sys::HtmlElement>());
                            if let Some(el) = element {
                                let next = collection_from(&items, &el, direction)
                                    .and_then(collection_element);
                                if let Some(next_input) = &next {
                                    let next_idx = collection_index_of(
                                        &items,
                                        next_input.unchecked_ref(),
                                    )
                                    .unwrap_or(usize::MAX);
                                    if next_idx <= last_selectable_index.get_untracked() {
                                        let next_el = next_input.clone();
                                        let window = web_sys::window().expect("Window should exist");
                                        let cb = web_sys::wasm_bindgen::closure::Closure::once_into_js(move || {
                                            focus_input(Some(&next_el));
                                        });
                                        window.queue_microtask(cb.unchecked_ref());
                                    }
                                }
                            }
                        }
                    }
                    _ => {
                        // Only handle single printable characters without modifier keys.
                        // Modifier keys indicate keyboard shortcuts (e.g. Cmd+V for paste)
                        // which must not be intercepted.
                        let is_single_char = key.chars().count() == 1 && key != " ";
                        if !is_single_char || event.meta_key() || event.ctrl_key() || event.alt_key() {
                            return;
                        }

                        let current_value = event.current_target()
                            .map(|ct| ct.unchecked_into::<web_sys::HtmlInputElement>().value())
                            .unwrap_or_default();

                        if current_value.is_empty() {
                            // Typing into empty input — dispatch directly
                            event.prevent_default();
                            dispatch.with_value(|d| d(UpdateAction::SetChar {
                                char: key,
                                index,
                            }));
                        } else if current_value == key {
                            // Same value as key press — focus next
                            event.prevent_default();
                            let element = event.current_target()
                                .map(|ct| ct.unchecked_into::<web_sys::HtmlElement>());
                            if let Some(el) = element {
                                let next = collection_from(&items, &el, 1)
                                    .and_then(collection_element);
                                focus_input(next.as_ref().map(|e| {
                                    e.unchecked_ref::<web_sys::HtmlInputElement>()
                                }));
                            }
                        } else {
                            // Different value on filled input
                            event.prevent_default();
                            let ct = event.current_target()
                                .map(|ct| ct.unchecked_into::<web_sys::HtmlInputElement>());
                            if let Some(input) = ct {
                                let sel_start = input.selection_start().ok().flatten().unwrap_or(0);
                                let sel_end = input.selection_end().ok().flatten().unwrap_or(0);
                                let is_selected = sel_start == 0 && sel_end > 0;

                                if is_selected {
                                    // Replace selected value
                                    dispatch.with_value(|d| d(UpdateAction::SetChar {
                                        char: key,
                                        index,
                                    }));
                                } else {
                                    // Cursor at specific position — set for current or next index
                                    let html_el: web_sys::HtmlElement = input.clone().unchecked_into();
                                    let next_input = collection_from(&items, &html_el, 1)
                                        .and_then(collection_element);
                                    let last_input = collection_at(&items, -1)
                                        .and_then(collection_element);

                                    if next_input.as_ref() != last_input.as_ref()
                                        && last_input.as_ref().map(|e| e.unchecked_ref::<web_sys::HtmlElement>() != &html_el).unwrap_or(true)
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
                                    }
                                }
                            }
                        }
                    }
                }
            })), None)
            on:pointerdown=compose_callbacks(on_pointer_down, Some(Callback::new(move |event: ev::PointerEvent| {
                event.prevent_default();
                let last = last_selectable_index.get_untracked();
                let items = get_items.with_value(|f| f());
                let element = collection_at(&items, last as isize)
                    .and_then(collection_element);
                focus_input(element.as_ref());
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
