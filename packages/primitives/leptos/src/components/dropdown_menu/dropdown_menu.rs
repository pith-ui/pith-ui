use super::*;

/* -------------------------------------------------------------------------------------------------
 * DropdownMenu
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn DropdownMenu(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] modal: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let modal = prop_or(modal, true);
    let trigger_ref = AnyNodeRef::new();

    let (open_state, set_open_state) = use_controllable_state(UseControllableStateParams {
        prop: open,
        default_prop: default_open,
        on_change: on_open_change.map(|on_open_change| {
            Callback::new(move |value: Option<bool>| {
                if let Some(value) = value {
                    on_open_change.run(value);
                }
            })
        }),
    });

    let open_signal = Signal::derive(move || open_state.get().unwrap_or(false));

    let trigger_id = use_id(None);
    let content_id = use_id(None);

    let context = DropdownMenuContextValue {
        trigger_id,
        trigger_ref,
        content_id,
        open: open_signal,
        on_open_change: Callback::new(move |value: bool| {
            set_open_state.run(Some(value));
        }),
        on_open_toggle: Callback::new(move |_| {
            set_open_state.run(Some(!open_signal.get_untracked()));
        }),
        modal,
    };

    view! {
        <Provider value=context>
            <Menu
                open=open_signal
                on_open_change=Callback::new(move |value: bool| {
                    set_open_state.run(Some(value));
                })
                dir=dir
                modal=modal
            >
                {children.with_value(|children| children())}
            </Menu>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * DropdownMenuTrigger
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn DropdownMenuTrigger(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context = expect_context::<DropdownMenuContextValue>();
    let composed_refs = use_composed_refs(vec![node_ref, context.trigger_ref]);
    let disabled = prop_or_default(disabled);

    view! {
        <MenuAnchor as_child=true>
            <Primitive
                element=html::button
                as_child=as_child
                node_ref=composed_refs
                attr:r#type="button"
                attr:id=move || context.trigger_id.get()
                attr:aria-haspopup="menu"
                attr:aria-expanded=move || context.open.get().to_string()
                attr:aria-controls=move || context.open.get().then(|| context.content_id.get())
                attr:data-state=move || if context.open.get() { "open" } else { "closed" }
                attr:data-disabled=data_attr(disabled)
                attr:disabled=data_attr(disabled)
                on:pointerdown=move |event: ev::PointerEvent| {
                    // Only call handler if it's the left button (mousedown gets triggered by all mouse buttons)
                    // but not when the control key is pressed (avoiding MacOS right click).
                    if !disabled.get_untracked() && event.button() == 0 && !event.ctrl_key() {
                        context.on_open_toggle.run(());
                        // Prevent trigger focusing when opening.
                        // This allows the content to be given focus without competition.
                        if !context.open.get_untracked() {
                            event.prevent_default();
                        }
                    }
                }
                on:keydown=move |event: ev::KeyboardEvent| {
                    if disabled.get_untracked() {
                        return;
                    }
                    if event.key() == "Enter" || event.key() == " " {
                        context.on_open_toggle.run(());
                    }
                    if event.key() == "ArrowDown" {
                        context.on_open_change.run(true);
                    }
                    // Prevent keydown from scrolling window / first focused item to execute
                    // that keydown (inadvertently closing the menu).
                    if event.key() == "Enter" || event.key() == " " || event.key() == "ArrowDown" {
                        event.prevent_default();
                    }
                }
            >
                {children.with_value(|children| children())}
            </Primitive>
        </MenuAnchor>
    }
}
