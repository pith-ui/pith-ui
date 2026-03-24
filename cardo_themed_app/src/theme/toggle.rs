use cardo_ui::toggle::*;
use leptos::prelude::*;
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui new-york toggle
// ---------------------------------------------------------------------------

#[derive(TwVariant)]
pub enum ToggleVariant {
    #[tw(default, class = "bg-transparent")]
    Default,
    #[tw(class = "border border-input bg-transparent shadow-xs hover:bg-accent hover:text-accent-foreground")]
    Outline,
}
cardo_ui_themes::impl_cardo_tw_variant!(ToggleVariant);

#[derive(TwVariant)]
pub enum ToggleSize {
    #[tw(class = "h-8 min-w-8 px-1.5")]
    Sm,
    #[tw(default, class = "h-9 min-w-9 px-2")]
    Default,
    #[tw(class = "h-10 min-w-10 px-2.5")]
    Lg,
}
cardo_ui_themes::impl_cardo_tw_variant!(ToggleSize);

#[derive(TwClass)]
#[tw(
    class = "inline-flex items-center justify-center gap-2 rounded-md text-sm font-medium whitespace-nowrap transition-[color,box-shadow] outline-none hover:bg-muted hover:text-muted-foreground focus-visible:focus-ring disabled:pointer-events-none disabled:opacity-50 data-[state=on]:bg-accent data-[state=on]:text-accent-foreground"
)]
pub struct ToggleClass {
    pub variant: ToggleVariant,
    pub size: ToggleSize,
}
cardo_ui_themes::impl_cardo_tw_class!(ToggleClass);

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

#[component]
pub fn ThemedToggle(
    #[prop(into, optional)] variant: ToggleVariant,
    #[prop(into, optional)] size: ToggleSize,
    #[prop(into, optional)] pressed: MaybeProp<bool>,
    #[prop(into, optional)] default_pressed: MaybeProp<bool>,
    #[prop(into, optional)] on_pressed_change: Option<Callback<bool>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let class = ToggleClass { variant, size }.to_class();

    view! {
        <Toggle
            attr:class=class
            pressed=pressed
            default_pressed=default_pressed
            on_pressed_change=move |state: bool| {
                if let Some(cb) = on_pressed_change {
                    cb.run(state);
                }
            }
            disabled=disabled
        >
            {children()}
        </Toggle>
    }
}
