use cardo_ui::accordion::AccordionType;
use leptos::prelude::*;

use crate::theme::accordion::*;

#[component]
pub fn AccordionPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Accordion"</h1>
                <p class="text-muted-foreground mb-6">
                    "Vertically stacked set of interactive headings that each reveal a section of content."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Single (Collapsible)"</h2>
                <div class="max-w-lg">
                    <ThemedAccordion r#type=AccordionType::Single collapsible=true>
                        <ThemedAccordionItem value="item-1">
                            <ThemedAccordionTrigger>"Is it accessible?"</ThemedAccordionTrigger>
                            <ThemedAccordionContent>
                                "Yes. It adheres to the WAI-ARIA design pattern."
                            </ThemedAccordionContent>
                        </ThemedAccordionItem>
                        <ThemedAccordionItem value="item-2">
                            <ThemedAccordionTrigger>"Is it styled?"</ThemedAccordionTrigger>
                            <ThemedAccordionContent>
                                "Yes. It comes with default styles that matches the other components' aesthetic."
                            </ThemedAccordionContent>
                        </ThemedAccordionItem>
                        <ThemedAccordionItem value="item-3">
                            <ThemedAccordionTrigger>"Is it animated?"</ThemedAccordionTrigger>
                            <ThemedAccordionContent>
                                "Yes. It's animated by default, but you can disable it if you prefer."
                            </ThemedAccordionContent>
                        </ThemedAccordionItem>
                    </ThemedAccordion>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Single (Default Open)"</h2>
                <div class="max-w-lg">
                    <ThemedAccordion r#type=AccordionType::Single collapsible=true default_value="item-2">
                        <ThemedAccordionItem value="item-1">
                            <ThemedAccordionTrigger>"First item"</ThemedAccordionTrigger>
                            <ThemedAccordionContent>
                                "Content for the first item."
                            </ThemedAccordionContent>
                        </ThemedAccordionItem>
                        <ThemedAccordionItem value="item-2">
                            <ThemedAccordionTrigger>"Second item (default open)"</ThemedAccordionTrigger>
                            <ThemedAccordionContent>
                                "This item is open by default."
                            </ThemedAccordionContent>
                        </ThemedAccordionItem>
                    </ThemedAccordion>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Disabled Item"</h2>
                <div class="max-w-lg">
                    <ThemedAccordion r#type=AccordionType::Single collapsible=true>
                        <ThemedAccordionItem value="enabled">
                            <ThemedAccordionTrigger>"Enabled item"</ThemedAccordionTrigger>
                            <ThemedAccordionContent>"This item works normally."</ThemedAccordionContent>
                        </ThemedAccordionItem>
                        <ThemedAccordionItem value="disabled" disabled=true>
                            <ThemedAccordionTrigger>"Disabled item"</ThemedAccordionTrigger>
                            <ThemedAccordionContent>"You should not see this."</ThemedAccordionContent>
                        </ThemedAccordionItem>
                    </ThemedAccordion>
                </div>
            </section>
        </div>
    }
}
