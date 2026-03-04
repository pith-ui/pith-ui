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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicI32, AtomicU32, Ordering};

    use any_spawner::Executor;
    use leptos::reactive::owner::Owner;

    /// No-op executor that accepts (and drops) futures.
    /// Effects registered during tests won't run, which is fine — we only test
    /// the synchronous signal derivation and callback routing.
    struct NoopExecutor;

    impl any_spawner::CustomExecutor for NoopExecutor {
        fn spawn(&self, _fut: any_spawner::PinnedFuture<()>) {}
        fn spawn_local(&self, _fut: any_spawner::PinnedLocalFuture<()>) {}
        fn poll_local(&self) {}
    }

    /// Helper: run a closure inside a fresh reactive Owner.
    /// Initializes a no-op executor on first call (required when leptos `effects` feature is active).
    fn with_owner<T>(f: impl FnOnce() -> T) -> T {
        // Ignore error if already set (another test initialized it first).
        let _ = Executor::init_custom_executor(NoopExecutor);
        let owner = Owner::new_root(None);
        owner.with(f)
    }

    // ── Uncontrolled mode ────────────────────────────────────

    #[test]
    fn uncontrolled_uses_default_prop() {
        with_owner(|| {
            let (value, _set) = use_controllable_state(UseControllableStateParams {
                prop: MaybeProp::from(None::<i32>),
                default_prop: MaybeProp::from(Some(42)),
                on_change: None,
            });
            assert_eq!(value.get(), Some(42));
        });
    }

    #[test]
    fn uncontrolled_defaults_to_none_when_no_default() {
        with_owner(|| {
            let (value, _set) = use_controllable_state::<i32>(UseControllableStateParams {
                prop: MaybeProp::from(None::<i32>),
                default_prop: MaybeProp::from(None::<i32>),
                on_change: None,
            });
            assert_eq!(value.get(), None);
        });
    }

    #[test]
    fn uncontrolled_set_value_updates_state() {
        with_owner(|| {
            let (value, set_value) = use_controllable_state(UseControllableStateParams {
                prop: MaybeProp::from(None::<i32>),
                default_prop: MaybeProp::from(Some(0)),
                on_change: None,
            });
            assert_eq!(value.get(), Some(0));

            set_value.run(Some(99));
            assert_eq!(value.get(), Some(99));
        });
    }

    #[test]
    fn uncontrolled_set_none_clears_state() {
        with_owner(|| {
            let (value, set_value) = use_controllable_state(UseControllableStateParams {
                prop: MaybeProp::from(None::<i32>),
                default_prop: MaybeProp::from(Some(5)),
                on_change: None,
            });
            assert_eq!(value.get(), Some(5));

            set_value.run(None);
            assert_eq!(value.get(), None);
        });
    }

    // ── Controlled mode ──────────────────────────────────────

    #[test]
    fn controlled_uses_prop_value() {
        with_owner(|| {
            let (value, _set) = use_controllable_state(UseControllableStateParams {
                prop: MaybeProp::from(Some(100)),
                default_prop: MaybeProp::from(Some(0)),
                on_change: None,
            });
            assert_eq!(value.get(), Some(100));
        });
    }

    #[test]
    fn controlled_ignores_default_prop() {
        with_owner(|| {
            let (value, _set) = use_controllable_state(UseControllableStateParams {
                prop: MaybeProp::from(Some(100)),
                default_prop: MaybeProp::from(Some(999)),
                on_change: None,
            });
            // Should use prop, not default_prop
            assert_eq!(value.get(), Some(100));
        });
    }

    #[test]
    fn controlled_set_value_calls_on_change_with_new_value() {
        with_owner(|| {
            let received = Arc::new(AtomicI32::new(i32::MIN));
            let received_clone = received.clone();

            let on_change = Callback::new(move |v: Option<i32>| {
                if let Some(val) = v {
                    received_clone.store(val, Ordering::SeqCst);
                }
            });

            let (_value, set_value) = use_controllable_state(UseControllableStateParams {
                prop: MaybeProp::from(Some(10)),
                default_prop: MaybeProp::from(None::<i32>),
                on_change: Some(on_change),
            });

            set_value.run(Some(20));
            assert_eq!(received.load(Ordering::SeqCst), 20);
        });
    }

    #[test]
    fn controlled_set_value_does_not_call_on_change_when_same_value() {
        with_owner(|| {
            let call_count = Arc::new(AtomicU32::new(0));
            let count_clone = call_count.clone();

            let on_change = Callback::new(move |_: Option<i32>| {
                count_clone.fetch_add(1, Ordering::SeqCst);
            });

            let (_value, set_value) = use_controllable_state(UseControllableStateParams {
                prop: MaybeProp::from(Some(10)),
                default_prop: MaybeProp::from(None::<i32>),
                on_change: Some(on_change),
            });

            // Setting the same value should not trigger on_change
            set_value.run(Some(10));
            assert_eq!(call_count.load(Ordering::SeqCst), 0);
        });
    }

    #[test]
    fn controlled_set_value_does_not_update_internal_state() {
        with_owner(|| {
            let (value, set_value) = use_controllable_state(UseControllableStateParams {
                prop: MaybeProp::from(Some(10)),
                default_prop: MaybeProp::from(None::<i32>),
                on_change: None,
            });

            // In controlled mode, set_value should NOT change the value signal
            // (it's controlled by the prop).
            set_value.run(Some(20));
            assert_eq!(value.get(), Some(10));
        });
    }

    // ── Reactive controlled prop ─────────────────────────────

    #[test]
    fn controlled_value_follows_prop_signal() {
        with_owner(|| {
            let (prop_read, prop_write) = signal(Some(1i32));
            let prop = MaybeProp::from(Signal::derive(move || prop_read.get()));

            let (value, _set) = use_controllable_state(UseControllableStateParams {
                prop,
                default_prop: MaybeProp::from(None::<i32>),
                on_change: None,
            });

            assert_eq!(value.get(), Some(1));

            prop_write.set(Some(2));
            assert_eq!(value.get(), Some(2));

            prop_write.set(Some(3));
            assert_eq!(value.get(), Some(3));
        });
    }

    #[test]
    fn switching_to_controlled_uses_prop_value() {
        with_owner(|| {
            let (prop_read, prop_write) = signal(None::<i32>);
            let prop = MaybeProp::from(Signal::derive(move || prop_read.get()));

            let (value, set_value) = use_controllable_state(UseControllableStateParams {
                prop,
                default_prop: MaybeProp::from(Some(0)),
                on_change: None,
            });

            // Starts uncontrolled
            assert_eq!(value.get(), Some(0));

            // Update via set_value (uncontrolled path)
            set_value.run(Some(5));
            assert_eq!(value.get(), Some(5));

            // Switch to controlled
            prop_write.set(Some(100));
            assert_eq!(value.get(), Some(100));
        });
    }
}
