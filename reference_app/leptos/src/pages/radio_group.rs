use leptos::prelude::*;
use radix_leptos_primitives::radio_group::*;

#[component]
pub fn RadioGroupPage() -> impl IntoView {
    let (disabled, set_disabled) = signal(false);
    view! {
        <RadioGroup
            attr:class="radio-group-root"
            disabled=disabled
            attr:aria-label="Favourite pet"
        >
            <label class="radio-group-label">
                <RadioGroupItem value="cat" attr:class="radio-group-item">
                    <RadioGroupIndicator attr:class="radio-group-indicator" />
                </RadioGroupItem>
                "Cat"
            </label>
            <label class="radio-group-label">
                <RadioGroupItem value="dog" disabled=true attr:class="radio-group-item">
                    <RadioGroupIndicator attr:class="radio-group-indicator" />
                </RadioGroupItem>
                "Dog"
            </label>
            <label class="radio-group-label">
                <RadioGroupItem value="rabbit" attr:class="radio-group-item">
                    <RadioGroupIndicator attr:class="radio-group-indicator" />
                </RadioGroupItem>
                "Rabbit"
            </label>
        </RadioGroup>

        <br /><br />

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
    }
}
