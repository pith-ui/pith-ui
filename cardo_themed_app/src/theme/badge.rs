use leptos::prelude::*;
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// Style definitions
// ---------------------------------------------------------------------------

#[derive(TwVariant)]
pub enum BadgeVariant {
    #[tw(default, class = "bg-accent-9 text-white")]
    Solid,
    #[tw(class = "bg-accent-3 text-accent-11")]
    Soft,
    #[tw(class = "border border-accent-7 text-accent-11 bg-transparent")]
    Outline,
}
cardo_ui_themes::impl_cardo_tw_variant!(BadgeVariant);

#[derive(Clone, Copy, Default)]
pub enum BadgeColor {
    #[default]
    Accent,
    Danger,
    Success,
    Neutral,
}

#[derive(TwVariant)]
pub enum BadgeSize {
    #[tw(class = "h-5 px-1.5 text-xs")]
    Sm,
    #[tw(default, class = "h-6 px-2 text-xs")]
    Md,
    #[tw(class = "h-7 px-2.5 text-sm")]
    Lg,
}
cardo_ui_themes::impl_cardo_tw_variant!(BadgeSize);

#[derive(TwClass)]
#[tw(class = "inline-flex items-center justify-center font-medium rounded-full whitespace-nowrap")]
pub struct BadgeClass {
    pub variant: BadgeVariant,
    pub size: BadgeSize,
}
cardo_ui_themes::impl_cardo_tw_class!(BadgeClass);

// ---------------------------------------------------------------------------
// Color override helpers
//
// BadgeVariant defines the *structural* variant (solid/soft/outline).
// The color scale override replaces accent-* with danger-*, success-*, etc.
// This is one approach — the rough edge is visible: we string-replace.
// A production system might use a more type-safe approach.
// ---------------------------------------------------------------------------

fn apply_color(class: &str, color: BadgeColor) -> String {
    match color {
        BadgeColor::Accent => class.to_string(),
        BadgeColor::Danger => class.replace("accent-", "danger-"),
        BadgeColor::Success => class.replace("accent-", "success-"),
        BadgeColor::Neutral => class.replace("accent-", "neutral-"),
    }
}

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

#[component]
pub fn Badge(
    #[prop(into, optional)] variant: BadgeVariant,
    #[prop(into, optional)] size: BadgeSize,
    #[prop(into, optional)] color: BadgeColor,
    children: Children,
) -> impl IntoView {
    let base = BadgeClass { variant, size }.to_class();
    let class = apply_color(&base, color);
    view! {
        <span class=class>{children()}</span>
    }
}
