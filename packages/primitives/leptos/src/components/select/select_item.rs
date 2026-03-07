use super::*;

/* -------------------------------------------------------------------------------------------------
 * SelectItem
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn SelectItem(
    #[prop(into)] value: String,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] text_value: MaybeProp<String>,
    #[prop(into, optional)] on_pointer_up: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_down: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_move: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_leave: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_focus: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] on_blur: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<SelectContextValue>();
    let content_context = expect_context::<SelectContentContextValue>();
    let item_ref_callback =
        expect_context::<Callback<(Option<SendWrapper<web_sys::HtmlElement>>, String, bool)>>();

    let disabled = prop_or_default(disabled);
    let value = StoredValue::new(value);
    let is_selected = Signal::derive(move || {
        value
            .try_get_value()
            .is_some_and(|val| context.value.get().is_some_and(|v| v == val))
    });
    let (is_focused, set_is_focused) = signal(false);
    let (text_value_state, set_text_value) = signal(text_value.get_untracked().unwrap_or_default());
    let text_id = use_id(None);
    let pointer_type_ref: StoredValue<String> = StoredValue::new("touch".to_string());
    let item_node_ref = AnyNodeRef::new();

    // Register with item_ref_callback when mounted
    let composed_item_ref = use_composed_refs(vec![node_ref, item_node_ref]);
    Effect::new(move |_| {
        if let Some(val) = value.try_get_value() {
            let node = item_node_ref.get().map(|el| {
                let el: web_sys::HtmlElement = (*el).clone().unchecked_into();
                SendWrapper::new(el)
            });
            item_ref_callback.run((node, val, disabled.get_untracked()));
        }
    });

    let handle_select = move || {
        if !disabled.get_untracked()
            && let Some(val) = value.try_get_value()
        {
            context.on_value_change.run(val);
            // Defer the close to the next task so that reactive effects triggered by the
            // value change (e.g. text copying in SelectItemText) can settle before the
            // content is unmounted. Synchronous close would dispose child scopes while
            // queued effects still reference their StoredValues, causing WASM panics.
            let cb = Closure::once_into_js(move || {
                context.on_open_change.run(false);
            });
            web_sys::window()
                .expect("Window should exist.")
                .set_timeout_with_callback(cb.unchecked_ref())
                .expect("setTimeout should succeed.");
        }
    };

    let on_item_text_change =
        Callback::new(move |node: Option<SendWrapper<web_sys::HtmlElement>>| {
            if let Some(node) = &node {
                let text = node.text_content().unwrap_or_default().trim().to_string();
                if !text.is_empty() {
                    set_text_value.set(text);
                }
            }
        });

    let item_context = SelectItemContextValue {
        value: value.get_value(),
        disabled: disabled.get_untracked(),
        text_id,
        is_selected,
        on_item_text_change,
    };

    let on_pointer_up_stored = StoredValue::new(on_pointer_up);
    let on_pointer_down_stored = StoredValue::new(on_pointer_down);
    let on_pointer_move_stored = StoredValue::new(on_pointer_move);
    let on_pointer_leave_stored = StoredValue::new(on_pointer_leave);
    let on_key_down_stored = StoredValue::new(on_key_down);
    let on_focus_stored = StoredValue::new(on_focus);
    let on_blur_stored = StoredValue::new(on_blur);
    let on_click_stored = StoredValue::new(on_click);

    view! {
        <Provider value=item_context>
            <CollectionItemSlot
                item_data_type=ITEM_DATA_PHANTOM
                item_data=MaybeProp::derive(move || {
                    value.try_get_value().map(|val| SelectItemData {
                        value: val,
                        disabled: disabled.get(),
                        text_value: text_value_state.get(),
                    })
                })
                node_ref=composed_item_ref
            >
                <AttributeInterceptor let:attrs>
                    <Primitive
                        element=html::div
                        as_child=as_child
                        node_ref=composed_item_ref
                        attr:role="option"
                        attr:aria-labelledby=move || text_id.get()
                        attr:data-highlighted=move || is_focused.get().then_some("")
                        attr:aria-selected=move || if is_selected.get() && is_focused.get() { Some("true".to_string()) } else { None }
                        attr:data-state=move || if is_selected.get() { "checked" } else { "unchecked" }
                        attr:aria-disabled=move || disabled.get().then_some("true".to_string())
                        attr:data-disabled=data_attr(disabled)
                        attr:tabindex=move || if disabled.get() { None } else { Some("-1".to_string()) }
                        // Event handlers are inlined rather than using compose_callbacks
                        // with Callback::new(...) because Callback::new creates a StoredValue
                        // in the reactive scope. When the scope is disposed during unmount,
                        // browser events (e.g. blur from focus restoration) can fire after
                        // disposal and try to invoke the disposed Callback, causing a WASM panic.
                        on:focus=move |event: ev::FocusEvent| {
                            if let Some(Some(cb)) = on_focus_stored.try_get_value() {
                                cb.run(event.clone());
                            }
                            if !event.default_prevented() {
                                set_is_focused.set(true);
                            }
                        }
                        on:blur=move |event: ev::FocusEvent| {
                            if let Some(Some(cb)) = on_blur_stored.try_get_value() {
                                cb.run(event.clone());
                            }
                            if !event.default_prevented() {
                                set_is_focused.set(false);
                            }
                        }
                        on:click=move |event: ev::MouseEvent| {
                            if let Some(Some(cb)) = on_click_stored.try_get_value() {
                                cb.run(event.clone());
                            }
                            if !event.default_prevented()
                                && pointer_type_ref.try_get_value().is_some_and(|v| v != "mouse")
                            {
                                handle_select();
                            }
                        }
                        on:pointerup=move |event: ev::PointerEvent| {
                            if let Some(Some(cb)) = on_pointer_up_stored.try_get_value() {
                                cb.run(event.clone());
                            }
                            if !event.default_prevented()
                                && pointer_type_ref.try_get_value().is_some_and(|v| v == "mouse")
                            {
                                handle_select();
                            }
                        }
                        on:pointerdown=move |event: ev::PointerEvent| {
                            if let Some(Some(cb)) = on_pointer_down_stored.try_get_value() {
                                cb.run(event.clone());
                            }
                            if !event.default_prevented() {
                                let _ = pointer_type_ref.try_set_value(event.pointer_type());
                            }
                        }
                        on:pointermove=move |event: ev::PointerEvent| {
                            if let Some(Some(cb)) = on_pointer_move_stored.try_get_value() {
                                cb.run(event.clone());
                            }
                            if !event.default_prevented() {
                                let _ = pointer_type_ref.try_set_value(event.pointer_type());
                                if disabled.get_untracked() {
                                    content_context.on_item_leave.run(());
                                } else if event.pointer_type() == "mouse"
                                    && let Some(target) = event.current_target()
                                {
                                    let el: web_sys::HtmlElement = target.unchecked_into();
                                    let mut opts = web_sys::FocusOptions::new();
                                    opts.set_prevent_scroll(true);
                                    let _ = el.focus_with_options(&opts);
                                }
                            }
                        }
                        on:pointerleave=move |event: ev::PointerEvent| {
                            if let Some(Some(cb)) = on_pointer_leave_stored.try_get_value() {
                                cb.run(event.clone());
                            }
                            if !event.default_prevented()
                                && let Some(target) = event.current_target()
                            {
                                let el: web_sys::Element = target.unchecked_into();
                                let active = web_sys::window()
                                    .and_then(|w| w.document())
                                    .and_then(|d| d.active_element());
                                if active.as_ref() == Some(&el) {
                                    content_context.on_item_leave.run(());
                                }
                            }
                        }
                        on:keydown=move |event: ev::KeyboardEvent| {
                            if let Some(Some(cb)) = on_key_down_stored.try_get_value() {
                                cb.run(event.clone());
                            }
                            if !event.default_prevented() {
                                let is_typing_ahead = content_context.search_ref.try_get_value().is_some_and(|s| !s.is_empty());
                                if is_typing_ahead && event.key() == " " {
                                    return;
                                }
                                if SELECTION_KEYS.contains(&event.key().as_str()) {
                                    handle_select();
                                }
                                if event.key() == " " {
                                    event.prevent_default();
                                }
                            }
                        }
                        {..attrs}
                    >
                        {children.try_with_value(|children| children.as_ref().map(|c| c()))}
                    </Primitive>
                </AttributeInterceptor>
            </CollectionItemSlot>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * SelectItemText
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn SelectItemText(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<SelectContextValue>();
    let item_context = expect_context::<SelectItemContextValue>();
    let item_text_ref = AnyNodeRef::new();

    let composed_ref = use_composed_refs(vec![node_ref, item_text_ref]);

    // Notify parent about text node for textValue extraction
    Effect::new(move |_| {
        if let Some(el) = item_text_ref.get() {
            let el: web_sys::HtmlElement = (*el).clone().unchecked_into();
            item_context
                .on_item_text_change
                .run(Some(SendWrapper::new(el)));
        }
    });

    // When this item is selected AND the SelectValue has no static children,
    // portal the text content into SelectValue's span.
    // In Leptos, we can't use React portals, so we use an Effect to copy text content.
    let is_selected = item_context.is_selected;
    Effect::new(move |_| {
        if is_selected.get() && !context.value_node_has_children.get() {
            // Copy text content from this item text into the value node
            if let (Some(text_el), Some(value_el)) =
                (item_text_ref.get(), context.value_node_ref.get())
            {
                let text_el: &web_sys::HtmlElement = (*text_el).unchecked_ref();
                let value_el: &web_sys::HtmlElement = (*value_el).unchecked_ref();
                let text = text_el.text_content().unwrap_or_default();
                value_el.set_text_content(Some(&text));
            }
        }
    });

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::span
                as_child=as_child
                node_ref=composed_ref
                attr:id=move || item_context.text_id.get()
                {..attrs}
            >
                {children.try_with_value(|children| children.as_ref().map(|c| c()))}
            </Primitive>
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * SelectItemIndicator
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn SelectItemIndicator(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let item_context = expect_context::<SelectItemContextValue>();

    view! {
        <Show when=move || item_context.is_selected.get()>
            <AttributeInterceptor let:attrs>
                <Primitive
                    element=html::span
                    as_child=as_child
                    node_ref=node_ref
                    attr:aria-hidden="true"
                    {..attrs}
                >
                    {children.try_with_value(|children| children.as_ref().map(|c| c()))}
                </Primitive>
            </AttributeInterceptor>
        </Show>
    }
}

/* -------------------------------------------------------------------------------------------------
 * SelectGroup
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn SelectGroup(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let group_id = use_id(None);

    let group_context = SelectGroupContextValue { id: group_id };

    view! {
        <Provider value=group_context>
            <AttributeInterceptor let:attrs>
                <Primitive
                    element=html::div
                    as_child=as_child
                    node_ref=node_ref
                    attr:role="group"
                    attr:aria-labelledby=move || group_id.get()
                    {..attrs}
                >
                    {children.try_with_value(|children| children.as_ref().map(|c| c()))}
                </Primitive>
            </AttributeInterceptor>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * SelectLabel
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn SelectLabel(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let group_context = expect_context::<SelectGroupContextValue>();

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attr:id=move || group_context.id.get()
                {..attrs}
            >
                {children.try_with_value(|children| children.as_ref().map(|c| c()))}
            </Primitive>
        </AttributeInterceptor>
    }
}
