use super::*;

/* -------------------------------------------------------------------------------------------------
 * ComboboxItem
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ComboboxItem(
    #[prop(into)] value: String,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] text_value: MaybeProp<String>,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] on_pointer_move: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_leave: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<ComboboxContextValue>();

    let disabled = prop_or_default(disabled);
    let value = StoredValue::new(value);
    let is_selected = Signal::derive(move || {
        value.try_get_value().is_some_and(|val| {
            if context.multiple {
                context.values.get().contains(&val)
            } else {
                context.value.get().is_some_and(|v| v == val)
            }
        })
    });
    let (text_value_state, set_text_value) = signal(text_value.get_untracked().unwrap_or_default());
    let text_id = use_id(None);
    let item_id = use_id(None);
    let item_node_ref = AnyNodeRef::new();
    let composed_item_ref = use_composed_refs(vec![node_ref, item_node_ref]);

    // Set the generated id on the element so aria-activedescendant works
    let is_highlighted = Signal::derive(move || {
        context
            .active_descendant_id
            .get()
            .is_some_and(|id| id == item_id.get())
    });

    let handle_select = move || {
        if !disabled.get_untracked()
            && let Some(val) = value.try_get_value()
        {
            context.on_value_change.run(val.clone());
            if context.multiple {
                // Clear input text after multi-select
                context.on_input_value_change.run(String::new());
            } else {
                // Defer the close to next task so reactive effects can settle
                let cb = wasm_bindgen::closure::Closure::once_into_js(move || {
                    context.on_open_change.run(false);
                    context.active_descendant_id.set(None);
                });
                web_sys::window()
                    .expect("Window should exist.")
                    .set_timeout_with_callback(cb.unchecked_ref())
                    .expect("setTimeout should succeed.");
                // Set input text to selected item's text
                context.on_input_value_change.run(text_value_state.get_untracked());
            }
            // Return focus to the input after selection
            if let Some(input_el) = context.input_ref.get_untracked() {
                let el: &web_sys::HtmlElement = (*input_el).unchecked_ref();
                let _ = el.focus();
            }
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

    let item_context = ComboboxItemContextValue {
        value: value.get_value(),
        disabled: disabled.get_untracked(),
        text_id,
        is_selected,
    };

    let on_click_stored = StoredValue::new(on_click);
    let on_pointer_move_stored = StoredValue::new(on_pointer_move);
    let on_pointer_leave_stored = StoredValue::new(on_pointer_leave);

    // Provide the text change callback separately so ComboboxItemText can use it
    let text_change_provider = StoredValue::new(on_item_text_change);

    view! {
        <Provider value=item_context>
            <Provider value=text_change_provider>
                <CollectionItemSlot
                    item_data_type=ITEM_DATA_PHANTOM
                    item_data=MaybeProp::derive(move || {
                        value.try_get_value().map(|val| ComboboxItemData {
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
                            attr:id=move || item_id.get()
                            attr:aria-labelledby=move || text_id.get()
                            attr:aria-selected=move || if is_selected.get() { Some("true".to_string()) } else { None }
                            attr:data-state=move || if is_selected.get() { "checked" } else { "unchecked" }
                            attr:data-highlighted=move || is_highlighted.get().then_some("")
                            attr:aria-disabled=move || disabled.get().then_some("true".to_string())
                            attr:data-disabled=data_attr(disabled)
                            on:pointerdown=move |event: ev::PointerEvent| {
                                // Prevent default to stop the input from losing focus when
                                // clicking items. This is critical for multi-select where
                                // the popup must stay open and the blur handler must not fire.
                                event.prevent_default();
                            }
                            on:click=move |event: ev::MouseEvent| {
                                if let Some(Some(cb)) = on_click_stored.try_get_value() {
                                    cb.run(event.clone());
                                }
                                if !event.default_prevented() {
                                    handle_select();
                                }
                            }
                            on:pointermove=move |event: ev::PointerEvent| {
                                if let Some(Some(cb)) = on_pointer_move_stored.try_get_value() {
                                    cb.run(event.clone());
                                }
                                if !event.default_prevented() && !disabled.get_untracked() {
                                    context.active_descendant_id.set(Some(item_id.get_untracked()));
                                }
                            }
                            on:pointerleave=move |event: ev::PointerEvent| {
                                if let Some(Some(cb)) = on_pointer_leave_stored.try_get_value() {
                                    cb.run(event.clone());
                                }
                                if !event.default_prevented() {
                                    // Clear highlight when leaving this item
                                    let current = context.active_descendant_id.get_untracked();
                                    if current.as_ref().is_some_and(|id| *id == item_id.get_untracked()) {
                                        context.active_descendant_id.set(None);
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
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ComboboxItemText
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ComboboxItemText(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let item_context = expect_context::<ComboboxItemContextValue>();
    let on_item_text_change =
        expect_context::<StoredValue<Callback<Option<SendWrapper<web_sys::HtmlElement>>>>>();
    let item_text_ref = AnyNodeRef::new();
    let composed_ref = use_composed_refs(vec![node_ref, item_text_ref]);

    // Notify parent about text node for textValue extraction
    Effect::new(move |_| {
        if let Some(el) = item_text_ref.get() {
            let el: web_sys::HtmlElement = (*el).clone().unchecked_into();
            if let Some(cb) = on_item_text_change.try_get_value() {
                cb.run(Some(SendWrapper::new(el)));
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
 * ComboboxItemIndicator
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ComboboxItemIndicator(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let item_context = expect_context::<ComboboxItemContextValue>();

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
 * ComboboxGroup
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ComboboxGroup(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let group_id = use_id(None);

    let group_context = ComboboxGroupContextValue { id: group_id };

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
 * ComboboxLabel
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ComboboxLabel(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let group_context = expect_context::<ComboboxGroupContextValue>();

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
