use leptos::prelude::*;
use radix_leptos_icons::{EyeClosedIcon, EyeOpenIcon};
use radix_leptos_primitives::password_toggle_field::*;

fn eye_open_icon() -> impl IntoView {
    view! { <EyeOpenIcon attr:data-testid="eye-open" /> }
}

fn eye_closed_icon() -> impl IntoView {
    view! { <EyeClosedIcon attr:data-testid="eye-closed" /> }
}

#[component]
pub fn PasswordToggleFieldPage() -> impl IntoView {
    let (visible, set_visible) = signal(false);
    let (submitted, set_submitted) = signal(String::new());

    let on_submit = move |event: web_sys::SubmitEvent| {
        event.prevent_default();
        set_submitted.set("submitted".to_string());
    };

    let on_reset = move |_: web_sys::Event| {
        set_submitted.set(String::new());
    };

    view! {
        // Icon variant (controlled, outside form)
        <label for="password">"Password"</label>
        <div class="ptf-field">
            <PasswordToggleField
                visible=visible
                on_visibility_change=Callback::new(move |v: bool| set_visible.set(v))
            >
                <PasswordToggleFieldInput id="password" class:ptf-input=true />
                <PasswordToggleFieldToggle class:ptf-toggle=true>
                    <PasswordToggleFieldIcon
                        visible_icon=ViewFn::from(eye_open_icon)
                        hidden_icon=ViewFn::from(eye_closed_icon)
                    />
                </PasswordToggleFieldToggle>
            </PasswordToggleField>
        </div>

        // Slot variant (uncontrolled, outside form)
        <label for="pin">"PIN"</label>
        <div class="ptf-field">
            <PasswordToggleField>
                <PasswordToggleFieldInput id="pin" class:ptf-input=true />
                <PasswordToggleFieldToggle class:ptf-toggle=true>
                    <PasswordToggleFieldSlot
                        render=Callback::new(|visible: bool| {
                            if visible { "Hide".into_any() } else { "Show".into_any() }
                        })
                    />
                </PasswordToggleFieldToggle>
            </PasswordToggleField>
        </div>

        // Form variant (uncontrolled, inside form)
        <form on:submit=on_submit on:reset=on_reset>
            <label for="form-password">"Form Password"</label>
            <div class="ptf-field">
                <PasswordToggleField>
                    <PasswordToggleFieldInput id="form-password" class:ptf-input=true />
                    <PasswordToggleFieldToggle class:ptf-toggle=true>
                        <PasswordToggleFieldSlot
                            render=Callback::new(|visible: bool| {
                                if visible { "Hide".into_any() } else { "Show".into_any() }
                            })
                        />
                    </PasswordToggleFieldToggle>
                </PasswordToggleField>
            </div>
            <button type="submit">"submit"</button>
            " "
            <button type="reset">"reset form"</button>
        </form>

        <pre data-testid="form-result">{move || submitted.get()}</pre>

        // Controls
        <br />
        <label>
            <input
                type="checkbox"
                prop:checked=move || visible.get()
                on:change=move |ev| set_visible.set(event_target_checked(&ev))
            />
            " visible"
        </label>

        <br />
        <br />
        <button data-testid="outside">"outside"</button>
    }
}
