use chrono::NaiveTime;
use leptos::prelude::*;
use web_sys::wasm_bindgen::JsCast;

use pith_ui::time_field::*;

#[component]
pub fn TimeFieldPage() -> impl IntoView {
    // ── State ────────────────────────────────────────────────

    let (time, set_time) = signal::<Option<NaiveTime>>(None);
    let (disabled, set_disabled) = signal(false);
    let (read_only, set_read_only) = signal(false);
    let (use_h24, set_use_h24) = signal(false);
    let (use_seconds, set_use_seconds) = signal(false);

    let hour_cycle = Signal::derive(move || {
        if use_h24.get() {
            HourCycle::H24
        } else {
            HourCycle::H12
        }
    });

    let granularity = Signal::derive(move || {
        if use_seconds.get() {
            TimeGranularity::Second
        } else {
            TimeGranularity::Minute
        }
    });

    // ── Helpers ──────────────────────────────────────────────

    fn checkbox(
        label: &'static str,
        getter: Signal<bool>,
        setter: WriteSignal<bool>,
    ) -> AnyView {
        view! {
            <label>
                <input
                    type="checkbox"
                    prop:checked=move || getter.get()
                    on:change=move |ev| {
                        let target: web_sys::HtmlInputElement =
                            ev.target().unwrap().unchecked_into();
                        setter.set(target.checked());
                    }
                />
                {format!(" {}", label)}
            </label>
        }
        .into_any()
    }

    // ── View ─────────────────────────────────────────────────

    view! {
        <h1>"TimeField"</h1>

        <div class="time-field-root">
            <TimeField
                value=MaybeProp::derive(move || time.get())
                on_value_change=Callback::new(move |t: NaiveTime| set_time.set(Some(t)))
                hour_cycle=hour_cycle
                granularity=granularity
                disabled=disabled
                read_only=read_only
                name="meeting_time"
            >
                <TimeFieldInput
                    class:time-field-input=true
                    aria_label="Time"
                />
            </TimeField>
        </div>

        // ── Readout ─────────────────────────────────────────
        <div class="readout">
            <span data-testid="time-value">
                {move || {
                    time.get()
                        .map(|t| t.format("%H:%M:%S").to_string())
                        .unwrap_or_else(|| "none".to_string())
                }}
            </span>
        </div>

        // ── Controls ────────────────────────────────────────
        <div class="controls">
            {checkbox("disabled", disabled.into(), set_disabled)}
            {checkbox("read only", read_only.into(), set_read_only)}
            {checkbox("24-hour", use_h24.into(), set_use_h24)}
            {checkbox("seconds", use_seconds.into(), set_use_seconds)}

            <button
                type="button"
                data-testid="reset"
                on:click=move |_| {
                    set_time.set(None);
                    set_disabled.set(false);
                    set_read_only.set(false);
                    set_use_h24.set(false);
                    set_use_seconds.set(false);
                }
            >
                "reset"
            </button>
        </div>
    }
}
