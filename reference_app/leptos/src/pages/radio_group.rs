use leptos::prelude::*;
use cardo_ui::radio_group::*;

#[component]
pub fn RadioGroupPage() -> impl IntoView {
    let (disabled, set_disabled) = signal(false);
    let (value, set_value) = signal(String::new());
    view! {
        <RadioGroup
            class:radio-group-root=true
            attr:data-custom="radio-group-root-custom"
            disabled=disabled
            attr:aria-label="Favourite pet"
            value=value
            on_value_change=Callback::new(move |v: String| set_value.set(v))
        >
            <label class="radio-group-label">
                <RadioGroupItem value="cat" class:radio-group-item=true attr:data-custom="radio-group-item-custom">
                    <RadioGroupIndicator class:radio-group-indicator=true attr:data-custom="radio-group-indicator-custom" />
                </RadioGroupItem>
                "Cat"
            </label>
            <label class="radio-group-label">
                <RadioGroupItem value="dog" disabled=true class:radio-group-item=true>
                    <RadioGroupIndicator class:radio-group-indicator=true />
                </RadioGroupItem>
                "Dog"
            </label>
            <label class="radio-group-label">
                <RadioGroupItem value="rabbit" class:radio-group-item=true>
                    <RadioGroupIndicator class:radio-group-indicator=true />
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

        <br />

        <span data-testid="radio-value">{move || value.get()}</span>
        <button data-testid="set-rabbit" on:click=move |_| set_value.set("rabbit".to_string())>
            "set rabbit"
        </button>
        <button data-testid="clear-value" on:click=move |_| set_value.set(String::new())>
            "clear"
        </button>
    }
}
