use cardo_ui::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Style definitions — Collapsible is behavior-only, minimal styling
// ---------------------------------------------------------------------------

// No shadcn classes for Collapsible — it's a behavior-only primitive.
// Themed wrappers just pass through props.

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

#[component]
pub fn ThemedCollapsible(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    let forward_cb = Callback::new(move |val: bool| {
        if let Some(cb) = on_open_change {
            cb.run(val);
        }
    });

    view! {
        <Collapsible
            open=open
            default_open=default_open
            disabled=disabled
            on_open_change=forward_cb
        >
            {children()}
        </Collapsible>
    }
}

#[component]
pub fn ThemedCollapsibleTrigger(children: ChildrenFn) -> impl IntoView {
    view! {
        <CollapsibleTrigger as_child=true>
            {children()}
        </CollapsibleTrigger>
    }
}

#[component]
pub fn ThemedCollapsibleContent(children: ChildrenFn) -> impl IntoView {
    view! {
        <CollapsibleContent>
            {children()}
        </CollapsibleContent>
    }
}
