// Stylance generates constants for all CSS classes in the module; not all are used in every story.
#![allow(dead_code)]

use leptos::prelude::*;
use radix_leptos_dialog::*;
use radix_leptos_one_time_password_field::*;
use radix_leptos_separator::*;
use web_sys::wasm_bindgen::JsCast;

stylance::import_crate_style!(
    classes,
    "src/primitives/one_time_password_field.stories.module.css"
);
stylance::import_crate_style!(dialog_classes, "src/primitives/dialog.stories.module.css");

const VALID_CODE: &str = "123456";

#[derive(Clone, Copy, Debug, PartialEq)]
enum FormState {
    Idle,
    Valid,
    Invalid,
}

#[component]
fn ErrorMessage(children: Children) -> impl IntoView {
    view! {
        <div class=classes::errorMessage>{children()}</div>
    }
}

#[component]
fn SuccessDialog(open: ReadSignal<bool>, on_open_change: Callback<bool>) -> impl IntoView {
    view! {
        <Dialog
            open=open
            on_open_change=on_open_change
        >
            <DialogPortal>
                <DialogOverlay attr:class=dialog_classes::overlay />
                <DialogContent attr:class=dialog_classes::contentDefault>
                    <DialogTitle>"Password match"</DialogTitle>
                    <DialogDescription>"Success!"</DialogDescription>
                    <DialogClose attr:class=dialog_classes::close>"close"</DialogClose>
                </DialogContent>
            </DialogPortal>
        </Dialog>
    }
}

#[component]
pub fn Uncontrolled() -> impl IntoView {
    let (show_success, set_show_success) = signal(false);
    let (form_state, set_form_state) = signal(FormState::Idle);
    let (error_msg, set_error_msg) = signal(String::new());

    let data_state = Signal::derive(move || match form_state.get() {
        FormState::Idle => None,
        FormState::Valid => Some("valid"),
        FormState::Invalid => Some("invalid"),
    });

    view! {
        <div class=classes::viewport>
            <form
                class=classes::form
                on:submit=move |event: leptos::ev::SubmitEvent| {
                    event.prevent_default();
                    let form: web_sys::HtmlFormElement = event.target()
                        .expect("Event should have target")
                        .unchecked_into();
                    let form_data = web_sys::FormData::new_with_form(&form)
                        .expect("FormData should be created");
                    let code = form_data.get("code").as_string().unwrap_or_default();

                    if code.len() == VALID_CODE.len() && code != VALID_CODE {
                        set_form_state.set(FormState::Invalid);
                        set_error_msg.set("Invalid code".to_string());
                    } else if code.len() != VALID_CODE.len() {
                        set_form_state.set(FormState::Invalid);
                        set_error_msg.set("Please fill in all fields".to_string());
                    } else if js_sys::Math::random() > 0.675 {
                        set_form_state.set(FormState::Invalid);
                        set_error_msg.set("Server error".to_string());
                    } else {
                        set_form_state.set(FormState::Valid);
                        set_show_success.set(true);
                    }
                }
            >
                <div class=classes::field>
                    <OneTimePasswordField
                        attr:data-state=move || data_state.get()
                        attr:class=classes::otpRoot
                    >
                        <OneTimePasswordFieldInput />
                        <Separator orientation=Orientation::Vertical attr:class=classes::separator />
                        <OneTimePasswordFieldInput />
                        <Separator orientation=Orientation::Vertical attr:class=classes::separator />
                        <OneTimePasswordFieldInput />
                        <Separator orientation=Orientation::Vertical attr:class=classes::separator />
                        <OneTimePasswordFieldInput />
                        <Separator orientation=Orientation::Vertical attr:class=classes::separator />
                        <OneTimePasswordFieldInput />
                        <Separator orientation=Orientation::Vertical attr:class=classes::separator />
                        <OneTimePasswordFieldInput />

                        <OneTimePasswordFieldHiddenInput name="code" />
                    </OneTimePasswordField>
                    {move || (form_state.get() == FormState::Invalid).then(|| view! {
                        <ErrorMessage>{error_msg.get()}</ErrorMessage>
                    })}
                </div>
                <button type="reset">"Reset form"</button>
                <button>"Submit"</button>
            </form>
            <SuccessDialog
                open=show_success
                on_open_change=Callback::new(move |v: bool| set_show_success.set(v))
            />
        </div>
    }
}

#[component]
pub fn Controlled() -> impl IntoView {
    let (code, set_code) = signal(String::new());
    let (error, set_error) = signal::<Option<String>>(None);
    let (show_success, set_show_success) = signal(false);

    let is_invalid = Signal::derive(move || {
        let c = code.get();
        if c.len() == VALID_CODE.len() {
            c != VALID_CODE
        } else {
            false
        }
    });
    let is_valid = Signal::derive(move || {
        let c = code.get();
        if c.len() == VALID_CODE.len() {
            c == VALID_CODE
        } else {
            false
        }
    });

    let data_state = Signal::derive(move || {
        if error.get().is_some() || is_invalid.get() {
            Some("invalid")
        } else if is_valid.get() {
            Some("valid")
        } else {
            None
        }
    });

    view! {
        <div class=classes::viewport>
            <form
                class=classes::form
                on:submit=move |event: leptos::ev::SubmitEvent| {
                    event.prevent_default();
                    let c = code.get_untracked();

                    if is_invalid.get_untracked() {
                        set_error.set(Some("Invalid code".to_string()));
                    } else if c.len() != VALID_CODE.len() {
                        set_error.set(Some("Please fill in all fields".to_string()));
                    } else if js_sys::Math::random() > 0.675 {
                        set_error.set(Some("Server error".to_string()));
                    } else {
                        set_show_success.set(true);
                    }
                }
            >
                <div class=classes::field>
                    <OneTimePasswordField
                        attr:data-state=move || data_state.get()
                        attr:class=classes::otpRoot
                        on_value_change=Callback::new(move |v: String| set_code.set(v))
                        value=code
                    >
                        <OneTimePasswordFieldInput />
                        <Separator orientation=Orientation::Vertical attr:class=classes::separator />
                        <OneTimePasswordFieldInput />
                        <Separator orientation=Orientation::Vertical attr:class=classes::separator />
                        <OneTimePasswordFieldInput />
                        <Separator orientation=Orientation::Vertical attr:class=classes::separator />
                        <OneTimePasswordFieldInput />
                        <Separator orientation=Orientation::Vertical attr:class=classes::separator />
                        <OneTimePasswordFieldInput />
                        <Separator orientation=Orientation::Vertical attr:class=classes::separator />
                        <OneTimePasswordFieldInput />

                        <OneTimePasswordFieldHiddenInput name="code" />
                    </OneTimePasswordField>
                    {move || error.get().map(|err| view! {
                        <ErrorMessage>{err}</ErrorMessage>
                    })}
                </div>
                <button type="button" on:click=move |_| set_code.set(String::new())>"Reset state"</button>
                <button type="reset">"Reset form"</button>
                <button>"Submit"</button>
                <output
                    data-state=move || data_state.get()
                    class=classes::output
                >
                    {move || {
                        let c = code.get();
                        if c.is_empty() { "code".to_string() } else { c }
                    }}
                </output>
            </form>
            <SuccessDialog
                open=show_success
                on_open_change=Callback::new(move |v: bool| set_show_success.set(v))
            />
        </div>
    }
}
