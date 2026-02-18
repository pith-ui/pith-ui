use leptos::prelude::*;
use radix_leptos_progress::*;

#[component]
pub fn ProgressPage() -> impl IntoView {
    let (value, set_value) = signal::<Option<f64>>(Some(30.0));

    view! {
        <Progress
            attr:class="progress-root"
            value=value
            max=100.0
        >
            <ProgressIndicator
                attr:class="progress-indicator"
                attr:data-testid="progress-indicator"
                attr:style=move || {
                    value.get().map(|v| format!("width: {}%", v))
                }
            />
        </Progress>

        <br /><br />

        <span data-testid="progress-value">
            {move || match value.get() {
                Some(v) => format!("{}", v as i64),
                None => "indeterminate".to_string(),
            }}
        </span>

        <br /><br />

        <button on:click=move |_| {
            set_value.update(|v| {
                *v = match *v {
                    Some(current) => Some((current + 10.0).min(100.0)),
                    None => Some(10.0),
                };
            });
        }>
            "increment"
        </button>
        " "
        <button on:click=move |_| set_value.set(Some(100.0))>
            "set complete"
        </button>
        " "
        <button on:click=move |_| set_value.set(None)>
            "set indeterminate"
        </button>
    }
}
