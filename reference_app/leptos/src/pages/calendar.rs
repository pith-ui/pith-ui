use chrono::{Datelike, NaiveDate, Weekday};
use leptos::prelude::*;
use web_sys::wasm_bindgen::JsCast;

use cardo_ui::calendar::*;

#[component]
pub fn CalendarPage() -> impl IntoView {
    // ── State ────────────────────────────────────────────────

    let fixed_month = NaiveDate::from_ymd_opt(2024, 6, 1).unwrap();

    let (selected, set_selected) = signal::<Option<NaiveDate>>(None);
    let (month, set_month) = signal(fixed_month);
    let (disabled, set_disabled) = signal(false);
    let (read_only, set_read_only) = signal(false);
    let (week_start_mon, set_week_start_mon) = signal(false);
    let (fixed_weeks, set_fixed_weeks) = signal(false);
    let (disable_weekends, set_disable_weekends) = signal(false);
    let (unavailable_dates, set_unavailable_dates) = signal(false);
    let (use_min_max, set_use_min_max) = signal(false);

    let week_start = Signal::derive(move || {
        if week_start_mon.get() { Weekday::Mon } else { Weekday::Sun }
    });

    // Use `.get()` (not `.get_untracked()`) so that when these callbacks are
    // invoked inside reactive Signal::derive closures in the calendar cells,
    // Leptos tracks the toggle signals as dependencies and re-renders cells
    // when the toggle changes.
    let is_date_disabled_cb = Callback::new(move |date: NaiveDate| {
        if !disable_weekends.get() {
            return false;
        }
        matches!(date.weekday(), Weekday::Sat | Weekday::Sun)
    });

    let is_date_unavailable_cb = Callback::new(move |date: NaiveDate| {
        if !unavailable_dates.get() {
            return false;
        }
        date.day() == 10 || date.day() == 20
    });

    // min/max: Jun 5 – Jun 25 2024
    let min_date = Signal::derive(move || {
        use_min_max.get().then(|| NaiveDate::from_ymd_opt(2024, 6, 5).unwrap())
    });
    let max_date = Signal::derive(move || {
        use_min_max.get().then(|| NaiveDate::from_ymd_opt(2024, 6, 25).unwrap())
    });

    // ── Helpers ──────────────────────────────────────────────

    fn checkbox(label: &'static str, getter: Signal<bool>, setter: WriteSignal<bool>) -> AnyView {
        view! {
            <label>
                <input
                    type="checkbox"
                    prop:checked=move || getter.get()
                    on:change=move |ev| {
                        let target: web_sys::HtmlInputElement = ev.target().unwrap().unchecked_into();
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
        <h1>"Calendar"</h1>

        <Calendar
            class:calendar-root=true
            value=MaybeProp::derive(move || selected.get())
            on_value_change=Callback::new(move |d: NaiveDate| set_selected.set(Some(d)))
            month=month
            on_month_change=Callback::new(move |d: NaiveDate| set_month.set(d))
            disabled=disabled
            read_only=read_only
            week_start=week_start
            fixed_weeks=fixed_weeks
            min_date=MaybeProp::derive(move || min_date.get())
            max_date=MaybeProp::derive(move || max_date.get())
            is_date_disabled=is_date_disabled_cb
            is_date_unavailable=is_date_unavailable_cb
        >
            <CalendarHeader class:calendar-header=true>
                <CalendarPrevButton class:calendar-nav-button=true>
                    "◀"
                </CalendarPrevButton>
                <CalendarHeading class:calendar-heading=true />
                <CalendarNextButton class:calendar-nav-button=true>
                    "▶"
                </CalendarNextButton>
            </CalendarHeader>
            <CalendarGrid class:calendar-grid=true>
                <CalendarGridHead class:calendar-grid-head=true />
                <CalendarGridBody class:calendar-grid-body=true />
            </CalendarGrid>
        </Calendar>

        // ── Readouts ─────────────────────────────────────────
        <div class="readout">
            <span data-testid="selected-value">
                {move || selected.get().map(|d| d.to_string()).unwrap_or_else(|| "none".into())}
            </span>
            <br />
            <span data-testid="month-value">{move || month.get().to_string()}</span>
        </div>

        // ── Controls ─────────────────────────────────────────
        <div class="controls">
            {checkbox("disabled", disabled.into(), set_disabled)}
            {checkbox("read only", read_only.into(), set_read_only)}
            {checkbox("Monday start", week_start_mon.into(), set_week_start_mon)}
            {checkbox("fixed weeks", fixed_weeks.into(), set_fixed_weeks)}
            {checkbox("disable weekends", disable_weekends.into(), set_disable_weekends)}
            {checkbox("unavailable 10th/20th", unavailable_dates.into(), set_unavailable_dates)}
            {checkbox("min/max (Jun 5–25)", use_min_max.into(), set_use_min_max)}

            <button
                type="button"
                data-testid="reset"
                on:click=move |_| {
                    set_selected.set(None);
                    set_month.set(fixed_month);
                    set_disabled.set(false);
                    set_read_only.set(false);
                    set_week_start_mon.set(false);
                    set_fixed_weeks.set(false);
                    set_disable_weekends.set(false);
                    set_unavailable_dates.set(false);
                    set_use_min_max.set(false);
                }
            >
                "reset"
            </button>
        </div>
    }
}
