use pith_ui::checkbox::CheckedState;
use leptos::prelude::*;

use crate::theme::checkbox::*;

#[component]
pub fn CheckboxesPage() -> impl IntoView {
    let (controlled, set_controlled) = signal(CheckedState::True);

    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Checkbox"</h1>
                <p class="text-muted-foreground mb-6">
                    "shadcn/ui new-york checkbox wrapping the Pith UI primitive."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"States"</h2>
                <div class="flex items-center gap-6">
                    <label class="flex items-center gap-2 text-sm text-foreground">
                        <ThemedCheckbox />
                        "Unchecked"
                    </label>
                    <label class="flex items-center gap-2 text-sm text-foreground">
                        <ThemedCheckbox default_checked=CheckedState::True />
                        "Checked"
                    </label>
                    <label class="flex items-center gap-2 text-sm text-muted-foreground">
                        <ThemedCheckbox disabled=true />
                        "Disabled"
                    </label>
                    <label class="flex items-center gap-2 text-sm text-muted-foreground">
                        <ThemedCheckbox disabled=true default_checked=CheckedState::True />
                        "Disabled checked"
                    </label>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Controlled"</h2>
                <div class="flex items-center gap-4">
                    <label class="flex items-center gap-2 text-sm text-foreground">
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
        </div>
    }
}
