//! Accessible segmented time input field.
//!
//! An unstyled time field primitive where each time part (hour, minute,
//! second, AM/PM) is an independently focusable `role="spinbutton"` element,
//! following the [WAI-ARIA Spinbutton pattern](https://www.w3.org/WAI/ARIA/apg/patterns/spinbutton/).
//!
//! # Anatomy
//!
//! ```text
//! <TimeField>
//!     <TimeFieldInput />
//! </TimeField>
//! ```
//!
//! # Features
//!
//! - Segmented spinbutton input (hour, minute, second, AM/PM)
//! - 12-hour and 24-hour format support
//! - Configurable granularity (hour, minute, second)
//! - Full keyboard navigation (arrows, digits, Tab)
//! - Controlled and uncontrolled usage
//! - Min/max time bounds
//! - Hidden form input for native form submission
//! - Data attributes for styling
//!
//! # Keyboard Interactions
//!
//! | Key | Action |
//! |-----|--------|
//! | Arrow Up | Increment focused segment |
//! | Arrow Down | Decrement focused segment |
//! | Arrow Left | Focus previous segment |
//! | Arrow Right | Focus next segment |
//! | Tab | Focus next segment (or exit field) |
//! | Shift+Tab | Focus previous segment (or exit field) |
//! | 0–9 | Type numeric value with auto-advance |
//! | A / P | Set AM/PM (12-hour mode) |
//! | Backspace / Delete | Clear segment to placeholder |
//! | Home | Set segment to minimum value |
//! | End | Set segment to maximum value |
//!
//! # Data Attributes
//!
//! **Segment (spinbutton):**
//!
//! | Attribute | Values |
//! |-----------|--------|
//! | `data-type` | `"hour"`, `"minute"`, `"second"`, `"day-period"`, `"literal"` |
//! | `data-placeholder` | Present when segment has no value |
//! | `data-disabled` | Present when disabled |
//! | `data-readonly` | Present when read-only |
//! | `data-focused` | Present when segment has focus |

mod time_field;

pub use time_field::*;

// Re-export chrono types used in the public API.
pub use chrono::NaiveTime;

use chrono::Timelike;
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;

// ── Public types ─────────────────────────────────────────────────────

/// Whether to use 12-hour or 24-hour time format.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HourCycle {
    /// 12-hour format with AM/PM.
    #[default]
    H12,
    /// 24-hour format.
    H24,
}

/// Controls which time segments are displayed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum TimeGranularity {
    /// Show only hours (and AM/PM if H12).
    Hour,
    /// Show hours and minutes (default).
    #[default]
    Minute,
    /// Show hours, minutes, and seconds.
    Second,
}

// ── Internal types ───────────────────────────────────────────────────

/// The type of a segment in the time field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SegmentType {
    Hour,
    Minute,
    Second,
    DayPeriod,
    Literal,
}

impl SegmentType {
    pub(crate) fn data_attr(self) -> &'static str {
        match self {
            Self::Hour => "hour",
            Self::Minute => "minute",
            Self::Second => "second",
            Self::DayPeriod => "day-period",
            Self::Literal => "literal",
        }
    }

    pub(crate) fn aria_label(self) -> &'static str {
        match self {
            Self::Hour => "hour",
            Self::Minute => "minute",
            Self::Second => "second",
            Self::DayPeriod => "AM/PM",
            Self::Literal => "",
        }
    }
}

/// Description of one segment in the time field layout.
#[derive(Clone, Copy, Debug)]
pub(crate) struct SegmentInfo {
    pub segment_type: SegmentType,
    /// For literals, the display character(s).
    pub literal: &'static str,
    /// Index into the editable segment list (None for literals).
    pub editable_index: Option<usize>,
}

// ── Context ──────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
#[allow(dead_code)] // min_value, max_value, required reserved for future validation
pub(crate) struct TimeFieldContextValue {
    pub hour_cycle: Signal<HourCycle>,
    pub granularity: Signal<TimeGranularity>,
    pub disabled: Signal<bool>,
    pub read_only: Signal<bool>,
    pub required: Signal<bool>,
    pub min_value: Signal<Option<NaiveTime>>,
    pub max_value: Signal<Option<NaiveTime>>,

    // Per-segment values: None = placeholder state.
    pub hour: RwSignal<Option<u32>>,
    pub minute: RwSignal<Option<u32>>,
    pub second: RwSignal<Option<u32>>,
    pub day_period: RwSignal<Option<u32>>, // 0=AM, 1=PM

    /// Accumulated digit buffer for the currently focused segment.
    pub digit_buffer: RwSignal<String>,

    /// Ref to the input container (role="group") for focus management.
    pub input_ref: AnyNodeRef,

    /// Callback to invoke when a complete time is reconstructed.
    pub on_complete: Callback<NaiveTime>,
}

// ── Segment layout ───────────────────────────────────────────────────

/// Computes the ordered segment layout for the given configuration.
pub(crate) fn compute_segments(
    hour_cycle: HourCycle,
    granularity: TimeGranularity,
) -> Vec<SegmentInfo> {
    let mut segments = Vec::with_capacity(7);
    let mut editable = 0usize;

    segments.push(SegmentInfo {
        segment_type: SegmentType::Hour,
        literal: "",
        editable_index: Some(editable),
    });
    editable += 1;

    if granularity >= TimeGranularity::Minute {
        segments.push(SegmentInfo {
            segment_type: SegmentType::Literal,
            literal: ":",
            editable_index: None,
        });
        segments.push(SegmentInfo {
            segment_type: SegmentType::Minute,
            literal: "",
            editable_index: Some(editable),
        });
        editable += 1;
    }

    if granularity >= TimeGranularity::Second {
        segments.push(SegmentInfo {
            segment_type: SegmentType::Literal,
            literal: ":",
            editable_index: None,
        });
        segments.push(SegmentInfo {
            segment_type: SegmentType::Second,
            literal: "",
            editable_index: Some(editable),
        });
        editable += 1;
    }

    if hour_cycle == HourCycle::H12 {
        segments.push(SegmentInfo {
            segment_type: SegmentType::Literal,
            literal: "\u{a0}", // non-breaking space
            editable_index: None,
        });
        segments.push(SegmentInfo {
            segment_type: SegmentType::DayPeriod,
            literal: "",
            editable_index: Some(editable),
        });
    }

    segments
}

/// Returns the number of editable segments for the given configuration.
pub(crate) fn editable_segment_count(
    hour_cycle: HourCycle,
    granularity: TimeGranularity,
) -> usize {
    let mut count = 1; // hour
    if granularity >= TimeGranularity::Minute {
        count += 1;
    }
    if granularity >= TimeGranularity::Second {
        count += 1;
    }
    if hour_cycle == HourCycle::H12 {
        count += 1;
    }
    count
}

// ── Time decomposition / reconstruction ──────────────────────────────

/// Splits a `NaiveTime` into segment values for the given hour cycle.
///
/// Returns `(hour, minute, second, day_period)`.
pub(crate) fn decompose_time(time: NaiveTime, hour_cycle: HourCycle) -> (u32, u32, u32, u32) {
    let h24 = time.hour();
    let minute = time.minute();
    let second = time.second();

    match hour_cycle {
        HourCycle::H12 => {
            let period = if h24 >= 12 { 1u32 } else { 0u32 };
            let h12 = h24 % 12;
            let h12 = if h12 == 0 { 12 } else { h12 };
            (h12, minute, second, period)
        }
        HourCycle::H24 => (h24, minute, second, 0),
    }
}

/// Reconstructs a `NaiveTime` from segment values.
///
/// Returns `None` if any required segment is in placeholder state.
pub(crate) fn reconstruct_time(
    hour: Option<u32>,
    minute: Option<u32>,
    second: Option<u32>,
    day_period: Option<u32>,
    hour_cycle: HourCycle,
    granularity: TimeGranularity,
) -> Option<NaiveTime> {
    let h = hour?;
    let m = if granularity >= TimeGranularity::Minute {
        minute?
    } else {
        0
    };
    let s = if granularity >= TimeGranularity::Second {
        second?
    } else {
        0
    };

    let h24 = match hour_cycle {
        HourCycle::H12 => {
            let period = day_period?;
            let mut h24 = h % 12; // 12 → 0
            if period == 1 {
                h24 += 12;
            }
            h24
        }
        HourCycle::H24 => h,
    };

    NaiveTime::from_hms_opt(h24, m, s)
}

// ── Segment bounds ───────────────────────────────────────────────────

/// Returns `(min, max)` for a given segment type and hour cycle.
pub(crate) fn segment_bounds(segment_type: SegmentType, hour_cycle: HourCycle) -> (u32, u32) {
    match segment_type {
        SegmentType::Hour => match hour_cycle {
            HourCycle::H12 => (1, 12),
            HourCycle::H24 => (0, 23),
        },
        SegmentType::Minute | SegmentType::Second => (0, 59),
        SegmentType::DayPeriod => (0, 1),
        SegmentType::Literal => (0, 0),
    }
}

// ── Digit accumulation ───────────────────────────────────────────────

/// Result of processing a digit keystroke.
pub(crate) enum DigitResult {
    /// Value updated, stay on this segment (more digits possible).
    Wait(u32),
    /// Value committed, advance to the next segment.
    Advance(u32),
    /// Digit rejected (would produce invalid value).
    Reject,
}

/// Processes a digit keystroke for a segment.
///
/// `buffer` is the current accumulated digit string. `digit` is the new
/// character (must be `'0'..='9'`). `max` is the segment's maximum value.
///
/// Returns a `DigitResult` and the new buffer string.
pub(crate) fn process_digit(buffer: &str, digit: char, max: u32) -> (DigitResult, String) {
    let new_buffer = format!("{}{}", buffer, digit);
    let new_val: u32 = new_buffer.parse().unwrap_or(0);

    if new_val > max {
        // Accumulated value exceeds max. Start fresh with just this digit.
        let single = digit.to_digit(10).unwrap_or(0);
        if single > max {
            return (DigitResult::Reject, buffer.to_string());
        }
        let single_buffer = digit.to_string();
        if single * 10 > max || single_buffer.len() >= max_digits(max) {
            return (DigitResult::Advance(single), String::new());
        }
        return (DigitResult::Wait(single), single_buffer);
    }

    if new_val * 10 > max || new_buffer.len() >= max_digits(max) {
        (DigitResult::Advance(new_val), String::new())
    } else {
        (DigitResult::Wait(new_val), new_buffer)
    }
}

/// Number of decimal digits needed to represent `max`.
fn max_digits(max: u32) -> usize {
    if max == 0 {
        return 1;
    }
    ((max as f64).log10().floor() as usize) + 1
}

// ── Increment / decrement with wrapping ──────────────────────────────

/// Increments or decrements a segment value with wrapping.
pub(crate) fn wrap_value(current: u32, delta: i32, min: u32, max: u32) -> u32 {
    let range = (max - min + 1) as i32;
    let offset = (current as i32 - min as i32 + delta).rem_euclid(range);
    min + offset as u32
}

// ── Display formatting ───────────────────────────────────────────────

/// Formats a segment value for display.
pub(crate) fn format_segment_value(
    segment_type: SegmentType,
    value: Option<u32>,
    hour_cycle: HourCycle,
) -> String {
    match value {
        None => match segment_type {
            SegmentType::Hour | SegmentType::Minute | SegmentType::Second => {
                "\u{2013}\u{2013}".to_string() // "––"
            }
            SegmentType::DayPeriod => "AM".to_string(),
            SegmentType::Literal => String::new(),
        },
        Some(v) => match segment_type {
            SegmentType::Hour => match hour_cycle {
                HourCycle::H12 => format!("{}", v),
                HourCycle::H24 => format!("{:02}", v),
            },
            SegmentType::Minute | SegmentType::Second => format!("{:02}", v),
            SegmentType::DayPeriod => {
                if v == 0 {
                    "AM".to_string()
                } else {
                    "PM".to_string()
                }
            }
            SegmentType::Literal => String::new(),
        },
    }
}

/// Returns the `aria-valuetext` for a segment.
pub(crate) fn segment_value_text(
    segment_type: SegmentType,
    value: Option<u32>,
    hour_cycle: HourCycle,
) -> String {
    match value {
        None => "empty".to_string(),
        Some(v) => match segment_type {
            SegmentType::Hour => format_segment_value(segment_type, Some(v), hour_cycle),
            SegmentType::Minute => format!("{:02}", v),
            SegmentType::Second => format!("{:02}", v),
            SegmentType::DayPeriod => {
                if v == 0 {
                    "AM".to_string()
                } else {
                    "PM".to_string()
                }
            }
            SegmentType::Literal => String::new(),
        },
    }
}

// ── Focus helpers ────────────────────────────────────────────────────

/// Focuses the editable segment at the given index within the input
/// container. Uses `queueMicrotask` to defer focus until after DOM
/// reconciliation.
pub(crate) fn focus_segment(input_ref: AnyNodeRef, editable_index: usize) {
    use web_sys::wasm_bindgen::{closure::Closure, JsCast};

    let cb = Closure::once_into_js(move || {
        if let Some(container) = untrack(|| input_ref.get()) {
            let el: &web_sys::Element = container.unchecked_ref();
            let selector = format!("[role='spinbutton'][data-editable-index='{}']", editable_index);
            if let Ok(Some(segment)) = el.query_selector(&selector) {
                let segment: web_sys::HtmlElement = segment.unchecked_into();
                let _ = segment.focus();
            }
        }
    });
    web_sys::window()
        .expect("Window should exist.")
        .queue_microtask(cb.unchecked_ref());
}

// ── Tests ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── decompose_time ────────────────────────────────────

    #[test]
    fn decompose_h12_am() {
        let t = NaiveTime::from_hms_opt(9, 30, 15).unwrap();
        assert_eq!(decompose_time(t, HourCycle::H12), (9, 30, 15, 0));
    }

    #[test]
    fn decompose_h12_pm() {
        let t = NaiveTime::from_hms_opt(14, 5, 0).unwrap();
        assert_eq!(decompose_time(t, HourCycle::H12), (2, 5, 0, 1));
    }

    #[test]
    fn decompose_h12_noon() {
        let t = NaiveTime::from_hms_opt(12, 0, 0).unwrap();
        assert_eq!(decompose_time(t, HourCycle::H12), (12, 0, 0, 1));
    }

    #[test]
    fn decompose_h12_midnight() {
        let t = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        assert_eq!(decompose_time(t, HourCycle::H12), (12, 0, 0, 0));
    }

    #[test]
    fn decompose_h24() {
        let t = NaiveTime::from_hms_opt(14, 5, 30).unwrap();
        assert_eq!(decompose_time(t, HourCycle::H24), (14, 5, 30, 0));
    }

    #[test]
    fn decompose_h24_midnight() {
        let t = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        assert_eq!(decompose_time(t, HourCycle::H24), (0, 0, 0, 0));
    }

    // ── reconstruct_time ──────────────────────────────────

    #[test]
    fn reconstruct_h12_am() {
        let t = reconstruct_time(Some(9), Some(30), Some(0), Some(0), HourCycle::H12, TimeGranularity::Second);
        assert_eq!(t, NaiveTime::from_hms_opt(9, 30, 0));
    }

    #[test]
    fn reconstruct_h12_pm() {
        let t = reconstruct_time(Some(2), Some(5), None, Some(1), HourCycle::H12, TimeGranularity::Minute);
        assert_eq!(t, NaiveTime::from_hms_opt(14, 5, 0));
    }

    #[test]
    fn reconstruct_h12_noon() {
        let t = reconstruct_time(Some(12), Some(0), None, Some(1), HourCycle::H12, TimeGranularity::Minute);
        assert_eq!(t, NaiveTime::from_hms_opt(12, 0, 0));
    }

    #[test]
    fn reconstruct_h12_midnight() {
        let t = reconstruct_time(Some(12), Some(0), None, Some(0), HourCycle::H12, TimeGranularity::Minute);
        assert_eq!(t, NaiveTime::from_hms_opt(0, 0, 0));
    }

    #[test]
    fn reconstruct_h24() {
        let t = reconstruct_time(Some(14), Some(5), Some(30), None, HourCycle::H24, TimeGranularity::Second);
        assert_eq!(t, NaiveTime::from_hms_opt(14, 5, 30));
    }

    #[test]
    fn reconstruct_placeholder_returns_none() {
        let t = reconstruct_time(Some(9), None, None, Some(0), HourCycle::H12, TimeGranularity::Minute);
        assert_eq!(t, None);
    }

    #[test]
    fn reconstruct_h12_missing_period_returns_none() {
        let t = reconstruct_time(Some(9), Some(30), None, None, HourCycle::H12, TimeGranularity::Minute);
        assert_eq!(t, None);
    }

    // ── roundtrip ─────────────────────────────────────────

    #[test]
    fn roundtrip_h12() {
        for h in 0..24 {
            for m in [0, 15, 30, 45, 59] {
                let original = NaiveTime::from_hms_opt(h, m, 0).unwrap();
                let (dh, dm, ds, dp) = decompose_time(original, HourCycle::H12);
                let reconstructed = reconstruct_time(
                    Some(dh), Some(dm), Some(ds), Some(dp),
                    HourCycle::H12, TimeGranularity::Second,
                );
                assert_eq!(reconstructed, Some(original), "roundtrip failed for {:02}:{:02}", h, m);
            }
        }
    }

    #[test]
    fn roundtrip_h24() {
        for h in 0..24 {
            for m in [0, 15, 30, 45, 59] {
                let original = NaiveTime::from_hms_opt(h, m, 0).unwrap();
                let (dh, dm, ds, dp) = decompose_time(original, HourCycle::H24);
                let reconstructed = reconstruct_time(
                    Some(dh), Some(dm), Some(ds), Some(dp),
                    HourCycle::H24, TimeGranularity::Second,
                );
                assert_eq!(reconstructed, Some(original), "roundtrip failed for {:02}:{:02}", h, m);
            }
        }
    }

    // ── segment_bounds ────────────────────────────────────

    #[test]
    fn bounds_h12() {
        assert_eq!(segment_bounds(SegmentType::Hour, HourCycle::H12), (1, 12));
    }

    #[test]
    fn bounds_h24() {
        assert_eq!(segment_bounds(SegmentType::Hour, HourCycle::H24), (0, 23));
    }

    #[test]
    fn bounds_minute() {
        assert_eq!(segment_bounds(SegmentType::Minute, HourCycle::H12), (0, 59));
    }

    #[test]
    fn bounds_day_period() {
        assert_eq!(segment_bounds(SegmentType::DayPeriod, HourCycle::H12), (0, 1));
    }

    // ── process_digit ─────────────────────────────────────

    #[test]
    fn digit_h12_type_1_then_2() {
        // First digit: "1" — could become 10, 11, 12 → wait
        let (result, buf) = process_digit("", '1', 12);
        assert!(matches!(result, DigitResult::Wait(1)));
        assert_eq!(buf, "1");

        // Second digit: "12" — 12*10=120 > 12 → advance with 12
        let (result, buf) = process_digit(&buf, '2', 12);
        assert!(matches!(result, DigitResult::Advance(12)));
        assert_eq!(buf, "");
    }

    #[test]
    fn digit_h12_type_5() {
        // "5" — 5*10=50 > 12 → advance immediately
        let (result, buf) = process_digit("", '5', 12);
        assert!(matches!(result, DigitResult::Advance(5)));
        assert_eq!(buf, "");
    }

    #[test]
    fn digit_h12_type_1_then_3() {
        // "1" → wait
        let (result, buf) = process_digit("", '1', 12);
        assert!(matches!(result, DigitResult::Wait(1)));

        // "13" > 12 → start fresh with "3", 3*10=30 > 12 → advance with 3
        let (result, buf) = process_digit(&buf, '3', 12);
        assert!(matches!(result, DigitResult::Advance(3)));
        assert_eq!(buf, "");
    }

    #[test]
    fn digit_minute_type_3_then_0() {
        // "3" — 3*10=30 ≤ 59, len=1 < 2 → wait
        let (result, buf) = process_digit("", '3', 59);
        assert!(matches!(result, DigitResult::Wait(3)));
        assert_eq!(buf, "3");

        // "30" — 30*10=300 > 59 → advance with 30
        let (result, buf) = process_digit(&buf, '0', 59);
        assert!(matches!(result, DigitResult::Advance(30)));
        assert_eq!(buf, "");
    }

    #[test]
    fn digit_minute_type_6() {
        // "6" — 6*10=60 > 59 → advance immediately
        let (result, buf) = process_digit("", '6', 59);
        assert!(matches!(result, DigitResult::Advance(6)));
        assert_eq!(buf, "");
    }

    #[test]
    fn digit_h24_type_2_then_3() {
        // "2" — 2*10=20 ≤ 23, wait
        let (result, buf) = process_digit("", '2', 23);
        assert!(matches!(result, DigitResult::Wait(2)));

        // "23" — 23*10=230 > 23 → advance with 23
        let (result, buf) = process_digit(&buf, '3', 23);
        assert!(matches!(result, DigitResult::Advance(23)));
        assert_eq!(buf, "");
    }

    #[test]
    fn digit_h24_type_2_then_4() {
        let (_, buf) = process_digit("", '2', 23);
        // "24" > 23 → start fresh with "4", 4*10=40 > 23 → advance with 4
        let (result, buf) = process_digit(&buf, '4', 23);
        assert!(matches!(result, DigitResult::Advance(4)));
        assert_eq!(buf, "");
    }

    // ── wrap_value ────────────────────────────────────────

    #[test]
    fn wrap_increment() {
        assert_eq!(wrap_value(12, 1, 1, 12), 1); // H12: 12+1 → 1
        assert_eq!(wrap_value(23, 1, 0, 23), 0); // H24: 23+1 → 0
        assert_eq!(wrap_value(59, 1, 0, 59), 0); // minute: 59+1 → 0
    }

    #[test]
    fn wrap_decrement() {
        assert_eq!(wrap_value(1, -1, 1, 12), 12); // H12: 1-1 → 12
        assert_eq!(wrap_value(0, -1, 0, 23), 23); // H24: 0-1 → 23
        assert_eq!(wrap_value(0, -1, 0, 59), 59); // minute: 0-1 → 59
    }

    #[test]
    fn wrap_day_period() {
        assert_eq!(wrap_value(0, 1, 0, 1), 1); // AM → PM
        assert_eq!(wrap_value(1, 1, 0, 1), 0); // PM → AM
        assert_eq!(wrap_value(0, -1, 0, 1), 1); // AM → PM
    }

    // ── max_digits ────────────────────────────────────────

    #[test]
    fn max_digits_values() {
        assert_eq!(max_digits(1), 1);
        assert_eq!(max_digits(9), 1);
        assert_eq!(max_digits(12), 2);
        assert_eq!(max_digits(23), 2);
        assert_eq!(max_digits(59), 2);
    }

    // ── format_segment_value ──────────────────────────────

    #[test]
    fn format_placeholder() {
        assert_eq!(format_segment_value(SegmentType::Hour, None, HourCycle::H12), "\u{2013}\u{2013}");
        assert_eq!(format_segment_value(SegmentType::DayPeriod, None, HourCycle::H12), "AM");
    }

    #[test]
    fn format_h12_hour() {
        assert_eq!(format_segment_value(SegmentType::Hour, Some(9), HourCycle::H12), "9");
        assert_eq!(format_segment_value(SegmentType::Hour, Some(12), HourCycle::H12), "12");
    }

    #[test]
    fn format_h24_hour() {
        assert_eq!(format_segment_value(SegmentType::Hour, Some(9), HourCycle::H24), "09");
        assert_eq!(format_segment_value(SegmentType::Hour, Some(14), HourCycle::H24), "14");
    }

    #[test]
    fn format_minute() {
        assert_eq!(format_segment_value(SegmentType::Minute, Some(5), HourCycle::H12), "05");
        assert_eq!(format_segment_value(SegmentType::Minute, Some(30), HourCycle::H12), "30");
    }

    #[test]
    fn format_day_period() {
        assert_eq!(format_segment_value(SegmentType::DayPeriod, Some(0), HourCycle::H12), "AM");
        assert_eq!(format_segment_value(SegmentType::DayPeriod, Some(1), HourCycle::H12), "PM");
    }

    // ── compute_segments ──────────────────────────────────

    #[test]
    fn segments_h12_minute() {
        let segs = compute_segments(HourCycle::H12, TimeGranularity::Minute);
        let types: Vec<_> = segs.iter().map(|s| s.segment_type).collect();
        assert_eq!(types, vec![
            SegmentType::Hour,
            SegmentType::Literal,
            SegmentType::Minute,
            SegmentType::Literal,
            SegmentType::DayPeriod,
        ]);
        assert_eq!(editable_segment_count(HourCycle::H12, TimeGranularity::Minute), 3);
    }

    #[test]
    fn segments_h24_minute() {
        let segs = compute_segments(HourCycle::H24, TimeGranularity::Minute);
        let types: Vec<_> = segs.iter().map(|s| s.segment_type).collect();
        assert_eq!(types, vec![
            SegmentType::Hour,
            SegmentType::Literal,
            SegmentType::Minute,
        ]);
        assert_eq!(editable_segment_count(HourCycle::H24, TimeGranularity::Minute), 2);
    }

    #[test]
    fn segments_h12_second() {
        let segs = compute_segments(HourCycle::H12, TimeGranularity::Second);
        let types: Vec<_> = segs.iter().map(|s| s.segment_type).collect();
        // Hour : Minute : Second   DayPeriod
        assert_eq!(types, vec![
            SegmentType::Hour,
            SegmentType::Literal,
            SegmentType::Minute,
            SegmentType::Literal,
            SegmentType::Second,
            SegmentType::Literal,
            SegmentType::DayPeriod,
        ]);
        assert_eq!(editable_segment_count(HourCycle::H12, TimeGranularity::Second), 4);
    }

    #[test]
    fn segments_h12_hour_only() {
        let segs = compute_segments(HourCycle::H12, TimeGranularity::Hour);
        let types: Vec<_> = segs.iter().map(|s| s.segment_type).collect();
        assert_eq!(types, vec![
            SegmentType::Hour,
            SegmentType::Literal,
            SegmentType::DayPeriod,
        ]);
        assert_eq!(editable_segment_count(HourCycle::H12, TimeGranularity::Hour), 2);
    }
}
