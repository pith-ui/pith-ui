use leptos::prelude::*;

use crate::theme::toolbar::*;

#[component]
pub fn ToolbarPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Toolbar"</h1>
                <p class="text-muted-foreground mb-6">
                    "A container for grouping interactive controls like toggle groups, buttons, and links with roving focus navigation."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Text Editor Toolbar"</h2>
                <p class="text-sm text-muted-foreground">
                    "Formatting toggles with a separator and link button. Use arrow keys to navigate between items."
                </p>
                <ThemedToolbar>
                    <ThemedToolbarToggleGroup
                        r#type=ToggleGroupType::Multiple
                        default_value=vec!["bold".to_string()]
                    >
                        <ThemedToolbarToggleItem value=Signal::stored("bold".to_string())>
                            <svg class="size-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <path d="M6 12h9a4 4 0 0 1 0 8H7a1 1 0 0 1-1-1V5a1 1 0 0 1 1-1h7a4 4 0 0 1 0 8" />
                            </svg>
                        </ThemedToolbarToggleItem>
                        <ThemedToolbarToggleItem value=Signal::stored("italic".to_string())>
                            <svg class="size-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <line x1="19" x2="10" y1="4" y2="4" />
                                <line x1="14" x2="5" y1="20" y2="20" />
                                <line x1="15" x2="9" y1="4" y2="20" />
                            </svg>
                        </ThemedToolbarToggleItem>
                        <ThemedToolbarToggleItem value=Signal::stored("underline".to_string())>
                            <svg class="size-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <path d="M6 4v6a6 6 0 0 0 12 0V4" />
                                <line x1="4" x2="20" y1="20" y2="20" />
                            </svg>
                        </ThemedToolbarToggleItem>
                    </ThemedToolbarToggleGroup>

                    <ThemedToolbarSeparator />

                    <ThemedToolbarLink>
                        <svg class="size-4 mr-1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71" />
                            <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71" />
                        </svg>
                        "Link"
                    </ThemedToolbarLink>
                </ThemedToolbar>
            </section>
        </div>
    }
}
