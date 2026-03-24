use leptos::prelude::*;

use crate::theme::button::*;
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
                        <Button variant=ButtonVariant::Outline>"Open popover"</Button>
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
                                        class="col-span-2 flex h-8 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-xs placeholder:text-muted-foreground focus-visible:outline-none focus-visible:focus-ring"
                                        value="100%"
                                    />
                                </div>
                                <div class="grid grid-cols-3 items-center gap-4">
                                    <label class="text-sm font-medium text-foreground">"Height"</label>
                                    <input
                                        class="col-span-2 flex h-8 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-xs placeholder:text-muted-foreground focus-visible:outline-none focus-visible:focus-ring"
                                        value="25px"
                                    />
                                </div>
                                <div class="grid grid-cols-3 items-center gap-4">
                                    <label class="text-sm font-medium text-foreground">"Max. height"</label>
                                    <input
                                        class="col-span-2 flex h-8 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-xs placeholder:text-muted-foreground focus-visible:outline-none focus-visible:focus-ring"
                                        value="none"
                                    />
                                </div>
                                <div class="grid grid-cols-3 items-center gap-4">
                                    <label class="text-sm font-medium text-foreground">"Max. width"</label>
                                    <input
                                        class="col-span-2 flex h-8 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-xs placeholder:text-muted-foreground focus-visible:outline-none focus-visible:focus-ring"
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
                        <Button>"Show info"</Button>
                    </ThemedPopoverTrigger>
                    <ThemedPopoverContent>
                        <div class="space-y-2">
                            <h4 class="font-medium leading-none text-foreground">"About"</h4>
                            <p class="text-sm text-muted-foreground">
                                "This is a popover with rich content. It can contain any elements you need."
                            </p>
                            <ThemedPopoverClose>
                                <Button variant=ButtonVariant::Secondary size=ButtonSize::Sm>"Got it"</Button>
                            </ThemedPopoverClose>
                        </div>
                    </ThemedPopoverContent>
                </ThemedPopover>
            </section>
        </div>
    }
}
