use super::*;

/* -------------------------------------------------------------------------------------------------
 * Combobox (Root)
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn Combobox(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] values: MaybeProp<Vec<String>>,
    #[prop(into, optional)] default_values: MaybeProp<Vec<String>>,
    #[prop(into, optional)] on_values_change: Option<Callback<Vec<String>>>,
    #[prop(into, optional)] input_value: MaybeProp<String>,
    #[prop(into, optional)] default_input_value: MaybeProp<String>,
    #[prop(into, optional)] on_input_value_change: Option<Callback<String>>,
    #[prop(optional)] multiple: bool,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] required: MaybeProp<bool>,
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] form: MaybeProp<String>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let input_ref = AnyNodeRef::new();
    let trigger_ref = AnyNodeRef::new();

    let direction = use_direction(dir);

    // Open state
    let (open_signal, set_open) = use_controllable_state(UseControllableStateParams {
        prop: open,
        default_prop: default_open,
        on_change: on_open_change.map(|cb| {
            Callback::new(move |value: Option<bool>| {
                if let Some(value) = value {
                    cb.run(value);
                }
            })
        }),
    });
    let open_state = Signal::derive(move || open_signal.get().unwrap_or(false));

    // Single-select value state
    let (value_signal, set_value) = use_controllable_state(UseControllableStateParams {
        prop: MaybeProp::derive(move || value.get()),
        default_prop: default_value,
        on_change: on_value_change.map(|cb| {
            Callback::new(move |value: Option<String>| {
                if let Some(value) = value {
                    cb.run(value);
                }
            })
        }),
    });
    let value_state = Signal::derive(move || value_signal.get());

    // Multi-select values state
    let (values_signal, set_values) = use_controllable_state(UseControllableStateParams {
        prop: MaybeProp::derive(move || values.get()),
        default_prop: default_values,
        on_change: on_values_change.map(|cb| {
            Callback::new(move |value: Option<Vec<String>>| {
                if let Some(value) = value {
                    cb.run(value);
                }
            })
        }),
    });
    let values_state =
        Signal::derive(move || values_signal.get().unwrap_or_default());

    // Input value state
    let (input_value_signal, set_input_value) = use_controllable_state(UseControllableStateParams {
        prop: MaybeProp::derive(move || input_value.get()),
        default_prop: default_input_value,
        on_change: on_input_value_change.map(|cb| {
            Callback::new(move |value: Option<String>| {
                if let Some(value) = value {
                    cb.run(value);
                }
            })
        }),
    });
    let input_value_state =
        Signal::derive(move || input_value_signal.get().unwrap_or_default());

    let content_id = use_id(None);
    let disabled_state = prop_or_default(disabled);
    let required_state = prop_or_default(required);
    let active_descendant_id: RwSignal<Option<String>> = RwSignal::new(None);

    let on_value_change_cb = Callback::new(move |val: String| {
        if multiple {
            // Toggle the value in the multi-select list
            let mut current = values_state.get_untracked();
            if let Some(pos) = current.iter().position(|v| v == &val) {
                current.remove(pos);
            } else {
                current.push(val);
            }
            set_values.run(Some(current));
        } else {
            set_value.run(Some(val));
        }
    });

    let context = ComboboxContextValue {
        input_ref,
        trigger_ref,
        content_id,
        value: value_state,
        values: values_state,
        on_value_change: on_value_change_cb,
        input_value: input_value_state,
        on_input_value_change: Callback::new(move |val: String| {
            set_input_value.run(Some(val));
        }),
        open: open_state,
        on_open_change: Callback::new(move |val: bool| {
            set_open.run(Some(val));
        }),
        dir: direction,
        disabled: disabled_state,
        required: required_state,
        active_descendant_id,
        multiple,
    };

    // Native input for form integration
    let name = StoredValue::new(name);
    let form = StoredValue::new(form);

    view! {
        <Provider value=context>
            <Popper>
                <CollectionProvider<ComboboxItemData> item_data_type=ITEM_DATA_PHANTOM>
                    {children.try_with_value(|children| children())}
                </CollectionProvider<ComboboxItemData>>

                <ComboboxBubbleInput
                    value=value_state
                    values=values_state
                    multiple=multiple
                    name=Signal::derive(move || name.try_with_value(|n| n.get()).flatten())
                    form=Signal::derive(move || form.try_with_value(|f| f.get()).flatten())
                    disabled=disabled_state
                    required=required_state
                />
            </Popper>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ComboboxAnchor
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ComboboxAnchor(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    view! {
        <PopperAnchor as_child=true>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
            >
                {children.try_with_value(|children| children.as_ref().map(|children| children()))}
            </Primitive>
        </PopperAnchor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ComboboxInput
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ComboboxInput(
    #[prop(into, optional)] on_input: Option<Callback<ev::Event>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] on_focus: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] on_blur: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let context = expect_context::<ComboboxContextValue>();
    let composed_ref = use_composed_refs(vec![node_ref, context.input_ref]);

    let get_items = StoredValue::new(use_collection::<ComboboxItemData>());

    let on_input_stored = StoredValue::new(on_input);
    let on_key_down_stored = StoredValue::new(on_key_down);
    let on_click_stored = StoredValue::new(on_click);
    let on_focus_stored = StoredValue::new(on_focus);
    let on_blur_stored = StoredValue::new(on_blur);

    view! {
        <AttributeInterceptor let:attrs>
            <VoidPrimitive
                element=html::input
                as_child=as_child
                node_ref=composed_ref
                attr:r#type="text"
                attr:role="combobox"
                attr:aria-controls=move || context.content_id.get()
                attr:aria-expanded=move || context.open.get().to_string()
                attr:aria-activedescendant=move || context.active_descendant_id.get().unwrap_or_default()
                attr:aria-autocomplete="list"
                attr:aria-disabled=move || context.disabled.get().then_some("true".to_string())
                attr:data-state=move || if context.open.get() { "open" } else { "closed" }
                attr:data-disabled=data_attr(context.disabled)
                attr:disabled=data_attr(context.disabled)
                attr:dir=move || context.dir.get().to_string()
                attr:placeholder=move || placeholder.get()
                prop:value=move || context.input_value.get()
                on:input=move |event: ev::Event| {
                    if let Some(Some(cb)) = on_input_stored.try_get_value() {
                        cb.run(event.clone());
                    }
                    if !event.default_prevented() {
                        let target: web_sys::HtmlInputElement = event.target().unwrap().unchecked_into();
                        context.on_input_value_change.run(target.value());
                        // Open popup when typing
                        if !context.open.get_untracked() {
                            context.on_open_change.run(true);
                        }
                    }
                }
                on:keydown=move |event: ev::KeyboardEvent| {
                    if let Some(Some(cb)) = on_key_down_stored.try_get_value() {
                        cb.run(event.clone());
                    }
                    if event.default_prevented() {
                        return;
                    }
                    let key = event.key();
                    match key.as_str() {
                        "ArrowDown" => {
                            event.prevent_default();
                            if !context.open.get_untracked() {
                                context.on_open_change.run(true);
                            } else {
                                // Navigate to next item (including disabled items)
                                let _ = get_items.try_with_value(|get_items| {
                                    let items = get_items();
                                    let all_items: Vec<_> = items.iter().collect();
                                    navigate_items(&all_items, &context.active_descendant_id, true);
                                });
                            }
                        }
                        "ArrowUp" => {
                            event.prevent_default();
                            if context.open.get_untracked() {
                                // Navigate to previous item (including disabled items)
                                let _ = get_items.try_with_value(|get_items| {
                                    let items = get_items();
                                    let all_items: Vec<_> = items.iter().collect();
                                    navigate_items(&all_items, &context.active_descendant_id, false);
                                });
                            }
                        }
                        "Enter" => {
                            event.prevent_default();
                            if context.open.get_untracked() {
                                // Select the highlighted item
                                if let Some(active_id) = context.active_descendant_id.get_untracked() {
                                    let _ = get_items.try_with_value(|get_items| {
                                        let items = get_items();
                                        let item = items.iter().find(|item| {
                                            item.r#ref.get_untracked().is_some_and(|el| {
                                                let el: &web_sys::Element = (*el).unchecked_ref();
                                                el.id() == active_id
                                            })
                                        });
                                        if let Some(item) = item {
                                            if !item.data.disabled {
                                                context.on_value_change.run(item.data.value.clone());
                                                if !context.multiple {
                                                    context.on_open_change.run(false);
                                                    context.active_descendant_id.set(None);
                                                    // Clear input for single select after selection
                                                    context.on_input_value_change.run(item.data.text_value.clone());
                                                }
                                            }
                                        }
                                    });
                                }
                            }
                        }
                        "Escape" => {
                            if context.open.get_untracked() {
                                event.prevent_default();
                                context.on_open_change.run(false);
                                context.active_descendant_id.set(None);
                            }
                        }
                        "Home" => {
                            if context.open.get_untracked() {
                                event.prevent_default();
                                let _ = get_items.try_with_value(|get_items| {
                                    let items = get_items();
                                    let enabled_items: Vec<_> = items.iter().filter(|item| !item.data.disabled).collect();
                                    if let Some(first) = enabled_items.first() {
                                        if let Some(el) = first.r#ref.get_untracked() {
                                            let el: &web_sys::Element = (*el).unchecked_ref();
                                            context.active_descendant_id.set(Some(el.id()));
                                            scroll_item_into_view(el);
                                        }
                                    }
                                });
                            }
                        }
                        "End" => {
                            if context.open.get_untracked() {
                                event.prevent_default();
                                let _ = get_items.try_with_value(|get_items| {
                                    let items = get_items();
                                    let enabled_items: Vec<_> = items.iter().filter(|item| !item.data.disabled).collect();
                                    if let Some(last) = enabled_items.last() {
                                        if let Some(el) = last.r#ref.get_untracked() {
                                            let el: &web_sys::Element = (*el).unchecked_ref();
                                            context.active_descendant_id.set(Some(el.id()));
                                            scroll_item_into_view(el);
                                        }
                                    }
                                });
                            }
                        }
                        _ => {}
                    }
                }
                on:click=move |event: ev::MouseEvent| {
                    if let Some(Some(cb)) = on_click_stored.try_get_value() {
                        cb.run(event.clone());
                    }
                    if !event.default_prevented() && !context.disabled.get_untracked() && !context.open.get_untracked() {
                        context.on_open_change.run(true);
                    }
                }
                on:focus=move |event: ev::FocusEvent| {
                    if let Some(Some(cb)) = on_focus_stored.try_get_value() {
                        cb.run(event);
                    }
                }
                on:blur=move |event: ev::FocusEvent| {
                    if let Some(Some(cb)) = on_blur_stored.try_get_value() {
                        cb.run(event);
                    }
                }
                {..attrs}
            >
                {()}
            </VoidPrimitive>
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ComboboxTrigger
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ComboboxTrigger(
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<ComboboxContextValue>();
    let composed_ref = use_composed_refs(vec![node_ref, context.trigger_ref]);

    let on_click_stored = StoredValue::new(on_click);

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::button
                as_child=as_child
                node_ref=composed_ref
                attr:r#type="button"
                attr:tabindex="-1"
                attr:aria-label="Toggle popup"
                attr:data-state=move || if context.open.get() { "open" } else { "closed" }
                attr:data-disabled=data_attr(context.disabled)
                attr:disabled=data_attr(context.disabled)
                on:click=move |event: ev::MouseEvent| {
                    if let Some(Some(cb)) = on_click_stored.try_get_value() {
                        cb.run(event.clone());
                    }
                    if !event.default_prevented() && !context.disabled.get_untracked() {
                        context.on_open_change.run(!context.open.get_untracked());
                        // Focus the input
                        if let Some(input_el) = context.input_ref.get_untracked() {
                            let el: &web_sys::HtmlElement = (*input_el).unchecked_ref();
                            let _ = el.focus();
                        }
                    }
                }
                {..attrs}
            >
                {children.try_with_value(|children| children.as_ref().map(|children| children()))}
            </Primitive>
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ComboboxIcon
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ComboboxIcon(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::span
                as_child=as_child
                node_ref=node_ref
                attr:aria-hidden="true"
                {..attrs}
            >
                {children.try_with_value(|children| {
                    children.as_ref().map(|children| children()).unwrap_or_else(|| "▼".into_any())
                })}
            </Primitive>
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ComboboxClear
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ComboboxClear(
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<ComboboxContextValue>();
    let on_click_stored = StoredValue::new(on_click);

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::button
                as_child=as_child
                node_ref=node_ref
                attr:r#type="button"
                attr:tabindex="-1"
                attr:aria-label="Clear value"
                attr:data-disabled=data_attr(context.disabled)
                attr:disabled=data_attr(context.disabled)
                on:click=move |event: ev::MouseEvent| {
                    if let Some(Some(cb)) = on_click_stored.try_get_value() {
                        cb.run(event.clone());
                    }
                    if !event.default_prevented() && !context.disabled.get_untracked() {
                        context.on_value_change.run(String::new());
                        context.on_input_value_change.run(String::new());
                        // Focus the input
                        if let Some(input_el) = context.input_ref.get_untracked() {
                            let el: &web_sys::HtmlElement = (*input_el).unchecked_ref();
                            let _ = el.focus();
                        }
                    }
                }
                {..attrs}
            >
                {children.try_with_value(|children| children.as_ref().map(|children| children()))}
            </Primitive>
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ComboboxBubbleInput (internal)
 * -----------------------------------------------------------------------------------------------*/

/// Hidden native <input> element for form integration.
#[component]
fn ComboboxBubbleInput(
    value: Signal<Option<String>>,
    values: Signal<Vec<String>>,
    multiple: bool,
    name: Signal<Option<String>>,
    form: Signal<Option<String>>,
    disabled: Signal<bool>,
    required: Signal<bool>,
) -> impl IntoView {
    let bubble_ref = AnyNodeRef::new();
    let prev_value: StoredValue<Option<String>> = StoredValue::new(None);

    // Bubble value change to parent forms
    Effect::new(move |_| {
        let current_value = if multiple {
            let v = values.get();
            if v.is_empty() {
                None
            } else {
                Some(v.join(","))
            }
        } else {
            value.get()
        };
        let previous = prev_value.try_get_value().flatten();
        let _ = prev_value.try_set_value(current_value.clone());

        if previous != current_value
            && let Some(input_el) = bubble_ref.get()
        {
            let input_el: web_sys::HtmlInputElement = (*input_el).clone().unchecked_into();
            input_el.set_value(&current_value.clone().unwrap_or_default());
            let event_init = web_sys::EventInit::new();
            event_init.set_bubbles(true);
            let event = web_sys::Event::new_with_event_init_dict("change", &event_init)
                .expect("Event should be created.");
            let _ = input_el.dispatch_event(&event);
        }
    });

    view! {
        <input
            node_ref=bubble_ref
            type="hidden"
            aria-hidden="true"
            tabindex="-1"
            name=move || name.get()
            form=move || form.get()
            disabled=move || disabled.get()
            required=move || required.get()
            style=VISUALLY_HIDDEN_STYLES_STR
            prop:value=move || {
                if multiple {
                    let v = values.get();
                    if v.is_empty() { String::new() } else { v.join(",") }
                } else {
                    value.get().unwrap_or_default()
                }
            }
        />
    }
}

/* -------------------------------------------------------------------------------------------------
 * Utilities
 * -----------------------------------------------------------------------------------------------*/

/// Navigate to the next or previous enabled item in the collection
fn navigate_items(
    enabled_items: &[&CollectionItemValue<ComboboxItemData>],
    active_descendant_id: &RwSignal<Option<String>>,
    forward: bool,
) {
    if enabled_items.is_empty() {
        return;
    }

    let current_id = active_descendant_id.get_untracked();

    let current_index = current_id.as_ref().and_then(|id| {
        enabled_items.iter().position(|item| {
            item.r#ref.get_untracked().is_some_and(|el| {
                let el: &web_sys::Element = (*el).unchecked_ref();
                el.id() == *id
            })
        })
    });

    let next_index = match current_index {
        Some(idx) => {
            if forward {
                if idx + 1 < enabled_items.len() {
                    idx + 1
                } else {
                    idx
                }
            } else if idx > 0 {
                idx - 1
            } else {
                0
            }
        }
        None => {
            if forward {
                0
            } else {
                enabled_items.len() - 1
            }
        }
    };

    if let Some(item) = enabled_items.get(next_index) {
        if let Some(el) = item.r#ref.get_untracked() {
            let el: &web_sys::Element = (*el).unchecked_ref();
            active_descendant_id.set(Some(el.id()));
            scroll_item_into_view(el);
        }
    }
}

/// Scroll an item into view within its scrollable parent
fn scroll_item_into_view(el: &web_sys::Element) {
    let options = web_sys::ScrollIntoViewOptions::new();
    options.set_block(web_sys::ScrollLogicalPosition::Nearest);
    let el: &web_sys::HtmlElement = el.unchecked_ref();
    el.scroll_into_view_with_scroll_into_view_options(&options);
}
