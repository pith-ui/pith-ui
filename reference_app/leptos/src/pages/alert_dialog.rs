use leptos::prelude::*;
use radix_leptos_primitives::alert_dialog::*;

#[component]
pub fn AlertDialogPage() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (controlled_open, set_controlled_open) = signal(false);

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

        <br />
        <br />

        // Controlled alert dialog
        <AlertDialog
            open=controlled_open
            on_open_change=Callback::new(move |open: bool| set_controlled_open.set(open))
        >
            <AlertDialogTrigger attr:data-testid="controlled-trigger">"controlled delete"</AlertDialogTrigger>
            <AlertDialogPortal>
                <AlertDialogOverlay attr:class="alert-dialog-overlay" />
                <AlertDialogContent attr:class="alert-dialog-content" attr:data-testid="controlled-content">
                    <AlertDialogTitle>"Controlled Alert"</AlertDialogTitle>
                    <AlertDialogDescription>"This is a controlled alert dialog."</AlertDialogDescription>
                    <AlertDialogCancel attr:data-testid="controlled-cancel">"cancel"</AlertDialogCancel>
                    <AlertDialogAction attr:data-testid="controlled-action">"confirm"</AlertDialogAction>
                </AlertDialogContent>
            </AlertDialogPortal>
        </AlertDialog>

        <label>
            <input
                type="checkbox"
                data-testid="controlled-checkbox"
                prop:checked=move || controlled_open.get()
                on:change=move |ev| {
                    use web_sys::wasm_bindgen::JsCast;
                    let target: web_sys::HtmlInputElement = ev.target().unwrap().unchecked_into();
                    set_controlled_open.set(target.checked());
                }
            />
            " controlled open"
        </label>
        <span data-testid="controlled-state">{move || if controlled_open.get() { "open" } else { "closed" }}</span>
        <button data-testid="controlled-external-close" on:click=move |_| set_controlled_open.set(false)>
            "external close"
        </button>
    }
}
