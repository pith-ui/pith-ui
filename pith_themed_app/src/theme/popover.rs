use pith_ui::popover::{Popover, PopoverClose, PopoverContent, PopoverPortal, PopoverTrigger};
use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui new-york popover
// ---------------------------------------------------------------------------

const CONTENT_CLASS: &str =
    "z-50 w-72 rounded-md border bg-popover p-4 text-popover-foreground shadow-md outline-hidden";

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

#[component]
pub fn ThemedPopover(
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
        <Popover
            open=open
            default_open=default_open
            on_open_change=forward_cb
        >
            {children()}
        </Popover>
    }
}

#[component]
pub fn ThemedPopoverTrigger(children: ChildrenFn) -> impl IntoView {
    view! {
        <PopoverTrigger as_child=true>
            {children()}
        </PopoverTrigger>
    }
}

/// Themed popover content that composes PopoverPortal > PopoverContent.
#[component]
pub fn ThemedPopoverContent(
    #[prop(into, optional)] side_offset: Option<f64>,
    children: ChildrenFn,
) -> impl IntoView {
    let class = StoredValue::new(CONTENT_CLASS);
    let children = StoredValue::new(children);
    let offset = side_offset.unwrap_or(4.0);

    view! {
        <PopoverPortal>
            <PopoverContent
                attr:class=class.get_value()
                side_offset=Signal::stored(offset)
            >
                {children.with_value(|children| children())}
            </PopoverContent>
        </PopoverPortal>
    }
}

#[component]
pub fn ThemedPopoverClose(children: ChildrenFn) -> impl IntoView {
    view! {
        <PopoverClose as_child=true>
            {children()}
        </PopoverClose>
    }
}
