use cardo_ui::checkbox::CheckedState;
use leptos::prelude::*;

use crate::theme::checkbox::*;

#[component]
pub fn CheckboxesPage() -> impl IntoView {
    let (controlled, set_controlled) = signal(CheckedState::True);

    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-neutral-12 mb-1">"Checkbox"</h1>
                <p class="text-neutral-11 mb-6">
                    "Wraps the Cardo UI " <code class="text-accent-11 bg-accent-3 px-1 rounded-1 text-sm">"Checkbox"</code>
                    " primitive with token-driven styles applied via "
                    <code class="text-accent-11 bg-accent-3 px-1 rounded-1 text-sm">"attr:class"</code> "."
                </p>
            </div>

            // Sizes
            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-neutral-12">"Sizes"</h2>
                <div class="flex items-center gap-6">
                    <label class="flex items-center gap-2 text-sm text-neutral-12">
                        <ThemedCheckbox size=CheckboxSize::Sm default_checked=CheckedState::True />
                        "Small"
                    </label>
                    <label class="flex items-center gap-2 text-sm text-neutral-12">
                        <ThemedCheckbox size=CheckboxSize::Md default_checked=CheckedState::True />
                        "Medium"
                    </label>
                </div>
            </section>

            // States
            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-neutral-12">"States"</h2>
                <div class="flex items-center gap-6">
                    <label class="flex items-center gap-2 text-sm text-neutral-12">
                        <ThemedCheckbox />
                        "Unchecked"
                    </label>
                    <label class="flex items-center gap-2 text-sm text-neutral-12">
                        <ThemedCheckbox default_checked=CheckedState::True />
                        "Checked"
                    </label>
                    <label class="flex items-center gap-2 text-sm text-neutral-12">
                        <ThemedCheckbox default_checked=CheckedState::Indeterminate />
                        "Indeterminate"
                    </label>
                    <label class="flex items-center gap-2 text-sm text-neutral-11">
                        <ThemedCheckbox disabled=true />
                        "Disabled"
                    </label>
                </div>
            </section>

            // Controlled
            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-neutral-12">"Controlled"</h2>
                <div class="flex items-center gap-4">
                    <label class="flex items-center gap-2 text-sm text-neutral-12">
                        <ThemedCheckbox
                            checked=controlled
                            on_checked_change=move |state| set_controlled.set(state)
                        />
                        {move || match controlled.get() {
                            CheckedState::True => "Checked",
                            CheckedState::False => "Unchecked",
                            CheckedState::Indeterminate => "Indeterminate",
                        }}
                    </label>
                </div>
            </section>

            // Data attributes note
            <section class="space-y-2">
                <h2 class="text-lg font-semibold text-neutral-12">"How It Works"</h2>
                <div class="bg-neutral-2 border border-neutral-6 rounded-3 p-4 text-sm text-neutral-11 space-y-2">
                    <p>
                        "The primitive sets " <code class="text-accent-11">"data-state"</code>
                        " and " <code class="text-accent-11">"data-disabled"</code>
                        " attributes. Themed styles use Tailwind's arbitrary data attribute variants:"
                    </p>
                    <code class="block text-accent-11">
                        "data-[state=checked]:bg-accent-9"
                    </code>
                </div>
            </section>
        </div>
    }
}
