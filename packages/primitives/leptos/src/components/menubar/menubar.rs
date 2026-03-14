use super::*;

/* -------------------------------------------------------------------------------------------------
 * Menubar
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn Menubar(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] r#loop: MaybeProp<bool>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let direction = use_direction(dir);
    let r#loop = prop_or(r#loop, true);

    let (value_state, set_value_state) = use_controllable_state(UseControllableStateParams {
        prop: value,
        default_prop: default_value,
        on_change: adapt_callback(on_value_change),
    });

    let value_signal = Signal::derive(move || value_state.get().unwrap_or_default());

    // We need to manage tab stop id manually as `RovingFocusGroup` updates the stop
    // based on focus, and in some situations our triggers won't ever be given focus
    // (e.g. click to open and then outside to close)
    let current_tab_stop_id = RwSignal::new(None::<String>);

    let context = MenubarContextValue {
        value: value_signal,
        dir: direction,
        r#loop,
        on_menu_open: Callback::new(move |value: String| {
            set_value_state.run(Some(value.clone()));
            current_tab_stop_id.set(Some(value));
        }),
        on_menu_close: Callback::new(move |_: ()| {
            set_value_state.run(Some(String::new()));
        }),
        on_menu_toggle: Callback::new(move |value: String| {
            let prev = value_signal.get_untracked();
            let new_val = if !prev.is_empty() {
                String::new()
            } else {
                value.clone()
            };
            set_value_state.run(Some(new_val));
            // `onMenuOpen` and `onMenuToggle` are called exclusively so we
            // need to update the id in either case.
            current_tab_stop_id.set(Some(value));
        }),
    };

    view! {
        <Provider value=context>
            <CollectionProvider item_data_type=ITEM_DATA_PHANTOM>
                <CollectionSlot item_data_type=ITEM_DATA_PHANTOM>
                    <RovingFocusGroup
                        as_child=true
                        orientation=Orientation::Horizontal
                        r#loop=r#loop
                        dir=direction
                        current_tab_stop_id=Signal::derive(move || current_tab_stop_id.get())
                        on_current_tab_stop_id_change=Callback::new(move |id: Option<String>| {
                            current_tab_stop_id.set(id);
                        })
                    >
                        <Primitive
                            element=html::div
                            as_child=as_child
                            node_ref=node_ref
                            attr:role="menubar"
                        >
                            {children.with_value(|children| children())}
                        </Primitive>
                    </RovingFocusGroup>
                </CollectionSlot>
            </CollectionProvider>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * MenubarMenu
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn MenubarMenu(
    #[prop(into, optional)] value: Option<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let auto_value = use_id(None);
    // We need to provide an initial deterministic value as `use_id` will return
    // empty string on the first render and we don't want to match our internal "closed" value.
    let value = value.unwrap_or_else(|| auto_value.get_untracked());
    let context = expect_context::<MenubarContextValue>();
    let trigger_ref = AnyNodeRef::new();
    let was_keyboard_trigger_open_ref = SendWrapper::new(Rc::new(Cell::new(false)));
    let open = {
        let value = value.clone();
        Signal::derive(move || context.value.get() == value)
    };

    let trigger_id = use_id(None);
    let content_id = use_id(None);

    {
        let was_keyboard_trigger_open_ref = was_keyboard_trigger_open_ref.clone();
        Effect::new(move |_| {
            if !open.get() {
                was_keyboard_trigger_open_ref.set(false);
            }
        });
    }

    let menu_context = MenubarMenuContextValue {
        value: value.clone(),
        trigger_id,
        trigger_ref,
        content_id,
        was_keyboard_trigger_open_ref,
    };

    view! {
        <Provider value=menu_context>
            <Menu
                open=open
                on_open_change=Callback::new(move |open: bool| {
                    // Menu only calls `onOpenChange` when dismissing so we
                    // want to close our MenuBar based on the same events.
                    if !open {
                        context.on_menu_close.run(());
                    }
                })
                modal=false
                dir=context.dir
            >
                {children.with_value(|children| children())}
            </Menu>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * MenubarTrigger
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn MenubarTrigger(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context = expect_context::<MenubarContextValue>();
    let menu_context = expect_context::<MenubarMenuContextValue>();
    let trigger_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, trigger_ref, menu_context.trigger_ref]);

    let disabled = prop_or_default(disabled);
    let is_focused = RwSignal::new(false);

    let value = StoredValue::new(menu_context.value.clone());
    let open = Signal::derive(move || context.value.get() == value.get_value());

    let was_keyboard = StoredValue::new(menu_context.was_keyboard_trigger_open_ref.clone());

    view! {
        <CollectionItemSlot
            item_data_type=ITEM_DATA_PHANTOM
            item_data=Signal::derive(move || ItemData { value: value.get_value(), disabled: disabled.get() })
        >
            <RovingFocusGroupItem
                as_child=true
                focusable=Signal::derive(move || !disabled.get())
                tab_stop_id=Signal::derive(move || value.get_value())
            >
                <MenuAnchor as_child=true>
                    <Primitive
                        element=html::button
                        as_child=as_child
                        node_ref=composed_refs
                        attr:r#type="button"
                        attr:role="menuitem"
                        attr:id=move || menu_context.trigger_id.get()
                        attr:aria-haspopup="menu"
                        attr:aria-expanded=move || open.get().to_string()
                        attr:aria-controls=move || open.get().then(|| menu_context.content_id.get())
                        attr:data-highlighted=data_attr(is_focused.into())
                        attr:data-state=move || if open.get() { "open" } else { "closed" }
                        attr:data-disabled=data_attr(disabled)
                        attr:disabled=data_attr(disabled)
                        on:pointerdown=move |event: ev::PointerEvent| {
                            // only call handler if it's the left button (mousedown gets triggered by all mouse buttons)
                            // but not when the control key is pressed (avoiding MacOS right click)
                            if !disabled.get_untracked() && event.button() == 0 && !event.ctrl_key() {
                                context.on_menu_open.run(value.get_value());
                                // prevent trigger focusing when opening
                                // this allows the content to be given focus without competition
                                if !open.get_untracked() {
                                    event.prevent_default();
                                }
                            }
                        }
                        on:pointerenter=move |_event: ev::PointerEvent| {
                            let menubar_open = !context.value.get_untracked().is_empty();
                            if menubar_open && !open.get_untracked() {
                                context.on_menu_open.run(value.get_value());
                                if let Some(el) = trigger_ref.get() {
                                    let el: web_sys::HtmlElement = el.unchecked_into();
                                    el.focus().ok();
                                }
                            }
                        }
                        on:keydown=move |event: ev::KeyboardEvent| {
                            if disabled.get_untracked() {
                                return;
                            }
                            if event.key() == "Enter" || event.key() == " " {
                                context.on_menu_toggle.run(value.get_value());
                            }
                            if event.key() == "ArrowDown" {
                                context.on_menu_open.run(value.get_value());
                            }
                            // prevent keydown from scrolling window / first focused item to execute
                            // that keydown (inadvertently closing the menu)
                            if event.key() == "Enter" || event.key() == " " || event.key() == "ArrowDown" {
                                was_keyboard.with_value(|w| w.set(true));
                                event.prevent_default();
                            }
                        }
                        on:focus=move |_| is_focused.set(true)
                        on:blur=move |_| is_focused.set(false)
                    >
                        {children.with_value(|children| children())}
                    </Primitive>
                </MenuAnchor>
            </RovingFocusGroupItem>
        </CollectionItemSlot>
    }
}
