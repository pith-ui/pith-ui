use cardo_ui::progress::{Progress, ProgressIndicator};
use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui new-york progress
// ---------------------------------------------------------------------------

const PROGRESS_CLASS: &str = "relative h-2 w-full overflow-hidden rounded-full bg-primary/20";
const INDICATOR_CLASS: &str = "h-full w-full flex-1 bg-primary transition-all";

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

#[component]
pub fn ThemedProgress(
    #[prop(into, optional)] value: MaybeProp<f64>,
    #[prop(into, optional)] max: MaybeProp<f64>,
) -> impl IntoView {
    let indicator_class = StoredValue::new(INDICATOR_CLASS);

    view! {
        <Progress
            attr:class=PROGRESS_CLASS
            value=value
            max=max
        >
            <ProgressIndicator
                attr:class=indicator_class.get_value()
                attr:style=move || {
                    let max_val = max.get().unwrap_or(100.0);
                    let pct = value.get().map(|v| (v / max_val) * 100.0).unwrap_or(0.0);
                    format!("transform: translateX(-{}%)", 100.0 - pct)
                }
            />
        </Progress>
    }
}
