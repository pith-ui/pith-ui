use cardo_ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui new-york avatar
// ---------------------------------------------------------------------------

const AVATAR_CLASS: &str = "relative flex size-8 shrink-0 overflow-hidden rounded-full";
const IMAGE_CLASS: &str = "aspect-square size-full";
const FALLBACK_CLASS: &str =
    "flex size-full items-center justify-center rounded-full bg-muted text-sm text-muted-foreground";

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

#[component]
pub fn ThemedAvatar(
    #[prop(into, optional)] src: MaybeProp<String>,
    #[prop(into, optional)] alt: MaybeProp<String>,
    #[prop(into, optional)] fallback: Option<String>,
    #[prop(into, optional)] delay_ms: MaybeProp<i32>,
) -> impl IntoView {
    let fallback = StoredValue::new(fallback);
    let alt = StoredValue::new(alt);

    view! {
        <Avatar attr:class=AVATAR_CLASS>
            <AvatarImage attr:class=IMAGE_CLASS attr:alt=move || alt.get_value().get() src=src />
            <AvatarFallback attr:class=FALLBACK_CLASS delay_ms=delay_ms>
                {move || fallback.get_value().clone()}
            </AvatarFallback>
        </Avatar>
    }
}
