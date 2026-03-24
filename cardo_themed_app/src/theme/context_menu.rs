use cardo_ui::context_menu::*;
use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui new-york context-menu
// ---------------------------------------------------------------------------

const CONTENT_CLASS: &str =
    "z-50 min-w-[8rem] overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-md";

const ITEM_CLASS: &str = "relative flex cursor-default items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-hidden select-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50";

const CHECKBOX_ITEM_CLASS: &str = "relative flex cursor-default items-center gap-2 rounded-sm py-1.5 pr-2 pl-8 text-sm outline-hidden select-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50";

const INDICATOR_SPAN_CLASS: &str =
    "pointer-events-none absolute left-2 flex size-3.5 items-center justify-center";

const LABEL_CLASS: &str = "px-2 py-1.5 text-sm font-medium text-foreground";

const SEPARATOR_CLASS: &str = "-mx-1 my-1 h-px bg-border";

const SUB_TRIGGER_CLASS: &str = "flex cursor-default items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-hidden select-none focus:bg-accent focus:text-accent-foreground data-[state=open]:bg-accent data-[state=open]:text-accent-foreground";

const SUB_CONTENT_CLASS: &str =
    "z-50 min-w-[8rem] overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-lg";

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

#[component]
pub fn ThemedContextMenu(
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <ContextMenu
            on_open_change=move |val: bool| {
                if let Some(cb) = on_open_change {
                    cb.run(val);
                }
            }
        >
            {children()}
        </ContextMenu>
    }
}

#[component]
pub fn ThemedContextMenuTrigger(children: ChildrenFn) -> impl IntoView {
    view! {
        <ContextMenuTrigger>
            {children()}
        </ContextMenuTrigger>
    }
}

/// Themed context menu content that composes ContextMenuPortal > ContextMenuContent.
#[component]
pub fn ThemedContextMenuContent(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(CONTENT_CLASS);
    let children = StoredValue::new(children);

    view! {
        <ContextMenuPortal>
            <ContextMenuContent attr:class=class.get_value()>
                {children.with_value(|children| children())}
            </ContextMenuContent>
        </ContextMenuPortal>
    }
}

#[component]
pub fn ThemedContextMenuItem(
    #[prop(into, optional)] on_select: Option<Callback<leptos::ev::Event>>,
    children: ChildrenFn,
) -> impl IntoView {
    let class = StoredValue::new(ITEM_CLASS);
    let children = StoredValue::new(children);

    view! {
        <ContextMenuItem
            attr:class=class.get_value()
            on_select=move |event: leptos::ev::Event| {
                if let Some(cb) = on_select {
                    cb.run(event);
                }
            }
        >
            {children.with_value(|children| children())}
        </ContextMenuItem>
    }
}

#[component]
pub fn ThemedContextMenuCheckboxItem(
    #[prop(into, optional)] checked: MaybeProp<CheckedState>,
    #[prop(into, optional)] on_checked_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    let class = StoredValue::new(CHECKBOX_ITEM_CLASS);
    let indicator_class = StoredValue::new(INDICATOR_SPAN_CLASS);
    let children = StoredValue::new(children);

    view! {
        <ContextMenuCheckboxItem
            attr:class=class.get_value()
            checked=checked
            on_checked_change=move |val: bool| {
                if let Some(cb) = on_checked_change {
                    cb.run(val);
                }
            }
        >
            <span class=indicator_class.get_value()>
                <ContextMenuItemIndicator>
                    <CheckIcon />
                </ContextMenuItemIndicator>
            </span>
            {children.with_value(|children| children())}
        </ContextMenuCheckboxItem>
    }
}

#[component]
pub fn ThemedContextMenuRadioGroup(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <ContextMenuRadioGroup
            value=value
            on_value_change=move |val: String| {
                if let Some(cb) = on_value_change {
                    cb.run(val);
                }
            }
        >
            {children()}
        </ContextMenuRadioGroup>
    }
}

#[component]
pub fn ThemedContextMenuRadioItem(
    #[prop(into)] value: String,
    children: ChildrenFn,
) -> impl IntoView {
    let class = StoredValue::new(CHECKBOX_ITEM_CLASS);
    let indicator_class = StoredValue::new(INDICATOR_SPAN_CLASS);
    let children = StoredValue::new(children);

    view! {
        <ContextMenuRadioItem
            attr:class=class.get_value()
            value=value
        >
            <span class=indicator_class.get_value()>
                <ContextMenuItemIndicator>
                    <div class="size-2 rounded-full bg-current" />
                </ContextMenuItemIndicator>
            </span>
            {children.with_value(|children| children())}
        </ContextMenuRadioItem>
    }
}

#[component]
pub fn ThemedContextMenuLabel(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(LABEL_CLASS);

    view! {
        <ContextMenuLabel attr:class=class.get_value()>
            {children()}
        </ContextMenuLabel>
    }
}

#[component]
pub fn ThemedContextMenuSeparator() -> impl IntoView {
    let class = StoredValue::new(SEPARATOR_CLASS);

    view! {
        <ContextMenuSeparator attr:class=class.get_value() />
    }
}

#[component]
pub fn ThemedContextMenuSub(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <ContextMenuSub
            open=open
            default_open=default_open
            on_open_change=move |val: bool| {
                if let Some(cb) = on_open_change {
                    cb.run(val);
                }
            }
        >
            {children()}
        </ContextMenuSub>
    }
}

#[component]
pub fn ThemedContextMenuSubTrigger(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(SUB_TRIGGER_CLASS);
    let children = StoredValue::new(children);

    view! {
        <ContextMenuSubTrigger attr:class=class.get_value()>
            {children.with_value(|children| children())}
            <ChevronRightIcon />
        </ContextMenuSubTrigger>
    }
}

#[component]
pub fn ThemedContextMenuSubContent(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(SUB_CONTENT_CLASS);
    let children = StoredValue::new(children);

    view! {
        <ContextMenuPortal>
            <ContextMenuSubContent attr:class=class.get_value()>
                {children.with_value(|children| children())}
            </ContextMenuSubContent>
        </ContextMenuPortal>
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
