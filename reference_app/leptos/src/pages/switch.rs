use leptos::prelude::*;
use radix_leptos_switch::*;

#[component]
pub fn SwitchPage() -> impl IntoView {
    let (disabled, set_disabled) = signal(false);
    let (checked, set_checked) = signal(false);

    view! {
        <label>
            <Switch
                attr:class="switch-root"
                disabled=disabled
                checked=checked
                on_checked_change=Callback::new(move |value: bool| set_checked.set(value))
            >
                <SwitchThumb attr:class="switch-thumb" />
            </Switch>
            " airplane mode"
        </label>

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
                prop:checked=move || checked.get()
                on:change=move |ev| {
                    use web_sys::wasm_bindgen::JsCast;
                    let target: web_sys::HtmlInputElement = ev.target().unwrap().unchecked_into();
                    set_checked.set(target.checked());
                }
            />
            " checked"
        </label>
    }
}
