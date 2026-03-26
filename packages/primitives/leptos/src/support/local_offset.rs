//! Browser timezone offset utilities.
//!
//! Provides helpers to obtain the browser's current UTC offset via JS interop
//! and convert naive date/time values into timezone-aware `DateTime`s.
//!
//! Requires the `chrono` feature (enabled by `calendar` or `time-field`).

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};

/// Returns the browser's current UTC offset as a [`chrono::FixedOffset`].
///
/// Uses `js_sys::Date::getTimezoneOffset()` which returns minutes *west* of
/// UTC (e.g., US Eastern = 300 in winter, 240 in summer). This is converted
/// to chrono's *east-of-UTC* convention.
///
/// Falls back to UTC if the offset is out of range.
pub fn browser_utc_offset() -> FixedOffset {
    let date = js_sys::Date::new_0();
    // getTimezoneOffset() returns minutes west of UTC.
    let offset_minutes = date.get_timezone_offset() as i32;
    // chrono::FixedOffset::east_opt expects seconds east of UTC.
    FixedOffset::east_opt(-offset_minutes * 60).unwrap_or_else(|| {
        FixedOffset::east_opt(0).expect("UTC offset is always valid")
    })
}

/// Combines a [`NaiveDate`] and [`NaiveTime`] into a timezone-aware
/// [`DateTime`] using the browser's current UTC offset.
///
/// # Panics
///
/// Should not panic under normal circumstances. If the browser reports an
/// extreme offset that makes the local datetime invalid, falls back to
/// interpreting the naive datetime as UTC and converting.
pub fn to_local_datetime(date: NaiveDate, time: NaiveTime) -> DateTime<FixedOffset> {
    let naive = NaiveDateTime::new(date, time);
    let offset = browser_utc_offset();
    offset
        .from_local_datetime(&naive)
        .single()
        .unwrap_or_else(|| naive.and_utc().with_timezone(&offset))
}

/// Returns the current local date and time as a timezone-aware [`DateTime`]
/// using the browser's UTC offset.
pub fn local_now() -> DateTime<FixedOffset> {
    let now = js_sys::Date::new_0();
    let naive = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(
            now.get_full_year() as i32,
            now.get_month() + 1, // JS months are 0-based
            now.get_date(),
        )
        .expect("browser date should be valid"),
        NaiveTime::from_hms_milli_opt(
            now.get_hours(),
            now.get_minutes(),
            now.get_seconds(),
            now.get_milliseconds(),
        )
        .expect("browser time should be valid"),
    );
    let offset = browser_utc_offset();
    offset
        .from_local_datetime(&naive)
        .single()
        .unwrap_or_else(|| naive.and_utc().with_timezone(&offset))
}
