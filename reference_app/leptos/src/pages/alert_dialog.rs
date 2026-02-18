use leptos::prelude::*;
use radix_leptos_alert_dialog::*;

#[component]
pub fn AlertDialogPage() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <AlertDialog>
            <AlertDialogTrigger>"delete"</AlertDialogTrigger>
            <AlertDialogPortal>
                <AlertDialogOverlay attr:class="alert-dialog-overlay" />
                <AlertDialogContent attr:class="alert-dialog-content">
                    <AlertDialogTitle>"Are you sure?"</AlertDialogTitle>
                    <AlertDialogDescription>"This action cannot be undone."</AlertDialogDescription>
                    <AlertDialogCancel>"cancel"</AlertDialogCancel>
                    <AlertDialogAction>"confirm"</AlertDialogAction>
                </AlertDialogContent>
            </AlertDialogPortal>
        </AlertDialog>

        <br />
        <br />

        <label>
            "count up "
            <button type="button" on:click=move |_| set_count.update(|c| *c += 1)>
                {move || count.get().to_string()}
            </button>
        </label>
    }
}
