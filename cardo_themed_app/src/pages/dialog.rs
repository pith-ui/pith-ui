use leptos::prelude::*;

use crate::theme::dialog::*;

#[component]
pub fn DialogPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Dialog"</h1>
                <p class="text-muted-foreground mb-6">
                    "A window overlaid on the primary content. Renders into a portal with focus management."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Basic Dialog"</h2>
                <ThemedDialog>
                    <ThemedDialogTrigger>
                        <button class="inline-flex items-center justify-center rounded-md text-sm font-medium h-9 px-4 py-2 bg-primary text-primary-foreground shadow-xs hover:bg-primary/90">
                            "Open Dialog"
                        </button>
                    </ThemedDialogTrigger>
                    <ThemedDialogContent>
                        <ThemedDialogTitle>"Are you absolutely sure?"</ThemedDialogTitle>
                        <ThemedDialogDescription>
                            "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                        </ThemedDialogDescription>
                        <div class="flex justify-end gap-2 mt-2">
                            <ThemedDialogClose>
                                <button class="inline-flex items-center justify-center rounded-md text-sm font-medium h-9 px-4 py-2 border border-input bg-background shadow-xs hover:bg-accent hover:text-accent-foreground">
                                    "Cancel"
                                </button>
                            </ThemedDialogClose>
                            <ThemedDialogClose>
                                <button class="inline-flex items-center justify-center rounded-md text-sm font-medium h-9 px-4 py-2 bg-primary text-primary-foreground shadow-xs hover:bg-primary/90">
                                    "Continue"
                                </button>
                            </ThemedDialogClose>
                        </div>
                    </ThemedDialogContent>
                </ThemedDialog>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Form Dialog"</h2>
                <ThemedDialog>
                    <ThemedDialogTrigger>
                        <button class="inline-flex items-center justify-center rounded-md text-sm font-medium h-9 px-4 py-2 border border-input bg-background shadow-xs hover:bg-accent hover:text-accent-foreground">
                            "Edit Profile"
                        </button>
                    </ThemedDialogTrigger>
                    <ThemedDialogContent>
                        <ThemedDialogTitle>"Edit profile"</ThemedDialogTitle>
                        <ThemedDialogDescription>
                            "Make changes to your profile here. Click save when you're done."
                        </ThemedDialogDescription>
                        <div class="grid gap-4 py-4">
                            <div class="grid grid-cols-4 items-center gap-4">
                                <label class="text-right text-sm font-medium text-foreground">"Name"</label>
                                <input
                                    class="col-span-3 flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-xs placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
                                    value="Pedro Duarte"
                                />
                            </div>
                            <div class="grid grid-cols-4 items-center gap-4">
                                <label class="text-right text-sm font-medium text-foreground">"Username"</label>
                                <input
                                    class="col-span-3 flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-xs placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
                                    value="@peduarte"
                                />
                            </div>
                        </div>
                        <div class="flex justify-end">
                            <ThemedDialogClose>
                                <button class="inline-flex items-center justify-center rounded-md text-sm font-medium h-9 px-4 py-2 bg-primary text-primary-foreground shadow-xs hover:bg-primary/90">
                                    "Save changes"
                                </button>
                            </ThemedDialogClose>
                        </div>
                    </ThemedDialogContent>
                </ThemedDialog>
            </section>
        </div>
    }
}
