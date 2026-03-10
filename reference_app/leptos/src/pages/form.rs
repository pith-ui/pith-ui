use leptos::prelude::*;
use radix_leptos_primitives::form::*;

#[component]
pub fn FormPage() -> impl IntoView {
    let (data, set_data) = signal("{}".to_string());
    let (server_errors_name, set_server_errors_name) = signal(false);

    let on_submit = move |event: web_sys::SubmitEvent| {
        event.prevent_default();

        let form = event.target().and_then(|t| {
            use web_sys::wasm_bindgen::JsCast;
            t.dyn_into::<web_sys::HtmlFormElement>().ok()
        });

        if let Some(form) = form
            && let Ok(form_data) = web_sys::FormData::new_with_form(&form)
        {
            let name = form_data.get("name").as_string().unwrap_or_default();
            let email = form_data.get("email").as_string().unwrap_or_default();

            // Simulate async server validation: name must not be "taken"
            // Uses set_timeout because on_clear_server_errors fires synchronously on submit,
            // so a synchronous set would be immediately cleared.
            if name == "taken" {
                set_timeout(
                    move || set_server_errors_name.set(true),
                    std::time::Duration::ZERO,
                );
                return;
            }

            set_data.set(format!("{{\"name\":\"{name}\",\"email\":\"{email}\"}}"));
        }
    };

    let on_reset = move |_: web_sys::Event| {
        set_data.set("{}".to_string());
    };

    view! {
        <Form
            attr:class="form-root"
            on_clear_server_errors=Callback::new(move |_| set_server_errors_name.set(false))
            on:submit=on_submit
            on:reset=on_reset
        >
            <FormField
                name="name"
                attr:class="form-field"
                server_invalid=Signal::derive(move || server_errors_name.get())
            >
                <FormLabel attr:class="form-label">"Name"</FormLabel>
                <FormControl attr:class="form-control" attr:r#type="text" attr:required="" />
                <FormMessage attr:class="form-message" r#match=Match::BuiltIn(ValidityMatcher::ValueMissing)>
                    "Name is required"
                </FormMessage>
                <Show when=move || server_errors_name.get()>
                    <FormMessage attr:class="form-message">
                        "Name is already taken"
                    </FormMessage>
                </Show>
            </FormField>

            <FormField name="email" attr:class="form-field">
                <FormLabel attr:class="form-label">"Email"</FormLabel>
                <FormControl attr:class="form-control" attr:r#type="email" attr:required="" />
                <FormMessage attr:class="form-message" r#match=Match::BuiltIn(ValidityMatcher::ValueMissing)>
                    "Email is required"
                </FormMessage>
                <FormMessage attr:class="form-message" r#match=Match::BuiltIn(ValidityMatcher::TypeMismatch)>
                    "Please enter a valid email"
                </FormMessage>
            </FormField>

            <FormSubmit attr:class="form-submit">"Submit"</FormSubmit>
            <button type="reset">"reset"</button>
        </Form>

        <pre data-testid="form-result">"Data: " {move || data.get()}</pre>

        <button data-testid="outside-button">"outside"</button>

        <hr />
        <h3>"ValidityState"</h3>
        <Form attr:class="form-root" attr:data-testid="validity-form" on:submit=|e: web_sys::SubmitEvent| e.prevent_default()>
            <FormField name="vs-name" attr:class="form-field">
                <FormLabel attr:class="form-label">"VS Name"</FormLabel>
                <FormControl attr:class="form-control" attr:r#type="text" attr:required="" attr:data-testid="vs-name-input" />
                <FormValidityState children=Callback::new(move |validity: Option<Validity>| {
                    let text = match validity {
                        Some(v) => format!(
                            "{{\"valueMissing\":{},\"valid\":{}}}",
                            v.value_missing, v.valid
                        ),
                        None => "undefined".to_string(),
                    };
                    view! { <span data-testid="vs-name-validity">{text}</span> }.into_any()
                }) />
            </FormField>

            <FormField name="vs-email" attr:class="form-field">
                <FormLabel attr:class="form-label">"VS Email"</FormLabel>
                <FormControl attr:class="form-control" attr:r#type="email" attr:data-testid="vs-email-input" />
                <FormValidityState children=Callback::new(move |validity: Option<Validity>| {
                    let text = match validity {
                        Some(v) => format!(
                            "{{\"typeMismatch\":{},\"valid\":{}}}",
                            v.type_mismatch, v.valid
                        ),
                        None => "undefined".to_string(),
                    };
                    view! { <span data-testid="vs-email-validity">{text}</span> }.into_any()
                }) />
            </FormField>

            <FormSubmit attr:class="form-submit" attr:data-testid="vs-submit">"Check Validity"</FormSubmit>
        </Form>
    }
}
