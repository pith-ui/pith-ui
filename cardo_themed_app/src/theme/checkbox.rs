use cardo_ui::checkbox::*;
use leptos::prelude::*;
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// Style definitions
// ---------------------------------------------------------------------------

#[derive(TwVariant)]
pub enum CheckboxSize {
    #[tw(default, class = "size-4")]
    Sm,
    #[tw(class = "size-5")]
    Md,
}

#[derive(TwClass)]
#[tw(
    class = "inline-flex items-center justify-center shrink-0 rounded-1 border border-neutral-7 bg-neutral-1 transition-colors duration-normal data-[state=checked]:bg-accent-9 data-[state=checked]:border-accent-9 data-[state=checked]:text-white data-[state=indeterminate]:bg-accent-9 data-[state=indeterminate]:border-accent-9 data-[state=indeterminate]:text-white data-[disabled]:opacity-50 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-focus-ring focus-visible:ring-offset-2 focus-visible:ring-offset-page"
)]
pub struct CheckboxRootClass {
    pub size: CheckboxSize,
}

#[derive(TwVariant)]
pub enum CheckboxIndicatorSize {
    #[tw(default, class = "size-3.5")]
    Sm,
    #[tw(class = "size-4")]
    Md,
}

impl From<CheckboxSize> for CheckboxIndicatorSize {
    fn from(s: CheckboxSize) -> Self {
        match s {
            CheckboxSize::Sm => CheckboxIndicatorSize::Sm,
            CheckboxSize::Md => CheckboxIndicatorSize::Md,
        }
    }
}

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

#[component]
pub fn ThemedCheckbox(
    #[prop(into, optional)] size: CheckboxSize,
    #[prop(into, optional)] checked: MaybeProp<CheckedState>,
    #[prop(into, optional)] default_checked: MaybeProp<CheckedState>,
    #[prop(into, optional)] on_checked_change: Option<Callback<CheckedState>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] required: MaybeProp<bool>,
) -> impl IntoView {
    let root_class = StoredValue::new(CheckboxRootClass { size }.to_class());
    let indicator_size: CheckboxIndicatorSize = size.into();
    let indicator_class = StoredValue::new(indicator_size.as_class().to_string());

    view! {
        <Checkbox
            attr:class=root_class.get_value()
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
            <CheckboxIndicator attr:class=indicator_class.get_value()>
                <CheckIcon />
            </CheckboxIndicator>
        </Checkbox>
    }
}

/// Simple SVG check icon (Radix Icons check).
#[component]
fn CheckIcon() -> impl IntoView {
    view! {
        <svg
            width="15"
            height="15"
            viewBox="0 0 15 15"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
            class="block"
        >
            <path
                d="M11.4669 3.72684C11.7558 3.91574 11.8369 4.30308 11.648 4.59198L7.39799 11.092C7.29783 11.2452 7.13556 11.3467 6.95402 11.3699C6.77247 11.3931 6.58989 11.3354 6.45446 11.2124L3.70446 8.71241C3.44905 8.48022 3.43023 8.08494 3.66242 7.82953C3.89461 7.57412 4.28989 7.55529 4.5453 7.78749L6.75292 9.79441L10.6018 3.90792C10.7907 3.61902 11.178 3.53795 11.4669 3.72684Z"
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
            />
        </svg>
    }
}
