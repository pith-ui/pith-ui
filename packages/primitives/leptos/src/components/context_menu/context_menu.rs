use super::*;

/* -------------------------------------------------------------------------------------------------
 * ContextMenu
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ContextMenu(
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] modal: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let modal = prop_or(modal, true);
    let (open, set_open) = signal(false);
    let open_signal = Signal::derive(move || open.get());

    let handle_open_change = Callback::new(move |value: bool| {
        set_open.set(value);
        if let Some(on_open_change) = on_open_change {
            on_open_change.run(value);
        }
    });

    let context = ContextMenuContextValue {
        open: open_signal,
        on_open_change: handle_open_change,
        modal,
    };

    view! {
        <Provider value=context>
            <Menu
                open=open_signal
                on_open_change=handle_open_change
                dir=dir
                modal=modal
            >
                {children.with_value(|children| children())}
            </Menu>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ContextMenuTrigger
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ContextMenuTrigger(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context = expect_context::<ContextMenuContextValue>();
    let disabled = prop_or_default(disabled);
    let point = RwSignal::new(Point::default());
    let long_press_timer = RwSignal::new(0i32);

    let clear_long_press = move || {
        web_sys::window()
            .expect("Window should exist.")
            .clear_timeout_with_handle(long_press_timer.get_untracked());
    };

    // Clear timer on unmount.
    on_cleanup(move || {
        clear_long_press();
    });

    // Clear timer when disabled.
    Effect::new(move |_| {
        if disabled.get() {
            clear_long_press();
        }
    });

    // Set a virtual element as the Popper anchor so no extra DOM element is needed.
    // This mirrors React's virtualRef pattern. The Effect tracks point changes and
    // creates a new PointVirtualElement with the updated coordinates, which floating-ui
    // detects via PartialEq and uses to re-position.
    Effect::new(move |_| {
        let p = point.get();
        set_popper_virtual_ref(Box::new(PointVirtualElement { x: p.x, y: p.y }));
    });

    view! {
        <Primitive
            element=html::span
            as_child=as_child
            node_ref=node_ref
            style:-webkit-touch-callout="none"
            attr:data-state=move || if context.open.get() { "open" } else { "closed" }
            attr:data-disabled=data_attr(disabled)
            on:contextmenu=move |event: ev::MouseEvent| {
                if disabled.get_untracked() {
                    return;
                }
                // Clear the long press here because some platforms already support
                // long press to trigger a contextmenu event.
                clear_long_press();
                point.set(Point {
                    x: event.client_x() as f64,
                    y: event.client_y() as f64,
                });
                context.on_open_change.run(true);
                event.prevent_default();
            }
            on:pointerdown=move |event: ev::PointerEvent| {
                if disabled.get_untracked() || event.pointer_type() == "mouse" {
                    return;
                }
                // Clear the long press here in case there's multiple touch points.
                clear_long_press();
                let client_x = event.client_x();
                let client_y = event.client_y();
                let on_open_change = context.on_open_change;
                let timer_cb = Closure::once_into_js(move || {
                    point.set(Point {
                        x: client_x as f64,
                        y: client_y as f64,
                    });
                    on_open_change.run(true);
                });
                let timer_id = web_sys::window()
                    .expect("Window should exist.")
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        timer_cb.unchecked_ref(),
                        700,
                    )
                    .expect("Timeout should be set.");
                long_press_timer.set(timer_id);
            }
            on:pointermove=move |event: ev::PointerEvent| {
                if disabled.get_untracked() || event.pointer_type() == "mouse" {
                    return;
                }
                clear_long_press();
            }
            on:pointercancel=move |event: ev::PointerEvent| {
                if disabled.get_untracked() || event.pointer_type() == "mouse" {
                    return;
                }
                clear_long_press();
            }
            on:pointerup=move |event: ev::PointerEvent| {
                if disabled.get_untracked() || event.pointer_type() == "mouse" {
                    return;
                }
                clear_long_press();
            }
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}
