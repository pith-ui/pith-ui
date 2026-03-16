use leptos::prelude::*;
use radix_leptos_primitives::dialog::*;

#[component]
pub fn DialogPage() -> impl IntoView {
    let (modal, set_modal) = signal(true);
    let (animated, set_animated) = signal(false);
    let (count, set_count) = signal(0);
    let (has_destroy_button, set_has_destroy_button) = signal(true);
    let (controlled_open, set_controlled_open) = signal(false);
    let (event_log, set_event_log) = signal(Vec::<String>::new());
    let (prevent_escape, set_prevent_escape) = signal(false);
    let (prevent_outside_click, set_prevent_outside_click) = signal(false);

    view! {
        <Dialog modal=modal>
            <DialogTrigger>"open"</DialogTrigger>
            <DialogPortal>
                <DialogOverlay
                    attr:data-testid="overlay"
                    class:dialog-overlay=true
                    class:dialog-animated-overlay=animated
                    class:dialog-duration-50=animated
                />
                <DialogContent
                    class:dialog-content=true
                    class:dialog-animated-content=animated
                    class:dialog-duration-50=animated
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
                    class:dialog-overlay=true
                    style:background="tomato"
                />
                <DialogContent class:dialog-content=true>
                    <DialogTitle>"styled title"</DialogTitle>
                    <DialogClose>"close styled"</DialogClose>
                </DialogContent>
            </DialogPortal>
        </Dialog>

        <hr />

        // Controlled dialog
        <div data-testid="controlled-dialog-section">
            <h3>"Controlled Dialog"</h3>

            <label>
                <input
                    type="checkbox"
                    data-testid="controlled-dialog-checkbox"
                    prop:checked=move || controlled_open.get()
                    on:change=move |ev| {
                        use web_sys::wasm_bindgen::JsCast;
                        let target: web_sys::HtmlInputElement = ev.target().unwrap().unchecked_into();
                        set_controlled_open.set(target.checked());
                    }
                />
                " controlled open"
            </label>
            <button
                type="button"
                data-testid="controlled-dialog-external-close"
                on:click=move |_| set_controlled_open.set(false)
            >
                "external close"
            </button>
            <span data-testid="controlled-dialog-state">
                {move || if controlled_open.get() { "open" } else { "closed" }}
            </span>

            <Dialog
                open=controlled_open
                on_open_change=Callback::new(move |value: bool| set_controlled_open.set(value))
            >
                <DialogTrigger attr:data-testid="controlled-dialog-trigger">"open controlled"</DialogTrigger>
                <DialogPortal>
                    <DialogOverlay class:dialog-overlay=true />
                    <DialogContent class:dialog-content=true attr:data-testid="controlled-dialog-content">
                        <DialogTitle>"controlled title"</DialogTitle>
                        <DialogDescription>"controlled description"</DialogDescription>
                        <DialogClose attr:data-testid="controlled-dialog-close">"close controlled"</DialogClose>
                    </DialogContent>
                </DialogPortal>
            </Dialog>
        </div>

        <hr />

        // Callback contract dialog
        <div data-testid="callback-dialog-section">
            <h3>"Callback Dialog"</h3>
            <label>
                <input
                    type="checkbox"
                    data-testid="prevent-escape"
                    prop:checked=move || prevent_escape.get()
                    on:change=move |ev| {
                        use web_sys::wasm_bindgen::JsCast;
                        let target: web_sys::HtmlInputElement = ev.target().unwrap().unchecked_into();
                        set_prevent_escape.set(target.checked());
                    }
                />
                " prevent escape"
            </label>
            <label>
                <input
                    type="checkbox"
                    data-testid="prevent-outside-click"
                    prop:checked=move || prevent_outside_click.get()
                    on:change=move |ev| {
                        use web_sys::wasm_bindgen::JsCast;
                        let target: web_sys::HtmlInputElement = ev.target().unwrap().unchecked_into();
                        set_prevent_outside_click.set(target.checked());
                    }
                />
                " prevent outside click"
            </label>
            <button type="button" data-testid="clear-event-log" on:click=move |_| set_event_log.set(vec![])>
                "clear log"
            </button>
            <span data-testid="event-log">{move || event_log.get().join(",")}</span>

            <Dialog>
                <DialogTrigger attr:data-testid="callback-trigger">"open callback"</DialogTrigger>
                <DialogPortal>
                    <DialogOverlay class:dialog-overlay=true attr:data-testid="callback-overlay" />
                    <DialogContent
                        class:dialog-content=true
                        attr:data-testid="callback-content"
                        on_open_auto_focus=Callback::new(move |_: web_sys::Event| {
                            set_event_log.update(|log| log.push("openAutoFocus".to_string()));
                        })
                        on_close_auto_focus=Callback::new(move |_: web_sys::Event| {
                            set_event_log.update(|log| log.push("closeAutoFocus".to_string()));
                        })
                        on_escape_key_down=Callback::new(move |event: web_sys::KeyboardEvent| {
                            set_event_log.update(|log| log.push("escapeKeyDown".to_string()));
                            if prevent_escape.get_untracked() {
                                event.prevent_default();
                            }
                        })
                        on_pointer_down_outside=Callback::new(move |event: web_sys::CustomEvent| {
                            set_event_log.update(|log| log.push("pointerDownOutside".to_string()));
                            if prevent_outside_click.get_untracked() {
                                event.prevent_default();
                            }
                        })
                        on_interact_outside=Callback::new(move |_: web_sys::CustomEvent| {
                            set_event_log.update(|log| log.push("interactOutside".to_string()));
                        })
                    >
                        <DialogTitle>"callback title"</DialogTitle>
                        <DialogClose attr:data-testid="callback-close">"close callback"</DialogClose>
                    </DialogContent>
                </DialogPortal>
            </Dialog>
        </div>

        <hr />

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
