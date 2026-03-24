use cardo_ui::slider::{Slider, SliderRange, SliderThumb, SliderTrack};
use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui new-york slider
// ---------------------------------------------------------------------------

const SLIDER_CLASS: &str = "relative flex w-full touch-none items-center select-none data-[disabled]:opacity-50 data-[orientation=horizontal]:h-5 data-[orientation=vertical]:h-full data-[orientation=vertical]:min-h-44 data-[orientation=vertical]:w-auto data-[orientation=vertical]:flex-col";

const TRACK_CLASS: &str = "relative grow overflow-hidden rounded-full bg-muted data-[orientation=horizontal]:h-1.5 data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-full data-[orientation=vertical]:w-1.5";

const RANGE_CLASS: &str = "absolute bg-primary data-[orientation=horizontal]:h-full data-[orientation=vertical]:w-full";

const THUMB_CLASS: &str = "block size-4 shrink-0 rounded-full border border-primary bg-background shadow-sm ring-ring/50 transition-[color,box-shadow] hover:ring-4 focus-visible:ring-4 focus-visible:outline-hidden disabled:disabled-base";

type SliderValues = Vec<f64>;

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

#[component]
pub fn ThemedSlider(
    #[prop(into, optional)] value: MaybeProp<Vec<f64>>,
    #[prop(into, optional)] default_value: MaybeProp<Vec<f64>>,
    #[prop(into, optional)] on_value_change: Option<Callback<Vec<f64>>>,
    #[prop(into, optional)] on_value_commit: Option<Callback<Vec<f64>>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] min: MaybeProp<f64>,
    #[prop(into, optional)] max: MaybeProp<f64>,
    #[prop(into, optional)] step: MaybeProp<f64>,
    #[prop(into, optional)] name: MaybeProp<String>,
) -> impl IntoView {
    let track_class = StoredValue::new(TRACK_CLASS);
    let range_class = StoredValue::new(RANGE_CLASS);
    let thumb_class = StoredValue::new(THUMB_CLASS);

    let handle_value_change = move |val: SliderValues| {
        if let Some(cb) = on_value_change {
            cb.run(val);
        }
    };

    let handle_value_commit = move |val: SliderValues| {
        if let Some(cb) = on_value_commit {
            cb.run(val);
        }
    };

    view! {
        <Slider
            attr:class=SLIDER_CLASS
            value=value
            default_value=default_value
            on_value_change=handle_value_change
            on_value_commit=handle_value_commit
            disabled=disabled
            min=min
            max=max
            step=step
            name=name
        >
            <SliderTrack attr:class=track_class.get_value()>
                <SliderRange attr:class=range_class.get_value() />
            </SliderTrack>
            <SliderThumb attr:class=thumb_class.get_value() />
        </Slider>
    }
}
