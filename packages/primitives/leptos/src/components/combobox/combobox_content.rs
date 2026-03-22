use super::*;

/* -------------------------------------------------------------------------------------------------
 * ComboboxContent
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ComboboxContent(
    #[prop(into, optional)] on_escape_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<ev::CustomEvent>>,
    // PopperContent forwarded props
    #[prop(into, optional, default = Side::Bottom.into())] side: Signal<Side>,
    #[prop(into, optional, default = 0.0.into())] side_offset: Signal<f64>,
    #[prop(into, optional, default = Align::Start.into())] align: Signal<Align>,
    #[prop(into, optional, default = 0.0.into())] align_offset: Signal<f64>,
    #[prop(into, optional, default = 0.0.into())] arrow_padding: Signal<f64>,
    #[prop(into, optional, default = true.into())] avoid_collisions: Signal<bool>,
    #[prop(into, optional, default = SendWrapper::new(vec![]).into())] collision_boundary: Signal<
        SendWrapper<Vec<web_sys::Element>>,
    >,
    #[prop(into, optional, default = Padding::All(10.0).into())] collision_padding: Signal<Padding>,
    #[prop(into, optional, default = Sticky::Partial.into())] sticky: Signal<Sticky>,
    #[prop(into, optional, default = false.into())] hide_when_detached: Signal<bool>,
    #[prop(into, optional, default = UpdatePositionStrategy::Optimized.into())]
    update_position_strategy: Signal<UpdatePositionStrategy>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<ComboboxContextValue>();

    let on_escape_key_down = StoredValue::new(on_escape_key_down);
    let on_pointer_down_outside = StoredValue::new(on_pointer_down_outside);

    view! {
        {move || {
            if context.open.get() {
                Some(view! {
                    <ComboboxContentImpl
                        on_escape_key_down=on_escape_key_down
                        on_pointer_down_outside=on_pointer_down_outside
                        side=side
                        side_offset=side_offset
                        align=align
                        align_offset=align_offset
                        arrow_padding=arrow_padding
                        avoid_collisions=avoid_collisions
                        collision_boundary=collision_boundary
                        collision_padding=collision_padding
                        sticky=sticky
                        hide_when_detached=hide_when_detached
                        update_position_strategy=update_position_strategy
                        as_child=as_child
                        node_ref=node_ref
                    >
                        {children.try_with_value(|children| children.as_ref().map(|c| c()))}
                    </ComboboxContentImpl>
                }.into_any())
            } else {
                None
            }
        }}
    }
}

/* -------------------------------------------------------------------------------------------------
 * ComboboxContentImpl
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn ComboboxContentImpl(
    #[allow(unused_variables)] on_escape_key_down: StoredValue<
        Option<Callback<web_sys::KeyboardEvent>>,
    >,
    #[allow(unused_variables)] on_pointer_down_outside: StoredValue<
        Option<Callback<web_sys::CustomEvent>>,
    >,
    // PopperContent forwarded props
    #[prop(into, optional, default = Side::Bottom.into())] side: Signal<Side>,
    #[prop(into, optional, default = 0.0.into())] side_offset: Signal<f64>,
    #[prop(into, optional, default = Align::Start.into())] align: Signal<Align>,
    #[prop(into, optional, default = 0.0.into())] align_offset: Signal<f64>,
    #[prop(into, optional, default = 0.0.into())] arrow_padding: Signal<f64>,
    #[prop(into, optional, default = true.into())] avoid_collisions: Signal<bool>,
    #[prop(into, optional, default = SendWrapper::new(vec![]).into())] collision_boundary: Signal<
        SendWrapper<Vec<web_sys::Element>>,
    >,
    #[prop(into, optional, default = Padding::All(10.0).into())] collision_padding: Signal<Padding>,
    #[prop(into, optional, default = Sticky::Partial.into())] sticky: Signal<Sticky>,
    #[prop(into, optional, default = false.into())] hide_when_detached: Signal<bool>,
    #[prop(into, optional, default = UpdatePositionStrategy::Optimized.into())]
    update_position_strategy: Signal<UpdatePositionStrategy>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<ComboboxContextValue>();
    let content_ref = AnyNodeRef::new();
    let viewport_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, content_ref]);

    let (is_positioned, set_is_positioned) = signal(false);

    let on_dismiss = Callback::new(move |_: ()| {
        context.on_open_change.run(false);
        context.active_descendant_id.set(None);
    });

    let content_context = ComboboxContentContextValue {
        content_ref,
        viewport_ref,
        is_positioned,
    };

    // Unlike Select, Combobox does NOT trap focus -- focus stays on the input.
    // No FocusScope, no aria-hidden of siblings.

    view! {
        <Provider value=content_context>
            <DismissableLayer
                as_child=true
                disable_outside_pointer_events=false
                on_escape_key_down=Callback::new(move |event: web_sys::KeyboardEvent| {
                    let _ = on_escape_key_down.try_with_value(|cb| {
                        if let Some(cb) = cb {
                            cb.run(event);
                        }
                    });
                })
                on_pointer_down_outside=Callback::new(move |event: web_sys::CustomEvent| {
                    // If the pointer down is on the trigger or input, prevent dismissal
                    // so the trigger's click handler can properly toggle open/close.
                    let target = event.detail()
                        .dyn_into::<web_sys::Event>()
                        .ok()
                        .and_then(|e| e.target())
                        .and_then(|t| t.dyn_into::<web_sys::Node>().ok());

                    let is_in_trigger = target.as_ref().is_some_and(|t| {
                        context.trigger_ref.get_untracked()
                            .map(|el| {
                                let node: &web_sys::Node = (*el).unchecked_ref();
                                node.contains(Some(t)) || node == t
                            })
                            .unwrap_or(false)
                    });
                    let is_in_input = target.as_ref().is_some_and(|t| {
                        context.input_ref.get_untracked()
                            .map(|el| {
                                let node: &web_sys::Node = (*el).unchecked_ref();
                                node.contains(Some(t)) || node == t
                            })
                            .unwrap_or(false)
                    });

                    if is_in_trigger || is_in_input {
                        event.prevent_default();
                        return;
                    }

                    let _ = on_pointer_down_outside.try_with_value(|cb| {
                        if let Some(cb) = cb {
                            cb.run(event);
                        }
                    });
                })
                on_focus_outside=Callback::new(move |event: web_sys::CustomEvent| {
                    // Always prevent dismissal on focus outside for combobox.
                    // Focus stays on the input; we don't want focus-outside to dismiss.
                    event.prevent_default();
                })
                on_dismiss=Callback::new(move |_: ()| {
                    on_dismiss.run(());
                })
            >
                <PopperContent
                    side=side
                    side_offset=side_offset
                    align=align
                    align_offset=align_offset
                    arrow_padding=arrow_padding
                    avoid_collisions=avoid_collisions
                    collision_boundary=collision_boundary
                    collision_padding=collision_padding
                    sticky=sticky
                    hide_when_detached=hide_when_detached
                    update_position_strategy=update_position_strategy
                    as_child=as_child
                    node_ref=composed_refs
                    style:display="flex"
                    style:flex-direction="column"
                    style:outline="none"
                    style:box-sizing="border-box"
                    style:--radix-combobox-content-transform-origin="var(--radix-popper-transform-origin)"
                    style:--radix-combobox-content-available-width="var(--radix-popper-available-width)"
                    style:--radix-combobox-content-available-height="var(--radix-popper-available-height)"
                    style:--radix-combobox-trigger-width="var(--radix-popper-anchor-width)"
                    style:--radix-combobox-trigger-height="var(--radix-popper-anchor-height)"
                    on_placed=Some(Callback::new(move |_: ()| {
                        set_is_positioned.set(true);
                    }))
                    attr:role="listbox"
                    attr:id=move || context.content_id.get()
                    attr:data-state=move || if context.open.get() { "open" } else { "closed" }
                    attr:dir=move || context.dir.get().to_string()
                    attr:aria-multiselectable=move || if context.multiple { Some("true".to_string()) } else { None }
                    on:contextmenu=move |event: ev::MouseEvent| {
                        event.prevent_default();
                    }
                >
                    {children.try_with_value(|children| children.as_ref().map(|c| c()))}
                </PopperContent>
            </DismissableLayer>
        </Provider>
    }
}
