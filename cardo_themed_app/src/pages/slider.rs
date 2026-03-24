use leptos::prelude::*;

use crate::theme::slider::ThemedSlider;

type SliderValues = Vec<f64>;

#[component]
pub fn SliderPage() -> impl IntoView {
    let (value, set_value) = signal(vec![50.0]);

    let handle_change = move |val: SliderValues| set_value.set(val);

    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Slider"</h1>
                <p class="text-muted-foreground mb-6">
                    "An input where the user selects a value from within a given range."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Default"</h2>
                <div class="max-w-md">
                    <ThemedSlider default_value=vec![50.0] />
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Controlled"</h2>
                <div class="max-w-md space-y-2">
                    <ThemedSlider
                        value=value
                        on_value_change=handle_change
                    />
                    <p class="text-sm text-muted-foreground">
                        "Value: " {move || format!("{:.0}", value.get().first().copied().unwrap_or(0.0))}
                    </p>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Custom Range"</h2>
                <div class="max-w-md">
                    <ThemedSlider default_value=vec![25.0] min=0.0 max=50.0 step=5.0 />
                </div>
                <p class="text-xs text-muted-foreground">"Range: 0-50, step: 5"</p>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Disabled"</h2>
                <div class="max-w-md">
                    <ThemedSlider default_value=vec![40.0] disabled=true />
                </div>
            </section>
        </div>
    }
}
