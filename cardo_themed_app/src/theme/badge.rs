use leptos::prelude::*;
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui new-york badge
// ---------------------------------------------------------------------------

#[derive(TwVariant)]
pub enum BadgeVariant {
    #[tw(default, class = "bg-primary text-primary-foreground hover:bg-primary/90")]
    Default,
    #[tw(class = "bg-secondary text-secondary-foreground hover:bg-secondary/90")]
    Secondary,
    #[tw(class = "bg-destructive text-white hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:bg-destructive/60 dark:focus-visible:ring-destructive/40")]
    Destructive,
    #[tw(class = "border border-border text-foreground hover:bg-accent hover:text-accent-foreground")]
    Outline,
}
cardo_ui_themes::impl_cardo_tw_variant!(BadgeVariant);

#[derive(TwClass)]
#[tw(
    class = "inline-flex w-fit shrink-0 items-center justify-center gap-1 overflow-hidden rounded-full border border-transparent px-2 py-0.5 text-xs font-medium whitespace-nowrap transition-[color,box-shadow] focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50"
)]
pub struct BadgeClass {
    pub variant: BadgeVariant,
}
cardo_ui_themes::impl_cardo_tw_class!(BadgeClass);

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

#[component]
pub fn Badge(
    #[prop(into, optional)] variant: BadgeVariant,
    children: Children,
) -> impl IntoView {
    let class = BadgeClass { variant }.to_class();
    view! {
        <span class=class>{children()}</span>
    }
}
