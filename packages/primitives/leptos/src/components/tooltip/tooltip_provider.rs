use super::*;

/* -------------------------------------------------------------------------------------------------
 * TooltipProvider
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn TooltipProvider(
    #[prop(into, optional, default = MaybeProp::from(DEFAULT_DELAY_DURATION))]
    delay_duration: MaybeProp<f64>,
    #[prop(into, optional, default = MaybeProp::from(300.0))] skip_delay_duration: MaybeProp<f64>,
    #[prop(into, optional)] disable_hoverable_content: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let is_open_delayed = RwSignal::new(true);
    let is_pointer_in_transit = RwSignal::new(false);
    let skip_delay_timer_ref: StoredValue<Option<i32>> = StoredValue::new(None);

    let delay_duration_signal = prop_or(delay_duration, DEFAULT_DELAY_DURATION);
    let disable_hoverable_content_signal = prop_or_default(disable_hoverable_content);

    let on_open = Callback::new(move |_: ()| {
        clear_timeout(skip_delay_timer_ref);
        is_open_delayed.set(false);
    });

    let on_close = Callback::new(move |_: ()| {
        clear_timeout(skip_delay_timer_ref);
        let skip_delay = skip_delay_duration.get().unwrap_or(300.0);
        let timeout_id = set_timeout(
            move || {
                is_open_delayed.set(true);
            },
            skip_delay as i32,
        );
        skip_delay_timer_ref.set_value(Some(timeout_id));
    });

    let on_pointer_in_transit_change = Callback::new(move |in_transit: bool| {
        is_pointer_in_transit.set(in_transit);
    });

    on_cleanup(move || {
        clear_timeout(skip_delay_timer_ref);
    });

    let context = TooltipProviderContextValue {
        is_open_delayed,
        delay_duration: delay_duration_signal,
        on_open,
        on_close,
        on_pointer_in_transit_change,
        is_pointer_in_transit,
        disable_hoverable_content: disable_hoverable_content_signal,
    };

    view! {
        <Provider value=context>
            {children.with_value(|children| children())}
        </Provider>
    }
}
