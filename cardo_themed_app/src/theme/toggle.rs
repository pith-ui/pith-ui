use cardo_ui::toggle::*;
use leptos::prelude::*;
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// Style definitions
// ---------------------------------------------------------------------------

#[derive(TwVariant)]
pub enum ToggleVariant {
    #[tw(default, class = "hover:bg-neutral-3 data-[state=on]:bg-neutral-4 data-[state=on]:text-neutral-12")]
    Ghost,
    #[tw(class = "border border-neutral-6 hover:bg-neutral-3 hover:border-neutral-7 data-[state=on]:bg-accent-3 data-[state=on]:border-accent-7 data-[state=on]:text-accent-11")]
    Outline,
}
cardo_ui_themes::impl_cardo_tw_variant!(ToggleVariant);

#[derive(TwVariant)]
pub enum ToggleSize {
    #[tw(class = "h-7 px-2 text-xs")]
    Sm,
    #[tw(default, class = "h-9 px-3 text-sm")]
    Md,
    #[tw(class = "h-11 px-4 text-base")]
    Lg,
}
cardo_ui_themes::impl_cardo_tw_variant!(ToggleSize);

#[derive(TwClass)]
#[tw(
    class = "inline-flex items-center justify-center font-medium rounded-2 text-neutral-11 bg-transparent transition-colors duration-normal data-[disabled]:opacity-50 data-[disabled]:pointer-events-none focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-focus-ring focus-visible:ring-offset-2 focus-visible:ring-offset-page"
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
