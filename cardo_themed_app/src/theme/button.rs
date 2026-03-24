use leptos::prelude::*;
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// Style definitions (token-referencing utility classes only)
// ---------------------------------------------------------------------------

#[derive(TwVariant)]
pub enum ButtonVariant {
    #[tw(default, class = "bg-accent-9 text-white hover:bg-accent-10 active:bg-accent-10")]
    Solid,
    #[tw(class = "bg-accent-3 text-accent-11 hover:bg-accent-4 active:bg-accent-5")]
    Soft,
    #[tw(class = "border border-accent-7 text-accent-11 bg-transparent hover:bg-accent-2 active:bg-accent-3")]
    Outline,
    #[tw(class = "text-accent-11 bg-transparent hover:bg-accent-3 active:bg-accent-4")]
    Ghost,
}
cardo_ui_themes::impl_cardo_tw_variant!(ButtonVariant);

#[derive(TwVariant)]
pub enum ButtonSize {
    #[tw(class = "h-7 px-2 text-xs gap-1 rounded-1")]
    Sm,
    #[tw(default, class = "h-9 px-3 text-sm gap-1.5 rounded-2")]
    Md,
    #[tw(class = "h-11 px-4 text-base gap-2 rounded-3")]
    Lg,
}
cardo_ui_themes::impl_cardo_tw_variant!(ButtonSize);

#[derive(TwClass)]
#[tw(
    class = "inline-flex items-center justify-center font-medium transition-colors duration-normal select-none focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-focus-ring focus-visible:ring-offset-2 focus-visible:ring-offset-page disabled:opacity-50 disabled:pointer-events-none"
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
