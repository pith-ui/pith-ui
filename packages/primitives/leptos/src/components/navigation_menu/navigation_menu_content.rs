use super::*;

/* -------------------------------------------------------------------------------------------------
 * NavigationMenuContent
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn NavigationMenuContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] on_pointer_enter: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_leave: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] on_interact_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<NavigationMenuContextValue>();
    let item_context_signal = expect_context::<RwSignal<NavigationMenuItemContextValue>>();
    let item_context = item_context_signal.get_untracked();

    let composed_refs = use_composed_refs(vec![node_ref, item_context.content_ref]);

    let open = Memo::new(move |_| {
        let item_ctx = item_context_signal.get();
        item_ctx.value == context.value.get()
    });

    let item_value = StoredValue::new(item_context.value.clone());
    let trigger_ref = item_context.trigger_ref;
    let focus_proxy_ref = item_context.focus_proxy_ref;
    let was_escape_close_ref = item_context.was_escape_close_ref;
    let on_content_focus_outside = item_context.on_content_focus_outside;
    let on_root_content_close = item_context.on_root_content_close;

    let force_mount_val = force_mount.get_untracked().unwrap_or(false);

    // Viewport registration: when a viewport component exists, register content for viewport
    // rendering. Uses has_viewport_component (set synchronously during NavigationMenuViewport
    // construction) instead of the DOM element ref, avoiding a flash where content renders
    // inline before the viewport Effect fires.
    let has_viewport = Signal::derive(move || context.has_viewport_component.get());

    // Capture user attributes (e.g., data-testid, class) via AttributeInterceptor so they can
    // be forwarded to the viewport rendering path (extra_attrs). AttributeInterceptor intercepts
    // attrs at the type-composition level — BEFORE any DOM element is created — eliminating
    // timing issues with the old hidden-span approach.
    let captured_attrs: StoredValue<Vec<(String, String)>> = StoredValue::new(vec![]);

    Effect::new(move |_| {
        if has_viewport.get() {
            let content_data = ContentData {
                value: item_value.get_value(),
                trigger_ref,
                focus_proxy_ref,
                was_escape_close_ref,
                on_content_focus_outside,
                on_root_content_close,
                force_mount: force_mount_val,
                children: Some(std::sync::Arc::new(move || {
                    children.with_value(|c| c.as_ref().map(|c| c())).into_any()
                }) as ChildrenFn),
                on_pointer_enter,
                on_pointer_leave,
                on_escape_key_down,
                on_focus_outside,
                on_pointer_down_outside,
                on_interact_outside,
                content_ref: node_ref,
                item_content_ref: item_context.content_ref,
                extra_attrs: captured_attrs.get_value(),
            };
            context
                .on_viewport_content_change
                .run((item_value.get_value(), content_data));
        }
    });

    on_cleanup(move || {
        context
            .on_viewport_content_remove
            .run(item_value.get_value());
    });

    // Inline rendering: render via Presence when there is no viewport.
    // By placing <Presence> directly as the root (rather than inside a reactive closure),
    // caller spread attributes (e.g. attr:class) propagate through the transparent component
    // chain: Presence -> Show -> ContentImpl -> FocusGroup -> DismissableLayer -> Primitive div.
    let present = Signal::derive(move || {
        !has_viewport.get() && (force_mount.get().unwrap_or(false) || open.get())
    });

    let presence_ref = AnyNodeRef::new();

    view! {
        <AttributeInterceptor let:attrs>
            // Extract user attrs for viewport forwarding, then pass through to inline Presence.
            {captured_attrs.set_value(extract_attrs(attrs.clone()))}
            <Presence present=present node_ref={presence_ref} {..attrs}>
                <NavigationMenuContentImpl
                    value=item_value.get_value()
                    trigger_ref=trigger_ref
                    focus_proxy_ref=focus_proxy_ref
                    was_escape_close_ref=was_escape_close_ref
                    on_content_focus_outside=on_content_focus_outside
                    on_root_content_close=on_root_content_close
                    as_child=as_child
                    node_ref=composed_refs
                    presence_ref=presence_ref
                    on_pointer_enter=Callback::new(compose_callbacks(
                        on_pointer_enter,
                        Some(Callback::new(move |_: ev::PointerEvent| {
                            context.on_content_enter.run(());
                        })),
                        None,
                    ))
                    on_pointer_leave=Callback::new(compose_callbacks(
                        on_pointer_leave,
                        Some(Callback::new(move |event: ev::PointerEvent| {
                            if event.pointer_type() == "mouse" {
                                context.on_content_leave.run(());
                            }
                        })),
                        None,
                    ))
                    on_escape_key_down=on_escape_key_down.unwrap_or(Callback::new(|_| {}))
                    on_focus_outside=on_focus_outside.unwrap_or(Callback::new(|_| {}))
                    on_pointer_down_outside=on_pointer_down_outside.unwrap_or(Callback::new(|_| {}))
                    on_interact_outside=on_interact_outside.unwrap_or(Callback::new(|_| {}))
                    attr:data-state=move || get_open_state(open.get())
                    style:pointer-events=move || {
                        if !open.get() && context.is_root_menu { Some("none") } else { None }
                    }
                >
                    {children.with_value(|children| children.as_ref().map(|children| children()))}
                </NavigationMenuContentImpl>
            </Presence>
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * NavigationMenuContentImpl (internal)
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub(super) fn NavigationMenuContentImpl(
    #[prop(into)] value: String,
    trigger_ref: AnyNodeRef,
    focus_proxy_ref: AnyNodeRef,
    was_escape_close_ref: RwSignal<bool>,
    on_content_focus_outside: Callback<()>,
    on_root_content_close: Callback<()>,
    #[prop(into)] on_pointer_enter: Callback<ev::PointerEvent>,
    #[prop(into)] on_pointer_leave: Callback<ev::PointerEvent>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] on_interact_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    presence_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<NavigationMenuContextValue>();
    let content_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, content_ref, presence_ref]);

    let value_clone = value.clone();
    let trigger_id = Memo::new(move |_| make_trigger_id(&context.base_id.get(), &value_clone));
    let value_clone2 = value.clone();
    let content_id = Memo::new(move |_| make_content_id(&context.base_id.get(), &value_clone2));

    let get_items = use_collection::<NavigationMenuItemData>();

    let prev_motion_attribute: StoredValue<Option<&'static str>> = StoredValue::new(None);

    // Bubble dismiss to root content node
    Effect::new(move |_| {
        if context.is_root_menu
            && let Some(content) = content_ref.get()
        {
            let content: web_sys::HtmlElement = content.unchecked_into();
            let content_el = content.clone();
            let trigger_ref_clone = trigger_ref;
            let on_item_dismiss = context.on_item_dismiss;

            let handler = Closure::<dyn Fn(web_sys::Event)>::new(move |_: web_sys::Event| {
                on_item_dismiss.run(());
                on_root_content_close.run(());
                if let Some(active) = document().active_element()
                    && content_el.contains(Some(active.unchecked_ref()))
                    && let Some(trigger) = trigger_ref_clone.get_untracked()
                {
                    let trigger: web_sys::HtmlElement = trigger.unchecked_into();
                    trigger.focus().ok();
                }
            });

            content
                .add_event_listener_with_callback(
                    ROOT_CONTENT_DISMISS,
                    handler.as_ref().unchecked_ref(),
                )
                .ok();

            let content_for_cleanup = SendWrapper::new(content.clone());
            let handler_for_cleanup = SendWrapper::new(handler);

            Owner::on_cleanup(move || {
                content_for_cleanup
                    .remove_event_listener_with_callback(
                        ROOT_CONTENT_DISMISS,
                        handler_for_cleanup.as_ref().unchecked_ref(),
                    )
                    .ok();
            });
        }
    });

    let value_for_motion = value.clone();
    let motion_attribute = Memo::new(move |_| {
        let items = get_items();
        let mut values: Vec<String> = items.iter().map(|item| item.data.value.clone()).collect();
        if context.dir.get() == Direction::Rtl {
            values.reverse();
        }
        let current_value = context.value.get();
        let previous_value = context.previous_value.get();

        let attribute = compute_motion_attribute(
            &values,
            &current_value,
            &previous_value,
            &value_for_motion,
            prev_motion_attribute.get_value(),
        );

        prev_motion_attribute.set_value(attribute);
        attribute
    });

    let composed_on_escape = Callback::new(move |event: web_sys::KeyboardEvent| {
        if let Some(cb) = on_escape_key_down {
            cb.run(event);
        }
        was_escape_close_ref.set(true);
    });

    let composed_on_focus_outside = Callback::new(move |event: web_sys::CustomEvent| {
        on_content_focus_outside.run(());
        if let Some(cb) = on_focus_outside {
            cb.run(event.clone());
        }
        // Only dismiss content when focus moves outside of the menu
        if let Some(target) = event
            .target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
            && let Some(root) = context.root_navigation_menu.get_untracked()
        {
            let root: web_sys::Node = root.unchecked_into();
            if root.contains(Some(target.unchecked_ref())) {
                event.prevent_default();
            }
        }
    });

    let get_items_for_pdo = use_collection::<NavigationMenuItemData>();
    let composed_on_pointer_down_outside = Callback::new(move |event: web_sys::CustomEvent| {
        if let Some(cb) = on_pointer_down_outside {
            cb.run(event.clone());
        }
        // untrack: this runs in an event handler (non-reactive context), we just need
        // the current values without creating reactive subscriptions.
        untrack(|| {
            if let Some(target) = event
                .target()
                .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
            {
                let items = get_items_for_pdo();
                let is_trigger = items.iter().any(|item| {
                    item.r#ref
                        .get()
                        .map(|el| {
                            let el: web_sys::Node = el.unchecked_into();
                            el.contains(Some(target.unchecked_ref()))
                        })
                        .unwrap_or(false)
                });
                let is_root_viewport = context.is_root_menu
                    && context
                        .viewport
                        .get()
                        .map(|vp| {
                            let vp: &web_sys::Node = vp.unchecked_ref();
                            vp.contains(Some(target.unchecked_ref()))
                        })
                        .unwrap_or(false);

                if is_trigger || is_root_viewport || !context.is_root_menu {
                    event.prevent_default();
                }
            }
        });
    });

    view! {
        <FocusGroup>
            <DismissableLayer
                as_child=true
                disable_outside_pointer_events=false
                on_dismiss=Callback::new(move |_| {
                    if let Some(el) = content_ref.get_untracked() {
                        let el: web_sys::HtmlElement = el.unchecked_into();
                        let mut init = web_sys::EventInit::new();
                        init.set_bubbles(true);
                        init.set_cancelable(true);
                        let event = web_sys::Event::new_with_event_init_dict(ROOT_CONTENT_DISMISS, &init)
                            .expect("Event should be created.");
                        el.dispatch_event(&event).ok();
                    }
                })
                on_escape_key_down=composed_on_escape
                on_focus_outside=composed_on_focus_outside
                on_pointer_down_outside=composed_on_pointer_down_outside
                on_interact_outside=on_interact_outside.unwrap_or(Callback::new(|_| {}))
            >
                <Primitive
                    element=html::div
                    as_child=as_child
                    node_ref=composed_refs
                    attr:id=move || content_id.get()
                    attr:aria-labelledby=move || trigger_id.get()
                    attr:data-motion=move || motion_attribute.get()
                    attr:data-orientation=move || context.orientation.get().to_string()
                    on:pointerenter=move |event: ev::PointerEvent| {
                        on_pointer_enter.run(event);
                    }
                    on:pointerleave=move |event: ev::PointerEvent| {
                        on_pointer_leave.run(event);
                    }
                    on:keydown=move |event: ev::KeyboardEvent| {
                        let is_meta_key = event.alt_key() || event.ctrl_key() || event.meta_key();
                        let is_tab_key = event.key() == "Tab" && !is_meta_key;
                        if is_tab_key
                            && let Some(current_target) = event.current_target()
                        {
                            let current_target: web_sys::HtmlElement = current_target.unchecked_into();
                            let candidates = get_tabbable_candidates(&current_target);
                            let focused_element = document().active_element();
                            let index = candidates.iter().position(|c| {
                                focused_element.as_ref().map(|f| {
                                    c == f.unchecked_ref::<web_sys::HtmlElement>()
                                }).unwrap_or(false)
                            }).unwrap_or(0);
                            let is_moving_backwards = event.shift_key();
                            let next_candidates = if is_moving_backwards {
                                let mut slice = candidates[..index].to_vec();
                                slice.reverse();
                                slice
                            } else {
                                candidates[index + 1..].to_vec()
                            };

                            if focus_first(&next_candidates) {
                                event.prevent_default();
                            } else {
                                // Focus the proxy and let browser handle Tab
                                if let Some(proxy) = focus_proxy_ref.get() {
                                    let proxy: web_sys::HtmlElement = proxy.unchecked_into();
                                    proxy.focus().ok();
                                }
                            }
                        }
                    }
                >
                    {children.with_value(|children| children.as_ref().map(|children| children()))}
                </Primitive>
            </DismissableLayer>
        </FocusGroup>
    }
}

/* -------------------------------------------------------------------------------------------------
 * NavigationMenuLink
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn NavigationMenuLink(
    #[prop(into, optional)] active: MaybeProp<bool>,
    #[prop(into, optional)] on_select: Option<Callback<web_sys::Event>>,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <FocusGroupItem>
            <Primitive
                element=html::a
                as_child=as_child
                node_ref=node_ref
                attr:data-active=move || active.get().unwrap_or(false).then_some("")
                attr:aria-current=move || if active.get().unwrap_or(false) { Some("page") } else { None }
                on:click=compose_callbacks(
                    on_click,
                    Some(Callback::new(move |event: ev::MouseEvent| {
                        let target = event.target()
                            .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok());
                        if let Some(target) = target {
                            let mut init = web_sys::CustomEventInit::new();
                            init.set_bubbles(true);
                            init.set_cancelable(true);
                            let link_select_event = web_sys::CustomEvent::new_with_event_init_dict(
                                LINK_SELECT,
                                &init,
                            ).expect("CustomEvent should be created.");

                            if let Some(on_select) = on_select {
                                // Add one-time listener
                                let listener = Closure::once_into_js(move |event: web_sys::Event| {
                                    on_select.run(event);
                                });
                                let mut opts = web_sys::AddEventListenerOptions::new();
                                opts.set_once(true);
                                target
                                    .add_event_listener_with_callback_and_add_event_listener_options(
                                        LINK_SELECT,
                                        listener.unchecked_ref(),
                                        &opts,
                                    )
                                    .ok();
                            }

                            target.dispatch_event(&link_select_event).ok();

                            if !link_select_event.default_prevented() && !event.meta_key() {
                                let mut dismiss_init = web_sys::CustomEventInit::new();
                                dismiss_init.set_bubbles(true);
                                dismiss_init.set_cancelable(true);
                                let dismiss_event = web_sys::CustomEvent::new_with_event_init_dict(
                                    ROOT_CONTENT_DISMISS,
                                    &dismiss_init,
                                ).expect("CustomEvent should be created.");
                                target.dispatch_event(&dismiss_event).ok();
                            }
                        }
                    })),
                    Some(false),
                )
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </Primitive>
        </FocusGroupItem>
    }
}
