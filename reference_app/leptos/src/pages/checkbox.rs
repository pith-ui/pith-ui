use leptos::prelude::*;
use pith_ui::checkbox::*;

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
                class:checkbox-root=true
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
                <CheckboxIndicator class:checkbox-indicator=true>
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
            class:checkbox-root=true
            checked=Signal::derive(|| CheckedState::True)
            attr:data-testid="styled-checkbox"
        >
            <CheckboxIndicator
                class:checkbox-indicator=true
                attr:data-testid="styled-indicator"
                style:background="tomato"
                style:pointer-events="auto"
            >
                "\u{2713}"
            </CheckboxIndicator>
        </Checkbox>

        <br /><br />

        // Checkbox with style forwarded to root button and indicator
        <Checkbox
            checked=Signal::derive(|| CheckedState::True)
            attr:data-testid="style-forwarded-checkbox"
            style:background-color="rgb(0, 128, 0)"
            style:border="3px solid rgb(255, 0, 0)"
        >
            <CheckboxIndicator
                attr:id="style-forwarded-indicator"
                style:color="rgb(0, 0, 255)"
                style:font-size="24px"
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
