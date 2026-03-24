use cardo_ui::dropdown_menu::*;
use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui new-york dropdown-menu
// ---------------------------------------------------------------------------

const CONTENT_CLASS: &str =
    "popover-content-base shadow-md";

const ITEM_CLASS: &str = "relative flex cursor-default items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-hidden select-none menu-item-focus data-[disabled]:disabled-base";

const CHECKBOX_ITEM_CLASS: &str = "relative flex cursor-default items-center gap-2 rounded-sm py-1.5 pr-2 pl-8 text-sm outline-hidden select-none menu-item-focus data-[disabled]:disabled-base";

const INDICATOR_SPAN_CLASS: &str =
    "pointer-events-none absolute left-2 flex size-3.5 items-center justify-center";

const LABEL_CLASS: &str = "px-2 py-1.5 text-sm font-medium";

const SEPARATOR_CLASS: &str = "-mx-1 my-1 h-px bg-border";

const SUB_TRIGGER_CLASS: &str = "flex cursor-default items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-hidden select-none menu-item-focus data-[state=open]:bg-accent data-[state=open]:text-accent-foreground";

const SUB_CONTENT_CLASS: &str =
    "popover-content-base shadow-lg";

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

#[component]
pub fn ThemedDropdownMenu(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    let forward_cb = Callback::new(move |val: bool| {
        if let Some(cb) = on_open_change {
            cb.run(val);
        }
    });

    view! {
        <DropdownMenu
            open=open
            default_open=default_open
            on_open_change=forward_cb
        >
            {children()}
        </DropdownMenu>
    }
}

#[component]
pub fn ThemedDropdownMenuTrigger(children: ChildrenFn) -> impl IntoView {
    view! {
        <DropdownMenuTrigger as_child=true>
            {children()}
        </DropdownMenuTrigger>
    }
}

/// Themed dropdown menu content that composes DropdownMenuPortal > DropdownMenuContent.
#[component]
pub fn ThemedDropdownMenuContent(
    #[prop(into, optional)] side_offset: Option<f64>,
    children: ChildrenFn,
) -> impl IntoView {
    let class = StoredValue::new(CONTENT_CLASS);
    let children = StoredValue::new(children);
    let offset = side_offset.unwrap_or(4.0);

    view! {
        <DropdownMenuPortal>
            <DropdownMenuContent
                attr:class=class.get_value()
                side_offset=offset
            >
                {children.with_value(|children| children())}
            </DropdownMenuContent>
        </DropdownMenuPortal>
    }
}

#[component]
pub fn ThemedDropdownMenuItem(
    #[prop(into, optional)] on_select: Option<Callback<leptos::ev::Event>>,
    children: ChildrenFn,
) -> impl IntoView {
    let class = StoredValue::new(ITEM_CLASS);
    let children = StoredValue::new(children);

    view! {
        <DropdownMenuItem
            attr:class=class.get_value()
            on_select=move |event: leptos::ev::Event| {
                if let Some(cb) = on_select {
                    cb.run(event);
                }
            }
        >
            {children.with_value(|children| children())}
        </DropdownMenuItem>
    }
}

#[component]
pub fn ThemedDropdownMenuCheckboxItem(
    #[prop(into, optional)] checked: MaybeProp<CheckedState>,
    #[prop(into, optional)] on_checked_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    let class = StoredValue::new(CHECKBOX_ITEM_CLASS);
    let indicator_class = StoredValue::new(INDICATOR_SPAN_CLASS);
    let children = StoredValue::new(children);

    view! {
        <DropdownMenuCheckboxItem
            attr:class=class.get_value()
            checked=checked
            on_checked_change=move |val: bool| {
                if let Some(cb) = on_checked_change {
                    cb.run(val);
                }
            }
        >
            <span class=indicator_class.get_value()>
                <DropdownMenuItemIndicator>
                    <CheckIcon />
                </DropdownMenuItemIndicator>
            </span>
            {children.with_value(|children| children())}
        </DropdownMenuCheckboxItem>
    }
}

#[component]
pub fn ThemedDropdownMenuRadioGroup(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <DropdownMenuRadioGroup
            value=value
            on_value_change=move |val: String| {
                if let Some(cb) = on_value_change {
                    cb.run(val);
                }
            }
        >
            {children()}
        </DropdownMenuRadioGroup>
    }
}

#[component]
pub fn ThemedDropdownMenuRadioItem(
    #[prop(into)] value: String,
    children: ChildrenFn,
) -> impl IntoView {
    let class = StoredValue::new(CHECKBOX_ITEM_CLASS);
    let indicator_class = StoredValue::new(INDICATOR_SPAN_CLASS);
    let children = StoredValue::new(children);

    view! {
        <DropdownMenuRadioItem
            attr:class=class.get_value()
            value=value
        >
            <span class=indicator_class.get_value()>
                <DropdownMenuItemIndicator>
                    <div class="size-2 rounded-full bg-current" />
                </DropdownMenuItemIndicator>
            </span>
            {children.with_value(|children| children())}
        </DropdownMenuRadioItem>
    }
}

#[component]
pub fn ThemedDropdownMenuLabel(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(LABEL_CLASS);

    view! {
        <DropdownMenuLabel attr:class=class.get_value()>
            {children()}
        </DropdownMenuLabel>
    }
}

#[component]
pub fn ThemedDropdownMenuSeparator() -> impl IntoView {
    let class = StoredValue::new(SEPARATOR_CLASS);

    view! {
        <DropdownMenuSeparator attr:class=class.get_value() />
    }
}

#[component]
pub fn ThemedDropdownMenuSub(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <DropdownMenuSub
            open=open
            default_open=default_open
            on_open_change=move |val: bool| {
                if let Some(cb) = on_open_change {
                    cb.run(val);
                }
            }
        >
            {children()}
        </DropdownMenuSub>
    }
}

#[component]
pub fn ThemedDropdownMenuSubTrigger(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(SUB_TRIGGER_CLASS);
    let children = StoredValue::new(children);

    view! {
        <DropdownMenuSubTrigger attr:class=class.get_value()>
            {children.with_value(|children| children())}
            <ChevronRightIcon />
        </DropdownMenuSubTrigger>
    }
}

#[component]
pub fn ThemedDropdownMenuSubContent(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(SUB_CONTENT_CLASS);
    let children = StoredValue::new(children);

    view! {
        <DropdownMenuPortal>
            <DropdownMenuSubContent attr:class=class.get_value()>
                {children.with_value(|children| children())}
            </DropdownMenuSubContent>
        </DropdownMenuPortal>
    }
}

// ---------------------------------------------------------------------------
// Shared icons
// ---------------------------------------------------------------------------

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

#[component]
fn ChevronRightIcon() -> impl IntoView {
    view! {
        <svg
            class="ml-auto size-4"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="m9 18 6-6-6-6" />
        </svg>
    }
}
