use cardo_ui::switch::*;
use leptos::prelude::*;
use tailwind_fuse::*;

// ---------------------------------------------------------------------------
// Style definitions
// ---------------------------------------------------------------------------

/// Public size prop for the themed switch.
#[derive(Clone, Copy, Default)]
pub enum SwitchSize {
    #[default]
    Sm,
    Md,
}

// Root styles per size
#[derive(TwVariant)]
pub enum SwitchRootSize {
    #[tw(default, class = "w-9 h-5")]
    Sm,
    #[tw(class = "w-11 h-6")]
    Md,
}

#[derive(TwClass)]
#[tw(
    class = "inline-flex items-center shrink-0 rounded-full p-0.5 transition-colors duration-normal bg-neutral-5 data-[state=checked]:bg-accent-9 data-[disabled]:opacity-50 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-focus-ring focus-visible:ring-offset-2 focus-visible:ring-offset-page"
)]
pub struct SwitchRootClass {
    pub size: SwitchRootSize,
}

// Thumb styles per size
#[derive(TwVariant)]
pub enum SwitchThumbSize {
    #[tw(default, class = "size-4 data-[state=checked]:translate-x-4")]
    Sm,
    #[tw(class = "size-5 data-[state=checked]:translate-x-5")]
    Md,
}

#[derive(TwClass)]
#[tw(class = "pointer-events-none block rounded-full bg-white shadow-1 transition-transform duration-normal")]
pub struct SwitchThumbClass {
    pub size: SwitchThumbSize,
}

// Map public SwitchSize to internal root/thumb sizes
impl SwitchSize {
    fn root_size(self) -> SwitchRootSize {
        match self {
            SwitchSize::Sm => SwitchRootSize::Sm,
            SwitchSize::Md => SwitchRootSize::Md,
        }
    }

    fn thumb_size(self) -> SwitchThumbSize {
        match self {
            SwitchSize::Sm => SwitchThumbSize::Sm,
            SwitchSize::Md => SwitchThumbSize::Md,
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
    let root_class = StoredValue::new(
        SwitchRootClass {
            size: size.root_size(),
        }
        .to_class(),
    );
    let thumb_class = StoredValue::new(
        SwitchThumbClass {
            size: size.thumb_size(),
        }
        .to_class(),
    );

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
