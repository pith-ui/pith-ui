use cardo_ui::toggle_group::ToggleGroupType;
use leptos::prelude::*;

use crate::theme::toggle_group::*;

#[component]
pub fn ToggleGroupPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Toggle Group"</h1>
                <p class="text-muted-foreground mb-6">
                    "A set of two-state buttons that can be toggled on or off. "
                    "Supports single and multiple selection modes."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Single Select"</h2>
                <p class="text-sm text-muted-foreground">"Only one item can be active at a time."</p>
                <ThemedToggleGroup
                    r#type=ToggleGroupType::Single
                    default_value=vec!["bold".to_string()]
                >
                    <ThemedToggleGroupItem value=Signal::stored("bold".to_string())>
                        <svg class="size-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M6 12h9a4 4 0 0 1 0 8H7a1 1 0 0 1-1-1V5a1 1 0 0 1 1-1h7a4 4 0 0 1 0 8" />
                        </svg>
                    </ThemedToggleGroupItem>
                    <ThemedToggleGroupItem value=Signal::stored("italic".to_string())>
                        <svg class="size-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <line x1="19" x2="10" y1="4" y2="4" />
                            <line x1="14" x2="5" y1="20" y2="20" />
                            <line x1="15" x2="9" y1="4" y2="20" />
                        </svg>
                    </ThemedToggleGroupItem>
                    <ThemedToggleGroupItem value=Signal::stored("underline".to_string())>
                        <svg class="size-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M6 4v6a6 6 0 0 0 12 0V4" />
                            <line x1="4" x2="20" y1="20" y2="20" />
                        </svg>
                    </ThemedToggleGroupItem>
                </ThemedToggleGroup>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Multiple Select"</h2>
                <p class="text-sm text-muted-foreground">"Multiple items can be active simultaneously."</p>
                <ThemedToggleGroup
                    r#type=ToggleGroupType::Multiple
                    default_value=vec!["bold".to_string(), "italic".to_string()]
                >
                    <ThemedToggleGroupItem value=Signal::stored("bold".to_string())>
                        <svg class="size-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M6 12h9a4 4 0 0 1 0 8H7a1 1 0 0 1-1-1V5a1 1 0 0 1 1-1h7a4 4 0 0 1 0 8" />
                        </svg>
                    </ThemedToggleGroupItem>
                    <ThemedToggleGroupItem value=Signal::stored("italic".to_string())>
                        <svg class="size-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <line x1="19" x2="10" y1="4" y2="4" />
                            <line x1="14" x2="5" y1="20" y2="20" />
                            <line x1="15" x2="9" y1="4" y2="20" />
                        </svg>
                    </ThemedToggleGroupItem>
                    <ThemedToggleGroupItem value=Signal::stored("underline".to_string())>
                        <svg class="size-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M6 4v6a6 6 0 0 0 12 0V4" />
                            <line x1="4" x2="20" y1="20" y2="20" />
                        </svg>
                    </ThemedToggleGroupItem>
                </ThemedToggleGroup>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Disabled"</h2>
                <ThemedToggleGroup r#type=ToggleGroupType::Single disabled=true>
                    <ThemedToggleGroupItem value=Signal::stored("a".to_string())>"A"</ThemedToggleGroupItem>
                    <ThemedToggleGroupItem value=Signal::stored("b".to_string())>"B"</ThemedToggleGroupItem>
                    <ThemedToggleGroupItem value=Signal::stored("c".to_string())>"C"</ThemedToggleGroupItem>
                </ThemedToggleGroup>
            </section>
        </div>
    }
}
