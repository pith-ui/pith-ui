use leptos::prelude::*;

use crate::theme::select::*;

#[component]
pub fn SelectPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Select"</h1>
                <p class="text-muted-foreground mb-6">
                    "Displays a list of options for the user to pick from — triggered by a button."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Basic Select"</h2>
                <div class="w-[240px]">
                    <ThemedSelect>
                        <ThemedSelectTrigger placeholder="Select a fruit..." />
                        <ThemedSelectContent>
                            <ThemedSelectItem value="apple">"Apple"</ThemedSelectItem>
                            <ThemedSelectItem value="banana">"Banana"</ThemedSelectItem>
                            <ThemedSelectItem value="blueberry">"Blueberry"</ThemedSelectItem>
                            <ThemedSelectItem value="grapes">"Grapes"</ThemedSelectItem>
                            <ThemedSelectItem value="pineapple">"Pineapple"</ThemedSelectItem>
                        </ThemedSelectContent>
                    </ThemedSelect>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Grouped Select"</h2>
                <div class="w-[240px]">
                    <ThemedSelect>
                        <ThemedSelectTrigger placeholder="Select a timezone..." />
                        <ThemedSelectContent>
                            <ThemedSelectGroup>
                                <ThemedSelectLabel>"North America"</ThemedSelectLabel>
                                <ThemedSelectItem value="est">"Eastern Standard Time (EST)"</ThemedSelectItem>
                                <ThemedSelectItem value="cst">"Central Standard Time (CST)"</ThemedSelectItem>
                                <ThemedSelectItem value="mst">"Mountain Standard Time (MST)"</ThemedSelectItem>
                                <ThemedSelectItem value="pst">"Pacific Standard Time (PST)"</ThemedSelectItem>
                            </ThemedSelectGroup>
                            <ThemedSelectSeparator />
                            <ThemedSelectGroup>
                                <ThemedSelectLabel>"Europe"</ThemedSelectLabel>
                                <ThemedSelectItem value="gmt">"Greenwich Mean Time (GMT)"</ThemedSelectItem>
                                <ThemedSelectItem value="cet">"Central European Time (CET)"</ThemedSelectItem>
                                <ThemedSelectItem value="eet">"Eastern European Time (EET)"</ThemedSelectItem>
                            </ThemedSelectGroup>
                        </ThemedSelectContent>
                    </ThemedSelect>
                </div>
            </section>
        </div>
    }
}
