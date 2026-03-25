use pith_ui::tooltip::{Tooltip, TooltipContent, TooltipPortal, TooltipProvider, TooltipTrigger};
use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui new-york tooltip
// ---------------------------------------------------------------------------

const CONTENT_CLASS: &str =
    "z-50 w-fit rounded-md bg-primary px-3 py-1.5 text-xs text-primary-foreground";

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

/// A convenience wrapper that composes the full tooltip stack:
/// TooltipProvider > Tooltip > TooltipTrigger + TooltipPortal > TooltipContent.
///
/// Pass `content` for the tooltip text and wrap the trigger element as children.
#[component]
pub fn ThemedTooltip(
    /// The text shown in the tooltip popup.
    #[prop(into)]
    content: &'static str,
    /// Which side the tooltip appears on (defaults to top via TooltipContent).
    #[prop(into, optional)]
    side_offset: Option<f64>,
    children: ChildrenFn,
) -> impl IntoView {
    let class = StoredValue::new(CONTENT_CLASS);
    let content = StoredValue::new(content);
    let children = StoredValue::new(children);
    let offset = side_offset.unwrap_or(4.0);

    view! {
        <TooltipProvider>
            <Tooltip>
                <TooltipTrigger as_child=true>
                    {children.with_value(|children| children())}
                </TooltipTrigger>
                <TooltipPortal>
                    <TooltipContent
                        attr:class=class.get_value()
                        side_offset=Signal::stored(offset)
                    >
                        {content.get_value()}
                    </TooltipContent>
                </TooltipPortal>
            </Tooltip>
        </TooltipProvider>
    }
}
