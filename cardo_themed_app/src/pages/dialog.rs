use leptos::prelude::*;

use crate::theme::button::*;
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
                        <Button>"Open Dialog"</Button>
                    </ThemedDialogTrigger>
                    <ThemedDialogContent>
                        <ThemedDialogTitle>"Are you absolutely sure?"</ThemedDialogTitle>
                        <ThemedDialogDescription>
                            "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                        </ThemedDialogDescription>
                        <div class="flex justify-end gap-2 mt-2">
                            <ThemedDialogClose>
                                <Button variant=ButtonVariant::Outline>"Cancel"</Button>
                            </ThemedDialogClose>
                            <ThemedDialogClose>
                                <Button>"Continue"</Button>
                            </ThemedDialogClose>
                        </div>
                    </ThemedDialogContent>
                </ThemedDialog>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Form Dialog"</h2>
                <ThemedDialog>
                    <ThemedDialogTrigger>
                        <Button variant=ButtonVariant::Outline>"Edit Profile"</Button>
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
                                    class="col-span-3 flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-xs placeholder:text-muted-foreground focus-visible:outline-none focus-visible:focus-ring"
                                    value="Pedro Duarte"
                                />
                            </div>
                            <div class="grid grid-cols-4 items-center gap-4">
                                <label class="text-right text-sm font-medium text-foreground">"Username"</label>
                                <input
                                    class="col-span-3 flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-xs placeholder:text-muted-foreground focus-visible:outline-none focus-visible:focus-ring"
                                    value="@peduarte"
                                />
                            </div>
                        </div>
                        <div class="flex justify-end">
                            <ThemedDialogClose>
                                <Button>"Save changes"</Button>
                            </ThemedDialogClose>
                        </div>
                    </ThemedDialogContent>
                </ThemedDialog>
            </section>
        </div>
    }
}
