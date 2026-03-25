use leptos::prelude::*;

use crate::theme::aspect_ratio::*;

#[component]
pub fn AspectRatioPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Aspect Ratio"</h1>
                <p class="text-muted-foreground mb-6">
                    "Displays content within a desired ratio."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"16:9 Ratio"</h2>
                <div class="max-w-lg overflow-hidden rounded-md">
                    <ThemedAspectRatio ratio={16.0 / 9.0}>
                        <div class="flex size-full items-center justify-center rounded-md bg-muted">
                            <span class="text-sm text-muted-foreground">"16:9"</span>
                        </div>
                    </ThemedAspectRatio>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"1:1 Square"</h2>
                <div class="max-w-xs overflow-hidden rounded-md">
                    <ThemedAspectRatio ratio=1.0>
                        <div class="flex size-full items-center justify-center rounded-md bg-muted">
                            <span class="text-sm text-muted-foreground">"1:1"</span>
                        </div>
                    </ThemedAspectRatio>
                </div>
            </section>
        </div>
    }
}
