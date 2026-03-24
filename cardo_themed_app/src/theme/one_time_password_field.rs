use cardo_ui::one_time_password_field::*;
use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui input-inspired OTP
// ---------------------------------------------------------------------------

const ROOT_CLASS: &str = "flex items-center gap-2";

const INPUT_CLASS: &str = "size-10 text-center rounded-md border border-input bg-transparent text-sm shadow-xs outline-none focus-visible:focus-ring disabled:disabled-cursor dark:bg-input/30";

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

#[component]
pub fn ThemedOneTimePasswordField(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let class = StoredValue::new(ROOT_CLASS);
    let children = StoredValue::new(children);

    view! {
        <OneTimePasswordField
            attr:class=class.get_value()
            value=value
            default_value=default_value
            on_value_change=move |val: String| {
                if let Some(cb) = on_value_change {
                    cb.run(val);
                }
            }
            disabled=disabled
        >
            {children.with_value(|children| children())}
            <OneTimePasswordFieldHiddenInput />
        </OneTimePasswordField>
    }
}

#[component]
pub fn ThemedOneTimePasswordFieldInput() -> impl IntoView {
    let class = StoredValue::new(INPUT_CLASS);

    view! {
        <OneTimePasswordFieldInput attr:class=class.get_value() />
    }
}
