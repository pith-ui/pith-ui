use leptos::prelude::*;

use crate::theme::switch::*;

#[component]
pub fn SwitchesPage() -> impl IntoView {
    let (controlled, set_controlled) = signal(true);

    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Switch"</h1>
                <p class="text-muted-foreground mb-6">
                    "shadcn/ui new-york switch wrapping the Pith UI primitive. "
                    "Two sizes: default and sm."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Sizes"</h2>
                <div class="flex items-center gap-6">
                    <label class="flex items-center gap-2 text-sm text-foreground">
                        <ThemedSwitch size=SwitchSize::Default default_checked=true />
                        "Default"
                    </label>
                    <label class="flex items-center gap-2 text-sm text-foreground">
                        <ThemedSwitch size=SwitchSize::Sm default_checked=true />
                        "Small"
                    </label>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"States"</h2>
                <div class="flex items-center gap-6">
                    <label class="flex items-center gap-2 text-sm text-foreground">
                        <ThemedSwitch />
                        "Off"
                    </label>
                    <label class="flex items-center gap-2 text-sm text-foreground">
                        <ThemedSwitch default_checked=true />
                        "On"
                    </label>
                    <label class="flex items-center gap-2 text-sm text-muted-foreground">
                        <ThemedSwitch disabled=true />
                        "Disabled"
                    </label>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Controlled"</h2>
                <div class="flex items-center gap-4">
                    <label class="flex items-center gap-2 text-sm text-foreground">
                        <ThemedSwitch
                            checked=controlled
                            on_checked_change=move |state| set_controlled.set(state)
                        />
                        {move || if controlled.get() { "On" } else { "Off" }}
                    </label>
                </div>
            </section>
        </div>
    }
}
