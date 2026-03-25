use pith_ui::hover_card::*;
use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui new-york hover-card
// ---------------------------------------------------------------------------

const CONTENT_CLASS: &str =
    "z-50 w-64 rounded-md border bg-popover p-4 text-popover-foreground shadow-md outline-hidden";

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

#[component]
pub fn ThemedHoverCard(
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
        <HoverCard
            open=open
            default_open=default_open
            on_open_change=forward_cb
        >
            {children()}
        </HoverCard>
    }
}

#[component]
pub fn ThemedHoverCardTrigger(children: ChildrenFn) -> impl IntoView {
    view! {
        <HoverCardTrigger as_child=true>
            {children()}
        </HoverCardTrigger>
    }
}

/// Themed hover card content that composes HoverCardPortal > HoverCardContent.
#[component]
pub fn ThemedHoverCardContent(
    #[prop(into, optional)] side: Option<Side>,
    #[prop(into, optional)] side_offset: Option<f64>,
    children: ChildrenFn,
) -> impl IntoView {
    let class = StoredValue::new(CONTENT_CLASS);
    let children = StoredValue::new(children);
    let offset = side_offset.unwrap_or(4.0);
    let side = side.unwrap_or(Side::Bottom);

    view! {
        <HoverCardPortal>
            <HoverCardContent
                attr:class=class.get_value()
                side=Signal::stored(side)
                side_offset=Signal::stored(offset)
            >
                {children.with_value(|children| children())}
            </HoverCardContent>
        </HoverCardPortal>
    }
}
