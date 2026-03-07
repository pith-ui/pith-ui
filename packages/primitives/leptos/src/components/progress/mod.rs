use std::fmt::{Display, Formatter};

use crate::support::primitive::Primitive;
use leptos::{context::Provider, html, prelude::*};
use leptos_node_ref::AnyNodeRef;

const DEFAULT_MAX: f64 = 100.0;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ProgressState {
    Indeterminate,
    Complete,
    Loading,
}

impl Display for ProgressState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ProgressState::Indeterminate => "indeterminate",
                ProgressState::Complete => "complete",
                ProgressState::Loading => "loading",
            }
        )
    }
}

#[derive(Clone, Copy, Debug)]
struct ProgressContextValue {
    value: Signal<Option<f64>>,
    max: Signal<f64>,
}

#[component]
pub fn Progress(
    #[prop(into, optional)] value: MaybeProp<f64>,
    #[prop(into, optional)] max: MaybeProp<f64>,
    #[prop(into, optional)] get_value_label: Option<Callback<(f64, f64), String>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let max = Signal::derive(move || {
        max.get()
            .and_then(|max| match max == 0.0 {
                true => None,
                false => Some(max),
            })
            .unwrap_or(DEFAULT_MAX)
    });
    let value = Signal::derive(move || value.get().map(|value| value.min(max.get()).max(0.0)));

    let value_label = Signal::derive(move || {
        value.get().map(|v| match get_value_label {
            Some(cb) => cb.run((v, max.get())),
            None => default_get_value_label(v, max.get()),
        })
    });

    let context_value = ProgressContextValue { value, max };

    view! {
        <Provider value=context_value>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attr:aria-valuemax=move || max.get().to_string()
                attr:aria-valuemin="0"
                attr:aria-valuenow=move || value.get().map(|v| v.to_string())
                attr:aria-valuetext=move || value_label.get()
                attr:role="progressbar"
                attr:data-state=move || get_progress_state(value.get(), max.get()).to_string()
                attr:data-value=move || value.get().map(|v| v.to_string())
                attr:data-max=move || max.get().to_string()
            >
                {children.with_value(|children| children())}
            </Primitive>
        </Provider>
    }
}

#[component]
pub fn ProgressIndicator(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<ProgressContextValue>();

    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref=node_ref
            attr:data-state=move || get_progress_state(context.value.get(), context.max.get()).to_string()
            attr:data-value=move || context.value.get().map(|v| v.to_string())
            attr:data-max=move || context.max.get().to_string()
        >
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </Primitive>
    }
}

fn default_get_value_label(value: f64, max: f64) -> String {
    format!("{}%", ((value / max) * 100.0).round())
}

fn get_progress_state(value: Option<f64>, max_value: f64) -> ProgressState {
    match value {
        Some(value) => match value == max_value {
            true => ProgressState::Complete,
            false => ProgressState::Loading,
        },
        None => ProgressState::Indeterminate,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── default_get_value_label ─────────────────────────────

    #[test]
    fn value_label_fifty_percent() {
        assert_eq!(default_get_value_label(50.0, 100.0), "50%");
    }

    #[test]
    fn value_label_hundred_percent() {
        assert_eq!(default_get_value_label(100.0, 100.0), "100%");
    }

    #[test]
    fn value_label_zero_percent() {
        assert_eq!(default_get_value_label(0.0, 100.0), "0%");
    }

    #[test]
    fn value_label_rounds_fraction() {
        // 1/3 = 33.333...% rounds to 33%
        assert_eq!(default_get_value_label(1.0, 3.0), "33%");
    }

    #[test]
    fn value_label_rounds_up() {
        // 2/3 = 66.666...% rounds to 67%
        assert_eq!(default_get_value_label(2.0, 3.0), "67%");
    }

    #[test]
    fn value_label_custom_max() {
        assert_eq!(default_get_value_label(25.0, 50.0), "50%");
    }

    // ── get_progress_state ──────────────────────────────────

    #[test]
    fn progress_state_none_is_indeterminate() {
        assert_eq!(
            get_progress_state(None, 100.0),
            ProgressState::Indeterminate
        );
    }

    #[test]
    fn progress_state_value_equals_max_is_complete() {
        assert_eq!(
            get_progress_state(Some(100.0), 100.0),
            ProgressState::Complete
        );
    }

    #[test]
    fn progress_state_value_less_than_max_is_loading() {
        assert_eq!(
            get_progress_state(Some(50.0), 100.0),
            ProgressState::Loading
        );
    }

    #[test]
    fn progress_state_zero_is_loading() {
        assert_eq!(get_progress_state(Some(0.0), 100.0), ProgressState::Loading);
    }

    #[test]
    fn progress_state_custom_max_complete() {
        assert_eq!(
            get_progress_state(Some(50.0), 50.0),
            ProgressState::Complete
        );
    }

    #[test]
    fn progress_state_custom_max_loading() {
        assert_eq!(get_progress_state(Some(25.0), 50.0), ProgressState::Loading);
    }
}
