use super::*;

/* -------------------------------------------------------------------------------------------------
 * NavigationMenuViewport
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn NavigationMenuViewport(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<NavigationMenuContextValue>();

    // Signal viewport existence synchronously during construction so Content components
    // know to use viewport rendering before any menu open interaction occurs.
    context.has_viewport_component.set(true);
    on_cleanup(move || {
        context.has_viewport_component.set(false);
    });

    let open = Signal::derive(move || !context.value.get().is_empty());

    let present = Signal::derive(move || force_mount.get().unwrap_or(false) || open.get());
    let presence_ref = AnyNodeRef::new();

    view! {
        <Presence present=present node_ref=presence_ref>
            <NavigationMenuViewportImpl
                as_child=as_child
                node_ref=node_ref
                presence_ref=presence_ref
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </NavigationMenuViewportImpl>
        </Presence>
    }
}

/* -------------------------------------------------------------------------------------------------
 * NavigationMenuViewportImpl (internal)
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn NavigationMenuViewportImpl(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    presence_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let _children = StoredValue::new(children);

    let context = expect_context::<NavigationMenuContextValue>();
    let viewport_content_context = expect_context::<ViewportContentContextValue>();

    let viewport_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, viewport_ref, presence_ref]);

    // Register viewport element
    Effect::new(move |_| {
        if let Some(el) = viewport_ref.get() {
            let html_el: web_sys::HtmlElement = el.unchecked_into();
            context.viewport.set(Some(SendWrapper::new(html_el)));
        }
    });

    on_cleanup(move || {
        context.viewport.set(None);
    });

    let size: RwSignal<Option<(f64, f64)>> = RwSignal::new(None);
    let content_el: RwSignal<Option<SendWrapper<web_sys::HtmlElement>>> = RwSignal::new(None);

    let open = Signal::derive(move || !context.value.get().is_empty());
    let active_content_value = Memo::new(move |_| {
        if open.get() {
            context.value.get()
        } else {
            context.previous_value.get()
        }
    });

    let handle_size_change = Callback::new(move |_: ()| {
        if let Some(el) = content_el.get_untracked() {
            size.set(Some((el.offset_width() as f64, el.offset_height() as f64)));
        }
    });

    use_resize_observer(Signal::derive(move || content_el.get()), handle_size_change);

    // Set CSS custom properties via Effect
    Effect::new(move |_| {
        if let Some(vp) = viewport_ref.get() {
            let vp: web_sys::HtmlElement = vp.unchecked_into();
            let style = vp.style();
            if let Some((w, h)) = size.get() {
                let _ =
                    style.set_property("--radix-navigation-menu-viewport-width", &format!("{w}px"));
                let _ = style
                    .set_property("--radix-navigation-menu-viewport-height", &format!("{h}px"));
            }
            if !open.get() && context.is_root_menu {
                let _ = style.set_property("pointer-events", "none");
            } else {
                let _ = style.remove_property("pointer-events");
            }
        }
    });

    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref=composed_refs
            attr:data-state=move || get_open_state(open.get())
            attr:data-orientation=move || context.orientation.get().to_string()
            on:pointerenter=move |_: ev::PointerEvent| {
                context.on_content_enter.run(());
            }
            on:pointerleave=move |event: ev::PointerEvent| {
                if event.pointer_type() == "mouse" {
                    context.on_content_leave.run(());
                }
            }
        >
            {move || {
                let items = viewport_content_context.items.get();
                let active_val = active_content_value.get();
                items.iter().map(|(value, data)| {
                    let is_active = active_val == *value;
                    let present = Signal::derive({
                        let force_mount = data.force_mount;
                        move || force_mount || is_active
                    });
                    let presence_ref = AnyNodeRef::new();

                    let data_value = StoredValue::new(data.value.clone());
                    let data_trigger_ref = data.trigger_ref;
                    let data_focus_proxy_ref = data.focus_proxy_ref;
                    let data_was_escape_close_ref = data.was_escape_close_ref;
                    let data_on_content_focus_outside = data.on_content_focus_outside;
                    let data_on_root_content_close = data.on_root_content_close;
                    let data_children = StoredValue::new(data.children.clone());
                    let data_on_pointer_enter = data.on_pointer_enter;
                    let data_on_pointer_leave = data.on_pointer_leave;
                    let data_on_escape_key_down = StoredValue::new(data.on_escape_key_down);
                    let data_on_focus_outside = StoredValue::new(data.on_focus_outside);
                    let data_on_pointer_down_outside = StoredValue::new(data.on_pointer_down_outside);
                    let data_on_interact_outside = StoredValue::new(data.on_interact_outside);
                    let data_content_ref = data.content_ref;
                    let data_item_content_ref = data.item_content_ref;
                    let data_extra_attrs = StoredValue::new(data.extra_attrs.clone());

                    // Capture content element ref for viewport sizing
                    let inner_ref = AnyNodeRef::new();
                    let combined_ref = use_composed_refs(vec![data_content_ref, data_item_content_ref, inner_ref]);

                    // When active, set content_el for resize observation
                    Effect::new(move |_| {
                        if is_active
                            && let Some(el) = inner_ref.get()
                        {
                            let html_el: web_sys::HtmlElement = el.unchecked_into();
                            content_el.set(Some(SendWrapper::new(html_el)));
                        }
                    });

                    // Apply user attributes captured from NavigationMenuContent to the
                    // rendered content element. These were lost during the viewport
                    // registration process (React forwards them via {...contentProps}).
                    Effect::new(move |_| {
                        if let Some(el) = inner_ref.get() {
                            let el: web_sys::HtmlElement = el.unchecked_into();
                            data_extra_attrs.with_value(|attrs| {
                                for (name, value) in attrs {
                                    el.set_attribute(name, value).ok();
                                }
                            });
                        }
                    });

                    view! {
                        <Presence present=present node_ref=presence_ref>
                            <NavigationMenuContentImpl
                                value=data_value.get_value()
                                trigger_ref=data_trigger_ref
                                focus_proxy_ref=data_focus_proxy_ref
                                was_escape_close_ref=data_was_escape_close_ref
                                on_content_focus_outside=data_on_content_focus_outside
                                on_root_content_close=data_on_root_content_close
                                node_ref=combined_ref
                                presence_ref=presence_ref
                                on_pointer_enter=Callback::new(compose_callbacks(
                                    data_on_pointer_enter,
                                    Some(Callback::new(move |_: ev::PointerEvent| {
                                        context.on_content_enter.run(());
                                    })),
                                    None,
                                ))
                                on_pointer_leave=Callback::new(compose_callbacks(
                                    data_on_pointer_leave,
                                    Some(Callback::new(move |event: ev::PointerEvent| {
                                        if event.pointer_type() == "mouse" {
                                            context.on_content_leave.run(());
                                        }
                                    })),
                                    None,
                                ))
                                on_escape_key_down=data_on_escape_key_down.get_value().unwrap_or(Callback::new(|_| {}))
                                on_focus_outside=data_on_focus_outside.get_value().unwrap_or(Callback::new(|_| {}))
                                on_pointer_down_outside=data_on_pointer_down_outside.get_value().unwrap_or(Callback::new(|_| {}))
                                on_interact_outside=data_on_interact_outside.get_value().unwrap_or(Callback::new(|_| {}))
                                attr:data-state=move || get_open_state(is_active)
                                style:pointer-events=move || {
                                    if !is_active && context.is_root_menu { Some("none") } else { None }
                                }
                            >
                                {data_children.with_value(|c| c.as_ref().map(|c| c()))}
                            </NavigationMenuContentImpl>
                        </Presence>
                    }
                }).collect_view()
            }}
        </Primitive>
    }
}

/* -------------------------------------------------------------------------------------------------
 * FocusGroup (internal)
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub(super) fn FocusGroup(children: ChildrenFn) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<NavigationMenuContextValue>();

    view! {
        <CollectionProvider<FocusGroupItemData> item_data_type=PhantomData>
            <CollectionSlot<FocusGroupItemData> item_data_type=PhantomData>
                <Primitive
                    element=html::div
                    as_child=MaybeProp::from(true)
                    attr:dir=move || context.dir.get().to_string()
                >
                    {children.with_value(|children| children())}
                </Primitive>
            </CollectionSlot<FocusGroupItemData>>
        </CollectionProvider<FocusGroupItemData>>
    }
}

/* -------------------------------------------------------------------------------------------------
 * FocusGroupItem (internal)
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub(super) fn FocusGroupItem(children: ChildrenFn) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<NavigationMenuContextValue>();
    let get_items = StoredValue::new(use_collection::<FocusGroupItemData>());

    view! {
        <CollectionItemSlot<FocusGroupItemData>
            item_data_type=PhantomData
            item_data=FocusGroupItemData
        >
            <Primitive
                element=html::button
                as_child=MaybeProp::from(true)
                on:keydown=move |event: ev::KeyboardEvent| {
                    // Check defaultPrevented so composed handlers (e.g. trigger's ArrowDown
                    // into content) can prevent FocusGroupItem from moving focus.
                    // In React, composeEventHandlers gates on !event.defaultPrevented.
                    if event.default_prevented() {
                        return;
                    }
                    let is_focus_navigation_key = event.key() == "Home"
                        || event.key() == "End"
                        || ARROW_KEYS.contains(&event.key().as_str());
                    if is_focus_navigation_key {
                        // untrack: event handler reads signals without needing subscriptions.
                        untrack(|| {
                            let items = get_items.with_value(|gi| gi());
                            let mut candidate_nodes: Vec<web_sys::HtmlElement> = items
                                .iter()
                                .filter_map(|item| {
                                    item.r#ref.get().map(|el| el.unchecked_into())
                                })
                                .collect();

                            let prev_item_key = if context.dir.get() == Direction::Rtl {
                                "ArrowRight"
                            } else {
                                "ArrowLeft"
                            };
                            let prev_keys = [prev_item_key, "ArrowUp", "End"];
                            if prev_keys.contains(&event.key().as_str()) {
                                candidate_nodes.reverse();
                            }

                            if ARROW_KEYS.contains(&event.key().as_str())
                                && let Some(current_target) = event.current_target()
                            {
                                let current_target: web_sys::HtmlElement = current_target.unchecked_into();
                                if let Some(current_index) = candidate_nodes.iter().position(|n| *n == current_target) {
                                    candidate_nodes = candidate_nodes[current_index + 1..].to_vec();
                                }
                            }

                            // Use setTimeout to defer focus (avoid batching issues)
                            let candidates = SendWrapper::new(candidate_nodes);
                            set_timeout(move || {
                                focus_first(&candidates);
                            }, 0);
                        });

                        event.prevent_default();
                    }
                }
            >
                {children.with_value(|children| children())}
            </Primitive>
        </CollectionItemSlot<FocusGroupItemData>>
    }
}
