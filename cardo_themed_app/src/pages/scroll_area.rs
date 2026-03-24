use cardo_ui::scroll_area::ScrollAreaType;
use leptos::prelude::*;

use crate::theme::scroll_area::*;

#[component]
pub fn ScrollAreaPage() -> impl IntoView {
    let tags: &[&str] = &[
        "v1.2.0-beta.44",
        "v1.2.0-beta.43",
        "v1.2.0-beta.42",
        "v1.2.0-beta.41",
        "v1.2.0-beta.40",
        "v1.2.0-beta.39",
        "v1.2.0-beta.38",
        "v1.2.0-beta.37",
        "v1.2.0-beta.36",
        "v1.2.0-beta.35",
        "v1.2.0-beta.34",
        "v1.2.0-beta.33",
        "v1.2.0-beta.32",
        "v1.2.0-beta.31",
        "v1.2.0-beta.30",
        "v1.2.0-beta.29",
        "v1.2.0-beta.28",
        "v1.2.0-beta.27",
        "v1.2.0-beta.26",
        "v1.2.0-beta.25",
    ];

    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Scroll Area"</h1>
                <p class="text-muted-foreground mb-6">
                    "Augments native scroll functionality for custom, cross-browser styling."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Tags"</h2>
                <ThemedScrollArea class="h-72 w-48 rounded-md border" r#type=ScrollAreaType::Always>
                    <div class="p-4">
                        <h4 class="mb-4 text-sm font-medium leading-none text-foreground">"Tags"</h4>
                        {tags.iter().map(|tag| {
                            let tag = tag.to_string();
                            view! {
                                <div class="text-sm text-foreground">{tag}</div>
                                <div class="my-2 h-px bg-border" />
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </ThemedScrollArea>
            </section>
        </div>
    }
}
