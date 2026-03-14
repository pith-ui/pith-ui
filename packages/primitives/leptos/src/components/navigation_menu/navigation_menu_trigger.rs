use super::*;

/* -------------------------------------------------------------------------------------------------
 * NavigationMenuTrigger
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn NavigationMenuTrigger(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_pointer_enter: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_move: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_leave: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<NavigationMenuContextValue>();
    let item_context_signal = expect_context::<RwSignal<NavigationMenuItemContextValue>>();

    let trigger_node_ref = AnyNodeRef::new();
    let item_trigger_ref = item_context_signal.get_untracked().trigger_ref;
    let composed_refs = use_composed_refs(vec![node_ref, trigger_node_ref, item_trigger_ref]);

    let has_pointer_move_opened_ref: StoredValue<bool> = StoredValue::new(false);
    let was_click_close_ref: StoredValue<bool> = StoredValue::new(false);

    let open = Memo::new(move |_| {
        let item_ctx = item_context_signal.get();
        item_ctx.value == context.value.get()
    });

    let trigger_id = Memo::new(move |_| {
        let item_ctx = item_context_signal.get();
        make_trigger_id(&context.base_id.get(), &item_ctx.value)
    });
    let content_id = Memo::new(move |_| {
        let item_ctx = item_context_signal.get();
        make_content_id(&context.base_id.get(), &item_ctx.value)
    });

    let item_value = Memo::new(move |_| item_context_signal.get().value.clone());

    view! {
        <CollectionItemSlot<NavigationMenuItemData>
            item_data_type=PhantomData
            item_data=Signal::derive(move || NavigationMenuItemData { value: item_value.get() })
            node_ref=composed_refs
        >
            <FocusGroupItem>
                <Primitive
                    element=html::button
                    as_child=as_child
                    node_ref=composed_refs
                    attr:id=move || trigger_id.get()
                    attr:disabled=move || disabled.get().unwrap_or(false).then_some("")
                    attr:data-disabled=move || disabled.get().unwrap_or(false).then_some("")
                    attr:data-state=move || open_closed_state(open.get())
                    attr:aria-expanded=move || open.get().to_string()
                    attr:aria-controls=move || content_id.get()
                    on:pointerenter=compose_callbacks(
                        on_pointer_enter,
                        Some(Callback::new(move |_: ev::PointerEvent| {
                            was_click_close_ref.set_value(false);
                            item_context_signal.get_untracked().was_escape_close_ref.set(false);
                        })),
                        None,
                    )
                    on:pointermove=compose_callbacks(
                        on_pointer_move,
                        Some(Callback::new(move |event: ev::PointerEvent| {
                            if event.pointer_type() != "mouse" { return; }
                            if disabled.get().unwrap_or(false)
                                || was_click_close_ref.get_value()
                                || item_context_signal.get_untracked().was_escape_close_ref.get_untracked()
                                || has_pointer_move_opened_ref.get_value()
                            {
                                return;
                            }
                            let item_ctx = item_context_signal.get_untracked();
                            context.on_trigger_enter.run(item_ctx.value.clone());
                            has_pointer_move_opened_ref.set_value(true);
                        })),
                        None,
                    )
                    on:pointerleave=compose_callbacks(
                        on_pointer_leave,
                        Some(Callback::new(move |event: ev::PointerEvent| {
                            if event.pointer_type() != "mouse" { return; }
                            if disabled.get().unwrap_or(false) { return; }
                            context.on_trigger_leave.run(());
                            has_pointer_move_opened_ref.set_value(false);
                        })),
                        None,
                    )
                    on:click=compose_callbacks(
                        on_click,
                        Some(Callback::new(move |_: ev::MouseEvent| {
                            let item_ctx = item_context_signal.get_untracked();
                            context.on_item_select.run(item_ctx.value.clone());
                            was_click_close_ref.set_value(open.get_untracked());
                        })),
                        None,
                    )
                    on:keydown=compose_callbacks(
                        on_key_down,
                        Some(Callback::new(move |event: ev::KeyboardEvent| {
                            let vertical_entry_key = if context.dir.get_untracked() == Direction::Rtl {
                                "ArrowLeft"
                            } else {
                                "ArrowRight"
                            };
                            let entry_key = match context.orientation.get_untracked() {
                                Orientation::Horizontal => "ArrowDown",
                                Orientation::Vertical => vertical_entry_key,
                            };
                            if open.get_untracked() && event.key() == entry_key {
                                item_context_signal.get_untracked().on_entry_key_down.run(());
                                event.prevent_default();
                            }
                        })),
                        None,
                    )
                >
                    {children.with_value(|children| children.as_ref().map(|children| children()))}
                </Primitive>
            </FocusGroupItem>
        </CollectionItemSlot<NavigationMenuItemData>>

        // Focus proxy and aria-owns when open
        {move || {
            let is_open = open.get();
            let item_ctx = item_context_signal.get();
            if is_open {
                let has_viewport = context.viewport.get().is_some();
                let cid = content_id.get();
                Some(view! {
                    <VisuallyHidden
                        attr:aria-hidden="true"
                        attr:tabindex="0"
                        node_ref=item_ctx.focus_proxy_ref
                        on:focus=move |event: ev::FocusEvent| {
                            let item_ctx = item_context_signal.get_untracked();
                            let prev = event.related_target()
                                .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok());
                            let was_trigger_focused = prev.as_ref().map(|p| {
                                trigger_node_ref.get().map(|t| {
                                    let t: web_sys::HtmlElement = t.unchecked_into();
                                    *p == t
                                }).unwrap_or(false)
                            }).unwrap_or(false);

                            let was_focus_from_content = prev.as_ref().map(|p| {
                                item_ctx.content_ref.get().map(|content| {
                                    let content: web_sys::Node = content.unchecked_into();
                                    content.contains(Some(p.unchecked_ref()))
                                }).unwrap_or(false)
                            }).unwrap_or(false);

                            if was_trigger_focused || !was_focus_from_content {
                                let side = if was_trigger_focused { "start" } else { "end" };
                                item_ctx.on_focus_proxy_enter.run(side);
                            }
                        }
                    >
                        {""}
                    </VisuallyHidden>

                    // Restructure a11y tree — the span must be out of flow so it
                    // does not create a line box inside the <li> and grow the <ul>.
                    {if has_viewport {
                        Some(view! { <span aria-owns=cid style="position: absolute;"></span> })
                    } else {
                        None
                    }}
                })
            } else {
                None
            }
        }}
    }
}
