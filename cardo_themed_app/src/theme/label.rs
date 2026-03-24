use cardo_ui::label::Label;
use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui new-york label
// ---------------------------------------------------------------------------

const LABEL_CLASS: &str = "flex items-center gap-2 text-sm leading-none font-medium select-none group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50 peer-disabled:cursor-not-allowed peer-disabled:opacity-50";

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

#[component]
pub fn ThemedLabel(children: ChildrenFn) -> impl IntoView {
    view! {
        <Label attr:class=LABEL_CLASS>
            {children()}
        </Label>
    }
}
