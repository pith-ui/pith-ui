use super::*;

/* -------------------------------------------------------------------------------------------------
 * SelectScrollUpButton
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn SelectScrollUpButton(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let content_context = expect_context::<SelectContentContextValue>();

    let (can_scroll_up, set_can_scroll_up) = signal(false);

    Effect::new(move |_| {
        if content_context.is_positioned.get()
            && let Some(viewport_el) = content_context.viewport_ref.get()
        {
            let viewport: web_sys::HtmlElement = (*viewport_el).clone().unchecked_into();
            let viewport_clone = viewport.clone();

            let handle_scroll: Closure<dyn FnMut()> = Closure::new(move || {
                set_can_scroll_up.set(viewport_clone.scroll_top() > 0);
            });
            let scroll_fn: js_sys::Function = handle_scroll.into_js_value().unchecked_into();
            let scroll_fn_cleanup = SendWrapper::new(scroll_fn.clone());

            // Initial check
            set_can_scroll_up.set(viewport.scroll_top() > 0);

            let _ = viewport.add_event_listener_with_callback("scroll", &scroll_fn);

            let viewport_cleanup = SendWrapper::new(viewport.clone());
            on_cleanup(move || {
                let _ = viewport_cleanup
                    .remove_event_listener_with_callback("scroll", &scroll_fn_cleanup);
            });
        }
    });

    let viewport_ref = content_context.viewport_ref;

    view! {
        <Show when=move || can_scroll_up.get()>
            <SelectScrollButtonImpl
                as_child=as_child
                node_ref=node_ref
                on_auto_scroll=Callback::new(move |_: ()| {
                    if let Some(viewport_el) = viewport_ref.get_untracked() {
                        let viewport: web_sys::HtmlElement = (*viewport_el).clone().unchecked_into();
                        let item_height = item_offset_height(&viewport);
                        viewport.set_scroll_top(viewport.scroll_top() - item_height);
                    }
                })
            >
                {children.try_with_value(|children| children.as_ref().map(|c| c()))}
            </SelectScrollButtonImpl>
        </Show>
    }
}

/* -------------------------------------------------------------------------------------------------
 * SelectScrollDownButton
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn SelectScrollDownButton(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let content_context = expect_context::<SelectContentContextValue>();

    let (can_scroll_down, set_can_scroll_down) = signal(false);

    Effect::new(move |_| {
        if content_context.is_positioned.get()
            && let Some(viewport_el) = content_context.viewport_ref.get()
        {
            let viewport: web_sys::HtmlElement = (*viewport_el).clone().unchecked_into();
            let viewport_clone = viewport.clone();

            let handle_scroll: Closure<dyn FnMut()> = Closure::new(move || {
                set_can_scroll_down.set(is_scrollable_down(
                    viewport_clone.scroll_top(),
                    viewport_clone.scroll_height(),
                    viewport_clone.client_height(),
                ));
            });
            let scroll_fn: js_sys::Function = handle_scroll.into_js_value().unchecked_into();
            let scroll_fn_cleanup = SendWrapper::new(scroll_fn.clone());

            // Initial check
            set_can_scroll_down.set(is_scrollable_down(
                viewport.scroll_top(),
                viewport.scroll_height(),
                viewport.client_height(),
            ));

            let _ = viewport.add_event_listener_with_callback("scroll", &scroll_fn);

            let viewport_cleanup = SendWrapper::new(viewport.clone());
            on_cleanup(move || {
                let _ = viewport_cleanup
                    .remove_event_listener_with_callback("scroll", &scroll_fn_cleanup);
            });
        }
    });

    let viewport_ref = content_context.viewport_ref;

    view! {
        <Show when=move || can_scroll_down.get()>
            <SelectScrollButtonImpl
                as_child=as_child
                node_ref=node_ref
                on_auto_scroll=Callback::new(move |_: ()| {
                    if let Some(viewport_el) = viewport_ref.get_untracked() {
                        let viewport: web_sys::HtmlElement = (*viewport_el).clone().unchecked_into();
                        let item_height = item_offset_height(&viewport);
                        viewport.set_scroll_top(viewport.scroll_top() + item_height);
                    }
                })
            >
                {children.try_with_value(|children| children.as_ref().map(|c| c()))}
            </SelectScrollButtonImpl>
        </Show>
    }
}

/* -------------------------------------------------------------------------------------------------
 * SelectScrollButtonImpl
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn SelectScrollButtonImpl(
    on_auto_scroll: Callback<()>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let content_context = expect_context::<SelectContentContextValue>();
    let auto_scroll_timer_ref: StoredValue<Option<i32>> = StoredValue::new(None);

    let clear_auto_scroll_timer = move || {
        if let Some(timer_id) = auto_scroll_timer_ref.try_get_value().flatten() {
            web_sys::window()
                .expect("Window should exist.")
                .clear_interval_with_handle(timer_id);
            let _ = auto_scroll_timer_ref.try_set_value(None);
        }
    };

    on_cleanup(clear_auto_scroll_timer);

    // When mounted, scroll active item into view
    let get_items = StoredValue::new(use_collection::<SelectItemData>());
    Effect::new(move |_| {
        let _ = get_items.try_with_value(|get_items| {
            let items = get_items();
            let active = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.active_element());
            if let Some(active) = active {
                for item in &items {
                    if let Some(el) = item.r#ref.get() {
                        let el: &web_sys::Element = (*el).unchecked_ref();
                        if el == &active {
                            let el: &web_sys::HtmlElement = el.unchecked_ref();
                            let options = web_sys::ScrollIntoViewOptions::new();
                            options.set_block(web_sys::ScrollLogicalPosition::Nearest);
                            el.scroll_into_view_with_scroll_into_view_options(&options);
                            break;
                        }
                    }
                }
            }
        });
    });

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                style:flex-shrink="0"
                attr:aria-hidden="true"
                on:pointerdown=move |_: ev::PointerEvent| {
                    if auto_scroll_timer_ref.try_get_value().flatten().is_none() {
                        let timer_id = web_sys::window()
                            .expect("Window should exist.")
                            .set_interval_with_callback_and_timeout_and_arguments_0(
                                Closure::wrap(Box::new(move || {
                                    on_auto_scroll.run(());
                                }) as Box<dyn FnMut()>)
                                    .into_js_value()
                                    .unchecked_ref(),
                                50,
                            )
                            .expect("setInterval should succeed.");
                        let _ = auto_scroll_timer_ref.try_set_value(Some(timer_id));
                    }
                }
                on:pointermove=move |_: ev::PointerEvent| {
                    content_context.on_item_leave.run(());
                    if auto_scroll_timer_ref.try_get_value().flatten().is_none() {
                        let timer_id = web_sys::window()
                            .expect("Window should exist.")
                            .set_interval_with_callback_and_timeout_and_arguments_0(
                                Closure::wrap(Box::new(move || {
                                    on_auto_scroll.run(());
                                }) as Box<dyn FnMut()>)
                                    .into_js_value()
                                    .unchecked_ref(),
                                50,
                            )
                            .expect("setInterval should succeed.");
                        let _ = auto_scroll_timer_ref.try_set_value(Some(timer_id));
                    }
                }
                on:pointerleave=move |_: ev::PointerEvent| {
                    clear_auto_scroll_timer();
                }
                {..attrs}
            >
                {children.try_with_value(|children| children.as_ref().map(|c| c()))}
            </Primitive>
        </AttributeInterceptor>
    }
}

/// Get the offset height of the first `[role="option"]` child in the viewport,
/// falling back to 32px. Matches React's `selectedItem.offsetHeight`.
fn item_offset_height(viewport: &web_sys::HtmlElement) -> i32 {
    viewport
        .query_selector("[role='option']")
        .ok()
        .flatten()
        .map(|el| {
            let el: &web_sys::HtmlElement = el.unchecked_ref();
            el.offset_height()
        })
        .unwrap_or(32)
}
