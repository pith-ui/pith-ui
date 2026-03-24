use leptos::prelude::*;

use crate::theme::popover::*;

#[component]
pub fn PopoverPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Popover"</h1>
                <p class="text-muted-foreground mb-6">
                    "Displays rich content in a portal, triggered by a button."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Basic"</h2>
                <ThemedPopover>
                    <ThemedPopoverTrigger>
                        <button class="inline-flex items-center justify-center rounded-md text-sm font-medium h-9 px-4 py-2 border border-input bg-background shadow-xs hover:bg-accent hover:text-accent-foreground">
                            "Open popover"
                        </button>
                    </ThemedPopoverTrigger>
                    <ThemedPopoverContent>
                        <div class="grid gap-4">
                            <div class="space-y-2">
                                <h4 class="font-medium leading-none text-foreground">"Dimensions"</h4>
                                <p class="text-sm text-muted-foreground">
                                    "Set the dimensions for the layer."
                                </p>
                            </div>
                            <div class="grid gap-2">
                                <div class="grid grid-cols-3 items-center gap-4">
                                    <label class="text-sm font-medium text-foreground">"Width"</label>
                                    <input
                                        class="col-span-2 flex h-8 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-xs placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
                                        value="100%"
                                    />
                                </div>
                                <div class="grid grid-cols-3 items-center gap-4">
                                    <label class="text-sm font-medium text-foreground">"Height"</label>
                                    <input
                                        class="col-span-2 flex h-8 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-xs placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
                                        value="25px"
                                    />
                                </div>
                                <div class="grid grid-cols-3 items-center gap-4">
                                    <label class="text-sm font-medium text-foreground">"Max. height"</label>
                                    <input
                                        class="col-span-2 flex h-8 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-xs placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
                                        value="none"
                                    />
                                </div>
                                <div class="grid grid-cols-3 items-center gap-4">
                                    <label class="text-sm font-medium text-foreground">"Max. width"</label>
                                    <input
                                        class="col-span-2 flex h-8 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-xs placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
                                        value="300px"
                                    />
                                </div>
                            </div>
                        </div>
                    </ThemedPopoverContent>
                </ThemedPopover>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"With Close Button"</h2>
                <ThemedPopover>
                    <ThemedPopoverTrigger>
                        <button class="inline-flex items-center justify-center rounded-md text-sm font-medium h-9 px-4 py-2 bg-primary text-primary-foreground shadow-xs hover:bg-primary/90">
                            "Show info"
                        </button>
                    </ThemedPopoverTrigger>
                    <ThemedPopoverContent>
                        <div class="space-y-2">
                            <h4 class="font-medium leading-none text-foreground">"About"</h4>
                            <p class="text-sm text-muted-foreground">
                                "This is a popover with rich content. It can contain any elements you need."
                            </p>
                            <ThemedPopoverClose>
                                <button class="inline-flex items-center justify-center rounded-md text-sm font-medium h-8 px-3 bg-secondary text-secondary-foreground shadow-xs hover:bg-secondary/80">
                                    "Got it"
                                </button>
                            </ThemedPopoverClose>
                        </div>
                    </ThemedPopoverContent>
                </ThemedPopover>
            </section>
        </div>
    }
}
