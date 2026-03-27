use pith_ui::aspect_ratio::AspectRatio;
use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Component — thin pass-through to the pith-ui primitive
// ---------------------------------------------------------------------------

#[component]
pub fn ThemedAspectRatio(
    #[prop(into, optional, default = 1.0)] ratio: f64,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <AspectRatio ratio=ratio>
            {children.with_value(|children| children())}
        </AspectRatio>
    }
}
