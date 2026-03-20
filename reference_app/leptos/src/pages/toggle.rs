use leptos::prelude::*;
use cardo_ui::toggle::*;

#[component]
pub fn TogglePage() -> impl IntoView {
    let (disabled, set_disabled) = signal(false);
    let (pressed, set_pressed) = signal(false);

    view! {
        <Toggle
            class:toggle-root=true
            disabled=disabled
            pressed=pressed
            on_pressed_change=move |value: bool| set_pressed.set(value)
        >
            "toggle"
        </Toggle>

        <br />
        <br />

        <label>
            <input
                type="checkbox"
                prop:checked=move || disabled.get()
                on:change=move |ev| {
                    use web_sys::wasm_bindgen::JsCast;
                    let target: web_sys::HtmlInputElement = ev.target().unwrap().unchecked_into();
                    set_disabled.set(target.checked());
                }
            />
            " disabled"
        </label>

        <br />

        <label>
            <input
                type="checkbox"
                prop:checked=move || pressed.get()
                on:change=move |ev| {
                    use web_sys::wasm_bindgen::JsCast;
                    let target: web_sys::HtmlInputElement = ev.target().unwrap().unchecked_into();
                    set_pressed.set(target.checked());
                }
            />
            " pressed"
        </label>

        <br />
        <span data-testid="pressed-state">
            {move || if pressed.get() { "true" } else { "false" }}
        </span>
    }
}
