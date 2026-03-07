use super::*;

const ITEM_SELECT: &str = "menu.itemSelect";

#[component]
pub fn MenuItem(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_select: Option<Callback<ev::Event>>,
    #[prop(into, optional)] text_value: MaybeProp<String>,
    #[prop(into, optional)] role: MaybeProp<String>,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] on_pointer_down: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_up: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let disabled = prop_or_default(disabled);

    let item_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, item_ref]);
    let root_context = expect_context::<MenuRootContextValue>();
    let content_context = expect_context::<MenuContentContextValue>();
    let is_pointer_down = RwSignal::new(false);

    let handle_select = Callback::new(move |_: ev::MouseEvent| {
        if disabled.get_untracked() {
            return;
        }

        if item_ref.get_untracked().is_none() {
            return;
        }

        // React uses dispatchDiscreteCustomEvent (which wraps in flushSync) to
        // dispatch a custom event and attach on_select as a one-time listener.
        // In Leptos we don't need flushSync, so we call on_select directly with
        // a cancelable CustomEvent — preserving the preventDefault() contract.
        let init = CustomEventInit::new();
        init.set_bubbles(true);
        init.set_cancelable(true);

        let item_select_event = CustomEvent::new_with_event_init_dict(ITEM_SELECT, &init)
            .expect("Item select event should be instantiated.");

        if let Some(on_select) = on_select {
            on_select.run(item_select_event.clone().unchecked_into());
        }

        if item_select_event.default_prevented() {
            let _ = is_pointer_down.try_set(false);
        } else {
            root_context.on_close.run(());
        }
    });

    view! {
        <MenuItemImpl
            disabled={disabled}
            text_value=text_value
            role=role
            as_child=as_child
            node_ref=composed_refs
            on:click=compose_callbacks(on_click, Some(handle_select), None)
            on:pointerdown=move |event| {
                if let Some(on_pointer_down) = on_pointer_down {
                    on_pointer_down.run(event);
                }
                let _ = is_pointer_down.try_set(true);
            }
            on:pointerup=compose_callbacks(on_pointer_up, Some(Callback::new(move |event: ev::PointerEvent| {
                // Pointer down can move to a different menu item which should activate it on pointer up.
                // We dispatch a click for selection to allow composition with click based triggers and to
                // prevent Firefox from getting stuck in text selection mode when the menu closes.
                if !is_pointer_down.try_get_untracked().unwrap_or(false)
                    && let Some(current_target) = event.current_target().map(|current_target| current_target.unchecked_into::<web_sys::HtmlElement>())
                {
                    current_target.click();
                }
            })), None)
            on:keydown=compose_callbacks(on_key_down, Some(Callback::new(move |event: ev::KeyboardEvent| {
                let is_typing_ahead = !content_context.search.try_get_untracked().unwrap_or_default().is_empty();
                if disabled.get_untracked() || (is_typing_ahead && event.key() == " ") {
                    return;
                }
                if SELECTION_KEYS.contains(&event.key().as_str()) {
                    let current_target = event.current_target().map(|current_target| current_target.unchecked_into::<web_sys::HtmlElement>()).expect("Event should have current target.");
                    current_target.click();

                    // We prevent default browser behaviour for selection keys as they should trigger a selection only:
                    // - prevents space from scrolling the page.
                    // - if keydown causes focus to move, prevents keydown from firing on the new target.
                    event.prevent_default();
                }
            })), None)
        >
            {children()}
        </MenuItemImpl>
    }
}

#[component]
pub(super) fn MenuItemImpl(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] text_value: MaybeProp<String>,
    #[prop(into, optional)] role: MaybeProp<String>,
    #[prop(into, optional)] on_pointer_move: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_leave: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_focus: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] on_blur: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let disabled = prop_or_default(disabled);

    let content_context = expect_context::<MenuContentContextValue>();
    let item_ref = AnyNodeRef::new();
    let composed_ref = use_composed_refs(vec![node_ref, item_ref]);
    let (is_focused, set_is_focused) = signal(false);

    // Get the item's `.textContent` as default strategy for typeahead `textValue`.
    let (text_content, set_text_content) = signal("".to_string());
    Effect::new(move |_| {
        if let Some(item) = item_ref.get() {
            let item: web_sys::HtmlElement = item.unchecked_into();
            set_text_content.set(item.text_content().unwrap_or("".into()).trim().to_string());
        }
    });

    let item_data = Signal::derive(move || ItemData {
        disabled: disabled.get(),
        text_value: text_value.get().unwrap_or(text_content.get()),
    });

    let children = StoredValue::new(children);

    view! {
        <CollectionItemSlot item_data_type=ITEM_DATA_PHANTHOM item_data=item_data>
            <RovingFocusGroupItem as_child=true focusable=Signal::derive(move || !disabled.get())>
                <AttributeInterceptor let:attrs>
                    <Primitive
                        element=html::div
                        as_child=as_child
                        node_ref=composed_ref
                        attr:role=move || role.get().unwrap_or("menuitem".into())
                        attr:data-highlighted=data_attr(is_focused.into())
                        attr:aria-disabled=move || disabled.get().then_some("true")
                        attr:data-disabled=data_attr(disabled)
                        /*
                        * We focus items on `pointermove` to achieve the following:
                        *
                        * - Mouse over an item (it focuses)
                        * - Leave mouse where it is and use keyboard to focus a different item
                        * - Wiggle mouse without it leaving previously focused item
                        * - Previously focused item should re-focus
                        *
                        * If we used `mouseover`/`mouseenter` it would not re-focus when the mouse
                        * wiggles. This is to match native menu implementation.
                        */
                        on:pointermove=compose_callbacks(on_pointer_move, Some(when_mouse(move |event| {
                            if disabled.get_untracked() {
                                content_context.on_item_leave.run(event);
                            } else {
                                content_context.on_item_enter.run(event.clone());
                                if !event.default_prevented() {
                                    let item = event.current_target().map(|target| target.unchecked_into::<web_sys::HtmlElement>()).expect("Current target should exist.");
                                    // TODO: focus options
                                    item.focus().expect("Element should be focused.");
                                }
                            }
                        })), None)
                        on:pointerleave=compose_callbacks(on_pointer_leave, Some(when_mouse(move |event| {
                            content_context.on_item_leave.run(event);
                        })), None)
                        on:focus=compose_callbacks(on_focus, Some(Callback::new(move |_| {
                            let _ = set_is_focused.try_set(true);
                        })), None)
                        on:blur=compose_callbacks(on_blur, Some(Callback::new(move |_| {
                            let _ = set_is_focused.try_set(false);
                        })), None)
                        {..attrs}
                    >
                        {children.with_value(|children| children())}
                    </Primitive>
                </AttributeInterceptor>
            </RovingFocusGroupItem>
        </CollectionItemSlot>
    }
}

#[component]
pub fn MenuGroup(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attr:role="group"
                {..attrs}
            >
                {children.with_value(|children| children())}
            </Primitive>
        </AttributeInterceptor>
    }
}

#[component]
pub fn MenuLabel(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref=node_ref
        >
            {children()}
        </Primitive>
    }
}
