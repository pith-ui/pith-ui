use cardo_ui::separator::{Orientation, Separator};
use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui new-york separator
// ---------------------------------------------------------------------------

const SEPARATOR_CLASS: &str = "shrink-0 bg-border data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-full data-[orientation=vertical]:w-px";

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

#[component]
pub fn ThemedSeparator(
    #[prop(into, optional)] orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] decorative: MaybeProp<bool>,
) -> impl IntoView {
    view! {
        <Separator
            attr:class=SEPARATOR_CLASS
            orientation=orientation
            decorative=decorative
        />
    }
}
