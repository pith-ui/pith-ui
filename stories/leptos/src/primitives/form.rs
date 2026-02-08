use std::rc::Rc;

use leptos::prelude::*;
use radix_leptos_form::*;

#[component]
pub fn Basic() -> impl IntoView {
    let (server_errors, set_server_errors) = signal::<(bool, bool)>((false, false));

    let on_submit = move |event: web_sys::SubmitEvent| {
        event.prevent_default();

        let form = event.target().and_then(|t| {
            use web_sys::wasm_bindgen::JsCast;
            t.dyn_into::<web_sys::HtmlFormElement>().ok()
        });

        if let Some(form) = form
            && let Ok(form_data) = web_sys::FormData::new_with_form(&form)
        {
            let email = form_data.get("email").as_string().unwrap_or_default();
            let password = form_data.get("password").as_string().unwrap_or_default();

            let email_error = !email.contains("@gmail.com");
            let password_error = !password.contains('#');

            if email_error || password_error {
                set_server_errors.set((email_error, password_error));
                return;
            }

            web_sys::window().and_then(|w| {
                w.alert_with_message(&format!("email: {email}, password: {password}"))
                    .ok()
            });
        }
    };

    view! {
        <Form
            attr:class="form"
            on_clear_server_errors=Callback::new(move |_| set_server_errors.set((false, false)))
            on:submit=on_submit
        >
            <FormField name="email" server_invalid=Signal::derive(move || server_errors.get().0)>
                <FormLabel>Email</FormLabel>
                <FormControl
                    attr:r#type="email"
                    attr:required=""
                    on_change=Callback::new(move |_: web_sys::Event| {
                        set_server_errors.update(|e| e.0 = false);
                    })
                />
                <FormMessage r#match=Match::BuiltIn(ValidityMatcher::ValueMissing) />
                <FormMessage
                    r#match=Match::BuiltIn(ValidityMatcher::TypeMismatch)
                    force_match=server_errors.get_untracked().0
                >
                    Email is invalid
                </FormMessage>
            </FormField>

            <FormField name="password" server_invalid=Signal::derive(move || server_errors.get().1)>
                <FormLabel>Password</FormLabel>
                <FormControl
                    attr:r#type="password"
                    attr:required=""
                    on_change=Callback::new(move |_: web_sys::Event| {
                        set_server_errors.update(|e| e.1 = false);
                    })
                />
                <FormMessage r#match=Match::BuiltIn(ValidityMatcher::ValueMissing)>
                    Password is required
                </FormMessage>
                <FormMessage
                    r#match=Match::Custom(Rc::new(|value: String, _form_data: web_sys::FormData| {
                        !value.chars().any(|c| c.is_ascii_digit())
                    }))
                    force_match=server_errors.get_untracked().1
                >
                    Password is not complex enough
                </FormMessage>
                <Show when=move || server_errors.get().1>
                    <FormMessage>Woops</FormMessage>
                </Show>
            </FormField>

            <FormSubmit>Submit</FormSubmit>
            <button type="reset">Reset</button>
        </Form>
    }
}

#[component]
pub fn Cypress() -> impl IntoView {
    let (data, set_data) = signal(String::new());
    let (simulate_server_errors, set_simulate_server_errors) = signal(false);
    let (server_errors, set_server_errors) = signal::<(bool, bool, bool)>((false, false, false));

    let on_submit = move |event: web_sys::SubmitEvent| {
        event.prevent_default();

        set_data.set(String::new());

        let form = event.target().and_then(|t| {
            use web_sys::wasm_bindgen::JsCast;
            t.dyn_into::<web_sys::HtmlFormElement>().ok()
        });

        if let Some(form) = form
            && let Ok(form_data) = web_sys::FormData::new_with_form(&form)
        {
            let email = form_data.get("email").as_string().unwrap_or_default();
            let pin = form_data.get("pin").as_string().unwrap_or_default();
            let name = form_data.get("name").as_string().unwrap_or_default();
            let age = form_data.get("age").as_string().unwrap_or_default();
            let password = form_data.get("password").as_string().unwrap_or_default();
            let secret = form_data.get("secret").as_string().unwrap_or_default();
            let async_secret = form_data.get("asyncSecret").as_string().unwrap_or_default();
            let country = form_data.get("country").as_string().unwrap_or_default();

            let simulate = simulate_server_errors.get_untracked();

            if simulate {
                set_server_errors.set((email.is_empty(), pin.chars().nth(3) != Some('9'), true));
            }

            set_data.set(format!(
                        "name: {name}, age: {age}, email: {email}, password: {password}, pin: {pin}, secret: {secret}, asyncSecret: {async_secret}, country: {country}"
                    ));
        }
    };

    view! {
        <Form
            attr:class="form"
            on_clear_server_errors=Callback::new(move |_| set_server_errors.set((false, false, false)))
            on:submit=on_submit
        >
            <FormField name="name">
                <FormLabel>Name (required)</FormLabel>
                <FormControl attr:r#type="text" attr:required="" />
                <FormMessage r#match=Match::BuiltIn(ValidityMatcher::ValueMissing) />
                <FormMessage r#match=Match::BuiltIn(ValidityMatcher::Valid)>valid!</FormMessage>
            </FormField>

            <FormField name="age">
                <FormLabel>Age (0-99)</FormLabel>
                <FormControl attr:r#type="number" attr:min="0" attr:max="99" attr:step="1" />
                <FormMessage r#match=Match::BuiltIn(ValidityMatcher::RangeOverflow) />
                <FormMessage r#match=Match::BuiltIn(ValidityMatcher::RangeUnderflow) />
                <FormMessage r#match=Match::BuiltIn(ValidityMatcher::StepMismatch) />
            </FormField>

            <FormField name="email" server_invalid=Signal::derive(move || server_errors.get().0)>
                <FormLabel>Email</FormLabel>
                <FormControl attr:r#type="email" />
                <FormMessage r#match=Match::BuiltIn(ValidityMatcher::TypeMismatch) />
                <Show when=move || server_errors.get().0>
                    <FormMessage>Email is actually required server side!</FormMessage>
                </Show>
            </FormField>

            <FormField name="password">
                <FormLabel>Password</FormLabel>
                <FormControl attr:r#type="password" attr:minlength="8" attr:maxlength="16" />
                <FormMessage r#match=Match::BuiltIn(ValidityMatcher::TooShort) />
                <FormMessage r#match=Match::BuiltIn(ValidityMatcher::TooLong) />
            </FormField>

            <FormField name="pin" server_invalid=Signal::derive(move || server_errors.get().1)>
                <FormLabel>Pin (4 digits)</FormLabel>
                <FormControl attr:r#type="text" attr:pattern="\\d{4,4}" />
                <FormMessage
                    r#match=Match::BuiltIn(ValidityMatcher::PatternMismatch)
                    force_match=server_errors.get_untracked().1
                />
            </FormField>

            <FormField name="secret">
                <FormLabel>Secret 1</FormLabel>
                <FormControl attr:r#type="text" />
                <FormMessage r#match=Match::Custom(Rc::new(|value: String, _form_data: web_sys::FormData| {
                    value != "shush"
                })) />
            </FormField>

            <FormField name="asyncSecret">
                <FormLabel>Secret 2</FormLabel>
                <FormControl attr:r#type="text" />
                <FormMessage r#match=Match::CustomAsync(Rc::new(|value: String, _form_data: web_sys::FormData| {
                    Box::pin(async move {
                        value != "shush"
                    })
                })) />
            </FormField>

            <FormField name="country">
                <FormLabel attr:r#for="my-country">Country</FormLabel>
                <FormControl id="my-country" attr:r#type="text" attr:pattern="France|Spain" />
                <FormMessage r#match=Match::BuiltIn(ValidityMatcher::PatternMismatch)>
                    {"Country should be \"France\" or \"Spain\""}
                </FormMessage>
            </FormField>

            <FormSubmit>submit</FormSubmit>
            <button type="reset">reset</button>
        </Form>
        <pre>Data: {move || data.get()}</pre>

        <label>
            <input
                type="checkbox"
                prop:checked=move || simulate_server_errors.get()
                on:change=move |event: web_sys::Event| {
                    use web_sys::wasm_bindgen::JsCast;
                    if let Some(target) = event.target()
                        .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                    {
                        set_simulate_server_errors.set(target.checked());
                    }
                }
            />
            " Simulate server errors?"
        </label>
    }
}
