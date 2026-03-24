use cardo_ui::switch::*;
use leptos::prelude::*;
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui new-york switch
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Default)]
pub enum SwitchSize {
    #[default]
    Default,
    Sm,
}

#[derive(TwVariant)]
enum SwitchRootSize {
    #[tw(default, class = "h-[1.15rem] w-8")]
    Default,
    #[tw(class = "h-3.5 w-6")]
    Sm,
}

#[derive(TwClass)]
#[tw(
    class = "peer inline-flex shrink-0 items-center rounded-full border border-transparent shadow-xs transition-all outline-none focus-visible:focus-ring disabled:disabled-cursor data-[state=checked]:bg-primary data-[state=unchecked]:bg-input dark:data-[state=unchecked]:bg-input/80"
)]
struct SwitchRootClass {
    size: SwitchRootSize,
}

#[derive(TwVariant)]
enum SwitchThumbSize {
    #[tw(default, class = "size-4")]
    Default,
    #[tw(class = "size-3")]
    Sm,
}

#[derive(TwClass)]
#[tw(
    class = "pointer-events-none block rounded-full bg-background ring-0 transition-transform data-[state=checked]:translate-x-[calc(100%-2px)] data-[state=unchecked]:translate-x-0 dark:data-[state=checked]:bg-primary-foreground dark:data-[state=unchecked]:bg-foreground"
)]
struct SwitchThumbClass {
    size: SwitchThumbSize,
}

impl SwitchSize {
    fn root(self) -> SwitchRootSize {
        match self {
            SwitchSize::Default => SwitchRootSize::Default,
            SwitchSize::Sm => SwitchRootSize::Sm,
        }
    }

    fn thumb(self) -> SwitchThumbSize {
        match self {
            SwitchSize::Default => SwitchThumbSize::Default,
            SwitchSize::Sm => SwitchThumbSize::Sm,
        }
    }
}

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

#[component]
pub fn ThemedSwitch(
    #[prop(into, optional)] size: SwitchSize,
    #[prop(into, optional)] checked: MaybeProp<bool>,
    #[prop(into, optional)] default_checked: MaybeProp<bool>,
    #[prop(into, optional)] on_checked_change: Option<Callback<bool>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] name: MaybeProp<String>,
) -> impl IntoView {
    let root_class = StoredValue::new(SwitchRootClass { size: size.root() }.to_class());
    let thumb_class = StoredValue::new(SwitchThumbClass { size: size.thumb() }.to_class());

    view! {
        <Switch
            attr:class=root_class.get_value()
            checked=checked
            default_checked=default_checked
            on_checked_change=move |state: bool| {
                if let Some(cb) = on_checked_change {
                    cb.run(state);
                }
            }
            disabled=disabled
            name=name
        >
            <SwitchThumb attr:class=thumb_class.get_value() />
        </Switch>
    }
}
