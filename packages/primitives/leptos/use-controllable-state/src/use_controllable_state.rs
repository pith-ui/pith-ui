use leptos::prelude::*;

pub struct UseControllableStateParams<T: Send + Sync + 'static> {
    pub prop: MaybeProp<T>,
    pub default_prop: MaybeProp<T>,
    pub on_change: Option<Callback<Option<T>>>,
}

pub fn use_controllable_state<T: Clone + PartialEq + Send + Sync>(
    UseControllableStateParams {
        prop,
        default_prop,
        on_change,
    }: UseControllableStateParams<T>,
) -> (Signal<Option<T>>, Callback<Option<T>>) {
    let (uncontrolled_prop, set_uncontrolled_prop) =
        use_uncontrolled_state(UseUncontrollableStateParams {
            default_prop,
            on_change,
        });
    let is_controlled = Signal::derive(move || prop.get().is_some());
    let value = Signal::derive(move || {
        if is_controlled.get() {
            prop.get()
        } else {
            uncontrolled_prop.get()
        }
    });

    // Use get_untracked() because this callback runs inside event handlers
    // (non-reactive context). We only need the current value, not reactive tracking.
    let set_value = Callback::new(move |next_value| {
        if prop.get_untracked().is_some() {
            if next_value != prop.get_untracked()
                && let Some(on_change) = on_change
            {
                on_change.run(next_value);
            }
        } else {
            set_uncontrolled_prop.set(next_value);
        }
    });

    (value, set_value)
}

pub struct UseUncontrollableStateParams<T: Send + Sync + 'static> {
    pub default_prop: MaybeProp<T>,
    pub on_change: Option<Callback<Option<T>>>,
}

fn use_uncontrolled_state<T: Clone + PartialEq + Send + Sync>(
    UseUncontrollableStateParams {
        default_prop,
        on_change,
    }: UseUncontrollableStateParams<T>,
) -> (ReadSignal<Option<T>>, WriteSignal<Option<T>>) {
    let uncontrolled_state = signal::<Option<T>>(default_prop.get());
    let (value, _) = uncontrolled_state;
    let prev_value = RwSignal::new(value.get_untracked());

    Effect::new(move |_| {
        let value = value.get();
        if prev_value.get() != value
            && let Some(on_change) = on_change
        {
            on_change.run(value.clone());
            prev_value.set(value);
        }
    });

    uncontrolled_state
}
