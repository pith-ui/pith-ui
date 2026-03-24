use cardo_ui::combobox::*;
use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui new-york combobox
// ---------------------------------------------------------------------------

const ANCHOR_CLASS: &str = "flex h-9 w-full items-center rounded-md border border-input bg-transparent shadow-xs focus-within:focus-ring dark:bg-input/30";

const INPUT_CLASS: &str =
    "flex-1 bg-transparent px-3 py-1 text-sm outline-none placeholder:text-muted-foreground";

const TRIGGER_CLASS: &str = "flex items-center justify-center px-2 text-muted-foreground";

const CONTENT_CLASS: &str = "z-50 w-[var(--radix-popper-anchor-width)] overflow-hidden rounded-md border bg-popover text-popover-foreground shadow-md";

const VIEWPORT_CLASS: &str = "p-1 max-h-60 overflow-y-auto";

const ITEM_CLASS: &str = "relative flex w-full cursor-default items-center gap-2 rounded-sm py-1.5 pr-8 pl-2 text-sm outline-hidden select-none data-[highlighted]:bg-accent data-[highlighted]:text-accent-foreground data-[disabled]:disabled-base";

const ITEM_INDICATOR_CLASS: &str =
    "pointer-events-none absolute right-2 flex size-4 items-center justify-center";

const EMPTY_CLASS: &str = "py-6 text-center text-sm text-muted-foreground";

const LABEL_CLASS: &str = "px-2 py-1.5 text-xs text-muted-foreground";

const CHIPS_CLASS: &str = "flex min-h-9 flex-wrap items-center gap-1.5 rounded-md border border-input bg-transparent px-2.5 py-1.5 text-sm shadow-xs transition-[color,box-shadow] focus-within:focus-ring dark:bg-input/30";

const CHIP_CLASS: &str = "flex h-5.5 items-center gap-1 rounded-sm bg-muted px-1.5 text-xs font-medium text-foreground data-[highlighted]:ring-2 data-[highlighted]:ring-ring/50";

const CHIP_REMOVE_CLASS: &str = "inline-flex items-center justify-center size-3.5 rounded-sm text-muted-foreground hover:text-foreground";

type ComboboxValues = Vec<String>;

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

#[component]
pub fn ThemedCombobox(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] input_value: MaybeProp<String>,
    #[prop(into, optional)] default_input_value: MaybeProp<String>,
    #[prop(into, optional)] on_input_value_change: Option<Callback<String>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <Combobox
            open=open
            default_open=default_open
            on_open_change=move |val: bool| {
                if let Some(cb) = on_open_change {
                    cb.run(val);
                }
            }
            value=value
            default_value=default_value
            on_value_change=move |val: String| {
                if let Some(cb) = on_value_change {
                    cb.run(val);
                }
            }
            input_value=input_value
            default_input_value=default_input_value
            on_input_value_change=move |val: String| {
                if let Some(cb) = on_input_value_change {
                    cb.run(val);
                }
            }
            disabled=disabled
        >
            {children()}
        </Combobox>
    }
}

#[component]
pub fn ThemedComboboxAnchor(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(ANCHOR_CLASS);

    view! {
        <ComboboxAnchor attr:class=class.get_value()>
            {children()}
        </ComboboxAnchor>
    }
}

#[component]
pub fn ThemedComboboxInput(
    #[prop(into, optional)] placeholder: MaybeProp<String>,
) -> impl IntoView {
    let class = StoredValue::new(INPUT_CLASS);

    view! {
        <ComboboxInput attr:class=class.get_value() placeholder=placeholder />
    }
}

#[component]
pub fn ThemedComboboxTrigger() -> impl IntoView {
    let class = StoredValue::new(TRIGGER_CLASS);

    view! {
        <ComboboxTrigger attr:class=class.get_value()>
            <ChevronDownIcon />
        </ComboboxTrigger>
    }
}

#[component]
pub fn ThemedComboboxContent(children: ChildrenFn) -> impl IntoView {
    let content_class = StoredValue::new(CONTENT_CLASS);
    let children = StoredValue::new(children);

    view! {
        <ComboboxPortal>
            <ComboboxContent
                attr:class=content_class.get_value()
                side_offset=Signal::stored(4.0)
            >
                {children.with_value(|children| children())}
            </ComboboxContent>
        </ComboboxPortal>
    }
}

#[component]
pub fn ThemedComboboxViewport(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(VIEWPORT_CLASS);
    let children = StoredValue::new(children);

    view! {
        <ComboboxViewport attr:class=class.get_value()>
            {children.with_value(|children| children())}
        </ComboboxViewport>
    }
}

#[component]
pub fn ThemedComboboxItem(
    #[prop(into)] value: String,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] text_value: MaybeProp<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let class = StoredValue::new(ITEM_CLASS);
    let indicator_class = StoredValue::new(ITEM_INDICATOR_CLASS);
    let children = StoredValue::new(children);

    view! {
        <ComboboxItem attr:class=class.get_value() value=value disabled=disabled text_value=text_value>
            <ComboboxItemText>
                {children.with_value(|children| children())}
            </ComboboxItemText>
            <ComboboxItemIndicator attr:class=indicator_class.get_value()>
                <CheckIcon />
            </ComboboxItemIndicator>
        </ComboboxItem>
    }
}

#[component]
pub fn ThemedComboboxEmpty(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(EMPTY_CLASS);

    view! {
        <ComboboxEmpty attr:class=class.get_value()>
            {children()}
        </ComboboxEmpty>
    }
}

#[component]
pub fn ThemedComboboxGroup(children: ChildrenFn) -> impl IntoView {
    view! {
        <ComboboxGroup>
            {children()}
        </ComboboxGroup>
    }
}

#[component]
pub fn ThemedComboboxLabel(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(LABEL_CLASS);

    view! {
        <ComboboxLabel attr:class=class.get_value()>
            {children()}
        </ComboboxLabel>
    }
}

/// Multi-select combobox — pass `multiple=true` to ThemedCombobox.
/// Use ThemedComboboxChips as the anchor instead of ThemedComboboxAnchor.
#[component]
pub fn ThemedComboboxMulti(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] values: MaybeProp<ComboboxValues>,
    #[prop(into, optional)] default_values: MaybeProp<ComboboxValues>,
    #[prop(into, optional)] on_values_change: Option<Callback<ComboboxValues>>,
    #[prop(into, optional)] input_value: MaybeProp<String>,
    #[prop(into, optional)] default_input_value: MaybeProp<String>,
    #[prop(into, optional)] on_input_value_change: Option<Callback<String>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <Combobox
            multiple=true
            open=open
            default_open=default_open
            on_open_change=move |val: bool| {
                if let Some(cb) = on_open_change {
                    cb.run(val);
                }
            }
            values=values
            default_values=default_values
            on_values_change=move |val: ComboboxValues| {
                if let Some(cb) = on_values_change {
                    cb.run(val);
                }
            }
            input_value=input_value
            default_input_value=default_input_value
            on_input_value_change=move |val: String| {
                if let Some(cb) = on_input_value_change {
                    cb.run(val);
                }
            }
            disabled=disabled
        >
            {children()}
        </Combobox>
    }
}

#[component]
pub fn ThemedComboboxChips(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(CHIPS_CLASS);
    let children = StoredValue::new(children);

    view! {
        <ComboboxAnchor>
            <ComboboxChips attr:class=class.get_value()>
                {children.with_value(|children| children())}
            </ComboboxChips>
        </ComboboxAnchor>
    }
}

#[component]
pub fn ThemedComboboxChip(
    #[prop(into)] value: String,
    children: ChildrenFn,
) -> impl IntoView {
    let class = StoredValue::new(CHIP_CLASS);
    let remove_class = StoredValue::new(CHIP_REMOVE_CLASS);
    let remove_value = StoredValue::new(value.clone());
    let chip_value = StoredValue::new(value);
    let children = StoredValue::new(children);

    view! {
        <ComboboxChip attr:class=class.get_value() value=chip_value.get_value()>
            {children.with_value(|children| children())}
            <ComboboxChipRemove attr:class=remove_class.get_value() value=remove_value.get_value()>
                <XIcon />
            </ComboboxChipRemove>
        </ComboboxChip>
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
fn XIcon() -> impl IntoView {
    view! {
        <svg
            class="size-3"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="M18 6 6 18" />
            <path d="m6 6 12 12" />
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
