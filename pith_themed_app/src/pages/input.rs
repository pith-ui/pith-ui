use leptos::prelude::*;

use crate::theme::input::*;

#[component]
pub fn InputPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Input"</h1>
                <p class="text-muted-foreground mb-6">
                    "Displays a form input field or a component that looks like an input field."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Default"</h2>
                <div class="max-w-sm">
                    <ThemedInput />
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"With Placeholder"</h2>
                <div class="max-w-sm">
                    <ThemedInput placeholder="Email address" />
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Disabled"</h2>
                <div class="max-w-sm">
                    <ThemedInput placeholder="Disabled input" disabled=true />
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Password"</h2>
                <div class="max-w-sm">
                    <ThemedInput r#type="password" placeholder="Enter password" />
                </div>
            </section>
        </div>
    }
}
