use std::sync::Arc;

use cardo_ui::password_toggle_field::*;
use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui input-inspired password toggle
// ---------------------------------------------------------------------------

const INPUT_CLASS: &str = "h-9 w-full min-w-0 rounded-md border border-input bg-transparent px-3 py-1 pr-10 text-base shadow-xs transition-[color,box-shadow] outline-none placeholder:text-muted-foreground disabled:pointer-events-none disabled:disabled-cursor md:text-sm dark:bg-input/30 focus-visible:focus-ring";

const TOGGLE_CLASS: &str = "absolute right-0 top-0 h-full px-3 text-muted-foreground hover:text-foreground focus-visible:focus-ring rounded-r-md";

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

#[component]
pub fn ThemedPasswordToggleField(
    #[prop(into, optional)] visible: MaybeProp<bool>,
    #[prop(into, optional)] default_visible: MaybeProp<bool>,
    #[prop(into, optional)] on_visibility_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <PasswordToggleField
            visible=visible
            default_visible=default_visible
            on_visibility_change=move |val: bool| {
                if let Some(cb) = on_visibility_change {
                    cb.run(val);
                }
            }
        >
            {children()}
        </PasswordToggleField>
    }
}

#[component]
pub fn ThemedPasswordToggleFieldInput(
    #[prop(into, optional)] placeholder: Option<String>,
) -> impl IntoView {
    let class = StoredValue::new(INPUT_CLASS);
    let placeholder = StoredValue::new(placeholder);

    view! {
        <PasswordToggleFieldInput
            attr:class=class.get_value()
            attr:placeholder=placeholder.get_value()
        />
    }
}

#[component]
pub fn ThemedPasswordToggleFieldToggle() -> impl IntoView {
    let class = StoredValue::new(TOGGLE_CLASS);

    let visible_content: ChildrenFn = Arc::new(|| view! { <EyeOffIcon /> }.into_any());
    let hidden_content: ChildrenFn = Arc::new(|| view! { <EyeIcon /> }.into_any());
    let visible_content = StoredValue::new(visible_content);
    let hidden_content = StoredValue::new(hidden_content);

    view! {
        <PasswordToggleFieldToggle attr:class=class.get_value()>
            <PasswordToggleFieldSlot
                visible_content=visible_content.get_value()
                hidden_content=hidden_content.get_value()
            />
        </PasswordToggleFieldToggle>
    }
}

// ---------------------------------------------------------------------------
// Shared icons
// ---------------------------------------------------------------------------

#[component]
fn EyeIcon() -> impl IntoView {
    view! {
        <svg
            class="size-4"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="M2.062 12.348a1 1 0 0 1 0-.696 10.75 10.75 0 0 1 19.876 0 1 1 0 0 1 0 .696 10.75 10.75 0 0 1-19.876 0" />
            <circle cx="12" cy="12" r="3" />
        </svg>
    }
}

#[component]
fn EyeOffIcon() -> impl IntoView {
    view! {
        <svg
            class="size-4"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="M10.733 5.076a10.744 10.744 0 0 1 11.205 6.575 1 1 0 0 1 0 .696 10.747 10.747 0 0 1-1.444 2.49" />
            <path d="M14.084 14.158a3 3 0 0 1-4.242-4.242" />
            <path d="M17.479 17.499a10.75 10.75 0 0 1-15.417-5.151 1 1 0 0 1 0-.696 10.75 10.75 0 0 1 4.446-5.143" />
            <path d="m2 2 20 20" />
        </svg>
    }
}
