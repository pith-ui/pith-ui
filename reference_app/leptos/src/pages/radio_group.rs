use leptos::prelude::*;
use radix_leptos_primitives::radio_group::*;

#[component]
pub fn RadioGroupPage() -> impl IntoView {
    let (disabled, set_disabled) = signal(false);
    let (value, set_value) = signal(String::new());
    view! {
        <RadioGroup
            attr:class="radio-group-root"
            attr:data-custom="radio-group-root-custom"
            disabled=disabled
            attr:aria-label="Favourite pet"
            value=value
            on_value_change=Callback::new(move |v: String| set_value.set(v))
        >
            <label class="radio-group-label">
                <RadioGroupItem value="cat" attr:class="radio-group-item" attr:data-custom="radio-group-item-custom">
                    <RadioGroupIndicator attr:class="radio-group-indicator" attr:data-custom="radio-group-indicator-custom" />
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
