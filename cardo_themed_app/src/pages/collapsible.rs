use leptos::prelude::*;

use crate::theme::button::*;
use crate::theme::collapsible::*;

#[component]
pub fn CollapsiblePage() -> impl IntoView {
    let (open, set_open) = signal(false);

    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Collapsible"</h1>
                <p class="text-muted-foreground mb-6">
                    "An interactive component which expands/collapses a panel."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Demo"</h2>
                <div class="max-w-sm">
                    <ThemedCollapsible
                        open=open
                        on_open_change=move |val| set_open.set(val)
                    >
                        <div class="flex items-center justify-between rounded-lg border px-4 py-3">
                            <span class="text-sm font-semibold text-foreground">
                                "@peduarte starred 3 repositories"
                            </span>
                            <ThemedCollapsibleTrigger>
                                <Button variant=ButtonVariant::Ghost size=ButtonSize::Icon>
                                    <svg
                                        class="size-4 transition-transform duration-200"
                                        class:rotate-180=open
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 24 24"
                                        fill="none"
                                        stroke="currentColor"
                                        stroke-width="2"
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                    >
                                        <path d="m6 9 6 6 6-6" />
                                    </svg>
                                </Button>
                            </ThemedCollapsibleTrigger>
                        </div>

                        <div class="mt-2 rounded-md border px-4 py-2 text-sm text-foreground">
                            "@radix-ui/primitives"
                        </div>

                        <ThemedCollapsibleContent>
                            <div class="mt-2 space-y-2">
                                <div class="rounded-md border px-4 py-2 text-sm text-foreground">
                                    "@radix-ui/colors"
                                </div>
                                <div class="rounded-md border px-4 py-2 text-sm text-foreground">
                                    "@radix-ui/themes"
                                </div>
                            </div>
                        </ThemedCollapsibleContent>
                    </ThemedCollapsible>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Default Open"</h2>
                <div class="max-w-sm">
                    <ThemedCollapsible default_open=true>
                        <div class="flex items-center justify-between rounded-lg border px-4 py-3">
                            <span class="text-sm font-semibold text-foreground">
                                "Settings"
                            </span>
                            <ThemedCollapsibleTrigger>
                                <Button variant=ButtonVariant::Ghost size=ButtonSize::Icon>
                                    <svg
                                        class="size-4"
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 24 24"
                                        fill="none"
                                        stroke="currentColor"
                                        stroke-width="2"
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                    >
                                        <path d="m6 9 6 6 6-6" />
                                    </svg>
                                </Button>
                            </ThemedCollapsibleTrigger>
                        </div>

                        <ThemedCollapsibleContent>
                            <div class="mt-2 space-y-2">
                                <div class="rounded-md border px-4 py-2 text-sm text-foreground">
                                    "General"
                                </div>
                                <div class="rounded-md border px-4 py-2 text-sm text-foreground">
                                    "Privacy"
                                </div>
                                <div class="rounded-md border px-4 py-2 text-sm text-foreground">
                                    "Notifications"
                                </div>
                            </div>
                        </ThemedCollapsibleContent>
                    </ThemedCollapsible>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Disabled"</h2>
                <div class="max-w-sm">
                    <ThemedCollapsible disabled=true>
                        <div class="flex items-center justify-between rounded-lg border px-4 py-3 opacity-50">
                            <span class="text-sm font-semibold text-foreground">
                                "Cannot expand (disabled)"
                            </span>
                            <ThemedCollapsibleTrigger>
                                <Button variant=ButtonVariant::Ghost size=ButtonSize::Icon disabled=true>
                                    <svg
                                        class="size-4"
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 24 24"
                                        fill="none"
                                        stroke="currentColor"
                                        stroke-width="2"
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                    >
                                        <path d="m6 9 6 6 6-6" />
                                    </svg>
                                </Button>
                            </ThemedCollapsibleTrigger>
                        </div>

                        <ThemedCollapsibleContent>
                            <div class="mt-2 rounded-md border px-4 py-2 text-sm text-foreground">
                                "You should not see this."
                            </div>
                        </ThemedCollapsibleContent>
                    </ThemedCollapsible>
                </div>
            </section>
        </div>
    }
}
