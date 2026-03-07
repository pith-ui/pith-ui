use super::*;

#[component]
pub fn Menu(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] modal: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let open = prop_or_default(open);
    let modal = prop_or(modal, true);
    let on_open_change = on_open_change.unwrap_or(Callback::new(|_| {}));

    let content_ref = AnyNodeRef::new();
    let is_using_keyboard = RwSignal::new(false);
    let direction = use_direction(dir);

    let popper_anchor_ref = AnyNodeRef::new();
    let context_value = MenuContextValue {
        open,
        content_ref,
        on_open_change,
        popper_anchor_ref,
    };
    let root_context_value = MenuRootContextValue {
        is_using_keyboard: is_using_keyboard.into(),
        dir: direction,
        modal,
        on_close: Callback::new(move |_| on_open_change.run(false)),
    };

    let handle_pointer: SendWrapper<Rc<RefCell<Option<Closure<dyn Fn(ev::PointerEvent)>>>>> =
        SendWrapper::new(Rc::new(RefCell::new(Some(Closure::<
            dyn Fn(ev::PointerEvent),
        >::new(move |_| {
            is_using_keyboard.set(false);
        })))));

    let handle_key_down: SendWrapper<Rc<RefCell<Option<Closure<dyn Fn(ev::KeyboardEvent)>>>>> = {
        let handle_pointer = handle_pointer.clone();
        SendWrapper::new(Rc::new(RefCell::new(Some(Closure::<
            dyn Fn(ev::KeyboardEvent),
        >::new(move |_| {
            is_using_keyboard.set(true);

            let options = AddEventListenerOptions::new();
            options.set_capture(true);
            options.set_once(true);

            if let Some(hp) = handle_pointer.borrow().as_ref() {
                let cb: &wasm_bindgen::JsValue = hp.as_ref().unchecked_ref();
                document()
                    .add_event_listener_with_callback_and_add_event_listener_options(
                        "pointerdown",
                        cb.unchecked_ref(),
                        &options,
                    )
                    .expect("Pointer down event listener should be added.");
                document()
                    .add_event_listener_with_callback_and_add_event_listener_options(
                        "pointermove",
                        cb.unchecked_ref(),
                        &options,
                    )
                    .expect("Pointer move event listener should be added.");
            }
        })))))
    };

    Effect::new({
        let handle_key_down = handle_key_down.clone();
        move |_| {
            let options = AddEventListenerOptions::new();
            options.set_capture(true);

            // Capture phase ensures we set the boolean before any side effects execute
            // in response to the key or pointer event as they might depend on this value.
            if let Some(hk) = handle_key_down.borrow().as_ref() {
                let cb: &wasm_bindgen::JsValue = hk.as_ref().unchecked_ref();
                document()
                    .add_event_listener_with_callback_and_add_event_listener_options(
                        "keydown",
                        cb.unchecked_ref(),
                        &options,
                    )
                    .expect("Key down event listener should be added.");
            }
        }
    });

    on_cleanup(move || {
        let options = EventListenerOptions::new();
        options.set_capture(true);

        if let Some(hk) = handle_key_down.borrow().as_ref() {
            let cb: &wasm_bindgen::JsValue = hk.as_ref().unchecked_ref();
            document()
                .remove_event_listener_with_callback_and_event_listener_options(
                    "keydown",
                    cb.unchecked_ref(),
                    &options,
                )
                .expect("Key down event listener should be removed.");
        }

        if let Some(hp) = handle_pointer.borrow().as_ref() {
            let cb: &wasm_bindgen::JsValue = hp.as_ref().unchecked_ref();
            document()
                .remove_event_listener_with_callback_and_event_listener_options(
                    "pointerdown",
                    cb.unchecked_ref(),
                    &options,
                )
                .expect("Pointer down event listener should be removed.");
            document()
                .remove_event_listener_with_callback_and_event_listener_options(
                    "pointermove",
                    cb.unchecked_ref(),
                    &options,
                )
                .expect("Pointer move event listener should be removed.");
        }
    });

    view! {
        <Popper anchor_ref=popper_anchor_ref>
            <Provider value=context_value>
                <Provider value=root_context_value>
                    {children.with_value(|children| children())}
                </Provider>
            </Provider>
        </Popper>
    }
}

#[component]
pub fn MenuAnchor(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let menu_context = expect_context::<MenuContextValue>();
    // Compose the user-provided node_ref with the Menu's own popper anchor ref.
    // This ensures the Menu's Popper context gets the anchor element directly,
    // even when a closer Popper context (e.g., from a Tooltip wrapping the trigger)
    // would shadow it during PopperAnchor's normal expect_context lookup.
    let composed_refs = use_composed_refs(vec![node_ref, menu_context.popper_anchor_ref]);

    view! {
        <PopperAnchor as_child=as_child node_ref=composed_refs>
            {children()}
        </PopperAnchor>
    }
}

#[component]
pub fn MenuPortal(
    /// Specify a container element to portal the content into.
    #[prop(into, optional)]
    container: MaybeProp<SendWrapper<web_sys::Element>>,
    /// Optional ref for the container element.
    #[prop(optional)]
    container_ref: AnyNodeRef,
    /// Used to force mounting when more control is needed. Useful when controlling animation with animation libraries.
    #[prop(into, optional)]
    force_mount: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <ScopedPortal container=container container_ref=container_ref force_mount=force_mount>
            {children.with_value(|children| children())}
        </ScopedPortal>
    }
}
