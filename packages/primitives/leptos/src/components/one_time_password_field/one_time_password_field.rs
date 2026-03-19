use super::*;

/* -------------------------------------------------------------------------------------------------
 * OneTimePasswordField
 * -----------------------------------------------------------------------------------------------*/

/// Internal context to pass callback props from outer to inner component.
/// This avoids Leptos prop-forwarding issues with `Option<Callback<...>>`.
#[derive(Clone, Copy)]
struct OneTimePasswordFieldCallbacks {
    on_value_change: StoredValue<Option<Callback<String>>>,
    on_auto_submit: StoredValue<Option<Callback<String>>>,
    sanitize_value: StoredValue<Option<Callback<String, String>>>,
    on_paste: StoredValue<Option<Callback<ev::ClipboardEvent>>>,
}

#[component]
pub fn OneTimePasswordField(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] auto_submit: MaybeProp<bool>,
    #[prop(into, optional)] on_auto_submit: Option<Callback<String>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] read_only: MaybeProp<bool>,
    #[prop(into, optional)] auto_complete: MaybeProp<AutoComplete>,
    #[prop(into, optional)] auto_focus: MaybeProp<bool>,
    #[prop(into, optional)] form: MaybeProp<String>,
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    #[prop(into, optional)] r#type: MaybeProp<InputType>,
    #[prop(into, optional)] orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] validation_type: MaybeProp<InputValidationType>,
    #[prop(into, optional)] sanitize_value: Option<Callback<String, String>>,
    #[prop(into, optional)] on_paste: Option<Callback<ev::ClipboardEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let callbacks = OneTimePasswordFieldCallbacks {
        on_value_change: StoredValue::new(on_value_change),
        on_auto_submit: StoredValue::new(on_auto_submit),
        sanitize_value: StoredValue::new(sanitize_value),
        on_paste: StoredValue::new(on_paste),
    };
    provide_context(callbacks);

    view! {
        <CollectionProvider item_data_type=ITEM_DATA_PHANTOM>
            <OneTimePasswordFieldImpl
                value=value
                default_value=default_value
                auto_submit=auto_submit
                disabled=disabled
                read_only=read_only
                auto_complete=auto_complete
                auto_focus=auto_focus
                form=form
                name=name
                placeholder=placeholder
                r#type=r#type
                orientation=orientation
                dir=dir
                validation_type=validation_type
                as_child=as_child
                node_ref=node_ref
            >
                {children.with_value(|children| children())}
            </OneTimePasswordFieldImpl>
        </CollectionProvider>
    }
}

#[component]
fn OneTimePasswordFieldImpl(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] auto_submit: MaybeProp<bool>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] read_only: MaybeProp<bool>,
    #[prop(into, optional)] auto_complete: MaybeProp<AutoComplete>,
    #[prop(into, optional)] auto_focus: MaybeProp<bool>,
    #[prop(into, optional)] form: MaybeProp<String>,
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    #[prop(into, optional)] r#type: MaybeProp<InputType>,
    #[prop(into, optional)] orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] validation_type: MaybeProp<InputValidationType>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let callbacks = expect_context::<OneTimePasswordFieldCallbacks>();
    let on_value_change = callbacks.on_value_change.get_value();
    let on_auto_submit = callbacks.on_auto_submit.get_value();
    let sanitize_value = callbacks.sanitize_value.get_value();
    let on_paste = callbacks.on_paste.get_value();
    let children = StoredValue::new(children);

    let direction = use_direction(dir);
    let disabled_sig = prop_or_default(disabled);
    let read_only_sig = prop_or_default(read_only);
    let auto_complete_sig = prop_or_default(auto_complete);
    let auto_focus_sig = prop_or_default(auto_focus);
    let form_sig = Signal::derive(move || form.get());
    let name_sig = Signal::derive(move || name.get());
    let placeholder_sig = Signal::derive(move || placeholder.get());
    let type_sig = prop_or_default(r#type);
    let orientation_sig = prop_or(orientation, Orientation::Horizontal);
    let validation_type_sig = prop_or_default(validation_type);
    let auto_submit_sig = prop_or_default(auto_submit);

    let sanitize_value_fn: StoredValue<SendWrapper<Box<dyn Fn(String) -> Vec<String>>>> =
        StoredValue::new(SendWrapper::new(Box::new(move |input: String| {
            let input = remove_whitespace(&input);
            let validation = get_validation(validation_type_sig.get_untracked());
            let cleaned = if let Some(v) = validation {
                let re = js_sys::RegExp::new(v.regexp, "g");
                js_sys::JsString::from(input.as_str())
                    .replace_by_pattern(&re, "")
                    .as_string()
                    .unwrap_or_default()
            } else if let Some(sanitize) = sanitize_value {
                sanitize.run(input)
            } else {
                input
            };
            cleaned.chars().map(|c| c.to_string()).collect()
        })));

    let controlled_value: MaybeProp<Vec<String>> =
        MaybeProp::derive(move || value.get().map(|v| sanitize_value_fn.with_value(|f| f(v))));

    let default_value_vec: MaybeProp<Vec<String>> = MaybeProp::derive(move || {
        default_value
            .get()
            .map(|v| sanitize_value_fn.with_value(|f| f(v)))
    });

    let on_vec_change: Option<Callback<Option<Vec<String>>>> = on_value_change.map(|cb| {
        Callback::new(move |v: Option<Vec<String>>| {
            if let Some(v) = v {
                cb.run(v.join(""));
            }
        })
    });

    let (value_signal, set_value) = use_controllable_state(UseControllableStateParams {
        prop: controlled_value,
        default_prop: default_value_vec,
        on_change: on_vec_change,
    });

    let value_vec = Signal::derive(move || value_signal.get().unwrap_or_default());

    let hidden_input_ref = AnyNodeRef::new();
    let root_ref = AnyNodeRef::new();
    let get_items_fn = use_collection::<ItemData>();
    let get_items = StoredValue::new(get_items_fn);
    let composed_refs = use_composed_refs(vec![node_ref, root_ref]);

    let user_action: RwSignal<Option<KeyboardActionDetails>> = RwSignal::new(None);

    let locate_form = move || -> Option<web_sys::HtmlFormElement> {
        if let Some(form_id) = form_sig.get() {
            let doc = document();
            let el = doc.get_element_by_id(&form_id)?;
            el.dyn_into::<web_sys::HtmlFormElement>().ok()
        } else if let Some(node) = hidden_input_ref.get() {
            let input: &web_sys::HtmlInputElement = node.deref().unchecked_ref();
            input.form()
        } else {
            let inputs = get_items.with_value(|g| collection_inputs(&g()));
            inputs.first()?.form()
        }
    };

    let attempt_submit: StoredValue<SendWrapper<Box<dyn Fn()>>> =
        StoredValue::new(SendWrapper::new(Box::new(move || {
            if let Some(form) = locate_form() {
                let _ = form.request_submit();
            }
        })));

    let dispatch: StoredValue<SendWrapper<Box<dyn Fn(UpdateAction)>>> =
        StoredValue::new(SendWrapper::new(Box::new(move |action: UpdateAction| {
            let inputs = get_items.with_value(|g| collection_inputs(&g()));
            let current_value = value_vec.get_untracked();
            let size = inputs.len();

            match action {
                UpdateAction::SetChar { index, char } => {
                    let current_target = otp_input_at(&inputs, index as isize);

                    if current_value.get(index).is_some_and(|v| *v == char) {
                        // Same value — just move to next
                        if let Some(ct) = current_target {
                            let next = otp_input_from(&inputs, ct, 1);
                            focus_input(next);
                        }
                        return;
                    }

                    if char.is_empty() {
                        return;
                    }

                    // Validate
                    let validation = get_validation(validation_type_sig.get_untracked());
                    if let Some(v) = validation {
                        let re = js_sys::RegExp::new(v.regexp, "g");
                        let clean = js_sys::JsString::from(char.as_str())
                            .replace_by_pattern(&re, "")
                            .as_string()
                            .unwrap_or_default();
                        if clean != char {
                            return;
                        }
                    }

                    if current_value.len() >= size && size > 0 {
                        // Replace current value; move to next input
                        let mut new_value = current_value.clone();
                        if index < new_value.len() {
                            new_value[index] = char;
                        }
                        set_value.run(Some(new_value));
                        if let Some(ct) = current_target {
                            let next = otp_input_from(&inputs, ct, 1);
                            if next.is_some() {
                                focus_input(next);
                            } else {
                                // Last input: re-select so subsequent typing replaces
                                focus_input(Some(ct));
                            }
                        }
                        return;
                    }

                    let mut new_value = current_value.clone();
                    // Ensure the vec is large enough
                    while new_value.len() <= index {
                        new_value.push(String::new());
                    }
                    new_value[index] = char;

                    let last_element = otp_input_at(&inputs, -1);

                    set_value.run(Some(new_value));

                    if current_target != last_element {
                        if let Some(ct) = current_target {
                            let next = otp_input_from(&inputs, ct, 1);
                            focus_input(next);
                        }
                    } else if let Some(ct) = current_target {
                        ct.select();
                    }
                }

                UpdateAction::ClearChar { index, reason } => {
                    if current_value.get(index).is_none()
                        || current_value.get(index).is_some_and(|v| v.is_empty())
                    {
                        return;
                    }

                    let new_value: Vec<String> = current_value
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| *i != index)
                        .map(|(_, v)| v.clone())
                        .collect();

                    let current_target = otp_input_at(&inputs, index as isize);
                    let previous = current_target.and_then(|ct| otp_input_from(&inputs, ct, -1));

                    set_value.run(Some(new_value));

                    match reason {
                        ClearCharReason::Backspace => {
                            focus_input(previous);
                        }
                        ClearCharReason::Delete | ClearCharReason::Cut => {
                            focus_input(current_target);
                        }
                    }
                }

                UpdateAction::Clear { reason } => {
                    if current_value.is_empty() {
                        return;
                    }

                    match reason {
                        ClearReason::Backspace | ClearReason::Delete => {
                            set_value.run(Some(vec![]));
                            let first = otp_input_at(&inputs, 0);
                            focus_input(first);
                        }
                        ClearReason::Reset => {
                            set_value.run(Some(vec![]));
                        }
                    }
                }

                UpdateAction::Paste {
                    value: pasted_value,
                } => {
                    let new_value = sanitize_value_fn.with_value(|f| f(pasted_value));
                    if new_value.is_empty() {
                        return;
                    }

                    let focus_index = new_value.len() as isize - 1;
                    set_value.run(Some(new_value));
                    let target = otp_input_at(&inputs, focus_index);
                    focus_input(target);
                }
            }
        })));

    // Re-validate when the validation type changes
    let prev_validation_type: RwSignal<Option<InputValidationType>> = RwSignal::new(None);
    Effect::new(move |_| {
        let current_type = validation_type_sig.get();
        let prev = prev_validation_type.get_untracked();
        if prev.is_some() && prev != Some(current_type) && get_validation(current_type).is_some() {
            let current = value_vec.get_untracked().join("");
            let new_value = sanitize_value_fn.with_value(|f| f(current));
            set_value.run(Some(new_value));
        }
        prev_validation_type.set(Some(current_type));
    });

    // Form reset listener
    Effect::new(move |_| {
        if let Some(form) = locate_form() {
            let dispatch = dispatch;
            let closure = SendWrapper::new(
                web_sys::wasm_bindgen::closure::Closure::<dyn Fn()>::new(move || {
                    dispatch.with_value(|d| {
                        d(UpdateAction::Clear {
                            reason: ClearReason::Reset,
                        })
                    });
                }),
            );
            let form = SendWrapper::new(form);
            form.add_event_listener_with_callback("reset", closure.as_ref().unchecked_ref())
                .expect("Reset event listener should be added.");

            Owner::on_cleanup(move || {
                form.remove_event_listener_with_callback("reset", closure.as_ref().unchecked_ref())
                    .expect("Reset event listener should be removed.");
            });
        }
    });

    // Auto-submit effect
    let input_count: RwSignal<usize> = RwSignal::new(0);
    let prev_joined: RwSignal<String> = RwSignal::new(String::new());
    Effect::new(move |_| {
        let current = value_vec.get();
        let joined = current.join("");
        let prev = prev_joined.get_untracked();
        prev_joined.set(joined.clone());
        if prev == joined {
            return;
        }

        let size = input_count.get();

        if auto_submit_sig.get() && current.iter().all(|c| !c.is_empty()) && current.len() == size {
            if let Some(on_auto_submit) = on_auto_submit {
                on_auto_submit.run(joined);
            }
            attempt_submit.with_value(|f| f());
        }
    });

    let context_value = OneTimePasswordFieldContextValue {
        value: value_vec,
        dispatch,
        attempt_submit,
        auto_complete: auto_complete_sig,
        auto_focus: auto_focus_sig,
        disabled: disabled_sig,
        read_only: read_only_sig,
        form: form_sig,
        name: name_sig,
        placeholder: placeholder_sig,
        r#type: type_sig,
        orientation: orientation_sig,
        validation_type: validation_type_sig,
        user_action,
        sanitize_value: sanitize_value_fn,
        hidden_input_ref,
        get_items,
        index_counter: StoredValue::new(std::sync::atomic::AtomicUsize::new(0)),
        input_count,
    };

    view! {
        <Provider value=context_value>
            <CollectionSlot item_data_type=ITEM_DATA_PHANTOM node_ref=root_ref>
                <RovingFocusGroup
                    as_child=true
                    orientation=orientation_sig
                    dir=direction
                >
                    <Primitive
                        element=html::div
                        as_child=as_child
                        node_ref=composed_refs
                        attr:role="group"
                        on:paste=compose_callbacks(on_paste, Some(Callback::new(move |event: ev::ClipboardEvent| {
                            event.prevent_default();
                            let clipboard_event: &web_sys::ClipboardEvent = event.unchecked_ref();
                            if let Some(data) = clipboard_event.clipboard_data()
                                && let Ok(pasted) = data.get_data("Text") {
                                    dispatch.with_value(|d| d(UpdateAction::Paste { value: pasted }));
                                }
                        })), None)
                    >
                        {children.with_value(|children| children())}
                    </Primitive>
                </RovingFocusGroup>
            </CollectionSlot>
        </Provider>
    }
}
