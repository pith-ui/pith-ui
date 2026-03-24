use leptos::prelude::*;

use crate::theme::toggle::*;

#[component]
pub fn TogglesPage() -> impl IntoView {
    let (controlled, set_controlled) = signal(true);

    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-neutral-12 mb-1">"Toggle"</h1>
                <p class="text-neutral-11 mb-6">
                    "Wraps the Cardo UI " <code class="text-accent-11 bg-accent-3 px-1 rounded-1 text-sm">"Toggle"</code>
                    " primitive with variant and size props."
                </p>
            </div>

            // Variants at each size
            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-neutral-12">"Ghost (default)"</h2>
                <div class="flex items-center gap-3 flex-wrap">
                    <ThemedToggle variant=ToggleVariant::Ghost size=ToggleSize::Sm default_pressed=true>
                        "Sm"
                    </ThemedToggle>
                    <ThemedToggle variant=ToggleVariant::Ghost size=ToggleSize::Md default_pressed=true>
                        "Md"
                    </ThemedToggle>
                    <ThemedToggle variant=ToggleVariant::Ghost size=ToggleSize::Lg default_pressed=true>
                        "Lg"
                    </ThemedToggle>
                    <ThemedToggle variant=ToggleVariant::Ghost disabled=true>
                        "Disabled"
                    </ThemedToggle>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-neutral-12">"Outline"</h2>
                <div class="flex items-center gap-3 flex-wrap">
                    <ThemedToggle variant=ToggleVariant::Outline size=ToggleSize::Sm default_pressed=true>
                        "Sm"
                    </ThemedToggle>
                    <ThemedToggle variant=ToggleVariant::Outline size=ToggleSize::Md default_pressed=true>
                        "Md"
                    </ThemedToggle>
                    <ThemedToggle variant=ToggleVariant::Outline size=ToggleSize::Lg default_pressed=true>
                        "Lg"
                    </ThemedToggle>
                    <ThemedToggle variant=ToggleVariant::Outline disabled=true>
                        "Disabled"
                    </ThemedToggle>
                </div>
            </section>

            // Controlled
            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-neutral-12">"Controlled"</h2>
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
