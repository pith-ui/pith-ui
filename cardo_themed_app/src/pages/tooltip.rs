use leptos::prelude::*;

use crate::theme::tooltip::*;

#[component]
pub fn TooltipPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Tooltip"</h1>
                <p class="text-muted-foreground mb-6">
                    "A popup that displays information related to an element when the element receives keyboard focus or the mouse hovers over it."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Basic"</h2>
                <div class="flex items-center gap-4">
                    <ThemedTooltip content="Add to library">
                        <button class="inline-flex items-center justify-center rounded-md text-sm font-medium h-9 px-4 py-2 border border-input bg-background shadow-xs hover:bg-accent hover:text-accent-foreground">
                            "Hover me"
                        </button>
                    </ThemedTooltip>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Multiple Tooltips"</h2>
                <p class="text-sm text-muted-foreground">"Hover between buttons to see the shared delay."</p>
                <div class="flex items-center gap-4">
                    <ThemedTooltip content="Bold">
                        <button class="inline-flex items-center justify-center size-9 rounded-md border border-input bg-background shadow-xs hover:bg-accent hover:text-accent-foreground">
                            <svg class="size-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <path d="M6 12h9a4 4 0 0 1 0 8H7a1 1 0 0 1-1-1V5a1 1 0 0 1 1-1h7a4 4 0 0 1 0 8" />
                            </svg>
                        </button>
                    </ThemedTooltip>
                    <ThemedTooltip content="Italic">
                        <button class="inline-flex items-center justify-center size-9 rounded-md border border-input bg-background shadow-xs hover:bg-accent hover:text-accent-foreground">
                            <svg class="size-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <line x1="19" x2="10" y1="4" y2="4" />
                                <line x1="14" x2="5" y1="20" y2="20" />
                                <line x1="15" x2="9" y1="4" y2="20" />
                            </svg>
                        </button>
                    </ThemedTooltip>
                    <ThemedTooltip content="Underline">
                        <button class="inline-flex items-center justify-center size-9 rounded-md border border-input bg-background shadow-xs hover:bg-accent hover:text-accent-foreground">
                            <svg class="size-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <path d="M6 4v6a6 6 0 0 0 12 0V4" />
                                <line x1="4" x2="20" y1="20" y2="20" />
                            </svg>
                        </button>
                    </ThemedTooltip>
                </div>
            </section>
        </div>
    }
}
