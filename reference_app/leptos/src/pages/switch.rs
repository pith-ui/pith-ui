use leptos::prelude::*;
use cardo_ui::switch::*;

#[component]
pub fn SwitchPage() -> impl IntoView {
    let (disabled, set_disabled) = signal(false);
    let (checked, set_checked) = signal(false);
    let (required, set_required) = signal(false);

    view! {
        <label>
            <Switch
                class:switch-root=true
                attr:data-custom="switch-root-custom"
                disabled=disabled
                checked=checked
                on_checked_change=Callback::new(move |value: bool| set_checked.set(value))
                required=required
                name="airplane"
                value="on"
            >
                <SwitchThumb class:switch-thumb=true attr:data-custom="switch-thumb-custom" />
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

        <br />

        <label>
            <input
                type="checkbox"
                prop:checked=move || required.get()
                on:change=move |ev| {
                    use web_sys::wasm_bindgen::JsCast;
                    let target: web_sys::HtmlInputElement = ev.target().unwrap().unchecked_into();
                    set_required.set(target.checked());
                }
            />
            " required"
        </label>
    }
}
