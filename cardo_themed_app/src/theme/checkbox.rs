use cardo_ui::checkbox::*;
use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui new-york checkbox
//
// No TwVariant needed — shadcn checkbox is a single fixed style.
// ---------------------------------------------------------------------------

const ROOT_CLASS: &str = "peer size-4 shrink-0 rounded-[4px] border border-input shadow-xs transition-shadow outline-none focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:border-primary data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground dark:bg-input/30 dark:data-[state=checked]:bg-primary";

const INDICATOR_CLASS: &str = "grid place-content-center text-current";

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

#[component]
pub fn ThemedCheckbox(
    #[prop(into, optional)] checked: MaybeProp<CheckedState>,
    #[prop(into, optional)] default_checked: MaybeProp<CheckedState>,
    #[prop(into, optional)] on_checked_change: Option<Callback<CheckedState>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] required: MaybeProp<bool>,
) -> impl IntoView {
    view! {
        <Checkbox
            attr:class=ROOT_CLASS
            checked=checked
            default_checked=default_checked
            on_checked_change=move |state: CheckedState| {
                if let Some(cb) = on_checked_change {
                    cb.run(state);
                }
            }
            disabled=disabled
            name=name
            required=required
        >
            <CheckboxIndicator attr:class=INDICATOR_CLASS>
                <CheckIcon />
            </CheckboxIndicator>
        </Checkbox>
    }
}

/// SVG check icon (matches shadcn's lucide CheckIcon at size-3.5).
#[component]
fn CheckIcon() -> impl IntoView {
    view! {
        <svg
            class="size-3.5"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="3"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="M20 6 9 17l-5-5" />
        </svg>
    }
}
