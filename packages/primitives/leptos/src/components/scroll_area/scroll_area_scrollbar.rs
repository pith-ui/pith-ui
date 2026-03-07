use super::*;

/* -------------------------------------------------------------------------------------------------
 * ScrollAreaScrollbar
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ScrollAreaScrollbar(
    #[prop(into, optional)] orientation: Option<Orientation>,
    #[prop(into, optional)] force_mount: Option<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let orientation = orientation.unwrap_or_default();
    let force_mount = force_mount.unwrap_or(false);
    let context = expect_context::<ScrollAreaContextValue>();

    // Forward class via context to bypass Presence/Show boundary
    provide_context(ForwardedScrollbarClass(class.get_untracked()));

    let is_horizontal = orientation == Orientation::Horizontal;

    // Register scrollbar axis with context
    Effect::new(move |_| {
        if is_horizontal {
            context.scrollbar_x_enabled.set(true);
        } else {
            context.scrollbar_y_enabled.set(true);
        }
    });
    Owner::on_cleanup(move || {
        if is_horizontal {
            context.scrollbar_x_enabled.set(false);
        } else {
            context.scrollbar_y_enabled.set(false);
        }
    });

    view! {
        <AttributeInterceptor let:attrs>
            {match context.r#type {
                ScrollAreaType::Hover => view! {
                    <ScrollAreaScrollbarHover
                        orientation=orientation
                        force_mount=force_mount
                        as_child=as_child
                        node_ref=node_ref
                    >
                        {children.with_value(|children| children())}
                    </ScrollAreaScrollbarHover>
                }
                .add_any_attr(attrs)
                .into_any(),
                ScrollAreaType::Scroll => view! {
                    <ScrollAreaScrollbarScroll
                        orientation=orientation
                        force_mount=force_mount
                        as_child=as_child
                        node_ref=node_ref
                    >
                        {children.with_value(|children| children())}
                    </ScrollAreaScrollbarScroll>
                }
                .add_any_attr(attrs)
                .into_any(),
                ScrollAreaType::Auto => view! {
                    <ScrollAreaScrollbarAuto
                        orientation=orientation
                        force_mount=force_mount
                        as_child=as_child
                        node_ref=node_ref
                    >
                        {children.with_value(|children| children())}
                    </ScrollAreaScrollbarAuto>
                }
                .add_any_attr(attrs)
                .into_any(),
                ScrollAreaType::Always => view! {
                    <ScrollAreaScrollbarVisible
                        orientation=orientation
                        as_child=as_child
                        node_ref=node_ref
                    >
                        {children.with_value(|children| children())}
                    </ScrollAreaScrollbarVisible>
                }
                .add_any_attr(attrs)
                .into_any(),
            }}
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ScrollAreaScrollbarHover
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn ScrollAreaScrollbarHover(
    #[prop(into)] orientation: Orientation,
    #[prop(into)] force_mount: bool,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context = expect_context::<ScrollAreaContextValue>();
    let visible = RwSignal::new(false);

    Effect::new(move |_| {
        if let Some(scroll_area_node) = context.scroll_area.get() {
            let scroll_area: &web_sys::HtmlElement = scroll_area_node.unchecked_ref();

            let scroll_hide_delay = context.scroll_hide_delay;
            let hide_timer: StoredValue<i32> = StoredValue::new(0);

            let visible_enter = visible;
            let enter_closure = SendWrapper::new(Closure::<dyn Fn()>::new(move || {
                let window = web_sys::window().expect("Window should exist.");
                window.clear_timeout_with_handle(hide_timer.get_value());
                visible_enter.set(true);
            }));

            let visible_leave = visible;
            let leave_closure = SendWrapper::new(Closure::<dyn Fn()>::new(move || {
                let window = web_sys::window().expect("Window should exist.");
                let handle = window
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        &Closure::once_into_js(move || {
                            visible_leave.set(false);
                        })
                        .unchecked_into(),
                        scroll_hide_delay as i32,
                    )
                    .unwrap_or(0);
                hide_timer.set_value(handle);
            }));

            scroll_area
                .add_event_listener_with_callback(
                    "pointerenter",
                    enter_closure.as_ref().unchecked_ref(),
                )
                .ok();
            scroll_area
                .add_event_listener_with_callback(
                    "pointerleave",
                    leave_closure.as_ref().unchecked_ref(),
                )
                .ok();

            let scroll_area = SendWrapper::new(scroll_area.clone());
            Owner::on_cleanup(move || {
                let window = web_sys::window().expect("Window should exist.");
                window.clear_timeout_with_handle(hide_timer.get_value());
                scroll_area
                    .remove_event_listener_with_callback(
                        "pointerenter",
                        enter_closure.as_ref().unchecked_ref(),
                    )
                    .ok();
                scroll_area
                    .remove_event_listener_with_callback(
                        "pointerleave",
                        leave_closure.as_ref().unchecked_ref(),
                    )
                    .ok();
            });
        }
    });

    let presence_ref = AnyNodeRef::new();

    view! {
        <AttributeInterceptor let:attrs>
            {view! {
                <Presence present=Signal::derive(move || force_mount || visible.get()) node_ref=presence_ref>
                    <ScrollAreaScrollbarAuto
                        orientation=orientation
                        force_mount=false
                        as_child=as_child
                        node_ref=use_composed_refs(vec![node_ref, presence_ref])
                        attr:data-state=move || if visible.get() { "visible" } else { "hidden" }
                    >
                        {children.with_value(|children| children())}
                    </ScrollAreaScrollbarAuto>
                </Presence>
            }.add_any_attr(attrs)}
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ScrollAreaScrollbarScroll
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn ScrollAreaScrollbarScroll(
    #[prop(into)] orientation: Orientation,
    #[prop(into)] force_mount: bool,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context = expect_context::<ScrollAreaContextValue>();
    let is_horizontal = orientation == Orientation::Horizontal;

    let (state, send) = use_scrollbar_state_machine();
    let debounce_scroll_end =
        use_debounce_callback(move || send.run(ScrollbarMachineEvent::ScrollEnd), 100);

    // Hide timer for idle state
    Effect::new(move |_| {
        if state.get() == ScrollbarMachineState::Idle {
            let scroll_hide_delay = context.scroll_hide_delay;
            let window = web_sys::window().expect("Window should exist.");
            let handle = window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    &Closure::once_into_js(move || {
                        send.run(ScrollbarMachineEvent::Hide);
                    })
                    .unchecked_into(),
                    scroll_hide_delay as i32,
                )
                .unwrap_or(0);

            Owner::on_cleanup(move || {
                let window = web_sys::window().expect("Window should exist.");
                window.clear_timeout_with_handle(handle);
            });
        }
    });

    // Scroll listener on viewport
    Effect::new(move |_| {
        if let Some(viewport) = context.viewport.get() {
            let send = send;
            let debounce = debounce_scroll_end;
            let prev_scroll_pos = std::cell::Cell::new(if is_horizontal {
                viewport.scroll_left() as f64
            } else {
                viewport.scroll_top() as f64
            });

            let viewport_for_closure = viewport.clone();
            let scroll_closure = SendWrapper::new(Closure::<dyn Fn()>::new(move || {
                let scroll_pos = if is_horizontal {
                    viewport_for_closure.scroll_left() as f64
                } else {
                    viewport_for_closure.scroll_top() as f64
                };
                if prev_scroll_pos.get() != scroll_pos {
                    send.run(ScrollbarMachineEvent::Scroll);
                    debounce.run(());
                }
                prev_scroll_pos.set(scroll_pos);
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
            });
        }
    });

    let presence_ref = AnyNodeRef::new();

    view! {
        <AttributeInterceptor let:attrs>
            {view! {
                <Presence present=Signal::derive(move || force_mount || state.get() != ScrollbarMachineState::Hidden) node_ref=presence_ref>
                    <ScrollAreaScrollbarVisible
                        orientation=orientation
                        as_child=as_child
                        node_ref=use_composed_refs(vec![node_ref, presence_ref])
                        attr:data-state=move || if state.get() == ScrollbarMachineState::Hidden { "hidden" } else { "visible" }
                        on:pointerenter=move |_| send.run(ScrollbarMachineEvent::PointerEnter)
                        on:pointerleave=move |_| send.run(ScrollbarMachineEvent::PointerLeave)
                    >
                        {children.with_value(|children| children())}
                    </ScrollAreaScrollbarVisible>
                </Presence>
            }.add_any_attr(attrs)}
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ScrollAreaScrollbarAuto
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn ScrollAreaScrollbarAuto(
    #[prop(into)] orientation: Orientation,
    #[prop(into)] force_mount: bool,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context = expect_context::<ScrollAreaContextValue>();
    let visible = RwSignal::new(false);
    let is_horizontal = orientation == Orientation::Horizontal;

    let handle_resize = use_debounce_callback(
        move || {
            if let Some(viewport) = context.viewport.get_untracked() {
                let is_overflow = if is_horizontal {
                    viewport.offset_width() < viewport.scroll_width()
                } else {
                    viewport.offset_height() < viewport.scroll_height()
                };
                visible.set(is_overflow);
            }
        },
        10,
    );

    use_resize_observer(Signal::derive(move || context.viewport.get()), move || {
        handle_resize.run(())
    });
    use_resize_observer(Signal::derive(move || context.content.get()), move || {
        handle_resize.run(())
    });

    let presence_ref = AnyNodeRef::new();

    view! {
        <AttributeInterceptor let:attrs>
            {view! {
                <Presence present=Signal::derive(move || force_mount || visible.get()) node_ref=presence_ref>
                    <ScrollAreaScrollbarVisible
                        orientation=orientation
                        as_child=as_child
                        node_ref=use_composed_refs(vec![node_ref, presence_ref])
                        attr:data-state=move || if visible.get() { "visible" } else { "hidden" }
                    >
                        {children.with_value(|children| children())}
                    </ScrollAreaScrollbarVisible>
                </Presence>
            }.add_any_attr(attrs)}
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ScrollAreaScrollbarVisible
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn ScrollAreaScrollbarVisible(
    #[prop(into, optional)] orientation: Option<Orientation>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let orientation = orientation.unwrap_or_default();
    let context = expect_context::<ScrollAreaContextValue>();

    let thumb_ref: RwSignal<Option<SendWrapper<web_sys::HtmlElement>>> = RwSignal::new(None);
    let pointer_offset = RwSignal::new(0.0_f64);
    let sizes = RwSignal::new(Sizes::default());

    let has_thumb = Signal::derive(move || {
        let s = sizes.get();
        let r = get_thumb_ratio(s.viewport, s.content);
        r > 0.0 && r < 1.0
    });

    let on_thumb_change = Callback::new(move |thumb: Option<SendWrapper<web_sys::HtmlElement>>| {
        thumb_ref.set(thumb);
    });
    let on_thumb_pointer_up = Callback::new(move |()| {
        pointer_offset.set(0.0);
    });

    view! {
        <AttributeInterceptor let:attrs>
            {match orientation {
                Orientation::Horizontal => {
                    let on_thumb_pointer_down = Callback::new(move |(x, _y): (f64, f64)| {
                        pointer_offset.set(x);
                    });

                let on_thumb_position_change = Callback::new(move |()| {
                    if let Some(viewport) = context.viewport.get_untracked()
                        && let Some(thumb) = thumb_ref.get_untracked()
                    {
                        let scroll_pos = viewport.scroll_left() as f64;
                        let offset =
                            get_thumb_offset_from_scroll(scroll_pos, &sizes.get_untracked(), context.dir.get_untracked());
                        thumb
                            .style()
                            .set_property("transform", &format!("translate3d({}px, 0, 0)", offset))
                            .ok();
                    }
                });

                let on_wheel_scroll = Callback::new(move |scroll_pos: f64| {
                    if let Some(viewport) = context.viewport.get_untracked() {
                        viewport.set_scroll_left(scroll_pos as i32);
                    }
                });

                let on_drag_scroll = Callback::new(move |pointer_pos: f64| {
                    if let Some(viewport) = context.viewport.get_untracked() {
                        let scroll_pos = get_scroll_position_from_pointer(
                            pointer_pos,
                            pointer_offset.get_untracked(),
                            &sizes.get_untracked(),
                            context.dir.get_untracked(),
                        );
                        viewport.set_scroll_left(scroll_pos as i32);
                    }
                });

                    view! {
                        <ScrollAreaScrollbarX
                            sizes=sizes
                            has_thumb=has_thumb
                            on_thumb_change=on_thumb_change
                            on_thumb_pointer_up=on_thumb_pointer_up
                            on_thumb_pointer_down=on_thumb_pointer_down
                            on_thumb_position_change=on_thumb_position_change
                            on_wheel_scroll=on_wheel_scroll
                            on_drag_scroll=on_drag_scroll
                            as_child=as_child
                            node_ref=node_ref
                        >
                            {children.with_value(|children| children())}
                        </ScrollAreaScrollbarX>
                    }
                    .add_any_attr(attrs)
                    .into_any()
                }
                Orientation::Vertical => {
                let on_thumb_pointer_down = Callback::new(move |(_x, y): (f64, f64)| {
                    pointer_offset.set(y);
                });

                let on_thumb_position_change = Callback::new(move |()| {
                    if let Some(viewport) = context.viewport.get_untracked()
                        && let Some(thumb) = thumb_ref.get_untracked()
                    {
                        let scroll_pos = viewport.scroll_top() as f64;
                        let offset =
                            get_thumb_offset_from_scroll(scroll_pos, &sizes.get_untracked(), Direction::Ltr);
                        thumb
                            .style()
                            .set_property("transform", &format!("translate3d(0, {}px, 0)", offset))
                            .ok();
                    }
                });

                let on_wheel_scroll = Callback::new(move |scroll_pos: f64| {
                    if let Some(viewport) = context.viewport.get_untracked() {
                        viewport.set_scroll_top(scroll_pos as i32);
                    }
                });

                let on_drag_scroll = Callback::new(move |pointer_pos: f64| {
                    if let Some(viewport) = context.viewport.get_untracked() {
                        let scroll_pos = get_scroll_position_from_pointer(
                            pointer_pos,
                            pointer_offset.get_untracked(),
                            &sizes.get_untracked(),
                            Direction::Ltr,
                        );
                        viewport.set_scroll_top(scroll_pos as i32);
                    }
                });

                    view! {
                        <ScrollAreaScrollbarY
                            sizes=sizes
                            has_thumb=has_thumb
                            on_thumb_change=on_thumb_change
                            on_thumb_pointer_up=on_thumb_pointer_up
                            on_thumb_pointer_down=on_thumb_pointer_down
                            on_thumb_position_change=on_thumb_position_change
                            on_wheel_scroll=on_wheel_scroll
                            on_drag_scroll=on_drag_scroll
                            as_child=as_child
                            node_ref=node_ref
                        >
                            {children.with_value(|children| children())}
                        </ScrollAreaScrollbarY>
                    }
                    .add_any_attr(attrs)
                    .into_any()
                }
            }}
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ScrollAreaScrollbarX / ScrollAreaScrollbarY
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn ScrollAreaScrollbarX(
    sizes: RwSignal<Sizes>,
    has_thumb: Signal<bool>,
    on_thumb_change: Callback<Option<SendWrapper<web_sys::HtmlElement>>>,
    on_thumb_pointer_up: Callback<()>,
    on_thumb_pointer_down: Callback<(f64, f64)>,
    on_thumb_position_change: Callback<()>,
    on_wheel_scroll: Callback<f64>,
    on_drag_scroll: Callback<f64>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context = expect_context::<ScrollAreaContextValue>();
    let computed_style: RwSignal<Option<SendWrapper<web_sys::CssStyleDeclaration>>> =
        RwSignal::new(None);

    let scrollbar_ref = AnyNodeRef::new();
    let composed_ref = use_composed_refs(vec![node_ref, scrollbar_ref]);

    // Set computed style when ref is available
    Effect::new(move |_| {
        if let Some(node) = scrollbar_ref.get() {
            let node: &web_sys::HtmlElement = node.unchecked_ref();
            let window = web_sys::window().expect("Window should exist.");
            if let Ok(style) = window.get_computed_style(node) {
                computed_style.set(style.map(SendWrapper::new));
            }
        }
    });

    // Set scrollbar X element in context
    Effect::new(move |_| {
        if let Some(node) = scrollbar_ref.get() {
            let el: web_sys::HtmlElement = node.unchecked_into();
            context.scrollbar_x.set(Some(SendWrapper::new(el)));
        }
    });
    Owner::on_cleanup(move || {
        context.scrollbar_x.set(None);
    });

    let on_resize = Callback::new(move |()| {
        if let Some(node) = scrollbar_ref.get_untracked()
            && let Some(viewport) = context.viewport.get_untracked()
            && let Some(cs) = computed_style.get_untracked()
        {
            let scrollbar_el: &web_sys::HtmlElement = node.unchecked_ref();
            sizes.set(Sizes {
                content: viewport.scroll_width() as f64,
                viewport: viewport.offset_width() as f64,
                scrollbar: ScrollbarSizes {
                    size: scrollbar_el.client_width() as f64,
                    padding_start: to_int(
                        &cs.get_property_value("padding-left").unwrap_or_default(),
                    ),
                    padding_end: to_int(
                        &cs.get_property_value("padding-right").unwrap_or_default(),
                    ),
                },
            });
        }
    });

    let on_wheel_scroll_impl = Callback::new(
        move |(event, max_scroll_pos): (SendWrapper<web_sys::WheelEvent>, f64)| {
            if let Some(viewport) = context.viewport.get_untracked() {
                let scroll_pos = viewport.scroll_left() as f64 + event.delta_x();
                on_wheel_scroll.run(scroll_pos);
                if is_scrolling_within_scrollbar_bounds(scroll_pos, max_scroll_pos) {
                    event.prevent_default();
                }
            }
        },
    );

    let on_drag_scroll_impl = Callback::new(move |(x, _y): (f64, f64)| {
        on_drag_scroll.run(x);
    });

    let on_thumb_pointer_down_impl = Callback::new(move |(x, y): (f64, f64)| {
        on_thumb_pointer_down.run((x, y));
    });

    let dir = context.dir;

    view! {
        <AttributeInterceptor let:attrs>
            {view! {
                <ScrollAreaScrollbarImpl
                    data_orientation="horizontal"
                    sizes=sizes
                    has_thumb=has_thumb
                    on_thumb_change=on_thumb_change
                    on_thumb_pointer_up=on_thumb_pointer_up
                    on_thumb_pointer_down=on_thumb_pointer_down_impl
                    on_thumb_position_change=on_thumb_position_change
                    on_wheel_scroll=on_wheel_scroll_impl
                    on_drag_scroll=on_drag_scroll_impl
                    on_resize=on_resize
                    style=Signal::derive(move || {
                        let d = dir.get();
                        let thumb_width = get_thumb_size(&sizes.get());
                        format!(
                            "bottom: 0; {}: var(--radix-scroll-area-corner-width); {}: 0; --radix-scroll-area-thumb-width: {}px;",
                            if d == Direction::Rtl { "left" } else { "right" },
                            if d == Direction::Rtl { "right" } else { "left" },
                            thumb_width
                        )
                    })
                    as_child=as_child
                    node_ref=composed_ref
                >
                    {children.with_value(|children| children())}
                </ScrollAreaScrollbarImpl>
            }.add_any_attr(attrs)}
        </AttributeInterceptor>
    }
}

#[component]
fn ScrollAreaScrollbarY(
    sizes: RwSignal<Sizes>,
    has_thumb: Signal<bool>,
    on_thumb_change: Callback<Option<SendWrapper<web_sys::HtmlElement>>>,
    on_thumb_pointer_up: Callback<()>,
    on_thumb_pointer_down: Callback<(f64, f64)>,
    on_thumb_position_change: Callback<()>,
    on_wheel_scroll: Callback<f64>,
    on_drag_scroll: Callback<f64>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context = expect_context::<ScrollAreaContextValue>();
    let computed_style: RwSignal<Option<SendWrapper<web_sys::CssStyleDeclaration>>> =
        RwSignal::new(None);

    let scrollbar_ref = AnyNodeRef::new();
    let composed_ref = use_composed_refs(vec![node_ref, scrollbar_ref]);

    // Set computed style when ref is available
    Effect::new(move |_| {
        if let Some(node) = scrollbar_ref.get() {
            let node: &web_sys::HtmlElement = node.unchecked_ref();
            let window = web_sys::window().expect("Window should exist.");
            if let Ok(style) = window.get_computed_style(node) {
                computed_style.set(style.map(SendWrapper::new));
            }
        }
    });

    // Set scrollbar Y element in context
    Effect::new(move |_| {
        if let Some(node) = scrollbar_ref.get() {
            let el: web_sys::HtmlElement = node.unchecked_into();
            context.scrollbar_y.set(Some(SendWrapper::new(el)));
        }
    });
    Owner::on_cleanup(move || {
        context.scrollbar_y.set(None);
    });

    let on_resize = Callback::new(move |()| {
        if let Some(node) = scrollbar_ref.get_untracked()
            && let Some(viewport) = context.viewport.get_untracked()
            && let Some(cs) = computed_style.get_untracked()
        {
            let scrollbar_el: &web_sys::HtmlElement = node.unchecked_ref();
            sizes.set(Sizes {
                content: viewport.scroll_height() as f64,
                viewport: viewport.offset_height() as f64,
                scrollbar: ScrollbarSizes {
                    size: scrollbar_el.client_height() as f64,
                    padding_start: to_int(
                        &cs.get_property_value("padding-top").unwrap_or_default(),
                    ),
                    padding_end: to_int(
                        &cs.get_property_value("padding-bottom").unwrap_or_default(),
                    ),
                },
            });
        }
    });

    let on_wheel_scroll_impl = Callback::new(
        move |(event, max_scroll_pos): (SendWrapper<web_sys::WheelEvent>, f64)| {
            if let Some(viewport) = context.viewport.get_untracked() {
                let scroll_pos = viewport.scroll_top() as f64 + event.delta_y();
                on_wheel_scroll.run(scroll_pos);
                if is_scrolling_within_scrollbar_bounds(scroll_pos, max_scroll_pos) {
                    event.prevent_default();
                }
            }
        },
    );

    let on_drag_scroll_impl = Callback::new(move |(_x, y): (f64, f64)| {
        on_drag_scroll.run(y);
    });

    let on_thumb_pointer_down_impl = Callback::new(move |(x, y): (f64, f64)| {
        on_thumb_pointer_down.run((x, y));
    });

    let dir = context.dir;

    view! {
        <AttributeInterceptor let:attrs>
            {view! {
                <ScrollAreaScrollbarImpl
                    data_orientation="vertical"
                    sizes=sizes
                    has_thumb=has_thumb
                    on_thumb_change=on_thumb_change
                    on_thumb_pointer_up=on_thumb_pointer_up
                    on_thumb_pointer_down=on_thumb_pointer_down_impl
                    on_thumb_position_change=on_thumb_position_change
                    on_wheel_scroll=on_wheel_scroll_impl
                    on_drag_scroll=on_drag_scroll_impl
                    on_resize=on_resize
                    style=Signal::derive(move || {
                        let d = dir.get();
                        let thumb_height = get_thumb_size(&sizes.get());
                        format!(
                            "top: 0; {}: 0; bottom: var(--radix-scroll-area-corner-height); --radix-scroll-area-thumb-height: {}px;",
                            if d == Direction::Ltr { "right" } else { "left" },
                            thumb_height
                        )
                    })
                    as_child=as_child
                    node_ref=composed_ref
                >
                    {children.with_value(|children| children())}
                </ScrollAreaScrollbarImpl>
            }.add_any_attr(attrs)}
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ScrollAreaScrollbarImpl
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn ScrollAreaScrollbarImpl(
    #[prop(into)] data_orientation: &'static str,
    sizes: RwSignal<Sizes>,
    has_thumb: Signal<bool>,
    on_thumb_change: Callback<Option<SendWrapper<web_sys::HtmlElement>>>,
    on_thumb_pointer_up: Callback<()>,
    on_thumb_pointer_down: Callback<(f64, f64)>,
    on_thumb_position_change: Callback<()>,
    on_wheel_scroll: Callback<(SendWrapper<web_sys::WheelEvent>, f64)>,
    on_drag_scroll: Callback<(f64, f64)>,
    on_resize: Callback<()>,
    #[prop(into)] style: Signal<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context = expect_context::<ScrollAreaContextValue>();

    // Read forwarded class from public ScrollAreaScrollbar component (StoredValue is Copy,
    // allowing capture by Fn closures inside the view! macro / AttributeInterceptor).
    let forwarded_class = StoredValue::new(
        use_context::<ForwardedScrollbarClass>()
            .and_then(|c| c.0)
            .unwrap_or_default(),
    );

    let scrollbar: RwSignal<Option<SendWrapper<web_sys::HtmlElement>>> = RwSignal::new(None);
    let scrollbar_ref = AnyNodeRef::new();
    let composed_ref = use_composed_refs(vec![node_ref, scrollbar_ref]);

    // Track scrollbar element
    Effect::new(move |_| {
        if let Some(node) = scrollbar_ref.get() {
            let el: web_sys::HtmlElement = node.unchecked_into();
            scrollbar.set(Some(SendWrapper::new(el)));
        } else {
            scrollbar.set(None);
        }
    });

    let rect: StoredValue<Option<SendWrapper<web_sys::DomRect>>> = StoredValue::new(None);
    let prev_webkit_user_select: StoredValue<String> = StoredValue::new(String::new());

    let max_scroll_pos = Memo::new(move |_| {
        let s = sizes.get();
        s.content - s.viewport
    });

    // Document-level wheel listener (passive: false to allow preventDefault)
    Effect::new(move |_| {
        let _scrollbar = scrollbar.get();
        let _max = max_scroll_pos.get();

        let on_wheel_scroll = on_wheel_scroll;
        let scrollbar = scrollbar;
        let wheel_closure = SendWrapper::new(Closure::<dyn Fn(web_sys::WheelEvent)>::new(
            move |event: web_sys::WheelEvent| {
                if let Some(target) = event.target()
                    && let Some(sb) = scrollbar.get_untracked()
                {
                    let target_el: &web_sys::Element = target.unchecked_ref();
                    if sb.contains(Some(target_el)) {
                        on_wheel_scroll
                            .run((SendWrapper::new(event), max_scroll_pos.get_untracked()));
                    }
                }
            },
        ));

        let document = web_sys::window()
            .expect("Window should exist.")
            .document()
            .expect("Document should exist.");

        let options = web_sys::AddEventListenerOptions::new();
        options.set_passive(false);
        document
            .add_event_listener_with_callback_and_add_event_listener_options(
                "wheel",
                wheel_closure.as_ref().unchecked_ref(),
                &options,
            )
            .ok();

        let document = SendWrapper::new(document);
        Owner::on_cleanup(move || {
            document
                .remove_event_listener_with_callback(
                    "wheel",
                    wheel_closure.as_ref().unchecked_ref(),
                )
                .ok();
        });
    });

    // Update thumb position when sizes change
    Effect::new(move |_| {
        let _ = sizes.get();
        on_thumb_position_change.run(());
    });

    // Resize observers
    let handle_resize = use_debounce_callback(move || on_resize.run(()), 10);
    use_resize_observer(Signal::derive(move || scrollbar.get()), move || {
        handle_resize.run(())
    });
    use_resize_observer(Signal::derive(move || context.content.get()), move || {
        handle_resize.run(())
    });

    // Provide scrollbar context
    let scrollbar_context = ScrollbarContextValue {
        has_thumb,
        scrollbar,
        on_thumb_change,
        on_thumb_pointer_up,
        on_thumb_pointer_down,
        on_thumb_position_change,
    };

    view! {
        <Provider value=scrollbar_context>
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=composed_ref
                attr:class=forwarded_class.get_value()
                attr:data-orientation=data_orientation
                attr:style=move || format!("position: absolute; {}", style.get())
                on:pointerdown=move |event: ev::PointerEvent| {
                    let main_pointer = 0;
                    if event.button() == main_pointer {
                        if let Some(target) = event.target() {
                            let element: web_sys::HtmlElement = target.unchecked_into();
                            element.set_pointer_capture(event.pointer_id()).ok();
                        }
                        if let Some(sb) = scrollbar.get_untracked() {
                            rect.set_value(Some(SendWrapper::new(sb.get_bounding_client_rect())));
                        }
                        // Pointer capture doesn't prevent text selection in Safari
                        if let Some(body) = document().body() {
                            prev_webkit_user_select.set_value(
                                body.style().get_property_value("webkitUserSelect").unwrap_or_default()
                            );
                            body.style().set_property("webkitUserSelect", "none").ok();
                        }
                        if let Some(viewport) = context.viewport.get_untracked() {
                            viewport.style().set_property("scroll-behavior", "auto").ok();
                        }
                        // Handle drag scroll
                        if let Some(r) = rect.get_value() {
                            let x = event.client_x() as f64 - r.left();
                            let y = event.client_y() as f64 - r.top();
                            on_drag_scroll.run((x, y));
                        }
                    }
                }
                on:pointermove=move |event: ev::PointerEvent| {
                    if let Some(r) = rect.get_value() {
                        let x = event.client_x() as f64 - r.left();
                        let y = event.client_y() as f64 - r.top();
                        on_drag_scroll.run((x, y));
                    }
                }
                on:pointerup=move |event: ev::PointerEvent| {
                    if let Some(target) = event.target() {
                        let element: web_sys::HtmlElement = target.unchecked_into();
                        if element.has_pointer_capture(event.pointer_id()) {
                            element.release_pointer_capture(event.pointer_id()).ok();
                        }
                    }
                    if let Some(body) = document().body() {
                        body.style().set_property("webkitUserSelect", &prev_webkit_user_select.get_value()).ok();
                    }
                    if let Some(viewport) = context.viewport.get_untracked() {
                        viewport.style().set_property("scroll-behavior", "").ok();
                    }
                    rect.set_value(None);
                }
                {..attrs}
            >
                {children.with_value(|children| children())}
            </Primitive>
        </AttributeInterceptor>
        </Provider>
    }
}
