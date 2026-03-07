use leptos::prelude::*;
use radix_leptos_primitives::dialog::*;

#[component]
pub fn DialogPage() -> impl IntoView {
    let (modal, set_modal) = signal(true);
    let (animated, set_animated) = signal(false);
    let (count, set_count) = signal(0);
    let (has_destroy_button, set_has_destroy_button) = signal(true);

    view! {
        <Dialog modal=modal>
            <DialogTrigger>"open"</DialogTrigger>
            <DialogPortal>
                <DialogOverlay
                    attr:data-testid="overlay"
                    attr:class=move || {
                        let mut classes = vec!["dialog-overlay".to_string()];
                        if animated.get() {
                            classes.push("dialog-animated-overlay".to_string());
                            classes.push("dialog-duration-50".to_string());
                        }
                        classes.join(" ")
                    }
                />
                <DialogContent
                    attr:class=move || {
                        let mut classes = vec!["dialog-content".to_string()];
                        if animated.get() {
                            classes.push("dialog-animated-content".to_string());
                            classes.push("dialog-duration-50".to_string());
                        }
                        classes.join(" ")
                    }
                >
                    <DialogTitle>"title"</DialogTitle>
                    <DialogDescription>"description"</DialogDescription>
                    <DialogClose>"close"</DialogClose>
                    <Show when=move || has_destroy_button.get()>
                        <div>
                            <button type="button" on:click=move |_| set_has_destroy_button.set(false)>
                                "destroy me"
                            </button>
                        </div>
                    </Show>
                </DialogContent>
            </DialogPortal>
        </Dialog>

        // Second dialog for internal styles testing
        <Dialog>
            <DialogTrigger attr:data-testid="styled-dialog-trigger">"open styled"</DialogTrigger>
            <DialogPortal>
                <DialogOverlay
                    attr:data-testid="styled-overlay"
                    attr:class="dialog-overlay"
                    attr:style="background: tomato"
                />
                <DialogContent attr:class="dialog-content">
                    <DialogTitle>"styled title"</DialogTitle>
                    <DialogClose>"close styled"</DialogClose>
                </DialogContent>
            </DialogPortal>
        </Dialog>

        <br />
        <br />

        <label>
            <input
                type="checkbox"
                prop:checked=move || modal.get()
                on:change=move |ev| {
                    use web_sys::wasm_bindgen::JsCast;
                    let target: web_sys::HtmlInputElement = ev.target().unwrap().unchecked_into();
                    set_modal.set(target.checked());
                }
            />
            " modal"
        </label>

        <br />

        <label>
            <input
                type="checkbox"
                prop:checked=move || animated.get()
                on:change=move |ev| {
                    use web_sys::wasm_bindgen::JsCast;
                    let target: web_sys::HtmlInputElement = ev.target().unwrap().unchecked_into();
                    set_animated.set(target.checked());
                }
            />
            " animated"
        </label>

        <br />

        <label>
            "count up "
            <button type="button" on:click=move |_| set_count.update(|c| *c += 1)>
                {move || count.get().to_string()}
            </button>
        </label>

        <br />

        <label>
            "name: "
            <input type="text" placeholder="name" />
        </label>
    }
}
