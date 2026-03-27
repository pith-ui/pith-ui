use pith_ui::scroll_area::{
    Orientation, ScrollArea, ScrollAreaCorner, ScrollAreaScrollbar, ScrollAreaThumb, ScrollAreaType,
    ScrollAreaViewport,
};
use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui new-york scroll-area
// ---------------------------------------------------------------------------

const ROOT_CLASS: &str = "overflow-hidden";
const VIEWPORT_CLASS: &str = "size-full rounded-[inherit]";

const SCROLLBAR_VERTICAL_CLASS: &str =
    "flex touch-none p-px transition-colors select-none h-full w-2.5 border-l border-l-transparent";

const SCROLLBAR_HORIZONTAL_CLASS: &str =
    "flex touch-none p-px transition-colors select-none h-2.5 flex-col border-t border-t-transparent";

const THUMB_CLASS: &str = "relative flex-1 rounded-full bg-border";

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

/// Themed scroll area that composes ScrollArea > ScrollAreaViewport + vertical scrollbar.
/// Wraps children in a viewport with a styled vertical scrollbar.
#[allow(clippy::unused_unit)]
#[component]
pub fn ThemedScrollArea(
    #[prop(into, optional)] class: Option<&'static str>,
    #[prop(into, optional)] r#type: Option<ScrollAreaType>,
    children: ChildrenFn,
) -> impl IntoView {
    let viewport_class = StoredValue::new(VIEWPORT_CLASS);
    let scrollbar_v_class = StoredValue::new(SCROLLBAR_VERTICAL_CLASS);
    let scrollbar_h_class = StoredValue::new(SCROLLBAR_HORIZONTAL_CLASS);
    let thumb_class = StoredValue::new(THUMB_CLASS);
    let combined_class = StoredValue::new(format!(
        "{} {}",
        ROOT_CLASS,
        class.unwrap_or("relative")
    ));
    let children = StoredValue::new(children);

    view! {
        <ScrollArea attr:class=combined_class.get_value() r#type=r#type.unwrap_or_default()>
            <ScrollAreaViewport attr:class=viewport_class.get_value()>
                {children.with_value(|children| children())}
            </ScrollAreaViewport>
            <ScrollAreaScrollbar
                attr:class=scrollbar_v_class.get_value()
                orientation=Orientation::Vertical
            >
                <ScrollAreaThumb attr:class=thumb_class.get_value()>{()}</ScrollAreaThumb>
            </ScrollAreaScrollbar>
            <ScrollAreaScrollbar
                attr:class=scrollbar_h_class.get_value()
                orientation=Orientation::Horizontal
            >
                <ScrollAreaThumb attr:class=thumb_class.get_value()>{()}</ScrollAreaThumb>
            </ScrollAreaScrollbar>
            <ScrollAreaCorner>{()}</ScrollAreaCorner>
        </ScrollArea>
    }
}
