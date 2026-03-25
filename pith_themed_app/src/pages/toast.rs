use leptos::prelude::*;

use crate::theme::button::*;
use crate::theme::toast::*;

#[component]
pub fn ToastPage() -> impl IntoView {
    let (show_toast, set_show_toast) = signal(false);
    let (show_action_toast, set_show_action_toast) = signal(false);

    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Toast"</h1>
                <p class="text-muted-foreground mb-6">
                    "A succinct message that is displayed temporarily."
                </p>
            </div>

            <ThemedToastProvider>
                <section class="space-y-4">
                    <h2 class="text-lg font-semibold text-foreground">"Simple Toast"</h2>
                    <span on:click=move |_| set_show_toast.set(true)>
                        <Button variant=ButtonVariant::Outline>"Show Notification"</Button>
                    </span>
                    <ThemedToast
                        open=MaybeProp::derive(move || Some(show_toast.get()))
                        on_open_change=Callback::new(move |val: bool| set_show_toast.set(val))
                    >
                        <div class="grid gap-1">
                            <ThemedToastTitle>"Scheduled: Catch up"</ThemedToastTitle>
                            <ThemedToastDescription>"Friday, February 10, 2023 at 5:57 PM"</ThemedToastDescription>
                        </div>
                        <ThemedToastClose>
                            <XIcon />
                        </ThemedToastClose>
                    </ThemedToast>
                </section>

                <section class="space-y-4">
                    <h2 class="text-lg font-semibold text-foreground">"Toast with Action"</h2>
                    <span on:click=move |_| set_show_action_toast.set(true)>
                        <Button variant=ButtonVariant::Outline>"Add to Calendar"</Button>
                    </span>
                    <ThemedToast
                        open=MaybeProp::derive(move || Some(show_action_toast.get()))
                        on_open_change=Callback::new(move |val: bool| set_show_action_toast.set(val))
                    >
                        <div class="grid gap-1">
                            <ThemedToastTitle>"Event Added"</ThemedToastTitle>
                            <ThemedToastDescription>"Your event has been added to the calendar."</ThemedToastDescription>
                        </div>
                        <ThemedToastAction alt_text="Undo the action">
                            "Undo"
                        </ThemedToastAction>
                    </ThemedToast>
                </section>
            </ThemedToastProvider>
        </div>
    }
}
