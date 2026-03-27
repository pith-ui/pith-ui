use leptos::prelude::*;

use crate::theme::password_toggle_field::*;

#[component]
pub fn PasswordToggleFieldPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Password Toggle Field"</h1>
                <p class="text-muted-foreground mb-6">
                    "A password input with a visibility toggle button."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Default"</h2>
                <div class="max-w-sm relative">
                    <ThemedPasswordToggleField>
                        <ThemedPasswordToggleFieldInput placeholder="Enter password" />
                        <ThemedPasswordToggleFieldToggle />
                    </ThemedPasswordToggleField>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Initially Visible"</h2>
                <div class="max-w-sm relative">
                    <ThemedPasswordToggleField default_visible=true>
                        <ThemedPasswordToggleFieldInput placeholder="Your secret" />
                        <ThemedPasswordToggleFieldToggle />
                    </ThemedPasswordToggleField>
                </div>
            </section>
        </div>
    }
}
