use leptos::prelude::*;
use radix_leptos_primitives::popover::*;

#[component]
pub fn PopoverPage() -> impl IntoView {
    let (modal, set_modal) = signal(false);
    let (count, set_count) = signal(0);
    let (controlled_open, set_controlled_open) = signal(false);

    view! {
        <Popover modal=modal>
            <PopoverTrigger attr:class="popover-trigger" attr:data-custom="popover-trigger-custom">"open"</PopoverTrigger>
            <PopoverPortal>
                <PopoverContent attr:class="popover-content" side_offset=5.0 attr:data-custom="popover-content-custom" attr:style="color: rgb(255, 0, 0)">
                    <p>"Popover content"</p>
                    <PopoverClose attr:class="popover-close" attr:data-custom="popover-close-custom">"close"</PopoverClose>
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

        <br />
        <br />

        <h3>"Controlled Popover"</h3>

        <Popover
            open=Signal::derive(move || controlled_open.get())
            on_open_change=Callback::new(move |open: bool| set_controlled_open.set(open))
        >
            <PopoverTrigger
                attr:class="popover-trigger"
                attr:data-testid="controlled-trigger"
            >
                "controlled open"
            </PopoverTrigger>
            <PopoverPortal>
                <PopoverContent
                    attr:class="popover-content"
                    side_offset=5.0
                    attr:data-testid="controlled-content"
                >
                    <p>"Controlled popover content"</p>
                    <PopoverClose attr:class="popover-close">"close"</PopoverClose>
                    <PopoverArrow attr:class="popover-arrow" width=20.0 height=10.0 />
                </PopoverContent>
            </PopoverPortal>
        </Popover>

        <br />

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
            " open controlled"
        </label>
        <button
            type="button"
            data-testid="controlled-external-close"
            on:click=move |_| set_controlled_open.set(false)
        >
            "external close"
        </button>
        <span data-testid="controlled-open-state">
            {move || if controlled_open.get() { "open" } else { "closed" }}
        </span>

        <hr />

        <h3>"Anchor"</h3>
        <Popover>
            <div style="display: flex; gap: 200px; align-items: flex-start">
                <PopoverTrigger
                    attr:class="popover-trigger"
                    attr:data-testid="anchor-trigger"
                >
                    "anchor open"
                </PopoverTrigger>
                <PopoverAnchor as_child=true>
                    <div
                        data-testid="popover-anchor"
                        style="width: 100px; height: 30px; background: #ddd; display: flex; align-items: center; justify-content: center;"
                    >
                        "anchor"
                    </div>
                </PopoverAnchor>
            </div>
            <PopoverPortal>
                <PopoverContent
                    attr:class="popover-content"
                    side_offset=5.0
                    attr:data-testid="anchor-content"
                >
                    <p>"Anchored popover content"</p>
                    <PopoverClose attr:class="popover-close" attr:data-testid="anchor-close">
                        "close"
                    </PopoverClose>
                </PopoverContent>
            </PopoverPortal>
        </Popover>
    }
}
