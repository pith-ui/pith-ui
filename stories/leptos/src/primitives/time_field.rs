use chrono::NaiveTime;
use leptos::prelude::*;

use pith_ui::time_field::*;

stylance::import_crate_style!(classes, "src/primitives/time_field.stories.module.css");

// ── Stories ──────────────────────────────────────────────────────────

/// Interactive time field with controlled value and format options.
#[component]
pub fn Styled() -> impl IntoView {
    let (time, set_time) = signal::<Option<NaiveTime>>(None);

    view! {
        <label class=classes::label>"Meeting time"</label>
        <TimeField
            value=MaybeProp::derive(move || time.get())
            on_value_change=Callback::new(move |t: NaiveTime| set_time.set(Some(t)))
        >
            <TimeFieldInput attr:class=classes::field aria_label="Meeting time" />
        </TimeField>

        <div class=classes::readout>
            {move || match time.get() {
                Some(t) => format!("Value: {}", t.format("%I:%M %p")),
                None => "No time selected".to_string(),
            }}
        </div>
    }
}

/// 24-hour format with second granularity.
#[component]
pub fn TwentyFourHour() -> impl IntoView {
    let (time, set_time) = signal::<Option<NaiveTime>>(None);

    view! {
        <label class=classes::label>"Departure (24h)"</label>
        <TimeField
            value=MaybeProp::derive(move || time.get())
            on_value_change=Callback::new(move |t: NaiveTime| set_time.set(Some(t)))
            hour_cycle=HourCycle::H24
            granularity=TimeGranularity::Second
        >
            <TimeFieldInput attr:class=classes::field aria_label="Departure time" />
        </TimeField>

        <div class=classes::readout>
            {move || match time.get() {
                Some(t) => format!("Value: {}", t.format("%H:%M:%S")),
                None => "No time selected".to_string(),
            }}
        </div>
    }
}

/// Pre-filled with a default value.
#[component]
pub fn WithDefault() -> impl IntoView {
    let default = NaiveTime::from_hms_opt(14, 30, 0).unwrap();
    let (time, set_time) = signal(Some(default));

    view! {
        <label class=classes::label>"Reminder"</label>
        <TimeField
            value=MaybeProp::derive(move || time.get())
            on_value_change=Callback::new(move |t: NaiveTime| set_time.set(Some(t)))
        >
            <TimeFieldInput attr:class=classes::field aria_label="Reminder time" />
        </TimeField>

        <div class=classes::readout>
            {move || match time.get() {
                Some(t) => format!("Value: {}", t.format("%I:%M %p")),
                None => "No time selected".to_string(),
            }}
        </div>

        <button style="margin-top: 8px;" on:click=move |_| set_time.set(None)>
            "Clear"
        </button>
    }
}

/// Disabled state.
#[component]
pub fn Disabled() -> impl IntoView {
    view! {
        <label class=classes::label>"Disabled"</label>
        <TimeField
            default_value=NaiveTime::from_hms_opt(9, 0, 0).unwrap()
            disabled=true
        >
            <TimeFieldInput attr:class=classes::field aria_label="Disabled time" />
        </TimeField>
    }
}

/// Read-only state.
#[component]
pub fn ReadOnly() -> impl IntoView {
    view! {
        <label class=classes::label>"Read-only"</label>
        <TimeField
            default_value=NaiveTime::from_hms_opt(17, 45, 0).unwrap()
            read_only=true
        >
            <TimeFieldInput attr:class=classes::field aria_label="Read-only time" />
        </TimeField>
    }
}

/// All variants on one page.
#[component]
pub fn Chromatic() -> impl IntoView {
    view! {
        <h2>"12-hour (default)"</h2>
        <Styled />

        <h2>"24-hour with seconds"</h2>
        <TwentyFourHour />

        <h2>"With default value"</h2>
        <WithDefault />

        <h2>"Disabled"</h2>
        <Disabled />

        <h2>"Read-only"</h2>
        <ReadOnly />
    }
}
