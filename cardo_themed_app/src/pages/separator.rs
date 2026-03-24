use cardo_ui::separator::Orientation;
use leptos::prelude::*;

use crate::theme::separator::ThemedSeparator;

#[component]
pub fn SeparatorPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Separator"</h1>
                <p class="text-muted-foreground mb-6">
                    "Visual divider between content sections. Supports horizontal and vertical orientation."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Horizontal"</h2>
                <div class="space-y-3 max-w-md">
                    <p class="text-sm text-foreground">"Content above"</p>
                    <ThemedSeparator />
                    <p class="text-sm text-foreground">"Content below"</p>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Vertical"</h2>
                <div class="flex items-center gap-3 h-6">
                    <span class="text-sm text-foreground">"Left"</span>
                    <ThemedSeparator orientation=Orientation::Vertical />
                    <span class="text-sm text-foreground">"Center"</span>
                    <ThemedSeparator orientation=Orientation::Vertical />
                    <span class="text-sm text-foreground">"Right"</span>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Decorative"</h2>
                <div class="max-w-md">
                    <ThemedSeparator decorative=true />
                    <p class="text-xs text-muted-foreground mt-2">
                        "This separator has role=\"none\" and is hidden from assistive technology."
                    </p>
                </div>
            </section>
        </div>
    }
}
