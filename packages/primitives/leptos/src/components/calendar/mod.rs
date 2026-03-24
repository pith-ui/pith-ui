//! Accessible calendar grid for date selection.
//!
//! An unstyled calendar primitive that displays a month view with keyboard
//! navigation, following the [WAI-ARIA Grid pattern](https://www.w3.org/WAI/ARIA/apg/patterns/grid/).
//!
//! # Anatomy
//!
//! ```text
//! <Calendar>
//!     <CalendarHeader>
//!         <CalendarPrevButton>"◀"</CalendarPrevButton>
//!         <CalendarHeading />
//!         <CalendarNextButton>"▶"</CalendarNextButton>
//!     </CalendarHeader>
//!     <CalendarGrid>
//!         <CalendarGridHead />
//!         <CalendarGridBody />
//!     </CalendarGrid>
//! </Calendar>
//! ```
//!
//! # Features
//!
//! - Single date selection (controlled or uncontrolled)
//! - Full keyboard navigation (arrows, Page Up/Down, Home/End)
//! - Configurable first day of week
//! - Min/max date bounds
//! - Custom disabled and unavailable date predicates
//! - ARIA grid pattern with roving tabindex
//! - Data attributes for styling
//!
//! # Keyboard Interactions
//!
//! | Key | Action |
//! |-----|--------|
//! | Arrow Right | Next day |
//! | Arrow Left | Previous day |
//! | Arrow Down | Same day next week |
//! | Arrow Up | Same day previous week |
//! | Page Down | Same day next month |
//! | Page Up | Same day previous month |
//! | Shift + Page Down | Same day next year |
//! | Shift + Page Up | Same day previous year |
//! | Home | First day of week |
//! | End | Last day of week |
//! | Enter / Space | Select focused date |
//!
//! # Data Attributes
//!
//! **Cell trigger (button):**
//!
//! | Attribute | Values |
//! |-----------|--------|
//! | `data-date` | ISO date string (YYYY-MM-DD) |
//! | `data-selected` | Present when selected |
//! | `data-today` | Present when today |
//! | `data-disabled` | Present when disabled |
//! | `data-unavailable` | Present when unavailable |
//! | `data-outside-month` | Present when outside the displayed month |

mod calendar;
mod calendar_grid;

pub use calendar::*;
pub use calendar_grid::*;

// Re-export chrono types used in the public API.
pub use chrono::NaiveDate;
pub use chrono::Weekday;

use chrono::{Datelike, TimeDelta};
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;

// ── Context ──────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
pub(crate) struct CalendarContextValue {
    pub(crate) value: Signal<Option<NaiveDate>>,
    pub(crate) set_value: Callback<Option<NaiveDate>>,
    pub(crate) month: Signal<NaiveDate>,
    pub(crate) set_month: Callback<Option<NaiveDate>>,
    pub(crate) focused_date: RwSignal<NaiveDate>,
    pub(crate) min_date: Signal<Option<NaiveDate>>,
    pub(crate) max_date: Signal<Option<NaiveDate>>,
    pub(crate) disabled: Signal<bool>,
    pub(crate) read_only: Signal<bool>,
    pub(crate) is_date_disabled: Option<Callback<NaiveDate, bool>>,
    pub(crate) is_date_unavailable: Option<Callback<NaiveDate, bool>>,
    pub(crate) week_start: Signal<Weekday>,
    pub(crate) fixed_weeks: Signal<bool>,
    pub(crate) today: NaiveDate,
    pub(crate) heading_id: ReadSignal<String>,
    pub(crate) grid_ref: AnyNodeRef,
}

// ── Grid computation ─────────────────────────────────────────────────

/// Computes calendar weeks for a given month.
///
/// Returns a `Vec` of week rows, each containing exactly 7 dates.
/// Includes leading days from the previous month and trailing days from the
/// next month to fill complete weeks.
pub(crate) fn compute_calendar_weeks(
    year: i32,
    month: u32,
    week_start: Weekday,
    fixed_weeks: bool,
) -> Vec<[NaiveDate; 7]> {
    let first_of_month = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let last_of_month = last_day_of_month(year, month);

    // How many days before the 1st to align with week_start.
    let offset = (first_of_month.weekday().num_days_from_monday() as i32
        - week_start.num_days_from_monday() as i32
        + 7)
        % 7;
    let grid_start = first_of_month - TimeDelta::days(offset as i64);

    let min_weeks: usize = if fixed_weeks { 6 } else { 1 };
    let mut weeks = Vec::with_capacity(6);
    let mut day = grid_start;

    loop {
        let week: [NaiveDate; 7] = std::array::from_fn(|i| day + TimeDelta::days(i as i64));
        day += TimeDelta::days(7);
        weeks.push(week);

        if day > last_of_month && weeks.len() >= min_weeks {
            break;
        }
    }

    weeks
}

fn last_day_of_month(year: i32, month: u32) -> NaiveDate {
    let (next_year, next_month) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };
    NaiveDate::from_ymd_opt(next_year, next_month, 1)
        .unwrap()
        .pred_opt()
        .unwrap()
}

// ── Date arithmetic ──────────────────────────────────────────────────

pub(crate) fn add_months(date: NaiveDate, months: i32) -> NaiveDate {
    let total_months = date.year() * 12 + date.month() as i32 - 1 + months;
    let new_year = total_months.div_euclid(12);
    let new_month = (total_months.rem_euclid(12) + 1) as u32;
    let max_day = last_day_of_month(new_year, new_month).day();
    let new_day = date.day().min(max_day);
    NaiveDate::from_ymd_opt(new_year, new_month, new_day).unwrap()
}

pub(crate) fn start_of_week(date: NaiveDate, week_start: Weekday) -> NaiveDate {
    let offset = (date.weekday().num_days_from_monday() as i32
        - week_start.num_days_from_monday() as i32
        + 7)
        % 7;
    date - TimeDelta::days(offset as i64)
}

pub(crate) fn end_of_week(date: NaiveDate, week_start: Weekday) -> NaiveDate {
    start_of_week(date, week_start) + TimeDelta::days(6)
}

pub(crate) fn clamp_to_range(
    date: NaiveDate,
    min: Option<NaiveDate>,
    max: Option<NaiveDate>,
) -> NaiveDate {
    let mut d = date;
    if let Some(min) = min {
        d = d.max(min);
    }
    if let Some(max) = max {
        d = d.min(max);
    }
    d
}

// ── Cell state helpers ───────────────────────────────────────────────

/// Check disabled state for use in **event handlers** (non-reactive context).
/// Wraps the user callback in `untrack()` to avoid reactive tracking warnings.
pub(crate) fn cell_is_disabled(date: NaiveDate, ctx: CalendarContextValue) -> bool {
    if let Some(min) = ctx.min_date.get_untracked() {
        if date < min {
            return true;
        }
    }
    if let Some(max) = ctx.max_date.get_untracked() {
        if date > max {
            return true;
        }
    }
    untrack(|| ctx.is_date_disabled.map(|f| f.run(date)).unwrap_or(false))
}

/// Check unavailable state for use in **event handlers** (non-reactive context).
pub(crate) fn cell_is_unavailable(date: NaiveDate, ctx: CalendarContextValue) -> bool {
    untrack(|| ctx.is_date_unavailable.map(|f| f.run(date)).unwrap_or(false))
}

/// Determines which cell should be the tab target (tabindex="0") within
/// the visible month grid. Falls through a priority chain so there is
/// always exactly one tabbable cell.
pub(crate) fn is_tab_target(date: NaiveDate, ctx: CalendarContextValue) -> bool {
    let month = ctx.month.get();
    let in_month =
        |d: NaiveDate| d.month() == month.month() && d.year() == month.year();

    // 1. Focused date if it is in the visible month.
    let focused = ctx.focused_date.get();
    if in_month(focused) {
        return date == focused;
    }

    // 2. Selected value if visible.
    if let Some(selected) = ctx.value.get() {
        if in_month(selected) {
            return date == selected;
        }
    }

    // 3. Today if visible.
    if in_month(ctx.today) {
        return date == ctx.today;
    }

    // 4. First day of the displayed month.
    date == month
}

// ── Weekday helpers ──────────────────────────────────────────────────

pub(crate) fn ordered_weekdays(week_start: Weekday) -> [Weekday; 7] {
    let mut days = [week_start; 7];
    for i in 1..7 {
        days[i] = days[i - 1].succ();
    }
    days
}

pub(crate) fn weekday_short(wd: Weekday) -> &'static str {
    match wd {
        Weekday::Mon => "Mo",
        Weekday::Tue => "Tu",
        Weekday::Wed => "We",
        Weekday::Thu => "Th",
        Weekday::Fri => "Fr",
        Weekday::Sat => "Sa",
        Weekday::Sun => "Su",
    }
}

pub(crate) fn weekday_long(wd: Weekday) -> &'static str {
    match wd {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
        Weekday::Sun => "Sunday",
    }
}

pub(crate) fn month_name(month: u32) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => unreachable!(),
    }
}

pub(crate) fn format_date_label(date: NaiveDate) -> String {
    format!(
        "{}, {} {}, {}",
        weekday_long(date.weekday()),
        month_name(date.month()),
        date.day(),
        date.year()
    )
}

// ── Focus helpers ────────────────────────────────────────────────────

/// Defers focus to a cell via `queueMicrotask`, giving Leptos time to
/// reconcile the DOM after signal updates.
pub(crate) fn queue_focus_date(grid_ref: AnyNodeRef, date: NaiveDate) {
    use web_sys::wasm_bindgen::{JsCast, closure::Closure};

    let date_str = date.to_string();
    let cb = Closure::once_into_js(move || {
        // untrack: this runs in a queueMicrotask callback, outside any reactive scope.
        if let Some(grid) = untrack(|| grid_ref.get()) {
            let el: &web_sys::Element = grid.unchecked_ref();
            let selector = format!("button[data-date='{}']", date_str);
            if let Ok(Some(button)) = el.query_selector(&selector) {
                let button: web_sys::HtmlElement = button.unchecked_into();
                let _ = button.focus();
            }
        }
    });
    web_sys::window()
        .expect("Window should exist.")
        .queue_microtask(cb.unchecked_ref());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn last_day_feb_leap() {
        assert_eq!(last_day_of_month(2024, 2), NaiveDate::from_ymd_opt(2024, 2, 29).unwrap());
    }

    #[test]
    fn last_day_feb_non_leap() {
        assert_eq!(last_day_of_month(2023, 2), NaiveDate::from_ymd_opt(2023, 2, 28).unwrap());
    }

    #[test]
    fn last_day_december() {
        assert_eq!(last_day_of_month(2024, 12), NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
    }

    #[test]
    fn weeks_feb_2015_sunday_start() {
        // Feb 2015 starts on Sunday, 28 days — exactly 4 weeks with Sunday start.
        let weeks = compute_calendar_weeks(2015, 2, Weekday::Sun, false);
        assert_eq!(weeks.len(), 4);
        assert_eq!(weeks[0][0], NaiveDate::from_ymd_opt(2015, 2, 1).unwrap());
        assert_eq!(weeks[3][6], NaiveDate::from_ymd_opt(2015, 2, 28).unwrap());
    }

    #[test]
    fn weeks_fixed_always_six() {
        let weeks = compute_calendar_weeks(2015, 2, Weekday::Sun, true);
        assert_eq!(weeks.len(), 6);
    }

    #[test]
    fn weeks_march_sunday_start() {
        let weeks = compute_calendar_weeks(2024, 3, Weekday::Sun, false);
        // March 2024 starts Friday. With Sunday start, grid starts Feb 25.
        assert_eq!(weeks[0][0], NaiveDate::from_ymd_opt(2024, 2, 25).unwrap());
        // Must cover Mar 31 (Sunday).
        let last_week = weeks.last().unwrap();
        assert!(last_week.iter().any(|d| *d == NaiveDate::from_ymd_opt(2024, 3, 31).unwrap()));
    }

    #[test]
    fn add_months_basic() {
        let d = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
        assert_eq!(add_months(d, 1), NaiveDate::from_ymd_opt(2024, 2, 15).unwrap());
        assert_eq!(add_months(d, 12), NaiveDate::from_ymd_opt(2025, 1, 15).unwrap());
    }

    #[test]
    fn add_months_clamps_day() {
        let d = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();
        assert_eq!(add_months(d, 1), NaiveDate::from_ymd_opt(2024, 2, 29).unwrap());
    }

    #[test]
    fn add_months_negative() {
        let d = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
        assert_eq!(add_months(d, -1), NaiveDate::from_ymd_opt(2024, 2, 15).unwrap());
    }

    #[test]
    fn start_of_week_sunday() {
        let d = NaiveDate::from_ymd_opt(2024, 3, 13).unwrap(); // Wednesday
        let start = start_of_week(d, Weekday::Sun);
        assert_eq!(start, NaiveDate::from_ymd_opt(2024, 3, 10).unwrap()); // Sunday
    }

    #[test]
    fn start_of_week_monday() {
        let d = NaiveDate::from_ymd_opt(2024, 3, 13).unwrap(); // Wednesday
        let start = start_of_week(d, Weekday::Mon);
        assert_eq!(start, NaiveDate::from_ymd_opt(2024, 3, 11).unwrap()); // Monday
    }

    #[test]
    fn end_of_week_sunday_start() {
        let d = NaiveDate::from_ymd_opt(2024, 3, 13).unwrap(); // Wednesday
        let end = end_of_week(d, Weekday::Sun);
        assert_eq!(end, NaiveDate::from_ymd_opt(2024, 3, 16).unwrap()); // Saturday
    }

    #[test]
    fn ordered_weekdays_sunday() {
        let days = ordered_weekdays(Weekday::Sun);
        assert_eq!(days[0], Weekday::Sun);
        assert_eq!(days[1], Weekday::Mon);
        assert_eq!(days[6], Weekday::Sat);
    }

    #[test]
    fn ordered_weekdays_monday() {
        let days = ordered_weekdays(Weekday::Mon);
        assert_eq!(days[0], Weekday::Mon);
        assert_eq!(days[6], Weekday::Sun);
    }

    #[test]
    fn format_label() {
        let d = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(); // Friday
        assert_eq!(format_date_label(d), "Friday, March 15, 2024");
    }
}
