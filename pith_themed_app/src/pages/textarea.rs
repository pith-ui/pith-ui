use leptos::prelude::*;

use crate::theme::textarea::*;

#[component]
pub fn TextareaPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Textarea"</h1>
                <p class="text-muted-foreground mb-6">
                    "Displays a form textarea or a component that looks like a textarea."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Default"</h2>
                <div class="max-w-sm">
                    <ThemedTextarea />
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"With Placeholder"</h2>
                <div class="max-w-sm">
                    <ThemedTextarea placeholder="Type your message here." />
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Disabled"</h2>
                <div class="max-w-sm">
                    <ThemedTextarea placeholder="Disabled textarea" disabled=true />
                </div>
            </section>
        </div>
    }
}
