use super::*;

/* -------------------------------------------------------------------------------------------------
 * NavigationMenu
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn NavigationMenu(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional, default = MaybeProp::from(Orientation::Horizontal))]
    orientation: MaybeProp<Orientation>,
    #[prop(into, optional, default = MaybeProp::from(200.0))] delay_duration: MaybeProp<f64>,
    #[prop(into, optional, default = MaybeProp::from(300.0))] skip_delay_duration: MaybeProp<f64>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let direction = use_direction(dir);
    let orientation_signal = prop_or(orientation, Orientation::Horizontal);

    let open_timer_ref: StoredValue<Option<i32>> = StoredValue::new(None);
    let close_timer_ref: StoredValue<Option<i32>> = StoredValue::new(None);
    let skip_delay_timer_ref: StoredValue<Option<i32>> = StoredValue::new(None);
    let is_open_delayed = RwSignal::new(true);

    let (value_signal, set_value) = use_controllable_state(UseControllableStateParams {
        prop: value,
        default_prop: MaybeProp::derive(move || Some(default_value.get().unwrap_or_default())),
        on_change: Some(Callback::new(move |val: Option<String>| {
            let val = val.unwrap_or_default();
            let is_open = !val.is_empty();
            let has_skip_delay = skip_delay_duration.get().unwrap_or(300.0) > 0.0;
            let skip_dur = skip_delay_duration.get().unwrap_or(300.0) as i32;

            if is_open {
                clear_timeout(skip_delay_timer_ref);
                if has_skip_delay {
                    is_open_delayed.set(false);
                }
            } else {
                clear_timeout(skip_delay_timer_ref);
                let timeout_id = set_timeout(
                    move || {
                        is_open_delayed.set(true);
                    },
                    skip_dur,
                );
                skip_delay_timer_ref.set_value(Some(timeout_id));
            }

            if let Some(cb) = on_value_change {
                cb.run(val);
            }
        })),
    });

    let current_value = Signal::derive(move || value_signal.get().unwrap_or_default());

    let nav_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, nav_ref]);

    let handle_open = Callback::new(move |item_value: String| {
        clear_timeout(close_timer_ref);
        set_value.run(Some(item_value));
    });

    let start_close_timer = Callback::new(move |_: ()| {
        clear_timeout(close_timer_ref);
        let timeout_id = set_timeout(
            move || {
                set_value.run(Some(String::new()));
            },
            150,
        );
        close_timer_ref.set_value(Some(timeout_id));
    });

    let handle_delayed_open = Callback::new(move |item_value: String| {
        let is_open_item = current_value.get_untracked() == item_value;
        if is_open_item {
            clear_timeout(close_timer_ref);
        } else {
            let delay = delay_duration.get().unwrap_or(200.0) as i32;
            let timeout_id = set_timeout(
                move || {
                    clear_timeout(close_timer_ref);
                    set_value.run(Some(item_value));
                },
                delay,
            );
            open_timer_ref.set_value(Some(timeout_id));
        }
    });

    on_cleanup(move || {
        clear_timeout(open_timer_ref);
        clear_timeout(close_timer_ref);
        clear_timeout(skip_delay_timer_ref);
    });

    let on_trigger_enter = Callback::new(move |item_value: String| {
        clear_timeout(open_timer_ref);
        if is_open_delayed.get_untracked() {
            handle_delayed_open.run(item_value);
        } else {
            handle_open.run(item_value);
        }
    });

    let on_trigger_leave = Callback::new(move |_: ()| {
        clear_timeout(open_timer_ref);
        start_close_timer.run(());
    });

    let on_content_enter = Callback::new(move |_: ()| {
        clear_timeout(close_timer_ref);
    });

    let on_content_leave = start_close_timer;

    let on_item_select = Callback::new(move |item_value: String| {
        let prev = current_value.get_untracked();
        if prev == item_value {
            set_value.run(Some(String::new()));
        } else {
            set_value.run(Some(item_value));
        }
    });

    let on_item_dismiss = Callback::new(move |_: ()| {
        set_value.run(Some(String::new()));
    });

    view! {
        <NavigationMenuProvider
            is_root_menu=true
            value=current_value
            dir=direction
            orientation=orientation_signal
            root_navigation_menu=nav_ref
            on_trigger_enter=on_trigger_enter
            on_trigger_leave=on_trigger_leave
            on_content_enter=on_content_enter
            on_content_leave=on_content_leave
            on_item_select=on_item_select
            on_item_dismiss=on_item_dismiss
        >
            <Primitive
                element=html::nav
                as_child=as_child
                node_ref=composed_refs
                attr:aria-label="Main"
                attr:data-orientation=move || orientation_signal.get().to_string()
                attr:dir=move || direction.get().to_string()
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </Primitive>
        </NavigationMenuProvider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * NavigationMenuSub
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn NavigationMenuSub(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional, default = MaybeProp::from(Orientation::Horizontal))]
    orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<NavigationMenuContextValue>();
    let orientation_signal = prop_or(orientation, Orientation::Horizontal);

    let (value_signal, set_value) = use_controllable_state(UseControllableStateParams {
        prop: value,
        default_prop: MaybeProp::derive(move || Some(default_value.get().unwrap_or_default())),
        on_change: adapt_callback(on_value_change),
    });

    let current_value = Signal::derive(move || value_signal.get().unwrap_or_default());

    let on_trigger_enter = Callback::new(move |item_value: String| {
        set_value.run(Some(item_value));
    });

    let on_item_select = Callback::new(move |item_value: String| {
        set_value.run(Some(item_value));
    });

    let on_item_dismiss = Callback::new(move |_: ()| {
        set_value.run(Some(String::new()));
    });

    view! {
        <NavigationMenuProvider
            is_root_menu=false
            value=current_value
            dir=context.dir
            orientation=orientation_signal
            root_navigation_menu=context.root_navigation_menu
            on_trigger_enter=on_trigger_enter
            on_trigger_leave=Callback::new(|_: ()| {})
            on_content_enter=Callback::new(|_: ()| {})
            on_content_leave=Callback::new(|_: ()| {})
            on_item_select=on_item_select
            on_item_dismiss=on_item_dismiss
        >
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attr:data-orientation=move || orientation_signal.get().to_string()
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </Primitive>
        </NavigationMenuProvider>
    }
}
