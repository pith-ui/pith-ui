use leptos::prelude::*;

use crate::theme::badge::*;

#[component]
pub fn BadgesPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-neutral-12 mb-1">"Badge"</h1>
                <p class="text-neutral-11 mb-6">
                    "A label component with variant, size, and color props. "
                    "The color prop swaps the scale prefix (accent \u{2192} danger, success, neutral)."
                </p>
            </div>

            // Variants
            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-neutral-12">"Variants"</h2>
                <div class="flex items-center gap-3 flex-wrap">
                    <Badge variant=BadgeVariant::Solid>"Solid"</Badge>
                    <Badge variant=BadgeVariant::Soft>"Soft"</Badge>
                    <Badge variant=BadgeVariant::Outline>"Outline"</Badge>
                </div>
            </section>

            // Sizes
            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-neutral-12">"Sizes"</h2>
                <div class="flex items-center gap-3 flex-wrap">
                    <Badge size=BadgeSize::Sm>"Small"</Badge>
                    <Badge size=BadgeSize::Md>"Medium"</Badge>
                    <Badge size=BadgeSize::Lg>"Large"</Badge>
                </div>
            </section>

            // Colors
            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-neutral-12">"Color Scales"</h2>

                <h3 class="text-sm font-medium text-neutral-11">"Solid"</h3>
                <div class="flex items-center gap-3 flex-wrap">
                    <Badge color=BadgeColor::Accent>"Accent"</Badge>
                    <Badge color=BadgeColor::Danger>"Danger"</Badge>
                    <Badge color=BadgeColor::Success>"Success"</Badge>
                    <Badge color=BadgeColor::Neutral>"Neutral"</Badge>
                </div>

                <h3 class="text-sm font-medium text-neutral-11">"Soft"</h3>
                <div class="flex items-center gap-3 flex-wrap">
                    <Badge variant=BadgeVariant::Soft color=BadgeColor::Accent>"Accent"</Badge>
                    <Badge variant=BadgeVariant::Soft color=BadgeColor::Danger>"Danger"</Badge>
                    <Badge variant=BadgeVariant::Soft color=BadgeColor::Success>"Success"</Badge>
                    <Badge variant=BadgeVariant::Soft color=BadgeColor::Neutral>"Neutral"</Badge>
                </div>

                <h3 class="text-sm font-medium text-neutral-11">"Outline"</h3>
                <div class="flex items-center gap-3 flex-wrap">
                    <Badge variant=BadgeVariant::Outline color=BadgeColor::Accent>"Accent"</Badge>
                    <Badge variant=BadgeVariant::Outline color=BadgeColor::Danger>"Danger"</Badge>
                    <Badge variant=BadgeVariant::Outline color=BadgeColor::Success>"Success"</Badge>
                    <Badge variant=BadgeVariant::Outline color=BadgeColor::Neutral>"Neutral"</Badge>
                </div>
            </section>

            // Rough edge callout
            <section class="space-y-2">
                <h2 class="text-lg font-semibold text-neutral-12">"Rough Edge: Color Override"</h2>
                <div class="bg-neutral-2 border border-neutral-6 rounded-3 p-4 text-sm text-neutral-11">
                    <p>
                        "The color prop currently works via string replacement "
                        "(" <code class="text-accent-11">"accent-"</code> " \u{2192} "
                        <code class="text-danger-11">"danger-"</code> "). "
                        "This is pragmatic but fragile. A production system might use "
                        "a custom TwVariant per color, or CSS custom property overrides "
                        "scoped to the component."
                    </p>
                </div>
            </section>
        </div>
    }
}
