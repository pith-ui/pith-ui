use leptos::prelude::*;
use cardo_ui::one_time_password_field::*;
use cardo_ui::roving_focus::Orientation;

#[component]
pub fn OneTimePasswordFieldPage() -> impl IntoView {
    let (value, set_value) = signal(String::new());
    let (disabled, set_disabled) = signal(false);
    let (read_only, set_read_only) = signal(false);
    let (vertical, set_vertical) = signal(false);
    let (submitted, set_submitted) = signal(String::new());

    let on_submit = move |event: web_sys::SubmitEvent| {
        event.prevent_default();

        let form = event.target().and_then(|t| {
            use web_sys::wasm_bindgen::JsCast;
            t.dyn_into::<web_sys::HtmlFormElement>().ok()
        });

        if let Some(form) = form
            && let Ok(form_data) = web_sys::FormData::new_with_form(&form)
        {
            let code = form_data.get("code").as_string().unwrap_or_default();
            set_submitted.set(format!("Submitted: {code}"));
        }
    };

    let on_reset = move |_: web_sys::Event| {
        set_value.set(String::new());
        set_submitted.set(String::new());
    };

    let (uncontrolled_submitted, set_uncontrolled_submitted) = signal(String::new());
    let uncontrolled_on_submit = move |event: web_sys::SubmitEvent| {
        event.prevent_default();

        let form = event.target().and_then(|t| {
            use web_sys::wasm_bindgen::JsCast;
            t.dyn_into::<web_sys::HtmlFormElement>().ok()
        });

        if let Some(form) = form
            && let Ok(form_data) = web_sys::FormData::new_with_form(&form)
        {
            let code = form_data
                .get("uncontrolled-code")
                .as_string()
                .unwrap_or_default();
            set_uncontrolled_submitted.set(format!("Submitted: {code}"));
        }
    };

    let (auto_submitted, set_auto_submitted) = signal(String::new());

    view! {
        <form on:submit=on_submit on:reset=on_reset>
            <OneTimePasswordField
                class:otp-root=true
                attr:data-testid="main-otp-root"
                value=Signal::derive(move || Some(value.get()))
                on_value_change=Callback::new(move |v: String| {
                    set_value.set(v);
                })
                disabled=Signal::derive(move || disabled.get())
                read_only=Signal::derive(move || read_only.get())
                orientation=Signal::derive(move || if vertical.get() { Orientation::Vertical } else { Orientation::Horizontal })
                name="code"
            >
                <OneTimePasswordFieldInput />
                <OneTimePasswordFieldInput />
                <OneTimePasswordFieldInput />
                <OneTimePasswordFieldInput />
                <OneTimePasswordFieldInput />
                <OneTimePasswordFieldInput />
                <OneTimePasswordFieldHiddenInput />
            </OneTimePasswordField>

            <div class="controls">
                <button type="submit">"submit"</button>
                <button type="reset">"reset"</button>
            </div>
        </form>

        <output data-testid="otp-value">{move || value.get()}</output>
        <pre data-testid="form-result">{move || submitted.get()}</pre>

        <div class="controls">
            <label>
                <input
                    type="checkbox"
                    prop:checked=move || disabled.get()
                    on:change=move |ev| set_disabled.set(event_target_checked(&ev))
                />
                " disabled"
            </label>
            <label>
                <input
                    type="checkbox"
                    prop:checked=move || read_only.get()
                    on:change=move |ev| set_read_only.set(event_target_checked(&ev))
                />
                " read-only"
            </label>
            <label>
                <input
                    type="checkbox"
                    prop:checked=move || vertical.get()
                    on:change=move |ev| set_vertical.set(event_target_checked(&ev))
                />
                " vertical"
            </label>
        </div>

        <button data-testid="outside">"outside"</button>

        <div aria-hidden="true">
            <hr />
            <h3>"Uncontrolled"</h3>
            <form on:submit=uncontrolled_on_submit>
                <OneTimePasswordField
                    class:otp-root=true
                    attr:data-testid="uncontrolled-root"
                    default_value="12"
                    name="uncontrolled-code"
                >
                    <OneTimePasswordFieldInput />
                    <OneTimePasswordFieldInput />
                    <OneTimePasswordFieldInput />
                    <OneTimePasswordFieldInput />
                    <OneTimePasswordFieldHiddenInput />
                </OneTimePasswordField>
                <button type="submit" data-testid="uncontrolled-submit">"submit"</button>
                <pre data-testid="uncontrolled-result">{move || uncontrolled_submitted.get()}</pre>
            </form>

            <hr />
            <h3>"Password Type"</h3>
            <OneTimePasswordField
                class:otp-root=true
                attr:data-testid="password-root"
                r#type=InputType::Password
            >
                <OneTimePasswordFieldInput />
                <OneTimePasswordFieldInput />
                <OneTimePasswordFieldInput />
                <OneTimePasswordFieldInput />
                <OneTimePasswordFieldHiddenInput />
            </OneTimePasswordField>

            <hr />
            <h3>"Placeholder"</h3>
            <OneTimePasswordField
                class:otp-root=true
                attr:data-testid="placeholder-root"
                placeholder="○○○○"
            >
                <OneTimePasswordFieldInput />
                <OneTimePasswordFieldInput />
                <OneTimePasswordFieldInput />
                <OneTimePasswordFieldInput />
                <OneTimePasswordFieldHiddenInput />
            </OneTimePasswordField>

            <hr />
            <h3>"AutoSubmit"</h3>
            <OneTimePasswordField
                class:otp-root=true
                attr:data-testid="autosubmit-root"
                auto_submit=true
                on_auto_submit=Callback::new(move |v: String| set_auto_submitted.set(format!("AutoSubmitted: {v}")))
            >
                <OneTimePasswordFieldInput />
                <OneTimePasswordFieldInput />
                <OneTimePasswordFieldInput />
                <OneTimePasswordFieldInput />
                <OneTimePasswordFieldHiddenInput />
            </OneTimePasswordField>
            <pre data-testid="autosubmit-result">{move || auto_submitted.get()}</pre>

            <hr />
            <h3>"AutoComplete"</h3>
            <OneTimePasswordField
                class:otp-root=true
                attr:data-testid="autocomplete-root"
                auto_complete=AutoComplete::OneTimeCode
            >
                <OneTimePasswordFieldInput />
                <OneTimePasswordFieldInput />
                <OneTimePasswordFieldInput />
                <OneTimePasswordFieldInput />
                <OneTimePasswordFieldHiddenInput />
            </OneTimePasswordField>
        </div>
    }
}
