use cardo_ui::select::*;
use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui new-york select
// ---------------------------------------------------------------------------

const TRIGGER_CLASS: &str = "flex h-9 w-full items-center justify-between gap-2 rounded-md border border-input bg-transparent px-3 py-2 text-sm shadow-xs outline-none focus-visible:focus-ring disabled:disabled-cursor data-[placeholder]:text-muted-foreground dark:bg-input/30";

const CONTENT_CLASS: &str =
    "relative z-50 min-w-[8rem] overflow-hidden rounded-md border bg-popover text-popover-foreground shadow-md";

const VIEWPORT_CLASS: &str = "p-1";

const ITEM_CLASS: &str = "relative flex w-full cursor-default items-center gap-2 rounded-sm py-1.5 pr-8 pl-2 text-sm outline-hidden select-none menu-item-focus data-[disabled]:disabled-base";

const ITEM_INDICATOR_CLASS: &str =
    "absolute right-2 flex size-3.5 items-center justify-center";

const LABEL_CLASS: &str = "px-2 py-1.5 text-xs text-muted-foreground";

const SEPARATOR_CLASS: &str = "pointer-events-none -mx-1 my-1 h-px bg-border";

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

#[component]
pub fn ThemedSelect(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <Select
            value=value
            default_value=default_value
            on_value_change=move |val: String| {
                if let Some(cb) = on_value_change {
                    cb.run(val);
                }
            }
            open=open
            default_open=default_open
            on_open_change=move |val: bool| {
                if let Some(cb) = on_open_change {
                    cb.run(val);
                }
            }
        >
            {children()}
        </Select>
    }
}

/// Themed select trigger that composes SelectTrigger with SelectValue + chevron icon.
/// Pass `placeholder` instead of children — the trigger's content is managed internally.
#[component]
pub fn ThemedSelectTrigger(
    #[prop(into, optional)] placeholder: MaybeProp<String>,
) -> impl IntoView {
    let class = StoredValue::new(TRIGGER_CLASS);

    view! {
        <SelectTrigger attr:class=class.get_value()>
            <SelectValue placeholder=placeholder />
            <SelectIcon>
                <ChevronDownIcon />
            </SelectIcon>
        </SelectTrigger>
    }
}

/// Themed select content that composes SelectPortal > SelectContent > SelectViewport.
#[component]
pub fn ThemedSelectContent(children: ChildrenFn) -> impl IntoView {
    let content_class = StoredValue::new(CONTENT_CLASS);
    let viewport_class = StoredValue::new(VIEWPORT_CLASS);
    let children = StoredValue::new(children);

    view! {
        <SelectPortal>
            <SelectContent attr:class=content_class.get_value() position="popper">
                <SelectViewport attr:class=viewport_class.get_value()>
                    {children.with_value(|children| children())}
                </SelectViewport>
            </SelectContent>
        </SelectPortal>
    }
}

#[component]
pub fn ThemedSelectItem(
    #[prop(into)] value: String,
    children: ChildrenFn,
) -> impl IntoView {
    let class = StoredValue::new(ITEM_CLASS);
    let indicator_class = StoredValue::new(ITEM_INDICATOR_CLASS);
    let children = StoredValue::new(children);

    view! {
        <SelectItem attr:class=class.get_value() value=value>
            <SelectItemText>
                {children.with_value(|children| children())}
            </SelectItemText>
            <SelectItemIndicator attr:class=indicator_class.get_value()>
                <CheckIcon />
            </SelectItemIndicator>
        </SelectItem>
    }
}

#[component]
pub fn ThemedSelectGroup(children: ChildrenFn) -> impl IntoView {
    view! {
        <SelectGroup>
            {children()}
        </SelectGroup>
    }
}

#[component]
pub fn ThemedSelectLabel(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(LABEL_CLASS);

    view! {
        <SelectLabel attr:class=class.get_value()>
            {children()}
        </SelectLabel>
    }
}

#[component]
pub fn ThemedSelectSeparator() -> impl IntoView {
    let class = StoredValue::new(SEPARATOR_CLASS);

    view! {
        <SelectSeparator attr:class=class.get_value() />
    }
}

// ---------------------------------------------------------------------------
// Icons
// ---------------------------------------------------------------------------

#[component]
fn ChevronDownIcon() -> impl IntoView {
    view! {
        <svg
            class="size-4 opacity-50"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="m6 9 6 6 6-6" />
        </svg>
    }
}

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
