use leptos::prelude::*;

use crate::theme::checkbox::ThemedCheckbox;
use crate::theme::label::ThemedLabel;

#[component]
pub fn LabelPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Label"</h1>
                <p class="text-muted-foreground mb-6">
                    "Accessible text label for form controls. Prevents text selection on double-click."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"With Checkbox"</h2>
                <div class="flex items-center gap-4">
                    <ThemedLabel>
                        <ThemedCheckbox />
                        "Accept terms and conditions"
                    </ThemedLabel>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Standalone"</h2>
                <div class="flex flex-col gap-2">
                    <ThemedLabel>"Email address"</ThemedLabel>
                    <input
                        type="email"
                        placeholder="you@example.com"
                        class="h-9 w-64 rounded-md border border-input bg-transparent px-3 text-sm text-foreground placeholder:text-muted-foreground focus-visible:focus-ring outline-none"
                    />
                </div>
            </section>
        </div>
    }
}
