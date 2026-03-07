use super::*;

/* -------------------------------------------------------------------------------------------------
 * ToastProvider
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ToastProvider(
    /// An author-localized label for each toast. Used to help screen reader users
    /// associate the interruption with a toast.
    #[prop(into, optional, default = "Notification".to_string())]
    label: String,
    /// Time in milliseconds that each toast should remain visible for.
    #[prop(into, optional, default = 5000.into())]
    duration: Signal<i32>,
    /// Direction of pointer swipe that should close the toast.
    #[prop(into, optional, default = Signal::derive(|| SwipeDirection::Right))]
    swipe_direction: Signal<SwipeDirection>,
    /// Distance in pixels that the swipe must pass before a close is triggered.
    #[prop(into, optional, default = 50.0.into())]
    swipe_threshold: Signal<f64>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let (viewport, on_viewport_change) = signal(None::<SendWrapper<web_sys::HtmlElement>>);
    let (toast_count, set_toast_count) = signal(0i32);

    let context_value = ToastProviderContextValue {
        label: StoredValue::new(label),
        duration,
        swipe_direction,
        swipe_threshold,
        toast_count,
        set_toast_count,
        viewport,
        on_viewport_change,
        is_focused_toast_escape_key_down_ref: StoredValue::new(false),
        is_close_paused_ref: StoredValue::new(false),
    };

    view! {
        <Provider value=context_value>
            <CollectionProvider item_data_type=ITEM_DATA_PHANTOM>
                {children.with_value(|children| children())}
            </CollectionProvider>
        </Provider>
    }
}
