use pith_ui::radio_group::{RadioGroup, RadioGroupIndicator, RadioGroupItem};
use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui new-york radio group
// ---------------------------------------------------------------------------

const GROUP_CLASS: &str = "grid gap-3";

const ITEM_CLASS: &str = "aspect-square size-4 shrink-0 rounded-full border border-input text-primary shadow-xs transition-[color,box-shadow] outline-none focus-visible:focus-ring disabled:disabled-cursor data-[state=checked]:border-primary dark:bg-input/30";

const INDICATOR_CLASS: &str = "flex items-center justify-center";

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

#[component]
pub fn ThemedRadioGroup(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <RadioGroup
            attr:class=GROUP_CLASS
            value=value
            default_value=default_value
            on_value_change=move |val: String| {
                if let Some(cb) = on_value_change {
                    cb.run(val);
                }
            }
            disabled=disabled
        >
            {children()}
        </RadioGroup>
    }
}

#[component]
pub fn ThemedRadioGroupItem(
    #[prop(into)] value: String,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
) -> impl IntoView {
    let item_class = StoredValue::new(ITEM_CLASS);
    let indicator_class = StoredValue::new(INDICATOR_CLASS);

    view! {
        <RadioGroupItem attr:class=item_class.get_value() value=value disabled=disabled>
            <RadioGroupIndicator attr:class=indicator_class.get_value()>
                <div class="size-2.5 rounded-full bg-primary" />
            </RadioGroupIndicator>
        </RadioGroupItem>
    }
}
