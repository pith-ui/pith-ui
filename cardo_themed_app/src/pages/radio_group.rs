use leptos::prelude::*;

use crate::theme::label::ThemedLabel;
use crate::theme::radio_group::*;

#[component]
pub fn RadioGroupPage() -> impl IntoView {
    let (controlled, set_controlled) = signal("comfortable".to_string());

    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Radio Group"</h1>
                <p class="text-muted-foreground mb-6">
                    "A set of checkable buttons where only one can be checked at a time."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Default"</h2>
                <ThemedRadioGroup default_value="default">
                    <div class="flex items-center gap-2">
                        <ThemedRadioGroupItem value="default" />
                        <ThemedLabel>"Default"</ThemedLabel>
                    </div>
                    <div class="flex items-center gap-2">
                        <ThemedRadioGroupItem value="comfortable" />
                        <ThemedLabel>"Comfortable"</ThemedLabel>
                    </div>
                    <div class="flex items-center gap-2">
                        <ThemedRadioGroupItem value="compact" />
                        <ThemedLabel>"Compact"</ThemedLabel>
                    </div>
                </ThemedRadioGroup>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Controlled"</h2>
                <ThemedRadioGroup
                    value=controlled
                    on_value_change=move |val: String| set_controlled.set(val)
                >
                    <div class="flex items-center gap-2">
                        <ThemedRadioGroupItem value="default" />
                        <ThemedLabel>"Default"</ThemedLabel>
                    </div>
                    <div class="flex items-center gap-2">
                        <ThemedRadioGroupItem value="comfortable" />
                        <ThemedLabel>"Comfortable"</ThemedLabel>
                    </div>
                    <div class="flex items-center gap-2">
                        <ThemedRadioGroupItem value="compact" />
                        <ThemedLabel>"Compact"</ThemedLabel>
                    </div>
                </ThemedRadioGroup>
                <p class="text-sm text-muted-foreground">
                    "Selected: " {move || controlled.get()}
                </p>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Disabled"</h2>
                <ThemedRadioGroup default_value="option-1" disabled=true>
                    <div class="flex items-center gap-2">
                        <ThemedRadioGroupItem value="option-1" />
                        <ThemedLabel>"Option 1"</ThemedLabel>
                    </div>
                    <div class="flex items-center gap-2">
                        <ThemedRadioGroupItem value="option-2" />
                        <ThemedLabel>"Option 2"</ThemedLabel>
                    </div>
                </ThemedRadioGroup>
            </section>
        </div>
    }
}
