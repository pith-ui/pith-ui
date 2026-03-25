use leptos::prelude::*;

use crate::theme::progress::ThemedProgress;

#[component]
pub fn ProgressPage() -> impl IntoView {
    let (value, set_value) = signal(33.0);

    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Progress"</h1>
                <p class="text-muted-foreground mb-6">
                    "Displays an indicator showing the completion progress of a task."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Default"</h2>
                <div class="max-w-md">
                    <ThemedProgress value=60.0 />
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"States"</h2>
                <div class="max-w-md space-y-3">
                    <div>
                        <p class="text-sm text-muted-foreground mb-1">"0%"</p>
                        <ThemedProgress value=0.0 />
                    </div>
                    <div>
                        <p class="text-sm text-muted-foreground mb-1">"50%"</p>
                        <ThemedProgress value=50.0 />
                    </div>
                    <div>
                        <p class="text-sm text-muted-foreground mb-1">"100% (Complete)"</p>
                        <ThemedProgress value=100.0 />
                    </div>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Controlled"</h2>
                <div class="max-w-md space-y-3">
                    <ThemedProgress value=value />
                    <div class="flex items-center gap-3">
                        <button
                            class="h-8 rounded-md border border-input bg-background px-3 text-sm text-foreground hover:bg-accent hover:text-accent-foreground transition-colors"
                            on:click=move |_| set_value.update(|v| *v = (*v - 10.0).max(0.0))
                        >
                            "-10"
                        </button>
                        <span class="text-sm text-foreground min-w-[3ch] text-center">
                            {move || format!("{}%", value.get())}
                        </span>
                        <button
                            class="h-8 rounded-md border border-input bg-background px-3 text-sm text-foreground hover:bg-accent hover:text-accent-foreground transition-colors"
                            on:click=move |_| set_value.update(|v| *v = (*v + 10.0).min(100.0))
                        >
                            "+10"
                        </button>
                    </div>
                </div>
            </section>
        </div>
    }
}
