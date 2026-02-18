use leptos::prelude::*;
use radix_leptos_popover::*;

#[component]
pub fn PopoverPage() -> impl IntoView {
    let (modal, set_modal) = signal(false);
    let (count, set_count) = signal(0);

    view! {
        <Popover modal=modal>
            <PopoverTrigger attr:class="popover-trigger">"open"</PopoverTrigger>
            <PopoverPortal>
                <PopoverContent attr:class="popover-content" side_offset=5.0>
                    <p>"Popover content"</p>
                    <PopoverClose attr:class="popover-close">"close"</PopoverClose>
                    <PopoverArrow attr:class="popover-arrow" width=20.0 height=10.0 />
                </PopoverContent>
            </PopoverPortal>
        </Popover>

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
        <br />

        <button
            data-testid="count-button"
            on:click=move |_| set_count.update(|c| *c += 1)
        >
            "count up"
        </button>
        <span data-testid="count-value">{move || count.get().to_string()}</span>

        <br />
        <br />

        <input data-testid="outside-input" placeholder="name" />
    }
}
