use super::*;

#[component]
pub fn MenuContent(
    /// Used to force mounting when more control is needed. Useful when controlling animation with animation libraries.
    #[prop(into, optional)]
    force_mount: MaybeProp<bool>,
    /// CSS class applied directly to the inner content element (same element as data-state).
    #[prop(into, optional)]
    class: MaybeProp<String>,
    /// Event handler called when auto-focusing on close. Can be prevented.
    #[prop(into, optional)]
    on_close_auto_focus: Option<Callback<ev::Event>>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<CustomEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<CustomEvent>>,
    #[prop(into, optional)] on_interact_outside: Option<Callback<CustomEvent>>,
    /// Event handler called when the content receives initial focus. Can be prevented.
    #[prop(into, optional)]
    on_entry_focus: Option<Callback<ev::Event>>,
    /// Event handler called on keydown events on the content element.
    #[prop(into, optional)]
    on_key_down: Option<Callback<ev::KeyboardEvent>>,
    /// The preferred side of the trigger to render against when open.
    #[prop(into, optional)]
    side: MaybeProp<PopperSide>,
    /// The distance in pixels from the trigger.
    #[prop(into, optional)]
    side_offset: MaybeProp<f64>,
    /// The preferred alignment against the trigger.
    #[prop(into, optional)]
    align: MaybeProp<Align>,
    /// An offset in pixels from the "start" or "end" alignment options.
    #[prop(into, optional)]
    align_offset: MaybeProp<f64>,
    /// When `true`, overrides the `side` and `align` preferences to prevent collisions with boundary edges.
    #[prop(into, optional)]
    avoid_collisions: MaybeProp<bool>,
    /// The element(s) used as the collision boundary.
    #[prop(into, optional)]
    collision_boundary: MaybeProp<SendWrapper<Vec<web_sys::Element>>>,
    /// The padding between the boundary edges and the content.
    #[prop(into, optional)]
    collision_padding: MaybeProp<Padding>,
    /// The padding between the arrow and the edges of the content.
    #[prop(into, optional)]
    arrow_padding: MaybeProp<f64>,
    /// The sticky behavior on the align axis.
    #[prop(into, optional)]
    sticky: MaybeProp<Sticky>,
    /// Whether the content should be hidden when detached from its reference element.
    #[prop(into, optional)]
    hide_when_detached: MaybeProp<bool>,
    /// Whether keyboard navigation should loop around.
    #[prop(into, optional)]
    r#loop: MaybeProp<bool>,
    /// The id of the content element.
    #[prop(into, optional)]
    id: MaybeProp<String>,
    /// The id of the element that labels the content.
    #[prop(into, optional)]
    aria_labelledby: MaybeProp<String>,
    /// Additional inline styles to apply to the content element. Used by wrapper components
    /// (e.g., ContextMenuContent) to set CSS custom property aliases on the final rendered element.
    #[prop(into, optional)]
    content_style: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<MenuContextValue>();
    let root_context = expect_context::<MenuRootContextValue>();

    let present = Signal::derive(move || force_mount.get().unwrap_or(false) || context.open.get());

    // Wrap Option<Callback<T>> → Callback<T> for forwarding through view! macro.
    let on_close_auto_focus = wrap_callback(on_close_auto_focus);
    let on_escape_key_down = wrap_callback(on_escape_key_down);
    let on_pointer_down_outside = wrap_callback(on_pointer_down_outside);
    let on_focus_outside = wrap_callback(on_focus_outside);
    let on_interact_outside = wrap_callback(on_interact_outside);
    let on_entry_focus = wrap_callback(on_entry_focus);
    let on_key_down = wrap_callback(on_key_down);

    view! {
        <CollectionProvider item_data_type=ITEM_DATA_PHANTHOM>
            <Presence present=present node_ref=node_ref>
                <CollectionSlot item_data_type=ITEM_DATA_PHANTHOM>
                    <Show
                        when=move || root_context.modal.get()
                        fallback=move || view! {
                            <MenuRootContentNonModal
                                class=class
                                content_style=content_style
                                on_close_auto_focus=on_close_auto_focus
                                on_escape_key_down=on_escape_key_down
                                on_pointer_down_outside=on_pointer_down_outside
                                on_focus_outside=on_focus_outside
                                on_interact_outside=on_interact_outside
                                on_entry_focus=on_entry_focus
                                on_key_down=on_key_down
                                side=side
                                side_offset=side_offset
                                align=align
                                align_offset=align_offset
                                avoid_collisions=avoid_collisions
                                collision_boundary=collision_boundary
                                collision_padding=collision_padding
                                arrow_padding=arrow_padding
                                sticky=sticky
                                hide_when_detached=hide_when_detached
                                r#loop=r#loop
                                id=id
                                aria_labelledby=aria_labelledby
                                as_child=as_child
                                node_ref=node_ref
                            >
                                {children.with_value(|children| children())}
                            </MenuRootContentNonModal>
                        }
                    >
                        <MenuRootContentModal
                            class=class
                            content_style=content_style
                            on_close_auto_focus=on_close_auto_focus
                            on_escape_key_down=on_escape_key_down
                            on_pointer_down_outside=on_pointer_down_outside
                            on_focus_outside=on_focus_outside
                            on_interact_outside=on_interact_outside
                            on_entry_focus=on_entry_focus
                            on_key_down=on_key_down
                            side=side
                            side_offset=side_offset
                            align=align
                            align_offset=align_offset
                            avoid_collisions=avoid_collisions
                            collision_boundary=collision_boundary
                            collision_padding=collision_padding
                            arrow_padding=arrow_padding
                            sticky=sticky
                            hide_when_detached=hide_when_detached
                            r#loop=r#loop
                            id=id
                            aria_labelledby=aria_labelledby
                            as_child=as_child
                            node_ref=node_ref
                        >
                            {children.with_value(|children| children())}
                        </MenuRootContentModal>
                    </Show>
                </CollectionSlot>
            </Presence>
        </CollectionProvider>
    }
}

#[component]
pub(super) fn MenuRootContentModal(
    #[prop(into, optional)] on_close_auto_focus: Option<Callback<ev::Event>>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<CustomEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<CustomEvent>>,
    #[prop(into, optional)] on_interact_outside: Option<Callback<CustomEvent>>,
    #[prop(into, optional)] on_entry_focus: Option<Callback<ev::Event>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] side: MaybeProp<PopperSide>,
    #[prop(into, optional)] side_offset: MaybeProp<f64>,
    #[prop(into, optional)] align: MaybeProp<Align>,
    #[prop(into, optional)] align_offset: MaybeProp<f64>,
    #[prop(into, optional)] avoid_collisions: MaybeProp<bool>,
    #[prop(into, optional)] collision_boundary: MaybeProp<SendWrapper<Vec<web_sys::Element>>>,
    #[prop(into, optional)] collision_padding: MaybeProp<Padding>,
    #[prop(into, optional)] arrow_padding: MaybeProp<f64>,
    #[prop(into, optional)] sticky: MaybeProp<Sticky>,
    #[prop(into, optional)] hide_when_detached: MaybeProp<bool>,
    #[prop(into, optional)] r#loop: MaybeProp<bool>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] content_style: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let context = expect_context::<MenuContextValue>();
    let content_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, content_ref]);

    // Hide everything from ARIA except the `MenuContent`.
    let hidden_elements: RwSignal<Vec<SendWrapper<web_sys::Element>>> = RwSignal::new(Vec::new());

    Effect::new(move |_| {
        if let Some(content) = content_ref.get() {
            let content: web_sys::HtmlElement = content.unchecked_into();
            hide_others(&content, hidden_elements);
        }
    });

    on_cleanup(move || {
        unhide_others(hidden_elements);
    });

    // Wrap for forwarding through view! macro.
    let on_close_auto_focus = wrap_callback(on_close_auto_focus);
    let on_escape_key_down = wrap_callback(on_escape_key_down);
    let on_pointer_down_outside = wrap_callback(on_pointer_down_outside);
    let on_interact_outside = wrap_callback(on_interact_outside);
    let on_entry_focus = wrap_callback(on_entry_focus);
    let on_key_down = wrap_callback(on_key_down);

    view! {
        <MenuContentImpl
            // We make sure we're not trapping once it's been closed (closed != unmounted when animating out).
            trap_focus=context.open
            // Make sure to only disable pointer events when open. This avoids blocking interactions while animating out.
            disable_outside_pointer_events=context.open
            disable_outside_scroll=true
            // When focus is trapped, a `focusout` event may still happen. We make sure we don't trigger our `on_dismiss` in such case.
            on_focus_outside=compose_callbacks(on_focus_outside, Some(Callback::new(move |event: CustomEvent| {
                event.prevent_default();
            })), Some(false))
            on_close_auto_focus=on_close_auto_focus
            on_escape_key_down=on_escape_key_down
            on_pointer_down_outside=on_pointer_down_outside
            on_interact_outside=on_interact_outside
            on_entry_focus=on_entry_focus
            on_key_down=on_key_down
            on_dismiss=move |_| context.on_open_change.run(false)
            side=side
            side_offset=side_offset
            align=align
            align_offset=align_offset
            avoid_collisions=avoid_collisions
            collision_boundary=collision_boundary
            collision_padding=collision_padding
            arrow_padding=arrow_padding
            sticky=sticky
            hide_when_detached=hide_when_detached
            r#loop=r#loop
            id=id
            aria_labelledby=aria_labelledby
            class=class
            content_style=content_style
            as_child=as_child
            node_ref=composed_refs
        >
            {children()}
        </MenuContentImpl>
    }
}

#[component]
pub(super) fn MenuRootContentNonModal(
    #[prop(into, optional)] on_close_auto_focus: Option<Callback<ev::Event>>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<CustomEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<CustomEvent>>,
    #[prop(into, optional)] on_interact_outside: Option<Callback<CustomEvent>>,
    #[prop(into, optional)] on_entry_focus: Option<Callback<ev::Event>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] side: MaybeProp<PopperSide>,
    #[prop(into, optional)] side_offset: MaybeProp<f64>,
    #[prop(into, optional)] align: MaybeProp<Align>,
    #[prop(into, optional)] align_offset: MaybeProp<f64>,
    #[prop(into, optional)] avoid_collisions: MaybeProp<bool>,
    #[prop(into, optional)] collision_boundary: MaybeProp<SendWrapper<Vec<web_sys::Element>>>,
    #[prop(into, optional)] collision_padding: MaybeProp<Padding>,
    #[prop(into, optional)] arrow_padding: MaybeProp<f64>,
    #[prop(into, optional)] sticky: MaybeProp<Sticky>,
    #[prop(into, optional)] hide_when_detached: MaybeProp<bool>,
    #[prop(into, optional)] r#loop: MaybeProp<bool>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] content_style: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let context = expect_context::<MenuContextValue>();

    // Wrap for forwarding through view! macro.
    let on_close_auto_focus = wrap_callback(on_close_auto_focus);
    let on_escape_key_down = wrap_callback(on_escape_key_down);
    let on_pointer_down_outside = wrap_callback(on_pointer_down_outside);
    let on_focus_outside = wrap_callback(on_focus_outside);
    let on_interact_outside = wrap_callback(on_interact_outside);
    let on_entry_focus = wrap_callback(on_entry_focus);
    let on_key_down = wrap_callback(on_key_down);

    view! {
        <MenuContentImpl
            trap_focus=false
            disable_outside_pointer_events=false
            disable_outside_scroll=false
            on_close_auto_focus=on_close_auto_focus
            on_escape_key_down=on_escape_key_down
            on_pointer_down_outside=on_pointer_down_outside
            on_focus_outside=on_focus_outside
            on_interact_outside=on_interact_outside
            on_entry_focus=on_entry_focus
            on_key_down=on_key_down
            on_dismiss=move |_| context.on_open_change.run(false)
            side=side
            side_offset=side_offset
            align=align
            align_offset=align_offset
            avoid_collisions=avoid_collisions
            collision_boundary=collision_boundary
            collision_padding=collision_padding
            arrow_padding=arrow_padding
            sticky=sticky
            hide_when_detached=hide_when_detached
            r#loop=r#loop
            id=id
            aria_labelledby=aria_labelledby
            class=class
            content_style=content_style
            as_child=as_child
            node_ref=node_ref
        >
            {children()}
        </MenuContentImpl>
    }
}

#[component]
pub(super) fn MenuContentImpl(
    /// Event handler called when auto-focusing on open. Can be prevented.
    #[prop(into, optional)]
    on_open_auto_focus: Option<Callback<ev::Event>>,
    /// Event handler called when auto-focusing on close. Can be prevented.
    #[prop(into, optional)]
    on_close_auto_focus: Option<Callback<ev::Event>>,
    #[prop(into, optional)] disable_outside_pointer_events: MaybeProp<bool>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<CustomEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<CustomEvent>>,
    #[prop(into, optional)] on_interact_outside: Option<Callback<CustomEvent>>,
    #[prop(into, optional)] on_dismiss: Option<Callback<()>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_blur: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] on_pointer_move: Option<Callback<ev::PointerEvent>>,
    /// Whether scrolling outside the `MenuContent` should be prevented. Defaults to `false`.
    #[prop(into, optional)]
    disable_outside_scroll: MaybeProp<bool>,
    /// Whether focus should be trapped within the `MenuContent`. Defaults to `false`.
    #[prop(into, optional)]
    trap_focus: MaybeProp<bool>,
    /// Whether keyboard navigation should loop around. Defaults to `false`.
    #[prop(into, optional)]
    r#loop: MaybeProp<bool>,
    #[prop(into, optional)] on_entry_focus: Option<Callback<ev::Event>>,
    /// The preferred side of the trigger to render against when open. Forwarded to PopperContent.
    #[prop(into, optional)]
    side: MaybeProp<PopperSide>,
    /// The distance in pixels from the trigger. Forwarded to PopperContent.
    #[prop(into, optional)]
    side_offset: MaybeProp<f64>,
    /// The preferred alignment against the trigger. Forwarded to PopperContent.
    #[prop(into, optional)]
    align: MaybeProp<Align>,
    /// An offset in pixels from the "start" or "end" alignment options. Forwarded to PopperContent.
    #[prop(into, optional)]
    align_offset: MaybeProp<f64>,
    /// When `true`, overrides the `side` and `align` preferences to prevent collisions with boundary edges.
    #[prop(into, optional)]
    avoid_collisions: MaybeProp<bool>,
    /// The element(s) used as the collision boundary.
    #[prop(into, optional)]
    collision_boundary: MaybeProp<SendWrapper<Vec<web_sys::Element>>>,
    /// The padding between the boundary edges and the content.
    #[prop(into, optional)]
    collision_padding: MaybeProp<Padding>,
    /// The padding between the arrow and the edges of the content.
    #[prop(into, optional)]
    arrow_padding: MaybeProp<f64>,
    /// The sticky behavior on the align axis.
    #[prop(into, optional)]
    sticky: MaybeProp<Sticky>,
    /// Whether the content should be hidden when detached from its reference element.
    #[prop(into, optional)]
    hide_when_detached: MaybeProp<bool>,
    /// The id of the content element.
    #[prop(into, optional)]
    id: MaybeProp<String>,
    /// The id of the element that labels the content.
    #[prop(into, optional)]
    aria_labelledby: MaybeProp<String>,
    /// CSS class applied directly to the inner content element (same element as data-state).
    /// Use this instead of `attr:class` for reliable reactive class updates.
    #[prop(into, optional)]
    class: MaybeProp<String>,
    /// Additional inline styles for the content element (e.g., CSS custom property aliases).
    #[prop(into, optional)]
    content_style: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let r#loop = prop_or_default(r#loop);

    let context = expect_context::<MenuContextValue>();
    let root_context = expect_context::<MenuRootContextValue>();
    let get_items = StoredValue::new(use_collection::<ItemData>());
    let (current_item_id, set_current_item_id) = signal::<Option<String>>(None);
    let content_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, content_ref]);
    let composed_refs = use_internal_styles(composed_refs, &[("outline", "none")]);
    let timer = RwSignal::new(0);
    let search = RwSignal::new("".to_string());
    let pointer_grace_timer = RwSignal::new(0);
    let pointer_grace_intent: RwSignal<Option<GraceIntent>> = RwSignal::new(None);
    let pointer_dir = RwSignal::new(Side::Right);
    let last_pointer_x = RwSignal::new(0);

    let clear_search = StoredValue::new(SendWrapper::new(Closure::<dyn Fn()>::new(move || {
        let _ = search.try_set("".into());
        window().clear_timeout_with_handle(timer.try_get_untracked().unwrap_or(0));
    })));

    let handle_typeahead_search = Callback::new(move |key: String| {
        let search_value = search.try_get_untracked().unwrap_or_default() + &key;
        let items = get_items
            .try_with_value(|get_items| get_items())
            .unwrap_or_default();
        let items = items
            .iter()
            .filter(|item| !item.data.disabled)
            .collect::<Vec<_>>();
        let current_item = document().active_element();
        let current_match = items
            .iter()
            .find(|item| {
                item.r#ref
                    .get_untracked()
                    .map(|node| {
                        let element: web_sys::Element = node.unchecked_into();
                        element
                    })
                    .as_ref()
                    == current_item.as_ref()
            })
            .map(|item| item.data.text_value.clone());
        let values = items
            .iter()
            .map(|item| item.data.text_value.clone())
            .collect::<Vec<_>>();
        let next_match = get_next_match(values, search_value.clone(), current_match);
        let new_item = items
            .iter()
            .find(|item| {
                next_match
                    .as_ref()
                    .is_some_and(|next_match| item.data.text_value == *next_match)
            })
            .and_then(|item| item.r#ref.get_untracked());

        let _ = search.try_set(search_value.clone());
        window().clear_timeout_with_handle(timer.try_get_untracked().unwrap_or(0));
        if !search_value.is_empty() {
            // Reset search 1 second after it was last updated.
            let _ = clear_search.try_with_value(|cs| {
                let cb: &wasm_bindgen::JsValue = cs.as_ref().unchecked_ref();
                let _ = timer.try_set(
                    window()
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            cb.unchecked_ref(),
                            1000,
                        )
                        .expect("Timeout should be set"),
                );
            });
        }

        if let Some(new_item) = new_item {
            let new_item: web_sys::HtmlElement = new_item.unchecked_into();
            let cb = Closure::once_into_js(move || {
                new_item.focus().ok();
            });
            window()
                .set_timeout_with_callback(cb.unchecked_ref())
                .expect("Timeout should be set.");
        }
    });

    on_cleanup(move || {
        window().clear_timeout_with_handle(timer.try_get_untracked().unwrap_or(0));
    });

    // Make sure the whole tree has focus guards as our `MenuContent` may be the last element in the DOM (because of the `Portal`).
    use_focus_guards();

    let is_pointer_moving_to_submenu = move |event: &ev::PointerEvent| -> bool {
        let Some(dir) = pointer_dir.try_get_untracked() else {
            return false;
        };
        let is_moving_towards = Some(dir)
            == pointer_grace_intent
                .try_get_untracked()
                .flatten()
                .map(|intent| intent.side);
        is_moving_towards
            && is_pointer_in_grace_area(
                event,
                pointer_grace_intent
                    .try_get_untracked()
                    .flatten()
                    .map(|intent| intent.area),
            )
    };

    let content_context_value = MenuContentContextValue {
        search,
        on_item_enter: Callback::new(move |event| {
            if is_pointer_moving_to_submenu(&event) {
                event.prevent_default();
            }
        }),
        on_item_leave: Callback::new(move |event| {
            if is_pointer_moving_to_submenu(&event) {
                return;
            }
            if let Some(content) = content_ref.get_untracked() {
                let content: web_sys::HtmlElement = content.unchecked_into();
                content.focus().ok();
            }
            let _ = set_current_item_id.try_set(None);
        }),
        on_trigger_leave: Callback::new(move |event| {
            if is_pointer_moving_to_submenu(&event) {
                event.prevent_default();
            }
        }),
        pointer_grace_timer,
        on_pointer_grace_intent_change: Callback::new(move |intent| {
            let _ = pointer_grace_intent.try_set(intent);
        }),
    };

    let disable_outside = prop_or_default(disable_outside_pointer_events);

    let trapped = prop_or_default(trap_focus);

    let current_tab_stop_id_signal =
        Signal::derive(move || current_item_id.get().unwrap_or_default());

    let children = StoredValue::new(children);

    // Unwrap optional callbacks for DismissableLayer (its props are #[prop(into, optional)]
    // so they need concrete Callback values, not Option<Callback>).
    let on_escape_key_down = on_escape_key_down.unwrap_or(Callback::new(|_| {}));
    let on_pointer_down_outside = on_pointer_down_outside.unwrap_or(Callback::new(|_| {}));
    let on_focus_outside_cb = on_focus_outside.unwrap_or(Callback::new(|_| {}));
    let on_interact_outside = on_interact_outside.unwrap_or(Callback::new(|_| {}));
    let on_dismiss = on_dismiss.unwrap_or(Callback::new(|_| {}));

    // Event handlers for keydown, blur, and pointermove must be attached directly to the
    // content element (inner Primitive) via addEventListener, not via on: attributes on
    // <PopperContent>. PopperContent renders a wrapper div as its first DOM element for
    // positioning, and on: handlers set on <PopperContent> land on that wrapper div.
    // This causes event.current_target() to be the wrapper div instead of the content
    // element that has [data-radix-menu-content], breaking is_key_down_inside checks
    // and preventing typeahead from working.
    let keydown_handler = compose_callbacks(
        on_key_down,
        Some(Callback::new(move |event: ev::KeyboardEvent| {
            // Submenu key events bubble through portals. We only care about keys in this menu.
            let target = event
                .target()
                .map(|target| target.unchecked_into::<web_sys::HtmlElement>())
                .expect("Event should have target.");
            let is_key_down_inside = target
                .closest("[data-radix-menu-content]")
                .expect("Element should be able to query closest.")
                == event
                    .current_target()
                    .and_then(|current_target| current_target.dyn_into::<web_sys::Element>().ok());
            let is_modifier_key = event.ctrl_key() || event.alt_key() || event.meta_key();
            let is_character_key = event.key().len() == 1;

            if is_key_down_inside {
                // Menus should not be navigated using tab key so we prevent it.
                if event.key() == "Tab" {
                    event.prevent_default();
                }
                if !is_modifier_key && is_character_key {
                    handle_typeahead_search.run(event.key());
                }
            }

            // Focus first/last item based on key pressed.
            if let Some(content) = content_ref.get_untracked() {
                let content_el: &web_sys::Element = content.unchecked_ref();
                if *content_el == target.unchecked_into::<web_sys::Element>() {
                    if !FIRST_LAST_KEYS.contains(&event.key().as_str()) {
                        return;
                    }

                    event.prevent_default();

                    let items = get_items
                        .try_with_value(|get_items| get_items())
                        .unwrap_or_default();
                    let items = items.iter().filter(|item| !item.data.disabled);
                    let mut candidate_nodes: Vec<web_sys::HtmlElement> = items
                        .filter_map(|item| {
                            item.r#ref.get_untracked().map(|node| node.unchecked_into())
                        })
                        .collect();
                    if LAST_KEYS.contains(&event.key().as_str()) {
                        candidate_nodes.reverse();
                    }
                    focus_first(candidate_nodes);
                }
            }
        })),
        None,
    );
    let blur_handler = compose_callbacks(
        on_blur,
        Some(Callback::new(move |event: ev::FocusEvent| {
            // Clear search buffer when leaving the menu.
            let target = event
                .target()
                .map(|target| target.unchecked_into::<web_sys::Node>())
                .expect("Event should have target.");
            let current_target = event
                .current_target()
                .map(|current_target| current_target.unchecked_into::<web_sys::Node>())
                .expect("Event should have current target.");
            if !current_target.contains(Some(&target)) {
                window().clear_timeout_with_handle(timer.try_get_untracked().unwrap_or(0));
                let _ = search.try_set("".into());
            }
        })),
        None,
    );
    let pointermove_handler = compose_callbacks(
        on_pointer_move,
        Some(when_mouse(move |event: ev::PointerEvent| {
            let target = event
                .target()
                .map(|target| target.unchecked_into::<web_sys::HtmlElement>())
                .expect("Event should have target.");
            let current_target = event
                .current_target()
                .map(|current_target| current_target.unchecked_into::<web_sys::Node>())
                .expect("Event should have current target.");
            let pointer_x_has_changed =
                last_pointer_x.try_get_untracked().unwrap_or(0) != event.client_x();

            // We don't use `event.movementX` for this check because Safari will always return `0` on a pointer event.
            if current_target.contains(Some(&target)) && pointer_x_has_changed {
                let new_dir =
                    match event.client_x() > last_pointer_x.try_get_untracked().unwrap_or(0) {
                        true => Side::Right,
                        false => Side::Left,
                    };
                let _ = pointer_dir.try_set(new_dir);
                let _ = last_pointer_x.try_set(event.client_x());
            }
        })),
        None,
    );

    let keydown_closure: SendWrapper<Rc<RefCell<Option<Closure<dyn Fn(ev::KeyboardEvent)>>>>> =
        SendWrapper::new(Rc::new(RefCell::new(Some(Closure::<
            dyn Fn(ev::KeyboardEvent),
        >::new(keydown_handler)))));
    let blur_closure: SendWrapper<Rc<RefCell<Option<Closure<dyn Fn(ev::FocusEvent)>>>>> =
        SendWrapper::new(Rc::new(RefCell::new(Some(
            Closure::<dyn Fn(ev::FocusEvent)>::new(blur_handler),
        ))));
    let pointermove_closure: SendWrapper<Rc<RefCell<Option<Closure<dyn Fn(ev::PointerEvent)>>>>> =
        SendWrapper::new(Rc::new(RefCell::new(Some(Closure::<
            dyn Fn(ev::PointerEvent),
        >::new(
            pointermove_handler
        )))));

    // Attach event handlers to the content element after mount.
    Effect::new({
        let keydown_closure = keydown_closure.clone();
        let blur_closure = blur_closure.clone();
        let pointermove_closure = pointermove_closure.clone();
        move |_| {
            if let Some(node) = content_ref.get() {
                let el: web_sys::HtmlElement = node.unchecked_into();
                if let Some(c) = keydown_closure.borrow().as_ref() {
                    el.add_event_listener_with_callback("keydown", c.as_ref().unchecked_ref())
                        .ok();
                }
                if let Some(c) = blur_closure.borrow().as_ref() {
                    el.add_event_listener_with_callback("blur", c.as_ref().unchecked_ref())
                        .ok();
                }
                if let Some(c) = pointermove_closure.borrow().as_ref() {
                    el.add_event_listener_with_callback("pointermove", c.as_ref().unchecked_ref())
                        .ok();
                }
            }
        }
    });

    on_cleanup(move || {
        if let Some(node) = content_ref.get_untracked() {
            let el: web_sys::HtmlElement = node.unchecked_into();
            if let Some(c) = keydown_closure.borrow().as_ref() {
                el.remove_event_listener_with_callback("keydown", c.as_ref().unchecked_ref())
                    .ok();
            }
            if let Some(c) = blur_closure.borrow().as_ref() {
                el.remove_event_listener_with_callback("blur", c.as_ref().unchecked_ref())
                    .ok();
            }
            if let Some(c) = pointermove_closure.borrow().as_ref() {
                el.remove_event_listener_with_callback("pointermove", c.as_ref().unchecked_ref())
                    .ok();
            }
        }
    });

    // Scroll lock: prevent scrolling outside the menu when disableOutsideScroll is true.
    // React uses `react-remove-scroll`; we use a simple body overflow approach.
    Effect::new(move |_| {
        if disable_outside_scroll.get().unwrap_or(false)
            && let Some(body) = document().body()
        {
            let style = body.style();
            let prev_overflow = style.get_property_value("overflow").unwrap_or_default();
            style.set_property("overflow", "hidden").ok();

            let style = SendWrapper::new(style);
            on_cleanup(move || {
                if prev_overflow.is_empty() {
                    style.remove_property("overflow").ok();
                } else {
                    style.set_property("overflow", &prev_overflow).ok();
                }
            });
        }
    });

    view! {
        <Provider value=content_context_value>
            <FocusScope
                as_child=true
                trapped=trapped
                on_mount_auto_focus=compose_callbacks(
                    on_open_auto_focus,
                    Some(Callback::new(move |event: ev::Event| {
                        // Always prevent default to take control of focusing.
                        event.prevent_default();

                        if root_context.is_using_keyboard.get_untracked() {
                            // For keyboard users, defer focus to a RAF so that collection items
                            // have time to register (Leptos effects are async, unlike React's
                            // synchronous useEffect). We focus the first menu item directly,
                            // bypassing RovingFocusGroup's entry focus which depends on
                            // collection items being registered.
                            let content_ref = content_ref;
                            let cb = Closure::once_into_js(move || {
                                // Use try_read_untracked to avoid panicking if the
                                // reactive scope has already been disposed by the time
                                // this RAF callback fires.
                                if let Some(guard) = content_ref.try_read_untracked() {
                                    if let Some(ref content) = *guard {
                                        let el: &web_sys::HtmlElement = content.unchecked_ref();
                                        let selector = "[role=menuitem]:not([data-disabled]), \
                                                         [role=menuitemcheckbox]:not([data-disabled]), \
                                                         [role=menuitemradio]:not([data-disabled])";
                                        if let Ok(Some(first_item)) = el.query_selector(selector) {
                                            let first: web_sys::HtmlElement = first_item.unchecked_into();
                                            first.focus().ok();
                                        } else {
                                            el.focus().ok();
                                        }
                                    }
                                }
                            });
                            window().request_animation_frame(cb.unchecked_ref()).ok();
                        } else {
                            // For pointer users, focus the content element so DismissableLayer
                            // works correctly. Don't focus a specific item.
                            // Defer to RAF because PopperContent's attribute transfer Effect
                            // (which moves tabindex from the wrapper div to the inner content
                            // div) may not have run yet. Without tabindex, focus() is a no-op.
                            let content_ref = content_ref;
                            let cb = Closure::once_into_js(move || {
                                // Use try_read_untracked to avoid panicking if the
                                // reactive scope has already been disposed by the time
                                // this RAF callback fires.
                                if let Some(guard) = content_ref.try_read_untracked() {
                                    if let Some(ref content) = *guard {
                                        let content: web_sys::HtmlElement = content.clone().unchecked_into();
                                        content.focus().ok();
                                    }
                                }
                            });
                            window().request_animation_frame(cb.unchecked_ref()).ok();
                        }
                    })),
                    None,
                )
                on_unmount_auto_focus=on_close_auto_focus
            >
                <DismissableLayer
                    as_child=true
                    disable_outside_pointer_events=disable_outside
                    on_escape_key_down=on_escape_key_down
                    on_pointer_down_outside=on_pointer_down_outside
                    on_focus_outside=on_focus_outside_cb
                    on_interact_outside=on_interact_outside
                    on_dismiss=on_dismiss
                >
                    <RovingFocusGroup
                        as_child=true
                        dir=root_context.dir
                        orientation=Orientation::Vertical
                        r#loop=r#loop
                        current_tab_stop_id=current_tab_stop_id_signal
                        on_current_tab_stop_id_change=Callback::new(move |value: Option<String>| {
                            let _ = set_current_item_id.try_set(value);
                        })
                        on_entry_focus=compose_callbacks(on_entry_focus, Some(Callback::new(move |event: ev::Event| {
                            if !root_context.is_using_keyboard.get_untracked() {
                                event.prevent_default();
                            }
                        })), None)
                        prevent_scroll_on_entry_focus=true
                    >
                        <PopperContent
                            side=Signal::derive(move || side.get().unwrap_or(PopperSide::Bottom))
                            side_offset=Signal::derive(move || side_offset.get().unwrap_or(0.0))
                            align=Signal::derive(move || align.get().unwrap_or(Align::Center))
                            align_offset=Signal::derive(move || align_offset.get().unwrap_or(0.0))
                            avoid_collisions=Signal::derive(move || avoid_collisions.get().unwrap_or(true))
                            collision_boundary=Signal::derive(move || collision_boundary.get().unwrap_or_else(|| SendWrapper::new(vec![])))
                            collision_padding=Signal::derive(move || collision_padding.get().unwrap_or(Padding::All(0.0)))
                            arrow_padding=Signal::derive(move || arrow_padding.get().unwrap_or(0.0))
                            sticky=Signal::derive(move || sticky.get().unwrap_or(Sticky::Partial))
                            hide_when_detached=Signal::derive(move || hide_when_detached.get().unwrap_or(false))
                            dir=Signal::derive(move || Some(root_context.dir.get().to_string()))
                            as_child=as_child
                            node_ref=composed_refs
                            attr:class=move || class.get().unwrap_or_default()
                            attr:style=move || content_style.get().unwrap_or_default()
                            attr:role="menu"
                            attr:aria-orientation="vertical"
                            attr:data-state=move || open_closed_state(context.open.get())
                            attr:data-radix-menu-content=""
                            attr:dir=move || root_context.dir.get()
                            attr:id=move || id.get()
                            attr:aria-labelledby=move || aria_labelledby.get()
                        >
                            {children.with_value(|children| children())}
                        </PopperContent>
                    </RovingFocusGroup>
                </DismissableLayer>
            </FocusScope>
        </Provider>
    }
}
