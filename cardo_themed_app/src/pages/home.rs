use leptos::prelude::*;

/// Renders a single color swatch with the token name and resolved color.
#[component]
fn Swatch(label: &'static str, class: &'static str) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center gap-1">
            <div class={format!("w-12 h-12 rounded-2 shadow-1 {class}")} />
            <span class="text-xs text-neutral-11">{label}</span>
        </div>
    }
}

/// Renders a labeled row of 12 swatches for a color scale.
#[component]
fn ScaleRow(name: &'static str, prefix: &'static str) -> impl IntoView {
    let swatches: Vec<_> = (1..=12)
        .map(|i| {
            let label = format!("{i}");
            let class = format!("bg-{prefix}-{i}");
            let label_static: &'static str = Box::leak(label.into_boxed_str());
            let class_static: &'static str = Box::leak(class.into_boxed_str());
            view! { <Swatch label=label_static class=class_static /> }
        })
        .collect();

    view! {
        <div class="space-y-2">
            <h3 class="text-sm font-medium text-neutral-11">{name}</h3>
            <div class="flex gap-2 flex-wrap">
                {swatches}
            </div>
        </div>
    }
}

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <div>
                <h1 class="text-2xl font-bold text-neutral-12 mb-1">"Design System"</h1>
                <p class="text-neutral-11 mb-6">
                    "Token-driven design language. All colors below are resolved through CSS custom properties. "
                    "Swap " <code class="text-accent-11 bg-accent-3 px-1 rounded-1 text-sm">"--indigo-*"</code>
                    " to " <code class="text-accent-11 bg-accent-3 px-1 rounded-1 text-sm">"--violet-*"</code>
                    " in " <code class="text-accent-11 bg-accent-3 px-1 rounded-1 text-sm">"tokens.css"</code>
                    " to change the accent color everywhere."
                </p>
            </div>

            // Color palette
            <section class="space-y-6">
                <h2 class="text-lg font-semibold text-neutral-12">"Color Palette"</h2>
                <ScaleRow name="Accent" prefix="accent" />
                <ScaleRow name="Neutral" prefix="neutral" />
                <ScaleRow name="Danger" prefix="danger" />
                <ScaleRow name="Success" prefix="success" />
            </section>

            // Radix scale usage guide
            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-neutral-12">"Scale Usage Guide"</h2>
                <div class="bg-neutral-2 rounded-3 p-4 text-sm text-neutral-11 space-y-1 border border-neutral-6">
                    <div><span class="font-medium text-neutral-12">"Steps 1\u{2013}2:"</span>" Backgrounds"</div>
                    <div><span class="font-medium text-neutral-12">"Steps 3\u{2013}5:"</span>" Component surfaces & hover/active states"</div>
                    <div><span class="font-medium text-neutral-12">"Steps 6\u{2013}8:"</span>" Borders"</div>
                    <div><span class="font-medium text-neutral-12">"Steps 9\u{2013}10:"</span>" Solid fills"</div>
                    <div><span class="font-medium text-neutral-12">"Steps 11\u{2013}12:"</span>" Text"</div>
                </div>
            </section>

            // Layout tokens
            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-neutral-12">"Border Radius"</h2>
                <div class="flex items-end gap-4">
                    <div class="flex flex-col items-center gap-1">
                        <div class="w-16 h-16 bg-accent-3 border border-accent-7 rounded-1" />
                        <span class="text-xs text-neutral-11">"radius-1"</span>
                    </div>
                    <div class="flex flex-col items-center gap-1">
                        <div class="w-16 h-16 bg-accent-3 border border-accent-7 rounded-2" />
                        <span class="text-xs text-neutral-11">"radius-2"</span>
                    </div>
                    <div class="flex flex-col items-center gap-1">
                        <div class="w-16 h-16 bg-accent-3 border border-accent-7 rounded-3" />
                        <span class="text-xs text-neutral-11">"radius-3"</span>
                    </div>
                    <div class="flex flex-col items-center gap-1">
                        <div class="w-16 h-16 bg-accent-3 border border-accent-7 rounded-4" />
                        <span class="text-xs text-neutral-11">"radius-4"</span>
                    </div>
                    <div class="flex flex-col items-center gap-1">
                        <div class="w-16 h-16 bg-accent-3 border border-accent-7 rounded-full" />
                        <span class="text-xs text-neutral-11">"full"</span>
                    </div>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-neutral-12">"Shadows"</h2>
                <div class="flex items-end gap-6">
                    <div class="flex flex-col items-center gap-1">
                        <div class="w-20 h-20 bg-neutral-1 rounded-2 shadow-1" />
                        <span class="text-xs text-neutral-11">"shadow-1"</span>
                    </div>
                    <div class="flex flex-col items-center gap-1">
                        <div class="w-20 h-20 bg-neutral-1 rounded-2 shadow-2" />
                        <span class="text-xs text-neutral-11">"shadow-2"</span>
                    </div>
                    <div class="flex flex-col items-center gap-1">
                        <div class="w-20 h-20 bg-neutral-1 rounded-2 shadow-3" />
                        <span class="text-xs text-neutral-11">"shadow-3"</span>
                    </div>
                </div>
            </section>
        </div>
    }
}
