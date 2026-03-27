use leptos::prelude::*;

use crate::theme::badge::*;

#[component]
pub fn BadgesPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Badge"</h1>
                <p class="text-muted-foreground mb-6">
                    "shadcn/ui new-york badge with 4 variants. Single size, no color prop \u{2014} "
                    "color intent is expressed through the variant (default, secondary, destructive, outline)."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Variants"</h2>
                <div class="flex items-center gap-3 flex-wrap">
                    <Badge variant=BadgeVariant::Default>"Default"</Badge>
                    <Badge variant=BadgeVariant::Secondary>"Secondary"</Badge>
                    <Badge variant=BadgeVariant::Destructive>"Destructive"</Badge>
                    <Badge variant=BadgeVariant::Outline>"Outline"</Badge>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"In Context"</h2>
                <div class="flex items-center gap-3 flex-wrap text-sm">
                    <div class="flex items-center gap-1.5">
                        "Status:" <Badge>"Active"</Badge>
                    </div>
                    <div class="flex items-center gap-1.5">
                        "Errors:" <Badge variant=BadgeVariant::Destructive>"3 failed"</Badge>
                    </div>
                    <div class="flex items-center gap-1.5">
                        "Version:" <Badge variant=BadgeVariant::Outline>"v2.1.0"</Badge>
                    </div>
                    <div class="flex items-center gap-1.5">
                        "Type:" <Badge variant=BadgeVariant::Secondary>"Draft"</Badge>
                    </div>
                </div>
            </section>
        </div>
    }
}
