use leptos::prelude::*;

use crate::theme::switch::*;

#[component]
pub fn SwitchesPage() -> impl IntoView {
    let (controlled, set_controlled) = signal(true);

    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-neutral-12 mb-1">"Switch"</h1>
                <p class="text-neutral-11 mb-6">
                    "Wraps the Cardo UI " <code class="text-accent-11 bg-accent-3 px-1 rounded-1 text-sm">"Switch"</code>
                    " + " <code class="text-accent-11 bg-accent-3 px-1 rounded-1 text-sm">"SwitchThumb"</code>
                    " primitives. Demonstrates multi-element theming with paired size variants."
                </p>
            </div>

            // Sizes
            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-neutral-12">"Sizes"</h2>
                <div class="flex items-center gap-6">
                    <label class="flex items-center gap-2 text-sm text-neutral-12">
                        <ThemedSwitch size=SwitchSize::Sm default_checked=true />
                        "Small"
                    </label>
                    <label class="flex items-center gap-2 text-sm text-neutral-12">
                        <ThemedSwitch size=SwitchSize::Md default_checked=true />
                        "Medium"
                    </label>
                </div>
            </section>

            // States
            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-neutral-12">"States"</h2>
                <div class="flex items-center gap-6">
                    <label class="flex items-center gap-2 text-sm text-neutral-12">
                        <ThemedSwitch />
                        "Off"
                    </label>
                    <label class="flex items-center gap-2 text-sm text-neutral-12">
                        <ThemedSwitch default_checked=true />
                        "On"
                    </label>
                    <label class="flex items-center gap-2 text-sm text-neutral-11">
                        <ThemedSwitch disabled=true />
                        "Disabled"
                    </label>
                </div>
            </section>

            // Controlled
            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-neutral-12">"Controlled"</h2>
                <div class="flex items-center gap-4">
                    <label class="flex items-center gap-2 text-sm text-neutral-12">
                        <ThemedSwitch
                            checked=controlled
                            on_checked_change=move |state| set_controlled.set(state)
                        />
                        {move || if controlled.get() { "On" } else { "Off" }}
                    </label>
                </div>
            </section>

            // Multi-element note
            <section class="space-y-2">
                <h2 class="text-lg font-semibold text-neutral-12">"Rough Edge: Multi-Element TwClass"</h2>
                <div class="bg-neutral-2 border border-neutral-6 rounded-3 p-4 text-sm text-neutral-11 space-y-2">
                    <p>
                        "Switch has two styled elements (root + thumb). Each needs a separate "
                        <code class="text-accent-11">"TwClass"</code> " struct with paired size variants. "
                        "The wrapper maps a single " <code class="text-accent-11">"SwitchSize"</code>
                        " to both " <code class="text-accent-11">"SwitchRootSize"</code>
                        " and " <code class="text-accent-11">"SwitchThumbSize"</code> "."
                    </p>
                    <p>"This is more boilerplate than single-element components, but keeps each class definition self-contained."</p>
                </div>
            </section>
        </div>
    }
}
