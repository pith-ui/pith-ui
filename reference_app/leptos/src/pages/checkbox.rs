use leptos::prelude::*;
use radix_leptos_primitives::checkbox::*;

#[component]
pub fn CheckboxPage() -> impl IntoView {
    let (disabled, set_disabled) = signal(false);
    let (indeterminate, set_indeterminate) = signal(false);
    let (checked, set_checked) = signal(false);

    let checked_value = Memo::new(move |_| {
        if indeterminate.get() {
            CheckedState::Indeterminate
        } else if checked.get() {
            CheckedState::True
        } else {
            CheckedState::False
        }
    });

    view! {
        <label class="checkbox-label">
            <Checkbox
                attr:class="checkbox-root"
                disabled=disabled
                checked=Signal::derive(move || checked_value.get())
                on_checked_change=move |value: CheckedState| {
                    match value {
                        CheckedState::Indeterminate => {
                            set_indeterminate.set(true);
                            set_checked.set(false);
                        }
                        CheckedState::True => {
                            set_indeterminate.set(false);
                            set_checked.set(true);
                        }
                        CheckedState::False => {
                            set_indeterminate.set(false);
                            set_checked.set(false);
                        }
                    }
                }
            >
                <CheckboxIndicator attr:class="checkbox-indicator">
                    {move || {
                        if checked_value.get() == CheckedState::Indeterminate {
                            "\u{2212}"
                        } else {
                            "\u{2713}"
                        }
                    }}
                </CheckboxIndicator>
            </Checkbox>
            "accept terms"
        </label>

        <br /><br />

        // Always-checked checkbox for internal styles testing
        <Checkbox
            attr:class="checkbox-root"
            checked=Signal::derive(|| CheckedState::True)
            attr:data-testid="styled-checkbox"
        >
            <CheckboxIndicator
                attr:class="checkbox-indicator"
                attr:data-testid="styled-indicator"
                attr:style="background: tomato"
            >
                "\u{2713}"
            </CheckboxIndicator>
        </Checkbox>

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

        <label>
            <input
                type="checkbox"
                prop:checked=move || indeterminate.get()
                on:change=move |ev| {
                    use web_sys::wasm_bindgen::JsCast;
                    let target: web_sys::HtmlInputElement = ev.target().unwrap().unchecked_into();
                    let is_indeterminate = target.checked();
                    set_indeterminate.set(is_indeterminate);
                    if is_indeterminate {
                        set_checked.set(false);
                    }
                }
            />
            " indeterminate"
        </label>
    }
}
