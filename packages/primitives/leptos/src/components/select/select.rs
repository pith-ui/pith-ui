use super::*;

/* -------------------------------------------------------------------------------------------------
 * Select
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn Select(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] auto_complete: MaybeProp<String>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] required: MaybeProp<bool>,
    #[prop(into, optional)] form: MaybeProp<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let trigger_ref = AnyNodeRef::new();
    let value_node_ref = AnyNodeRef::new();
    let (value_node_has_children, set_value_node_has_children) = signal(false);

    let direction = use_direction(dir);

    let (open_signal, set_open) = use_controllable_state(UseControllableStateParams {
        prop: open,
        default_prop: default_open,
        on_change: adapt_callback(on_open_change),
    });
    let open_state = Signal::derive(move || open_signal.get().unwrap_or(false));

    let (value_signal, set_value) = use_controllable_state(UseControllableStateParams {
        prop: MaybeProp::derive(move || value.get()),
        default_prop: default_value,
        on_change: adapt_callback(on_value_change),
    });
    let value_state = Signal::derive(move || value_signal.get());

    let trigger_pointer_down_pos_ref: StoredValue<Option<(f64, f64)>> = StoredValue::new(None);

    let content_id = use_id(None);
    let disabled_state = prop_or_default(disabled);
    let required_state = prop_or_default(required);

    let context = SelectContextValue {
        trigger_ref,
        value_node_ref,
        value_node_has_children,
        content_id,
        value: value_state,
        on_value_change: Callback::new(move |val: String| {
            set_value.run(Some(val));
        }),
        open: open_state,
        required: required_state,
        on_open_change: Callback::new(move |val: bool| {
            set_open.run(Some(val));
        }),
        dir: direction,
        trigger_pointer_down_pos_ref,
        disabled: disabled_state,
    };

    // Native select for form integration
    let name = StoredValue::new(name);
    let auto_complete = StoredValue::new(auto_complete);
    let form = StoredValue::new(form);

    view! {
        <Provider value=context>
            <Provider value=(set_value_node_has_children,)>
                <Popper>
                    <CollectionProvider<SelectItemData> item_data_type=ITEM_DATA_PHANTOM>
                        {children.try_with_value(|children| children())}
                    </CollectionProvider<SelectItemData>>

                    <SelectBubbleInput
                        value=value_state
                        name=Signal::derive(move || name.try_with_value(|n| n.get()).flatten())
                        auto_complete=Signal::derive(move || auto_complete.try_with_value(|a| a.get()).flatten())
                        form=Signal::derive(move || form.try_with_value(|f| f.get()).flatten())
                        disabled=disabled_state
                        required=required_state
                    />
                </Popper>
            </Provider>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * SelectTrigger
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn SelectTrigger(
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] on_pointer_down: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<SelectContextValue>();
    let is_disabled =
        Signal::derive(move || context.disabled.get() || disabled.get().unwrap_or(false));

    let composed_trigger_ref = use_composed_refs(vec![node_ref, context.trigger_ref]);

    let get_items = StoredValue::new(use_collection::<SelectItemData>());
    let pointer_type_ref: StoredValue<String> = StoredValue::new("touch".to_string());

    // Typeahead search on the trigger (changes selected value immediately)
    let (search_ref, handle_typeahead_search, reset_typeahead) =
        use_typeahead_search(Callback::new(move |search: String| {
            let _ = get_items.try_with_value(|get_items| {
                let items = get_items();
                let enabled_items: Vec<_> =
                    items.iter().filter(|item| !item.data.disabled).collect();
                let current_value = context.value.get_untracked();
                let current_item = enabled_items
                    .iter()
                    .find(|item| Some(&item.data.value) == current_value.as_ref());
                if let Some(next_item) =
                    find_next_item(&enabled_items, &search, current_item.copied())
                {
                    context.on_value_change.run(next_item.data.value.clone());
                }
            });
        }));

    let on_click_stored = StoredValue::new(on_click);
    let on_pointer_down_stored = StoredValue::new(on_pointer_down);
    let on_key_down_stored = StoredValue::new(on_key_down);

    let handle_open = move |pointer_event: Option<(f64, f64)>| {
        if !is_disabled.get_untracked() {
            context.on_open_change.run(true);
            reset_typeahead.run(());
        }
        if let Some(pos) = pointer_event {
            context.trigger_pointer_down_pos_ref.set_value(Some(pos));
        }
    };

    view! {
        <PopperAnchor as_child=true>
            <AttributeInterceptor let:attrs>
                <Primitive
                    element=html::button
                    as_child=as_child
                    node_ref=composed_trigger_ref
                    attr:r#type="button"
                    attr:role="combobox"
                    attr:aria-controls=move || context.content_id.get()
                    attr:aria-expanded=move || context.open.get().to_string()
                    attr:aria-required=move || if context.required.get() { Some("true".to_string()) } else { None }
                    attr:aria-autocomplete="none"
                    attr:dir=move || context.dir.get().to_string()
                    attr:data-state=move || if context.open.get() { "open" } else { "closed" }
                    attr:disabled=data_attr(is_disabled)
                    attr:data-disabled=data_attr(is_disabled)
                    attr:data-placeholder=move || should_show_placeholder(&context.value.get()).then_some("")
                    on:click=compose_callbacks(
                        on_click_stored.get_value(),
                        Some(Callback::new(move |event: ev::MouseEvent| {
                            // Focus for Safari label compatibility
                            if let Some(target) = event.current_target() {
                                let el: web_sys::HtmlElement = target.unchecked_into();
                                let _ = el.focus();
                            }
                            // Open on click for touch/pen devices
                            if pointer_type_ref.try_get_value().is_some_and(|v| v != "mouse") {
                                handle_open(Some((event.page_x() as f64, event.page_y() as f64)));
                            }
                        })),
                        None,
                    )
                    on:pointerdown=compose_callbacks(
                        on_pointer_down_stored.get_value(),
                        Some(Callback::new(move |event: ev::PointerEvent| {
                            let _ = pointer_type_ref.try_set_value(event.pointer_type());

                            // Release implicit pointer capture
                            if let Some(target) = event.target() {
                                let el: web_sys::HtmlElement = target.unchecked_into();
                                if el.has_pointer_capture(event.pointer_id()) {
                                    let _ = el.release_pointer_capture(event.pointer_id());
                                }
                            }

                            // Only open on left mouse button click (not touch/pen)
                            if event.button() == 0 && !event.ctrl_key() && event.pointer_type() == "mouse" {
                                handle_open(Some((event.page_x() as f64, event.page_y() as f64)));
                                event.prevent_default();
                            }
                        })),
                        None,
                    )
                    on:keydown=compose_callbacks(
                        on_key_down_stored.get_value(),
                        Some(Callback::new(move |event: ev::KeyboardEvent| {
                            let is_typing_ahead = search_ref.try_get_value().is_some_and(|s| !s.is_empty());
                            let is_modifier_key = event.ctrl_key() || event.alt_key() || event.meta_key();
                            if !is_modifier_key && event.key().len() == 1 {
                                handle_typeahead_search.run(event.key());
                            }
                            if is_typing_ahead && event.key() == " " {
                                return;
                            }
                            if OPEN_KEYS.contains(&event.key().as_str()) {
                                handle_open(None);
                                event.prevent_default();
                            }
                        })),
                        None,
                    )
                    {..attrs}
                >
                    {children.try_with_value(|children| children.as_ref().map(|children| children()))}
                </Primitive>
            </AttributeInterceptor>
        </PopperAnchor>
    }
}
