//! Range slider for selecting numeric values.
//!
//! An accessible range input that supports single or multiple thumbs,
//! horizontal or vertical orientation, and RTL direction. Renders as a
//! `<span>` with `role="slider"` on each thumb.
//!
//! Implements the [WAI-ARIA Slider pattern](https://www.w3.org/WAI/ARIA/apd/patterns/slider-multithumb/).
//!
//! # Anatomy
//!
//! ```text
//! <Slider>
//!     <SliderTrack>
//!         <SliderRange />
//!     </SliderTrack>
//!     <SliderThumb />
//! </Slider>
//! ```
//!
//! # Features
//!
//! - Single or multiple thumbs
//! - Controlled and uncontrolled value state
//! - Horizontal and vertical orientation
//! - RTL support
//! - Min step distance between thumbs
//! - Native form participation via hidden `<input>`
//! - Touch device support with pointer capture
//!
//! # Keyboard Interactions
//!
//! | Key | Action |
//! |-----|--------|
//! | ArrowRight / ArrowUp | Increase value by one step |
//! | ArrowLeft / ArrowDown | Decrease value by one step |
//! | PageUp | Increase value by 10 steps |
//! | PageDown | Decrease value by 10 steps |
//! | Shift + Arrow | Increase/decrease by 10 steps |
//! | Home | Set to minimum value |
//! | End | Set to maximum value |
//!
//! # Data Attributes
//!
//! **Slider, SliderTrack, SliderRange, SliderThumb:**
//!
//! | Attribute | Values |
//! |-----------|--------|
//! | `data-orientation` | `horizontal`, `vertical` |
//! | `data-disabled` | Present when disabled |
//!
//! # CSS Custom Properties
//!
//! | Property | Description |
//! |----------|-------------|
//! | `--radix-slider-thumb-transform` | Transform applied to thumb for centering |

use std::marker::PhantomData;

use crate::internal::number::clamp;
use crate::internal::utils::linear_scale;
use crate::support::collection::{
    CollectionItemSlot, CollectionProvider, CollectionSlot, use_collection,
};
use crate::support::compose_refs::use_composed_refs;
use crate::support::direction::{Direction, use_direction};
use crate::support::primitive::{
    Primitive, compose_callbacks, data_attr, prop_or, prop_or_default,
};
use crate::support::use_controllable_state::{UseControllableStateParams, use_controllable_state};
use crate::support::use_previous::use_previous;
use crate::support::use_size::use_size;
use leptos::{context::Provider, ev, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::JsCast;

mod slider;
mod slider_parts;

pub use slider::*;
pub use slider_parts::*;

/* -------------------------------------------------------------------------------------------------
 * Constants
 * -----------------------------------------------------------------------------------------------*/

const PAGE_KEYS: [&str; 2] = ["PageUp", "PageDown"];
const ARROW_KEYS: [&str; 4] = ["ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight"];

type SlideDirection = &'static str;
const FROM_LEFT: SlideDirection = "from-left";
const FROM_RIGHT: SlideDirection = "from-right";
const FROM_BOTTOM: SlideDirection = "from-bottom";
const FROM_TOP: SlideDirection = "from-top";

fn back_keys(direction: SlideDirection) -> &'static [&'static str] {
    match direction {
        FROM_LEFT => &["Home", "PageDown", "ArrowDown", "ArrowLeft"],
        FROM_RIGHT => &["Home", "PageDown", "ArrowDown", "ArrowRight"],
        FROM_BOTTOM => &["Home", "PageDown", "ArrowDown", "ArrowLeft"],
        FROM_TOP => &["Home", "PageDown", "ArrowUp", "ArrowLeft"],
        _ => &[],
    }
}

/* -------------------------------------------------------------------------------------------------
 * Shared types and context
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Default, Debug, PartialEq)]
struct ItemData;

const ITEM_DATA_PHANTOM: PhantomData<ItemData> = PhantomData;

#[derive(Clone)]
struct SliderContextValue {
    name: Signal<Option<String>>,
    disabled: Signal<bool>,
    min: Signal<f64>,
    max: Signal<f64>,
    values: Signal<Vec<f64>>,
    value_index_to_change: RwSignal<usize>,
    thumbs: RwSignal<Vec<SendWrapper<web_sys::HtmlElement>>>,
    orientation: Signal<Orientation>,
    form: Signal<Option<String>>,
}

/// The orientation of a slider.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Orientation {
    /// Horizontal slider (default).
    #[default]
    Horizontal,
    /// Vertical slider.
    Vertical,
}

impl std::fmt::Display for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Orientation::Horizontal => write!(f, "horizontal"),
            Orientation::Vertical => write!(f, "vertical"),
        }
    }
}

/* -------------------------------------------------------------------------------------------------
 * SliderOrientationContext
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy, Debug)]
struct SliderOrientationContextValue {
    start_edge: &'static str,
    end_edge: &'static str,
    size: OrientationSize,
    direction: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum OrientationSize {
    Width,
    Height,
}

/* -------------------------------------------------------------------------------------------------
 * Utility functions
 * -----------------------------------------------------------------------------------------------*/

fn get_next_sorted_values(prev_values: &[f64], next_value: f64, at_index: usize) -> Vec<f64> {
    let mut next_values = prev_values.to_vec();
    if at_index < next_values.len() {
        next_values[at_index] = next_value;
    }
    next_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    next_values
}

fn convert_value_to_percentage(value: f64, min: f64, max: f64) -> f64 {
    let max_steps = max - min;
    let percent_per_step = 100.0 / max_steps;
    let percentage = percent_per_step * (value - min);
    clamp(percentage, [0.0, 100.0])
}

fn get_label(index: usize, total_values: usize) -> Option<String> {
    if total_values > 2 {
        Some(format!("Value {} of {}", index + 1, total_values))
    } else if total_values == 2 {
        ["Minimum", "Maximum"].get(index).map(|s| s.to_string())
    } else {
        None
    }
}

fn get_closest_value_index(values: &[f64], next_value: f64) -> usize {
    if values.len() == 1 {
        return 0;
    }
    let distances: Vec<f64> = values.iter().map(|v| (v - next_value).abs()).collect();
    let closest_distance = distances.iter().cloned().fold(f64::INFINITY, f64::min);
    distances
        .iter()
        .position(|&d| d == closest_distance)
        .unwrap_or(0)
}

fn get_thumb_in_bounds_offset(width: f64, left: f64, direction: f64) -> f64 {
    let half_width = width / 2.0;
    let half_percent = 50.0;
    let offset = linear_scale([0.0, half_percent], [0.0, half_width]);
    (half_width - offset(left) * direction) * direction
}

fn get_steps_between_values(values: &[f64]) -> Vec<f64> {
    values.windows(2).map(|w| w[1] - w[0]).collect()
}

fn has_min_steps_between_values(values: &[f64], min_steps_between_values: f64) -> bool {
    if min_steps_between_values > 0.0 {
        let steps_between = get_steps_between_values(values);
        let actual_min = steps_between.iter().cloned().fold(f64::INFINITY, f64::min);
        actual_min >= min_steps_between_values
    } else {
        true
    }
}

fn get_decimal_count(value: f64) -> u32 {
    let s = value.to_string();
    if let Some(dot_pos) = s.find('.') {
        (s.len() - dot_pos - 1) as u32
    } else {
        0
    }
}

fn round_value(value: f64, decimal_count: u32) -> f64 {
    let rounder = 10f64.powi(decimal_count as i32);
    (value * rounder).round() / rounder
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── back_keys ───────────────────────────────────────────

    #[test]
    fn back_keys_from_left() {
        let keys = back_keys(FROM_LEFT);
        assert!(keys.contains(&"Home"));
        assert!(keys.contains(&"PageDown"));
        assert!(keys.contains(&"ArrowDown"));
        assert!(keys.contains(&"ArrowLeft"));
        assert!(!keys.contains(&"ArrowRight"));
        assert!(!keys.contains(&"ArrowUp"));
    }

    #[test]
    fn back_keys_from_right() {
        let keys = back_keys(FROM_RIGHT);
        assert!(keys.contains(&"ArrowRight"));
        assert!(!keys.contains(&"ArrowLeft"));
    }

    #[test]
    fn back_keys_from_bottom() {
        let keys = back_keys(FROM_BOTTOM);
        assert!(keys.contains(&"ArrowDown"));
        assert!(keys.contains(&"ArrowLeft"));
    }

    #[test]
    fn back_keys_from_top() {
        let keys = back_keys(FROM_TOP);
        assert!(keys.contains(&"ArrowUp"));
        assert!(keys.contains(&"ArrowLeft"));
        assert!(!keys.contains(&"ArrowDown"));
    }

    #[test]
    fn back_keys_unknown_direction() {
        assert!(back_keys("unknown").is_empty());
    }

    // ── get_decimal_count ───────────────────────────────────

    #[test]
    fn decimal_count_integer() {
        assert_eq!(get_decimal_count(1.0), 0);
        assert_eq!(get_decimal_count(100.0), 0);
    }

    #[test]
    fn decimal_count_one_decimal() {
        assert_eq!(get_decimal_count(0.1), 1);
        assert_eq!(get_decimal_count(1.5), 1);
    }

    #[test]
    fn decimal_count_multiple_decimals() {
        assert_eq!(get_decimal_count(0.01), 2);
        assert_eq!(get_decimal_count(0.001), 3);
        assert_eq!(get_decimal_count(3.14159), 5);
    }

    // ── round_value ─────────────────────────────────────────

    #[test]
    fn round_value_zero_decimals() {
        assert_eq!(round_value(3.7, 0), 4.0);
        assert_eq!(round_value(3.2, 0), 3.0);
    }

    #[test]
    fn round_value_one_decimal() {
        assert_eq!(round_value(3.14, 1), 3.1);
        assert_eq!(round_value(3.15, 1), 3.2);
    }

    #[test]
    fn round_value_two_decimals() {
        assert_eq!(round_value(3.141, 2), 3.14);
        assert_eq!(round_value(3.145, 2), 3.15);
    }

    #[test]
    fn round_value_already_rounded() {
        assert_eq!(round_value(5.0, 2), 5.0);
    }

    // ── convert_value_to_percentage ─────────────────────────

    #[test]
    fn percentage_at_min() {
        assert_eq!(convert_value_to_percentage(0.0, 0.0, 100.0), 0.0);
    }

    #[test]
    fn percentage_at_max() {
        assert_eq!(convert_value_to_percentage(100.0, 0.0, 100.0), 100.0);
    }

    #[test]
    fn percentage_midpoint() {
        assert_eq!(convert_value_to_percentage(50.0, 0.0, 100.0), 50.0);
    }

    #[test]
    fn percentage_custom_range() {
        assert_eq!(convert_value_to_percentage(15.0, 10.0, 20.0), 50.0);
    }

    #[test]
    fn percentage_clamped_below() {
        assert_eq!(convert_value_to_percentage(-10.0, 0.0, 100.0), 0.0);
    }

    #[test]
    fn percentage_clamped_above() {
        assert_eq!(convert_value_to_percentage(200.0, 0.0, 100.0), 100.0);
    }

    // ── get_label ───────────────────────────────────────────

    #[test]
    fn label_single_value_is_none() {
        assert_eq!(get_label(0, 1), None);
    }

    #[test]
    fn label_two_values_gives_min_max() {
        assert_eq!(get_label(0, 2), Some("Minimum".to_string()));
        assert_eq!(get_label(1, 2), Some("Maximum".to_string()));
    }

    #[test]
    fn label_three_or_more_values_gives_indexed() {
        assert_eq!(get_label(0, 3), Some("Value 1 of 3".to_string()));
        assert_eq!(get_label(1, 3), Some("Value 2 of 3".to_string()));
        assert_eq!(get_label(2, 3), Some("Value 3 of 3".to_string()));
    }

    #[test]
    fn label_many_values() {
        assert_eq!(get_label(0, 5), Some("Value 1 of 5".to_string()));
        assert_eq!(get_label(4, 5), Some("Value 5 of 5".to_string()));
    }

    // ── get_closest_value_index ─────────────────────────────

    #[test]
    fn closest_single_value_always_zero() {
        assert_eq!(get_closest_value_index(&[50.0], 0.0), 0);
        assert_eq!(get_closest_value_index(&[50.0], 100.0), 0);
    }

    #[test]
    fn closest_picks_nearest() {
        assert_eq!(get_closest_value_index(&[10.0, 50.0, 90.0], 12.0), 0);
        assert_eq!(get_closest_value_index(&[10.0, 50.0, 90.0], 48.0), 1);
        assert_eq!(get_closest_value_index(&[10.0, 50.0, 90.0], 85.0), 2);
    }

    #[test]
    fn closest_equidistant_picks_first() {
        // 30 is equidistant from 20 and 40 — should pick index 0 (first match)
        assert_eq!(get_closest_value_index(&[20.0, 40.0], 30.0), 0);
    }

    #[test]
    fn closest_exact_match() {
        assert_eq!(get_closest_value_index(&[10.0, 20.0, 30.0], 20.0), 1);
    }

    // ── get_next_sorted_values ──────────────────────────────

    #[test]
    fn next_sorted_basic_replace() {
        assert_eq!(
            get_next_sorted_values(&[10.0, 50.0, 90.0], 60.0, 1),
            vec![10.0, 60.0, 90.0]
        );
    }

    #[test]
    fn next_sorted_reorders_on_crossover() {
        // Moving thumb at index 0 past thumb at index 1
        assert_eq!(
            get_next_sorted_values(&[10.0, 50.0], 70.0, 0),
            vec![50.0, 70.0]
        );
    }

    #[test]
    fn next_sorted_single_value() {
        assert_eq!(get_next_sorted_values(&[50.0], 30.0, 0), vec![30.0]);
    }

    #[test]
    fn next_sorted_index_out_of_bounds_no_change() {
        assert_eq!(
            get_next_sorted_values(&[10.0, 20.0], 99.0, 5),
            vec![10.0, 20.0]
        );
    }

    // ── get_steps_between_values ────────────────────────────

    #[test]
    fn steps_between_empty() {
        assert!(get_steps_between_values(&[]).is_empty());
    }

    #[test]
    fn steps_between_single() {
        assert!(get_steps_between_values(&[5.0]).is_empty());
    }

    #[test]
    fn steps_between_two_values() {
        assert_eq!(get_steps_between_values(&[10.0, 30.0]), vec![20.0]);
    }

    #[test]
    fn steps_between_multiple() {
        assert_eq!(
            get_steps_between_values(&[0.0, 25.0, 75.0, 100.0]),
            vec![25.0, 50.0, 25.0]
        );
    }

    // ── has_min_steps_between_values ────────────────────────

    #[test]
    fn min_steps_zero_always_true() {
        assert!(has_min_steps_between_values(&[10.0, 10.0], 0.0));
        assert!(has_min_steps_between_values(&[], 0.0));
    }

    #[test]
    fn min_steps_satisfied() {
        assert!(has_min_steps_between_values(&[0.0, 20.0, 40.0], 10.0));
        assert!(has_min_steps_between_values(&[0.0, 10.0, 20.0], 10.0));
    }

    #[test]
    fn min_steps_not_satisfied() {
        assert!(!has_min_steps_between_values(&[0.0, 5.0, 20.0], 10.0));
    }

    #[test]
    fn min_steps_single_value_always_true() {
        // No windows to compare, so the fold yields +inf >= anything
        assert!(has_min_steps_between_values(&[50.0], 100.0));
    }

    // ── get_thumb_in_bounds_offset ──────────────────────────

    #[test]
    fn thumb_offset_at_zero_percent() {
        // At 0%, thumb hangs off the left edge by half its width
        let offset = get_thumb_in_bounds_offset(20.0, 0.0, 1.0);
        assert_eq!(offset, 10.0);
    }

    #[test]
    fn thumb_offset_at_fifty_percent() {
        // At 50%, thumb is centered — no offset needed
        let offset = get_thumb_in_bounds_offset(20.0, 50.0, 1.0);
        assert_eq!(offset, 0.0);
    }

    #[test]
    fn thumb_offset_at_hundred_percent() {
        // At 100%, thumb hangs off the right edge
        let offset = get_thumb_in_bounds_offset(20.0, 100.0, 1.0);
        assert_eq!(offset, -10.0);
    }

    #[test]
    fn thumb_offset_reverse_direction() {
        // direction = -1.0 (RTL or inverted)
        let offset = get_thumb_in_bounds_offset(20.0, 0.0, -1.0);
        assert_eq!(offset, -10.0);
    }

    #[test]
    fn thumb_offset_zero_width() {
        assert_eq!(get_thumb_in_bounds_offset(0.0, 50.0, 1.0), 0.0);
    }
}
