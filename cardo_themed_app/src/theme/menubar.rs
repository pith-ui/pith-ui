use cardo_ui::menubar::*;
use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui new-york menubar
// ---------------------------------------------------------------------------

const ROOT_CLASS: &str =
    "flex h-9 items-center gap-1 rounded-md border bg-background p-1 shadow-xs";

const TRIGGER_CLASS: &str = "flex items-center rounded-sm px-2 py-1 text-sm font-medium outline-hidden select-none menu-item-focus data-[state=open]:bg-accent data-[state=open]:text-accent-foreground";

const CONTENT_CLASS: &str =
    "z-50 min-w-[12rem] overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-md";

const ITEM_CLASS: &str = "relative flex cursor-default items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-hidden select-none menu-item-focus data-[disabled]:disabled-base";

const CHECKBOX_ITEM_CLASS: &str = "relative flex cursor-default items-center gap-2 rounded-sm py-1.5 pr-2 pl-8 text-sm outline-hidden select-none menu-item-focus data-[disabled]:disabled-base";

const INDICATOR_SPAN_CLASS: &str =
    "pointer-events-none absolute left-2 flex size-3.5 items-center justify-center";

const LABEL_CLASS: &str = "px-2 py-1.5 text-sm font-medium";

const SEPARATOR_CLASS: &str = "-mx-1 my-1 h-px bg-border";

const SUB_TRIGGER_CLASS: &str = "flex cursor-default items-center rounded-sm px-2 py-1.5 text-sm outline-hidden select-none menu-item-focus data-[state=open]:bg-accent data-[state=open]:text-accent-foreground";

const SUB_CONTENT_CLASS: &str =
    "popover-content-base shadow-lg";

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

#[component]
pub fn ThemedMenubar(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(ROOT_CLASS);

    view! {
        <Menubar attr:class=class.get_value()>
            {children()}
        </Menubar>
    }
}

#[component]
pub fn ThemedMenubarMenu(
    #[prop(into, optional)] value: Option<String>,
    children: ChildrenFn,
) -> impl IntoView {
    // MenubarMenu expects Option<String> but we need to unwrap to pass it correctly.
    // When None, the primitive generates an auto-id.
    match value {
        Some(v) => view! {
            <MenubarMenu value=v>
                {children()}
            </MenubarMenu>
        }
        .into_any(),
        None => view! {
            <MenubarMenu>
                {children()}
            </MenubarMenu>
        }
        .into_any(),
    }
}

#[component]
pub fn ThemedMenubarTrigger(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(TRIGGER_CLASS);
    let children = StoredValue::new(children);

    view! {
        <MenubarTrigger attr:class=class.get_value()>
            {children.with_value(|children| children())}
        </MenubarTrigger>
    }
}

#[component]
pub fn ThemedMenubarContent(
    #[prop(into, optional)] side_offset: Option<f64>,
    children: ChildrenFn,
) -> impl IntoView {
    let class = StoredValue::new(CONTENT_CLASS);
    let children = StoredValue::new(children);
    let offset = side_offset.unwrap_or(4.0);

    view! {
        <MenubarPortal>
            <MenubarContent
                attr:class=class.get_value()
                side_offset=offset
            >
                {children.with_value(|children| children())}
            </MenubarContent>
        </MenubarPortal>
    }
}

#[component]
pub fn ThemedMenubarItem(
    #[prop(into, optional)] on_select: Option<Callback<leptos::ev::Event>>,
    children: ChildrenFn,
) -> impl IntoView {
    let class = StoredValue::new(ITEM_CLASS);
    let children = StoredValue::new(children);

    view! {
        <MenubarItem
            attr:class=class.get_value()
            on_select=move |event: leptos::ev::Event| {
                if let Some(cb) = on_select {
                    cb.run(event);
                }
            }
        >
            {children.with_value(|children| children())}
        </MenubarItem>
    }
}

#[component]
pub fn ThemedMenubarCheckboxItem(
    #[prop(into, optional)] checked: MaybeProp<CheckedState>,
    #[prop(into, optional)] on_checked_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    let class = StoredValue::new(CHECKBOX_ITEM_CLASS);
    let indicator_class = StoredValue::new(INDICATOR_SPAN_CLASS);
    let children = StoredValue::new(children);

    view! {
        <MenubarCheckboxItem
            attr:class=class.get_value()
            checked=checked
            on_checked_change=move |val: bool| {
                if let Some(cb) = on_checked_change {
                    cb.run(val);
                }
            }
        >
            <span class=indicator_class.get_value()>
                <MenubarItemIndicator>
                    <CheckIcon />
                </MenubarItemIndicator>
            </span>
            {children.with_value(|children| children())}
        </MenubarCheckboxItem>
    }
}

#[component]
pub fn ThemedMenubarRadioGroup(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <MenubarRadioGroup
            value=value
            on_value_change=move |val: String| {
                if let Some(cb) = on_value_change {
                    cb.run(val);
                }
            }
        >
            {children()}
        </MenubarRadioGroup>
    }
}

#[component]
pub fn ThemedMenubarRadioItem(
    #[prop(into)] value: String,
    children: ChildrenFn,
) -> impl IntoView {
    let class = StoredValue::new(CHECKBOX_ITEM_CLASS);
    let indicator_class = StoredValue::new(INDICATOR_SPAN_CLASS);
    let children = StoredValue::new(children);

    view! {
        <MenubarRadioItem
            attr:class=class.get_value()
            value=value
        >
            <span class=indicator_class.get_value()>
                <MenubarItemIndicator>
                    <div class="size-2 rounded-full bg-current" />
                </MenubarItemIndicator>
            </span>
            {children.with_value(|children| children())}
        </MenubarRadioItem>
    }
}

#[component]
pub fn ThemedMenubarLabel(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(LABEL_CLASS);

    view! {
        <MenubarLabel attr:class=class.get_value()>
            {children()}
        </MenubarLabel>
    }
}

#[component]
pub fn ThemedMenubarSeparator() -> impl IntoView {
    let class = StoredValue::new(SEPARATOR_CLASS);

    view! {
        <MenubarSeparator attr:class=class.get_value() />
    }
}

#[component]
pub fn ThemedMenubarSub(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <MenubarSub
            open=open
            default_open=default_open
            on_open_change=move |val: bool| {
                if let Some(cb) = on_open_change {
                    cb.run(val);
                }
            }
        >
            {children()}
        </MenubarSub>
    }
}

#[component]
pub fn ThemedMenubarSubTrigger(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(SUB_TRIGGER_CLASS);
    let children = StoredValue::new(children);

    view! {
        <MenubarSubTrigger attr:class=class.get_value()>
            {children.with_value(|children| children())}
            <ChevronRightIcon />
        </MenubarSubTrigger>
    }
}

#[component]
pub fn ThemedMenubarSubContent(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(SUB_CONTENT_CLASS);
    let children = StoredValue::new(children);

    view! {
        <MenubarPortal>
            <MenubarSubContent attr:class=class.get_value()>
                {children.with_value(|children| children())}
            </MenubarSubContent>
        </MenubarPortal>
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
