//! Hook that tracks the previous value of a signal.
//!
//! Returns a memo containing the value the signal held before its most
//! recent change. Used by bubble input components to detect when a value
//! actually changed and dispatch synthetic events.

use leptos::prelude::*;

/// Returns a memo containing the previous value of the given signal.
pub fn use_previous<T: Clone + PartialEq + Send + Sync + 'static>(value: Signal<T>) -> Memo<T> {
    let stored_value = StoredValue::new((value.get_untracked(), value.get_untracked()));

    Memo::new(move |_| {
        let value = value.get();
        let (current_value, previous_value) = stored_value.get_value();

        if current_value != value {
            stored_value.set_value((value.clone(), current_value.clone()));
            current_value
        } else {
            previous_value
        }
    })
}
