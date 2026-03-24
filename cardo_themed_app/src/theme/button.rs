use leptos::prelude::*;
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui new-york button
// ---------------------------------------------------------------------------

#[derive(TwVariant)]
pub enum ButtonVariant {
    #[tw(default, class = "bg-primary text-primary-foreground hover:bg-primary/80")]
    Default,
    #[tw(class = "bg-destructive/10 text-destructive hover:bg-destructive/20 focus-visible:focus-ring-destructive dark:bg-destructive/20 dark:hover:bg-destructive/30")]
    Destructive,
    #[tw(class = "border border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50")]
    Outline,
    #[tw(class = "bg-secondary text-secondary-foreground hover:bg-secondary/80")]
    Secondary,
    #[tw(class = "hover:bg-muted hover:text-foreground dark:hover:bg-muted/50")]
    Ghost,
    #[tw(class = "text-primary underline-offset-4 hover:underline")]
    Link,
}
cardo_ui_themes::impl_cardo_tw_variant!(ButtonVariant);

#[derive(TwVariant)]
pub enum ButtonSize {
    #[tw(default, class = "h-9 px-4 py-2")]
    Default,
    #[tw(class = "h-8 gap-1.5 rounded-md px-3")]
    Sm,
    #[tw(class = "h-10 rounded-md px-6")]
    Lg,
    #[tw(class = "size-9")]
    Icon,
}
cardo_ui_themes::impl_cardo_tw_variant!(ButtonSize);

#[derive(TwClass)]
#[tw(
    class = "inline-flex shrink-0 items-center justify-center gap-2 rounded-md text-sm font-medium whitespace-nowrap transition-all outline-none focus-visible:focus-ring disabled:disabled-base"
)]
pub struct ButtonClass {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
}
cardo_ui_themes::impl_cardo_tw_class!(ButtonClass);

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

#[component]
pub fn Button(
    #[prop(into, optional)] variant: ButtonVariant,
    #[prop(into, optional)] size: ButtonSize,
    #[prop(into, optional)] disabled: Option<bool>,
    children: Children,
) -> impl IntoView {
    let class = ButtonClass { variant, size }.to_class();
    view! {
        <button class=class disabled=disabled.unwrap_or(false)>
            {children()}
        </button>
    }
}
