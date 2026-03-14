use super::*;

#[component]
pub fn PasswordToggleField(
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] visible: MaybeProp<bool>,
    #[prop(into, optional)] default_visible: MaybeProp<bool>,
    #[prop(into, optional)] on_visibility_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let base_id = use_id(id.get());
    let default_input_id = Signal::derive(move || format!("{}-input", base_id.get()));

    let (input_id_state, set_input_id_state) = signal::<Option<String>>(None);
    let input_id = Signal::derive(move || {
        input_id_state
            .get()
            .unwrap_or_else(|| default_input_id.get())
    });

    let sync_input_id = Callback::new(move |provided_id: Option<String>| {
        set_input_id_state.set(provided_id);
    });

    let (visible_signal, set_visible) = use_controllable_state(UseControllableStateParams {
        prop: visible,
        default_prop: MaybeProp::derive(move || Some(default_visible.get().unwrap_or(false))),
        on_change: adapt_callback(on_visibility_change),
    });
    let visible = Signal::derive(move || visible_signal.get().unwrap_or(false));

    let input_ref = AnyNodeRef::new();
    let focus_state = StoredValue::new(INITIAL_FOCUS_STATE);

    let context = PasswordToggleFieldContextValue {
        input_id,
        input_ref,
        visible,
        set_visible: Callback::new(move |value: Option<bool>| {
            set_visible.run(value);
        }),
        sync_input_id,
        focus_state,
    };

    view! {
        <Provider value=context>
            {children.with_value(|children| children())}
        </Provider>
    }
}
