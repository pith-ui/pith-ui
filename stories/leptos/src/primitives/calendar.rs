use chrono::{Datelike, NaiveDate, Weekday};
use leptos::prelude::*;

use pith_ui::calendar::*;

stylance::import_crate_style!(classes, "src/primitives/calendar.stories.module.css");

// ── Stories ──────────────────────────────────────────────────────────

/// Basic uncontrolled calendar with default settings.
#[component]
pub fn Styled() -> impl IntoView {
    view! {
        <Calendar attr:class=classes::calendar>
            <CalendarHeader attr:class=classes::header>
                <CalendarPrevButton attr:class=classes::navButton>"◀"</CalendarPrevButton>
                <CalendarHeading attr:class=classes::heading />
                <CalendarNextButton attr:class=classes::navButton>"▶"</CalendarNextButton>
            </CalendarHeader>
            <CalendarGrid attr:class=classes::grid>
                <CalendarGridHead attr:class=classes::gridHead />
                <CalendarGridBody attr:class=classes::gridBody />
            </CalendarGrid>
        </Calendar>
    }
}

/// Controlled value — selected date and month tracked externally.
#[component]
pub fn Controlled() -> impl IntoView {
    let today = chrono::Local::now().date_naive();
    let (value, set_value) = signal(Some(today));
    let (month, set_month) =
        signal(NaiveDate::from_ymd_opt(today.year(), today.month(), 1).unwrap());

    view! {
        <p>
            "Selected: "
            {move || value.get().map(|d| d.to_string()).unwrap_or_else(|| "none".into())}
        </p>
        <p>"Month: " {move || month.get().to_string()}</p>

        <Calendar
            attr:class=classes::calendar
            value=MaybeProp::derive(move || value.get())
            on_value_change=Callback::new(move |d: NaiveDate| set_value.set(Some(d)))
            month=month
            on_month_change=Callback::new(move |d: NaiveDate| set_month.set(d))
        >
            <CalendarHeader attr:class=classes::header>
                <CalendarPrevButton attr:class=classes::navButton>"◀"</CalendarPrevButton>
                <CalendarHeading attr:class=classes::heading />
                <CalendarNextButton attr:class=classes::navButton>"▶"</CalendarNextButton>
            </CalendarHeader>
            <CalendarGrid attr:class=classes::grid>
                <CalendarGridHead attr:class=classes::gridHead />
                <CalendarGridBody attr:class=classes::gridBody />
            </CalendarGrid>
        </Calendar>

        <button on:click=move |_| set_value.set(None)>"Clear selection"</button>
    }
}

/// Chromatic — all visual states on one page for snapshot testing.
#[component]
pub fn Chromatic() -> impl IntoView {
    let fixed = NaiveDate::from_ymd_opt(2024, 3, 1).unwrap();

    view! {
        <h1>"Default (Sunday start)"</h1>
        <Calendar attr:class=classes::calendar default_month=fixed>
            <CalendarHeader attr:class=classes::header>
                <CalendarPrevButton attr:class=classes::navButton>"◀"</CalendarPrevButton>
                <CalendarHeading attr:class=classes::heading />
                <CalendarNextButton attr:class=classes::navButton>"▶"</CalendarNextButton>
            </CalendarHeader>
            <CalendarGrid attr:class=classes::grid>
                <CalendarGridHead attr:class=classes::gridHead />
                <CalendarGridBody attr:class=classes::gridBody />
            </CalendarGrid>
        </Calendar>

        <h1>"With selection"</h1>
        <Calendar attr:class=classes::calendar default_value=NaiveDate::from_ymd_opt(2024, 3, 15).unwrap() default_month=fixed>
            <CalendarHeader attr:class=classes::header>
                <CalendarPrevButton attr:class=classes::navButton>"◀"</CalendarPrevButton>
                <CalendarHeading attr:class=classes::heading />
                <CalendarNextButton attr:class=classes::navButton>"▶"</CalendarNextButton>
            </CalendarHeader>
            <CalendarGrid attr:class=classes::grid>
                <CalendarGridHead attr:class=classes::gridHead />
                <CalendarGridBody attr:class=classes::gridBody />
            </CalendarGrid>
        </Calendar>

        <h1>"Monday start"</h1>
        <Calendar attr:class=classes::calendar week_start=Weekday::Mon default_month=fixed>
            <CalendarHeader attr:class=classes::header>
                <CalendarPrevButton attr:class=classes::navButton>"◀"</CalendarPrevButton>
                <CalendarHeading attr:class=classes::heading />
                <CalendarNextButton attr:class=classes::navButton>"▶"</CalendarNextButton>
            </CalendarHeader>
            <CalendarGrid attr:class=classes::grid>
                <CalendarGridHead attr:class=classes::gridHead />
                <CalendarGridBody attr:class=classes::gridBody />
            </CalendarGrid>
        </Calendar>

        <h1>"Fixed weeks (6 rows)"</h1>
        <Calendar attr:class=classes::calendar fixed_weeks=true default_month=fixed>
            <CalendarHeader attr:class=classes::header>
                <CalendarPrevButton attr:class=classes::navButton>"◀"</CalendarPrevButton>
                <CalendarHeading attr:class=classes::heading />
                <CalendarNextButton attr:class=classes::navButton>"▶"</CalendarNextButton>
            </CalendarHeader>
            <CalendarGrid attr:class=classes::grid>
                <CalendarGridHead attr:class=classes::gridHead />
                <CalendarGridBody attr:class=classes::gridBody />
            </CalendarGrid>
        </Calendar>

        <h1>"Disabled"</h1>
        <Calendar attr:class=classes::calendar disabled=true default_month=fixed>
            <CalendarHeader attr:class=classes::header>
                <CalendarPrevButton attr:class=classes::navButton>"◀"</CalendarPrevButton>
                <CalendarHeading attr:class=classes::heading />
                <CalendarNextButton attr:class=classes::navButton>"▶"</CalendarNextButton>
            </CalendarHeader>
            <CalendarGrid attr:class=classes::grid>
                <CalendarGridHead attr:class=classes::gridHead />
                <CalendarGridBody attr:class=classes::gridBody />
            </CalendarGrid>
        </Calendar>

        <h1>"Min/Max constrained (Mar 5 – Mar 25)"</h1>
        <Calendar
            attr:class=classes::calendar
            default_month=fixed
            min_date=NaiveDate::from_ymd_opt(2024, 3, 5).unwrap()
            max_date=NaiveDate::from_ymd_opt(2024, 3, 25).unwrap()
        >
            <CalendarHeader attr:class=classes::header>
                <CalendarPrevButton attr:class=classes::navButton>"◀"</CalendarPrevButton>
                <CalendarHeading attr:class=classes::heading />
                <CalendarNextButton attr:class=classes::navButton>"▶"</CalendarNextButton>
            </CalendarHeader>
            <CalendarGrid attr:class=classes::grid>
                <CalendarGridHead attr:class=classes::gridHead />
                <CalendarGridBody attr:class=classes::gridBody />
            </CalendarGrid>
        </Calendar>

        <h1>"Disabled weekends"</h1>
        <Calendar
            attr:class=classes::calendar
            default_month=fixed
            is_date_disabled=Callback::new(|date: NaiveDate| {
                matches!(date.weekday(), Weekday::Sat | Weekday::Sun)
            })
        >
            <CalendarHeader attr:class=classes::header>
                <CalendarPrevButton attr:class=classes::navButton>"◀"</CalendarPrevButton>
                <CalendarHeading attr:class=classes::heading />
                <CalendarNextButton attr:class=classes::navButton>"▶"</CalendarNextButton>
            </CalendarHeader>
            <CalendarGrid attr:class=classes::grid>
                <CalendarGridHead attr:class=classes::gridHead />
                <CalendarGridBody attr:class=classes::gridBody />
            </CalendarGrid>
        </Calendar>

        <h1>"Unavailable dates (10th and 20th)"</h1>
        <Calendar
            attr:class=classes::calendar
            default_month=fixed
            is_date_unavailable=Callback::new(|date: NaiveDate| {
                date.day() == 10 || date.day() == 20
            })
        >
            <CalendarHeader attr:class=classes::header>
                <CalendarPrevButton attr:class=classes::navButton>"◀"</CalendarPrevButton>
                <CalendarHeading attr:class=classes::heading />
                <CalendarNextButton attr:class=classes::navButton>"▶"</CalendarNextButton>
            </CalendarHeader>
            <CalendarGrid attr:class=classes::grid>
                <CalendarGridHead attr:class=classes::gridHead />
                <CalendarGridBody attr:class=classes::gridBody />
            </CalendarGrid>
        </Calendar>

        <h1>"Read-only with selection"</h1>
        <Calendar attr:class=classes::calendar read_only=true default_value=NaiveDate::from_ymd_opt(2024, 3, 15).unwrap() default_month=fixed>
            <CalendarHeader attr:class=classes::header>
                <CalendarPrevButton attr:class=classes::navButton>"◀"</CalendarPrevButton>
                <CalendarHeading attr:class=classes::heading />
                <CalendarNextButton attr:class=classes::navButton>"▶"</CalendarNextButton>
            </CalendarHeader>
            <CalendarGrid attr:class=classes::grid>
                <CalendarGridHead attr:class=classes::gridHead />
                <CalendarGridBody attr:class=classes::gridBody />
            </CalendarGrid>
        </Calendar>
    }
}
