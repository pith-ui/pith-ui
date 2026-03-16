use super::*;

#[component]
pub fn MenuSub(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let parent_context = expect_context::<MenuContextValue>();
    let on_open_change = on_open_change.unwrap_or(Callback::new(|_| {}));
    let open = prop_or_default(open);

    let content_ref = AnyNodeRef::new();
    let trigger_ref = AnyNodeRef::new();
    let content_id = use_id(None);
    let trigger_id = use_id(None);

    let menu_context = MenuContextValue {
        open,
        content_ref,
        on_open_change,
        // Sub-menus don't have their own Popper (they use the parent's), but
        // MenuAnchor still expects this field. Use an independent ref for the
        // sub-trigger anchor.
        popper_anchor_ref: AnyNodeRef::new(),
    };

    let sub_context = MenuSubContextValue {
        content_id,
        trigger_id,
        trigger_ref,
    };

    // Close submenu when parent closes.
    Effect::new(move |_| {
        if !parent_context.open.get() {
            on_open_change.run(false);
        }
    });

    on_cleanup(move || {
        on_open_change.run(false);
    });

    view! {
        <Popper>
            <Provider value=menu_context>
                <Provider value=sub_context>
                    {children.with_value(|children| children())}
                </Provider>
            </Provider>
        </Popper>
    }
}

#[component]
pub fn MenuSubTrigger(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] text_value: MaybeProp<String>,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] on_pointer_move: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_leave: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<MenuContextValue>();
    let root_context = expect_context::<MenuRootContextValue>();
    let sub_context = expect_context::<MenuSubContextValue>();
    let content_context = expect_context::<MenuContentContextValue>();
    let open_timer: RwSignal<Option<i32>> = RwSignal::new(None);
    let disabled = prop_or_default(disabled);

    let composed_refs = use_composed_refs(vec![node_ref, sub_context.trigger_ref]);

    let clear_open_timer = move || {
        if let Some(timer_id) = open_timer.try_get_untracked().flatten() {
            window().clear_timeout_with_handle(timer_id);
            let _ = open_timer.try_set(None);
        }
    };

    on_cleanup(move || {
        clear_open_timer();
    });

    // Clean up grace area on unmount.
    on_cleanup(move || {
        window().clear_timeout_with_handle(
            content_context
                .pointer_grace_timer
                .try_get_untracked()
                .unwrap_or(0) as i32,
        );
        content_context.on_pointer_grace_intent_change.run(None);
    });

    view! {
        <MenuAnchor as_child=true>
            <MenuItemImpl
                disabled=disabled
                text_value=text_value
                as_child=as_child
                node_ref=composed_refs
                attr:id=move || sub_context.trigger_id.get()
                attr:aria-haspopup="menu"
                attr:aria-expanded=move || context.open.get().to_string()
                attr:aria-controls=move || sub_context.content_id.get()
                attr:data-state=move || open_closed_state(context.open.get())
                on:click=compose_callbacks(on_click, Some(Callback::new(move |event: ev::MouseEvent| {
                    if disabled.get_untracked() || event.default_prevented() {
                        return;
                    }
                    // We manually focus because iOS Safari doesn't always focus on click (e.g. buttons)
                    // and we rely heavily on `onFocusOutside` for submenus to close when switching
                    // between separate submenus.
                    if let Some(current_target) = event.current_target() {
                        let el: web_sys::HtmlElement = current_target.unchecked_into();
                        el.focus().ok();
                    }
                    if !context.open.get_untracked() {
                        context.on_open_change.run(true);
                    }
                })), None)
                // Pass pointer handlers as props (not `on:` directives) so they compose
                // with MenuItemImpl's internal handlers via compose_callbacks. This ensures
                // SubTrigger's grace area logic runs BEFORE MenuItemImpl's on_item_leave
                // checks it (compose_callbacks runs the prop handler first, then internal).
                on_pointer_move=Callback::new(compose_callbacks(on_pointer_move, Some(when_mouse(move |event: ev::PointerEvent| {
                    content_context.on_item_enter.run(event.clone());
                    if event.default_prevented() {
                        return;
                    }
                    if !disabled.get_untracked() && !context.open.get_untracked() && open_timer.try_get_untracked().flatten().is_none() {
                        content_context.on_pointer_grace_intent_change.run(None);
                        let timer_id = window()
                            .set_timeout_with_callback_and_timeout_and_arguments_0(
                                Closure::once_into_js(move || {
                                    context.on_open_change.run(true);
                                    let _ = open_timer.try_set(None);
                                })
                                .unchecked_ref(),
                                100,
                            )
                            .expect("Timeout should be set.");
                        let _ = open_timer.try_set(Some(timer_id));
                    }
                })), None))
                on_pointer_leave=Callback::new(compose_callbacks(on_pointer_leave, Some(when_mouse(move |event: ev::PointerEvent| {
                    clear_open_timer();

                    if let Some(content) = context.content_ref.get_untracked() {
                        let content_el: web_sys::HtmlElement = content.unchecked_into();
                        let content_rect = content_el.get_bounding_client_rect();
                        let data_side = content_el
                            .unchecked_ref::<web_sys::Element>()
                            .get_attribute("data-side");
                        let right_side = data_side.as_deref() == Some("right");
                        let bleed = if right_side { -5.0 } else { 5.0 };
                        let content_near_edge = if right_side {
                            content_rect.left()
                        } else {
                            content_rect.right()
                        };
                        let content_far_edge = if right_side {
                            content_rect.right()
                        } else {
                            content_rect.left()
                        };

                        content_context.on_pointer_grace_intent_change.run(Some(GraceIntent {
                            area: vec![
                                Point { x: event.client_x() as f64 + bleed, y: event.client_y() as f64 },
                                Point { x: content_near_edge, y: content_rect.top() },
                                Point { x: content_far_edge, y: content_rect.top() },
                                Point { x: content_far_edge, y: content_rect.bottom() },
                                Point { x: content_near_edge, y: content_rect.bottom() },
                            ],
                            side: if right_side { Side::Right } else { Side::Left },
                        }));

                        window().clear_timeout_with_handle(content_context.pointer_grace_timer.try_get_untracked().unwrap_or(0) as i32);
                        let timer_id = window()
                            .set_timeout_with_callback_and_timeout_and_arguments_0(
                                Closure::once_into_js(move || {
                                    content_context.on_pointer_grace_intent_change.run(None);
                                })
                                .unchecked_ref(),
                                300,
                            )
                            .expect("Timeout should be set.");
                        let _ = content_context.pointer_grace_timer.try_set(timer_id as u64);
                    } else {
                        content_context.on_trigger_leave.run(event.clone());
                        if event.default_prevented() {
                            return;
                        }
                        // There's 100ms where the user may leave an item before the submenu was opened.
                        content_context.on_pointer_grace_intent_change.run(None);
                    }
                })), None))
                on:keydown=compose_callbacks(on_key_down, Some(Callback::new(move |event: ev::KeyboardEvent| {
                    let is_typing_ahead = !content_context.search.try_get_untracked().unwrap_or_default().is_empty();
                    if disabled.get_untracked() || (is_typing_ahead && event.key() == " ") {
                        return;
                    }
                    if sub_open_keys(root_context.dir.get_untracked()).contains(&event.key().as_str()) {
                        context.on_open_change.run(true);
                        // Focus the first menu item in the submenu via RAF because:
                        // 1. The content element may not be fully mounted/registered yet.
                        // 2. When Presence keeps the element in DOM during exit animation
                        //    and the submenu is reopened, FocusScope's on_mount_auto_focus
                        //    Effect won't re-fire (container ref unchanged), so we must
                        //    handle first-item focus ourselves.
                        if let Some(content) = context.content_ref.get_untracked() {
                            let content: web_sys::HtmlElement = content.unchecked_into();
                            let cb = Closure::once_into_js(move || {
                                let selector = "[role=menuitem]:not([data-disabled]), \
                                                 [role=menuitemcheckbox]:not([data-disabled]), \
                                                 [role=menuitemradio]:not([data-disabled])";
                                if let Ok(Some(first_item)) = content.query_selector(selector) {
                                    let first: web_sys::HtmlElement = first_item.unchecked_into();
                                    first.focus().ok();
                                } else {
                                    content.focus().ok();
                                }
                            });
                            window().request_animation_frame(cb.unchecked_ref()).ok();
                        }
                        event.prevent_default();
                    }
                })), None)
            >
                {children.with_value(|children| children())}
            </MenuItemImpl>
        </MenuAnchor>
    }
}

#[component]
pub fn MenuSubContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] side_offset: MaybeProp<f64>,
    #[prop(into, optional)] align_offset: MaybeProp<f64>,
    #[prop(into, optional)] avoid_collisions: MaybeProp<bool>,
    #[prop(into, optional)] collision_boundary: MaybeProp<SendWrapper<Vec<web_sys::Element>>>,
    #[prop(into, optional)] collision_padding: MaybeProp<Padding>,
    #[prop(into, optional)] arrow_padding: MaybeProp<f64>,
    #[prop(into, optional)] sticky: MaybeProp<Sticky>,
    #[prop(into, optional)] hide_when_detached: MaybeProp<bool>,
    /// Whether keyboard navigation should loop around.
    #[prop(into, optional)]
    r#loop: MaybeProp<bool>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<CustomEvent>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    /// CSS class applied directly to the inner content element (same element as data-state).
    #[prop(into, optional)]
    class: MaybeProp<String>,
    /// Additional inline styles for the content element (e.g., CSS custom property aliases).
    #[prop(into, optional)]
    content_style: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<MenuContextValue>();
    let root_context = expect_context::<MenuRootContextValue>();
    let sub_context = expect_context::<MenuSubContextValue>();
    let content_ref = AnyNodeRef::new();
    // Include context.content_ref so MenuSubTrigger can focus the content element via context.
    let composed_refs = use_composed_refs(vec![node_ref, content_ref, context.content_ref]);

    let present = Signal::derive(move || force_mount.get().unwrap_or(false) || context.open.get());

    let sub_side = Signal::derive(move || match root_context.dir.get() {
        Direction::Rtl => PopperSide::Left,
        Direction::Ltr => PopperSide::Right,
    });

    // Forward user attrs through CollectionProvider/Presence to the content element.
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <CollectionProvider item_data_type=ITEM_DATA_PHANTHOM>
                <Presence present=present node_ref=composed_refs>
                    <CollectionSlot item_data_type=ITEM_DATA_PHANTHOM>
                        <MenuContentImpl
                        {..forwarded.spread()}
                        id=Signal::derive(move || Some(sub_context.content_id.get()))
                        aria_labelledby=Signal::derive(move || Some(sub_context.trigger_id.get()))
                        side=sub_side
                        side_offset=side_offset
                        align=Align::Start
                        align_offset=align_offset
                        avoid_collisions=avoid_collisions
                        collision_boundary=collision_boundary
                        collision_padding=collision_padding
                        arrow_padding=arrow_padding
                        sticky=sticky
                        hide_when_detached=hide_when_detached
                        r#loop=r#loop
                        disable_outside_pointer_events=false
                        disable_outside_scroll=false
                        trap_focus=false
                        class=class
                        content_style=content_style
                        as_child=as_child
                        node_ref=composed_refs
                        on_open_auto_focus=Callback::new(move |event: ev::Event| {
                            // When opening a submenu, focus content for keyboard users only.
                            // We defer to RAF because the content element and its children may not
                            // be fully mounted/registered when this callback fires (Leptos Effect
                            // timing differs from React's useEffect). We focus the first focusable
                            // menu item directly rather than the content element, bypassing
                            // RovingFocusGroup's entry focus which depends on collection items
                            // being registered (a timing-sensitive operation in Leptos).
                            if root_context.is_using_keyboard.get_untracked() {
                                let content_ref = content_ref;
                                let cb = Closure::once_into_js(move || {
                                    if let Some(content) = content_ref.get_untracked() {
                                        let el: &web_sys::HtmlElement = content.unchecked_ref();
                                        // Focus first non-disabled item directly.
                                        let selector = "[role=menuitem]:not([data-disabled]), \
                                                         [role=menuitemcheckbox]:not([data-disabled]), \
                                                         [role=menuitemradio]:not([data-disabled])";
                                        if let Ok(Some(first_item)) = el.query_selector(selector) {
                                            let first: web_sys::HtmlElement = first_item.unchecked_into();
                                            first.focus().ok();
                                        } else {
                                            // Fallback: focus the content element itself.
                                            el.focus().ok();
                                        }
                                    }
                                });
                                window().request_animation_frame(cb.unchecked_ref()).ok();
                            }
                            event.prevent_default();
                        })
                        on_close_auto_focus=Callback::new(move |event: ev::Event| {
                            // The menu might close because of focusing another menu item in the parent menu. We
                            // don't want it to refocus the trigger in that case so we handle trigger focus ourselves.
                            event.prevent_default();
                        })
                        on_focus_outside=compose_callbacks(on_focus_outside, Some(Callback::new(move |event: CustomEvent| {
                            // We prevent closing when the trigger is focused to avoid triggering a re-open animation
                            // on pointer interaction.
                            let target = event.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok());
                            let trigger = sub_context.trigger_ref.get_untracked().map(|n| {
                                let el: web_sys::Element = n.unchecked_into();
                                el
                            });
                            if target.as_ref() != trigger.as_ref() {
                                context.on_open_change.run(false);
                            }
                        })), Some(false))
                        on_escape_key_down=compose_callbacks(on_escape_key_down, Some(Callback::new(move |event: ev::KeyboardEvent| {
                            root_context.on_close.run(());
                            // Ensure pressing escape in submenu doesn't escape full screen mode.
                            event.prevent_default();
                        })), None)
                        on_key_down=compose_callbacks(on_key_down, Some(Callback::new(move |event: ev::KeyboardEvent| {
                            // Submenu key events bubble through portals. We only care about keys in this menu.
                            let is_key_down_inside = event.current_target()
                                .and_then(|ct| ct.dyn_into::<web_sys::Node>().ok())
                                .zip(event.target().and_then(|t| t.dyn_into::<web_sys::Node>().ok()))
                                .is_some_and(|(ct, target)| ct.contains(Some(&target)));
                            let is_close_key = sub_close_keys(root_context.dir.get_untracked()).contains(&event.key().as_str());
                            if is_key_down_inside && is_close_key {
                                context.on_open_change.run(false);
                                // We focus manually because we prevented it in `on_close_auto_focus`.
                                if let Some(trigger) = sub_context.trigger_ref.get_untracked() {
                                    let trigger: web_sys::HtmlElement = trigger.unchecked_into();
                                    trigger.focus().ok();
                                }
                                event.prevent_default();
                            }
                        })), None)
                    >
                        {children.with_value(|children| children())}
                    </MenuContentImpl>
                </CollectionSlot>
            </Presence>
        </CollectionProvider>
        </AttributeInterceptor>
    }
}
