use leptos::prelude::*;

use crate::theme::hover_card::*;

#[component]
pub fn HoverCardPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Hover Card"</h1>
                <p class="text-muted-foreground mb-6">
                    "For sighted users to preview content available behind a link."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"User Profile Card"</h2>
                <p class="text-sm text-muted-foreground">
                    "Hover over the link below to see a preview card."
                </p>
                <ThemedHoverCard>
                    <ThemedHoverCardTrigger>
                        <a
                            href="https://github.com/radix-ui"
                            target="_blank"
                            rel="noreferrer noopener"
                            class="text-sm font-medium underline underline-offset-4 text-primary"
                        >
                            "@radix-ui"
                        </a>
                    </ThemedHoverCardTrigger>
                    <ThemedHoverCardContent>
                        <div class="flex justify-between space-x-4">
                            <div class="shrink-0">
                                <div class="size-10 rounded-full bg-muted flex items-center justify-center text-sm font-semibold text-muted-foreground">
                                    "R"
                                </div>
                            </div>
                            <div class="space-y-1">
                                <h4 class="text-sm font-semibold">"@radix-ui"</h4>
                                <p class="text-sm text-muted-foreground">
                                    "Components, icons, and colors for building high-quality, accessible UI."
                                </p>
                                <div class="flex items-center pt-2">
                                    <span class="text-xs text-muted-foreground">
                                        "Joined December 2020"
                                    </span>
                                </div>
                            </div>
                        </div>
                    </ThemedHoverCardContent>
                </ThemedHoverCard>
            </section>
        </div>
    }
}
