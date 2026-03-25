use leptos::prelude::*;

use crate::theme::toggle::*;

#[component]
pub fn TogglesPage() -> impl IntoView {
    let (controlled, set_controlled) = signal(true);

    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Toggle"</h1>
                <p class="text-muted-foreground mb-6">
                    "shadcn/ui new-york toggle wrapping the Pith UI primitive. "
                    "Two variants (default, outline) and three sizes."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Default Variant"</h2>
                <div class="flex items-center gap-3 flex-wrap">
                    <ThemedToggle size=ToggleSize::Sm default_pressed=true>"Sm"</ThemedToggle>
                    <ThemedToggle size=ToggleSize::Default default_pressed=true>"Default"</ThemedToggle>
                    <ThemedToggle size=ToggleSize::Lg default_pressed=true>"Lg"</ThemedToggle>
                    <ThemedToggle disabled=true>"Disabled"</ThemedToggle>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Outline Variant"</h2>
                <div class="flex items-center gap-3 flex-wrap">
                    <ThemedToggle variant=ToggleVariant::Outline size=ToggleSize::Sm default_pressed=true>
                        "Sm"
                    </ThemedToggle>
                    <ThemedToggle variant=ToggleVariant::Outline size=ToggleSize::Default default_pressed=true>
                        "Default"
                    </ThemedToggle>
                    <ThemedToggle variant=ToggleVariant::Outline size=ToggleSize::Lg default_pressed=true>
                        "Lg"
                    </ThemedToggle>
                    <ThemedToggle variant=ToggleVariant::Outline disabled=true>
                        "Disabled"
                    </ThemedToggle>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Controlled"</h2>
                <div class="flex items-center gap-4">
                    <ThemedToggle
                        variant=ToggleVariant::Outline
                        pressed=controlled
                        on_pressed_change=move |state| set_controlled.set(state)
                    >
                        {move || if controlled.get() { "Pressed" } else { "Released" }}
                    </ThemedToggle>
                </div>
            </section>
        </div>
    }
}
