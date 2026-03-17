use super::*;

/* -------------------------------------------------------------------------------------------------
 * MenubarPortal
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn MenubarPortal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    #[prop(optional)] container_ref: AnyNodeRef,
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <MenuPortal container=container container_ref=container_ref force_mount=force_mount>
            {children.with_value(|children| children())}
        </MenuPortal>
    }
}

/* -------------------------------------------------------------------------------------------------
 * MenubarContent
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn MenubarContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] align: MaybeProp<Align>,
    #[prop(into, optional)] align_offset: MaybeProp<f64>,
    #[prop(into, optional)] avoid_collisions: MaybeProp<bool>,
    #[prop(into, optional)] collision_boundary: MaybeProp<SendWrapper<Vec<web_sys::Element>>>,
    #[prop(into, optional)] collision_padding: MaybeProp<Padding>,
    #[prop(into, optional)] arrow_padding: MaybeProp<f64>,
    #[prop(into, optional)] sticky: MaybeProp<Sticky>,
    #[prop(into, optional)] hide_when_detached: MaybeProp<bool>,
    #[prop(into, optional)] r#loop: MaybeProp<bool>,
    #[prop(into, optional)] side: MaybeProp<PopperSide>,
    #[prop(into, optional)] side_offset: MaybeProp<f64>,
    #[prop(into, optional)] on_close_auto_focus: Option<Callback<ev::Event>>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] on_interact_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let context = expect_context::<MenubarContextValue>();
    let menu_context = expect_context::<MenubarMenuContextValue>();
    // Get the collection items at this scope level (before MenuContent provides its own).
    let get_items = StoredValue::new(use_collection::<ItemData>());

    let has_interacted_outside_ref = SendWrapper::new(Rc::new(Cell::new(false)));

    let composed_refs = node_ref;

    let align = prop_or(align, Align::Start);

    // Wrap pass-through callbacks for view! macro forwarding.
    let on_escape_key_down = wrap_callback(on_escape_key_down);
    let on_pointer_down_outside = wrap_callback(on_pointer_down_outside);

    let was_keyboard = menu_context.was_keyboard_trigger_open_ref.clone();
    let menu_value = menu_context.value.clone();

    view! {
        <MenuContent
            force_mount=force_mount
            as_child=as_child
            node_ref=composed_refs
            id=Signal::derive(move || Some(menu_context.content_id.get()))
            aria_labelledby=Signal::derive(move || Some(menu_context.trigger_id.get()))
            attr:data-radix-menubar-content=""
            align=align
            align_offset=align_offset
            avoid_collisions=avoid_collisions
            collision_boundary=collision_boundary
            collision_padding=collision_padding
            arrow_padding=arrow_padding
            sticky=sticky
            hide_when_detached=hide_when_detached
            r#loop=r#loop
            side=side
            side_offset=side_offset
            style:--radix-menubar-content-transform-origin="var(--radix-popper-transform-origin)"
            style:--radix-menubar-content-available-width="var(--radix-popper-available-width)"
            style:--radix-menubar-content-available-height="var(--radix-popper-available-height)"
            style:--radix-menubar-trigger-width="var(--radix-popper-anchor-width)"
            style:--radix-menubar-trigger-height="var(--radix-popper-anchor-height)"
            on_close_auto_focus=compose_callbacks(
                on_close_auto_focus,
                Some(Callback::new({
                    let has_interacted_outside_ref = has_interacted_outside_ref.clone();
                    let trigger_ref = menu_context.trigger_ref;
                    move |event: ev::Event| {
                        let menubar_open = !context.value.get_untracked().is_empty();
                        if !menubar_open
                            && !has_interacted_outside_ref.get()
                            && let Some(trigger) = trigger_ref.get_untracked()
                        {
                            let el: web_sys::HtmlElement = trigger.unchecked_into();
                            el.focus().ok();
                        }
                        has_interacted_outside_ref.set(false);
                        // Always prevent auto focus because we either focus manually or want user agent focus
                        event.prevent_default();
                    }
                })),
                None,
            )
            on_focus_outside=compose_callbacks(
                on_focus_outside,
                Some(Callback::new(move |event: web_sys::CustomEvent| {
                    if let Some(target) = event.target() {
                        let target: web_sys::Element = target.unchecked_into();
                        let is_menubar_trigger = get_items.with_value(|get_items| {
                            get_items().iter().any(|item| {
                                item.r#ref
                                    .get_untracked()
                                    .map(|el| {
                                        let node: &web_sys::Node = (*el).unchecked_ref();
                                        node.contains(Some(&target))
                                    })
                                    .unwrap_or(false)
                            })
                        });
                        if is_menubar_trigger {
                            event.prevent_default();
                        }
                    }
                })),
                Some(false),
            )
            on_interact_outside=compose_callbacks(
                on_interact_outside,
                Some(Callback::new({
                    let has_interacted_outside_ref = has_interacted_outside_ref.clone();
                    move |_event: web_sys::CustomEvent| {
                        has_interacted_outside_ref.set(true);
                    }
                })),
                None,
            )
            on_entry_focus=Callback::new({
                let was_keyboard = was_keyboard.clone();
                move |event: ev::Event| {
                    if !was_keyboard.get() {
                        event.prevent_default();
                    }
                }
            })
            on_key_down=compose_callbacks(
                on_key_down,
                Some(Callback::new({
                    let menu_value = menu_value.clone();
                    move |event: ev::KeyboardEvent| {
                        if event.key() == "ArrowRight" || event.key() == "ArrowLeft" {
                            let Some(target) = event.target() else { return; };
                            let target: web_sys::Element = target.unchecked_into();
                            let target_is_sub_trigger = target.has_attribute("data-radix-menubar-subtrigger");
                            let Some(current_target) = event.current_target() else { return; };
                            let current_target: web_sys::Element = current_target.unchecked_into();
                            let is_key_down_inside_sub_menu = target
                                .closest("[data-radix-menubar-content]")
                                .ok()
                                .flatten()
                                .map(|el| el != current_target)
                                .unwrap_or(false);

                            let prev_menu_key = if context.dir.get_untracked() == Direction::Rtl {
                                "ArrowRight"
                            } else {
                                "ArrowLeft"
                            };
                            let is_prev_key = prev_menu_key == event.key();
                            let is_next_key = !is_prev_key;

                            // Prevent navigation when we're opening a submenu
                            if is_next_key && target_is_sub_trigger {
                                return;
                            }
                            // or we're inside a submenu and are moving backwards to close it
                            if is_key_down_inside_sub_menu && is_prev_key {
                                return;
                            }

                            let candidate_values = get_items.with_value(|get_items| {
                                let items = get_items();
                                let items: Vec<&CollectionItemValue<ItemData>> =
                                    items.iter().filter(|item| !item.data.disabled).collect();
                                let mut candidate_values: Vec<String> =
                                    items.iter().map(|item| item.data.value.clone()).collect();
                                if is_prev_key {
                                    candidate_values.reverse();
                                }

                                let current_index = candidate_values
                                    .iter()
                                    .position(|v| *v == menu_value)
                                    .unwrap_or(0);

                                if context.r#loop.get_untracked() {
                                    wrap_array(&mut candidate_values, current_index + 1).to_vec()
                                } else {
                                    candidate_values.split_off(current_index + 1)
                                }
                            });

                            if let Some(next_value) = candidate_values.first() {
                                context.on_menu_open.run(next_value.clone());
                            }
                        }
                    }
                })),
                Some(false),
            )
            on_escape_key_down=on_escape_key_down
            on_pointer_down_outside=on_pointer_down_outside
        >
            {children()}
        </MenuContent>
    }
}
