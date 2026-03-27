use leptos::prelude::*;

use crate::theme::alert_dialog::*;
use crate::theme::button::*;

#[component]
pub fn AlertDialogPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Alert Dialog"</h1>
                <p class="text-muted-foreground mb-6">
                    "A modal dialog that interrupts the user with important content and expects a response."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Destructive Action"</h2>
                <ThemedAlertDialog>
                    <ThemedAlertDialogTrigger>
                        <Button variant=ButtonVariant::Destructive>"Delete Account"</Button>
                    </ThemedAlertDialogTrigger>
                    <ThemedAlertDialogContent>
                        <ThemedAlertDialogHeader>
                            <ThemedAlertDialogTitle>"Are you absolutely sure?"</ThemedAlertDialogTitle>
                            <ThemedAlertDialogDescription>
                                "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                            </ThemedAlertDialogDescription>
                        </ThemedAlertDialogHeader>
                        <ThemedAlertDialogFooter>
                            <ThemedAlertDialogCancel>
                                <Button variant=ButtonVariant::Outline>"Cancel"</Button>
                            </ThemedAlertDialogCancel>
                            <ThemedAlertDialogAction>
                                <Button variant=ButtonVariant::Destructive>"Continue"</Button>
                            </ThemedAlertDialogAction>
                        </ThemedAlertDialogFooter>
                    </ThemedAlertDialogContent>
                </ThemedAlertDialog>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Basic Confirmation"</h2>
                <ThemedAlertDialog>
                    <ThemedAlertDialogTrigger>
                        <Button variant=ButtonVariant::Outline>"Show Alert"</Button>
                    </ThemedAlertDialogTrigger>
                    <ThemedAlertDialogContent>
                        <ThemedAlertDialogHeader>
                            <ThemedAlertDialogTitle>"Confirm Changes"</ThemedAlertDialogTitle>
                            <ThemedAlertDialogDescription>
                                "You have unsaved changes. Are you sure you want to leave this page? Your changes will be lost."
                            </ThemedAlertDialogDescription>
                        </ThemedAlertDialogHeader>
                        <ThemedAlertDialogFooter>
                            <ThemedAlertDialogCancel>
                                <Button variant=ButtonVariant::Outline>"Stay"</Button>
                            </ThemedAlertDialogCancel>
                            <ThemedAlertDialogAction>
                                <Button>"Leave"</Button>
                            </ThemedAlertDialogAction>
                        </ThemedAlertDialogFooter>
                    </ThemedAlertDialogContent>
                </ThemedAlertDialog>
            </section>
        </div>
    }
}
