use leptos::prelude::*;
use radix_leptos_primitives::progress::*;

#[component]
pub fn ProgressPage() -> impl IntoView {
    let (value, set_value) = signal::<Option<f64>>(Some(30.0));

    view! {
        <Progress
            class:progress-root=true
            attr:data-custom="progress-root-custom"
            value=value
            max=100.0
        >
            <ProgressIndicator
                class:progress-indicator=true
                attr:data-testid="progress-indicator"
                attr:data-custom="progress-indicator-custom"
                style:width=move || value.get().map(|v| format!("{}%", v))
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
        " "
        <button data-testid="set-negative" on:click=move |_| set_value.set(Some(-10.0))>
            "set negative"
        </button>
        " "
        <button data-testid="set-over-max" on:click=move |_| set_value.set(Some(200.0))>
            "set over max"
        </button>
    }
}
