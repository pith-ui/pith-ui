use super::*;

/* -------------------------------------------------------------------------------------------------
 * ScrollAreaThumb
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ScrollAreaThumb(
    #[prop(into, optional)] force_mount: Option<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let force_mount = force_mount.unwrap_or(false);
    let scrollbar_context = expect_context::<ScrollbarContextValue>();

    let presence_ref = AnyNodeRef::new();

    view! {
        <AttributeInterceptor let:attrs>
            {view! {
                <Presence present=Signal::derive(move || force_mount || scrollbar_context.has_thumb.get()) node_ref=presence_ref>
                    <ScrollAreaThumbImpl
                        as_child=as_child
                        node_ref=use_composed_refs(vec![node_ref, presence_ref])
                    >
                        {children.with_value(|children| children())}
                    </ScrollAreaThumbImpl>
                </Presence>
            }.add_any_attr(attrs)}
        </AttributeInterceptor>
    }
}

#[component]
fn ScrollAreaThumbImpl(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let scroll_area_context = expect_context::<ScrollAreaContextValue>();
    let scrollbar_context = expect_context::<ScrollbarContextValue>();
    let on_thumb_position_change = scrollbar_context.on_thumb_position_change;

    let thumb_ref = AnyNodeRef::new();
    let composed_ref = use_composed_refs(vec![node_ref, thumb_ref]);

    // Set thumb element in scrollbar context
    Effect::new(move |_| {
        if let Some(node) = thumb_ref.get() {
            let el: web_sys::HtmlElement = node.unchecked_into();
            scrollbar_context
                .on_thumb_change
                .run(Some(SendWrapper::new(el)));
        }
    });
    Owner::on_cleanup(move || {
        scrollbar_context.on_thumb_change.run(None);
    });

    // Track whether an unlinked scroll listener (rAF loop) is active.
    // We use a StoredValue<bool> flag + a StoredValue for the cancel handle.
    let raf_active = StoredValue::new(false);
    let raf_cancel_id = StoredValue::new(0i32);

    let debounce_scroll_end = use_debounce_callback(
        move || {
            // Cancel the rAF loop when scrolling ends
            if raf_active.get_value() {
                let window = web_sys::window().expect("Window should exist.");
                window
                    .cancel_animation_frame(raf_cancel_id.get_value())
                    .ok();
                raf_active.set_value(false);
            }
        },
        100,
    );

    // Scroll listener on viewport
    Effect::new(move |_| {
        if let Some(viewport) = scroll_area_context.viewport.get() {
            on_thumb_position_change.run(());

            let debounce = debounce_scroll_end;
            let viewport_for_raf = viewport.clone();
            let on_thumb_pos = on_thumb_position_change;

            let scroll_closure = SendWrapper::new(Closure::<dyn Fn()>::new(move || {
                debounce.run(());
                if !raf_active.get_value() {
                    // Start an unlinked scroll listener (rAF loop) that calls
                    // onThumbPositionChange on each frame until debounce stops it.
                    raf_active.set_value(true);
                    on_thumb_pos.run(());

                    let viewport_raf = viewport_for_raf.clone();
                    let prev_left =
                        std::rc::Rc::new(std::cell::Cell::new(viewport_raf.scroll_left() as f64));
                    let prev_top =
                        std::rc::Rc::new(std::cell::Cell::new(viewport_raf.scroll_top() as f64));

                    let closure_holder: RafClosureHolder =
                        std::rc::Rc::new(std::cell::RefCell::new(None));
                    let closure_holder_inner = closure_holder.clone();
                    let window = web_sys::window().expect("Window should exist.");
                    let window_inner = window.clone();

                    let raf_cb = Closure::new(move || {
                        if !raf_active.get_value() {
                            return;
                        }
                        let left = viewport_raf.scroll_left() as f64;
                        let top = viewport_raf.scroll_top() as f64;
                        if prev_left.get() != left || prev_top.get() != top {
                            on_thumb_pos.run(());
                        }
                        prev_left.set(left);
                        prev_top.set(top);
                        if let Some(c) = closure_holder_inner.borrow().as_ref() {
                            let id = window_inner
                                .request_animation_frame(c.as_ref().unchecked_ref())
                                .unwrap_or(0);
                            raf_cancel_id.set_value(id);
                        }
                    });

                    let id = window
                        .request_animation_frame(raf_cb.as_ref().unchecked_ref())
                        .unwrap_or(0);
                    raf_cancel_id.set_value(id);
                    *closure_holder.borrow_mut() = Some(raf_cb);
                }
            }));

            let viewport_target: web_sys::EventTarget =
                <web_sys::HtmlElement as Clone>::clone(&viewport).unchecked_into();
            viewport_target
                .add_event_listener_with_callback("scroll", scroll_closure.as_ref().unchecked_ref())
                .ok();

            let viewport_target = SendWrapper::new(viewport_target);
            Owner::on_cleanup(move || {
                viewport_target
                    .remove_event_listener_with_callback(
                        "scroll",
                        scroll_closure.as_ref().unchecked_ref(),
                    )
                    .ok();
                // Cancel any active rAF loop
                if raf_active.get_value() {
                    let window = web_sys::window().expect("Window should exist.");
                    window
                        .cancel_animation_frame(raf_cancel_id.get_value())
                        .ok();
                    raf_active.set_value(false);
                }
            });
        }
    });

    let has_thumb = scrollbar_context.has_thumb;
    let on_thumb_pointer_down = scrollbar_context.on_thumb_pointer_down;
    let on_thumb_pointer_up = scrollbar_context.on_thumb_pointer_up;

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=composed_ref
                style:width="var(--radix-scroll-area-thumb-width)"
                style:height="var(--radix-scroll-area-thumb-height)"
                attr:data-state=move || if has_thumb.get() { "visible" } else { "hidden" }
                on:pointerdown=move |event: ev::PointerEvent| {
                    if let Some(target) = event.target() {
                        let thumb: web_sys::HtmlElement = target.unchecked_into();
                        let thumb_rect = thumb.get_bounding_client_rect();
                        let x = event.client_x() as f64 - thumb_rect.left();
                        let y = event.client_y() as f64 - thumb_rect.top();
                        on_thumb_pointer_down.run((x, y));
                    }
                }
                on:pointerup=move |_: ev::PointerEvent| {
                    on_thumb_pointer_up.run(());
                }
                {..attrs}
            >
                {children.with_value(|children| children())}
            </Primitive>
        </AttributeInterceptor>
    }
}
