//! Custom scrollable area with styled scrollbars.
//!
//! Replaces native scrollbar rendering with custom-styled scrollbar
//! overlays while preserving native scroll behavior. Supports horizontal
//! and vertical scrollbars with configurable visibility modes.
//!
//! # Anatomy
//!
//! ```text
//! <ScrollArea>
//!     <ScrollAreaViewport />
//!     <ScrollAreaScrollbar orientation="vertical">
//!         <ScrollAreaThumb />
//!     </ScrollAreaScrollbar>
//!     <ScrollAreaScrollbar orientation="horizontal">
//!         <ScrollAreaThumb />
//!     </ScrollAreaScrollbar>
//!     <ScrollAreaCorner />
//! </ScrollArea>
//! ```
//!
//! # Features
//!
//! - Native scroll behavior with custom styled scrollbars
//! - Auto, always, scroll, and hover visibility modes
//! - Horizontal and vertical scrollbars
//! - Drag-to-scroll on thumb
//! - Click-to-jump on track
//! - Corner element when both scrollbars are visible
//! - RTL support
//!
//! # Data Attributes
//!
//! **ScrollAreaScrollbar:**
//!
//! | Attribute | Values |
//! |-----------|--------|
//! | `data-state` | `visible`, `hidden` |
//! | `data-orientation` | `horizontal`, `vertical` |

use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

use crate::internal::number::clamp;
use crate::internal::utils::linear_scale;
use crate::support::compose_refs::use_composed_refs;
use crate::support::direction::{Direction, use_direction};
use crate::support::presence::Presence;
use crate::support::primitive::Primitive;
use leptos::{
    attribute_interceptor::AttributeInterceptor, context::Provider, ev, html, prelude::*,
};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::{JsCast, closure::Closure};

mod scroll_area;
mod scroll_area_corner;
mod scroll_area_scrollbar;
mod scroll_area_thumb;

pub use scroll_area::*;
pub use scroll_area_corner::*;
pub use scroll_area_scrollbar::*;
pub use scroll_area_thumb::*;

/// Controls when scrollbars become visible.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ScrollAreaType {
    /// Scrollbars visible when content overflows.
    Auto,
    /// Scrollbars always visible.
    Always,
    /// Scrollbars visible while scrolling.
    Scroll,
    /// Scrollbars visible on pointer hover (default).
    #[default]
    Hover,
}

/// The orientation of a scrollbar.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Orientation {
    Horizontal,
    #[default]
    Vertical,
}

impl Display for Orientation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Orientation::Horizontal => "horizontal",
                Orientation::Vertical => "vertical",
            }
        )
    }
}

#[derive(Clone, Debug)]
struct Sizes {
    content: f64,
    viewport: f64,
    scrollbar: ScrollbarSizes,
}

#[derive(Clone, Debug)]
struct ScrollbarSizes {
    size: f64,
    padding_start: f64,
    padding_end: f64,
}

impl Default for Sizes {
    fn default() -> Self {
        Self {
            content: 0.0,
            viewport: 0.0,
            scrollbar: ScrollbarSizes {
                size: 0.0,
                padding_start: 0.0,
                padding_end: 0.0,
            },
        }
    }
}

/* -------------------------------------------------------------------------------------------------
 * Context
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone)]
struct ScrollAreaContextValue {
    r#type: ScrollAreaType,
    dir: Signal<Direction>,
    scroll_hide_delay: u32,
    scroll_area: AnyNodeRef,
    viewport: RwSignal<Option<SendWrapper<web_sys::HtmlElement>>>,
    content: RwSignal<Option<SendWrapper<web_sys::HtmlElement>>>,
    scrollbar_x: RwSignal<Option<SendWrapper<web_sys::HtmlElement>>>,
    scrollbar_x_enabled: RwSignal<bool>,
    scrollbar_y: RwSignal<Option<SendWrapper<web_sys::HtmlElement>>>,
    scrollbar_y_enabled: RwSignal<bool>,
    corner_width: RwSignal<f64>,
    corner_height: RwSignal<f64>,
}

#[derive(Clone)]
struct ScrollbarContextValue {
    has_thumb: Signal<bool>,
    #[allow(dead_code)]
    scrollbar: RwSignal<Option<SendWrapper<web_sys::HtmlElement>>>,
    on_thumb_change: Callback<Option<SendWrapper<web_sys::HtmlElement>>>,
    on_thumb_pointer_up: Callback<()>,
    on_thumb_pointer_down: Callback<(f64, f64)>,
    on_thumb_position_change: Callback<()>,
}

/* -------------------------------------------------------------------------------------------------
 * State machine (used by ScrollAreaScrollbarScroll)
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum ScrollbarMachineState {
    Hidden,
    Scrolling,
    Interacting,
    Idle,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum ScrollbarMachineEvent {
    Scroll,
    ScrollEnd,
    PointerEnter,
    PointerLeave,
    Hide,
}

fn use_scrollbar_state_machine() -> (
    ReadSignal<ScrollbarMachineState>,
    Callback<ScrollbarMachineEvent>,
) {
    use ScrollbarMachineEvent::*;
    use ScrollbarMachineState::*;

    let (state, set_state) = signal(Hidden);

    let machine: HashMap<
        ScrollbarMachineState,
        HashMap<ScrollbarMachineEvent, ScrollbarMachineState>,
    > = HashMap::from([
        (Hidden, HashMap::from([(Scroll, Scrolling)])),
        (
            Scrolling,
            HashMap::from([(ScrollEnd, Idle), (PointerEnter, Interacting)]),
        ),
        (
            Interacting,
            HashMap::from([(Scroll, Interacting), (PointerLeave, Idle)]),
        ),
        (
            Idle,
            HashMap::from([
                (Hide, Hidden),
                (Scroll, Scrolling),
                (PointerEnter, Interacting),
            ]),
        ),
    ]);

    (
        state,
        Callback::new(move |event| {
            let current_state = state.get_untracked();
            let next_state = machine
                .get(&current_state)
                .and_then(|events| events.get(&event));
            if let Some(next_state) = next_state {
                set_state.set(*next_state);
            }
        }),
    )
}

/* -------------------------------------------------------------------------------------------------
 * Helpers
 * -----------------------------------------------------------------------------------------------*/

fn to_int(value: &str) -> f64 {
    // Mimic JS parseInt(value, 10): skip whitespace, read optional sign + digits, ignore rest (e.g. "8px" → 8).
    let trimmed = value.trim_start();
    let numeric: String = trimmed
        .chars()
        .enumerate()
        .take_while(|(i, c)| c.is_ascii_digit() || (*i == 0 && (*c == '-' || *c == '+')))
        .map(|(_, c)| c)
        .collect();
    numeric.parse::<f64>().unwrap_or(0.0)
}

fn get_thumb_ratio(viewport_size: f64, content_size: f64) -> f64 {
    let ratio = viewport_size / content_size;
    if ratio.is_nan() { 0.0 } else { ratio }
}

fn get_thumb_size(sizes: &Sizes) -> f64 {
    let ratio = get_thumb_ratio(sizes.viewport, sizes.content);
    let scrollbar_padding = sizes.scrollbar.padding_start + sizes.scrollbar.padding_end;
    let thumb_size = (sizes.scrollbar.size - scrollbar_padding) * ratio;
    // minimum of 18 matches macOS minimum
    thumb_size.max(18.0)
}

fn get_scroll_position_from_pointer(
    pointer_pos: f64,
    pointer_offset: f64,
    sizes: &Sizes,
    dir: Direction,
) -> f64 {
    let thumb_size_px = get_thumb_size(sizes);
    let thumb_center = thumb_size_px / 2.0;
    let offset = if pointer_offset != 0.0 {
        pointer_offset
    } else {
        thumb_center
    };
    let thumb_offset_from_end = thumb_size_px - offset;
    let min_pointer_pos = sizes.scrollbar.padding_start + offset;
    let max_pointer_pos =
        sizes.scrollbar.size - sizes.scrollbar.padding_end - thumb_offset_from_end;
    let max_scroll_pos = sizes.content - sizes.viewport;
    let scroll_range = match dir {
        Direction::Ltr => [0.0, max_scroll_pos],
        Direction::Rtl => [-max_scroll_pos, 0.0],
    };
    let interpolate = linear_scale([min_pointer_pos, max_pointer_pos], scroll_range);
    interpolate(pointer_pos)
}

fn get_thumb_offset_from_scroll(scroll_pos: f64, sizes: &Sizes, dir: Direction) -> f64 {
    let thumb_size_px = get_thumb_size(sizes);
    let scrollbar_padding = sizes.scrollbar.padding_start + sizes.scrollbar.padding_end;
    let scrollbar = sizes.scrollbar.size - scrollbar_padding;
    let max_scroll_pos = sizes.content - sizes.viewport;
    let max_thumb_pos = scrollbar - thumb_size_px;
    let scroll_clamp_range = match dir {
        Direction::Ltr => [0.0, max_scroll_pos],
        Direction::Rtl => [-max_scroll_pos, 0.0],
    };
    let scroll_without_momentum = clamp(scroll_pos, scroll_clamp_range);
    let interpolate = linear_scale([0.0, max_scroll_pos], [0.0, max_thumb_pos]);
    interpolate(scroll_without_momentum)
}

fn is_scrolling_within_scrollbar_bounds(scroll_pos: f64, max_scroll_pos: f64) -> bool {
    scroll_pos > 0.0 && scroll_pos < max_scroll_pos
}

type RafClosureHolder = std::rc::Rc<std::cell::RefCell<Option<Closure<dyn Fn()>>>>;

/// Custom scroll handler to avoid scroll-linked effects.
/// Returns a cleanup function that cancels the rAF loop.
#[allow(dead_code)]
fn add_unlinked_scroll_listener(
    node: &web_sys::HtmlElement,
    handler: impl Fn() + 'static,
) -> impl FnOnce() {
    let node = node.clone();
    let prev_left = std::cell::Cell::new(node.scroll_left() as f64);
    let prev_top = std::cell::Cell::new(node.scroll_top() as f64);
    let raf_id = std::rc::Rc::new(std::cell::Cell::new(0i32));

    let window = web_sys::window().expect("Window should exist.");

    let closure: RafClosureHolder = std::rc::Rc::new(std::cell::RefCell::new(None));
    let closure_clone = closure.clone();

    let cb = Closure::new({
        let node = node.clone();
        let window = window.clone();
        let closure = closure.clone();
        let raf_id = raf_id.clone();
        move || {
            let left = node.scroll_left() as f64;
            let top = node.scroll_top() as f64;
            let is_horizontal_scroll = prev_left.get() != left;
            let is_vertical_scroll = prev_top.get() != top;
            if is_horizontal_scroll || is_vertical_scroll {
                handler();
            }
            prev_left.set(left);
            prev_top.set(top);
            if let Some(c) = closure.borrow().as_ref() {
                raf_id.set(
                    window
                        .request_animation_frame(c.as_ref().unchecked_ref())
                        .unwrap_or(0),
                );
            }
        }
    });

    raf_id.set(
        window
            .request_animation_frame(cb.as_ref().unchecked_ref())
            .unwrap_or(0),
    );
    *closure_clone.borrow_mut() = Some(cb);

    let window_clone = window.clone();
    move || {
        window_clone.cancel_animation_frame(raf_id.get()).ok();
        // Drop the closure to break the circular reference
        closure_clone.borrow_mut().take();
    }
}

fn use_debounce_callback(
    callback: impl Fn() + Send + Sync + 'static,
    delay_ms: i32,
) -> Callback<()> {
    let timer_id: StoredValue<i32> = StoredValue::new(0);

    Owner::on_cleanup(move || {
        let window = web_sys::window().expect("Window should exist.");
        window.clear_timeout_with_handle(timer_id.get_value());
    });

    let callback = SendWrapper::new(callback);
    let callback = StoredValue::new(callback);

    Callback::new(move |()| {
        let window = web_sys::window().expect("Window should exist.");
        window.clear_timeout_with_handle(timer_id.get_value());
        let cb = callback;
        let handle = window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                &Closure::once_into_js(move || {
                    cb.with_value(|cb| cb());
                })
                .unchecked_into(),
                delay_ms,
            )
            .unwrap_or(0);
        timer_id.set_value(handle);
    })
}

fn use_resize_observer(
    element: Signal<Option<SendWrapper<web_sys::HtmlElement>>>,
    on_resize: impl Fn() + 'static,
) {
    let on_resize = SendWrapper::new(on_resize);
    let on_resize = StoredValue::new(on_resize);

    Effect::new(move |_| {
        if let Some(el) = element.get() {
            let window = web_sys::window().expect("Window should exist.");
            let raf_id = std::cell::Cell::new(0i32);
            let window_clone = window.clone();

            let resize_callback = Closure::<dyn Fn(web_sys::js_sys::Array)>::new(
                move |_entries: web_sys::js_sys::Array| {
                    window_clone.cancel_animation_frame(raf_id.get()).ok();
                    let on_resize = on_resize;
                    let w = web_sys::window().expect("Window should exist.");
                    let handle = w
                        .request_animation_frame(
                            &Closure::once_into_js(move || {
                                on_resize.with_value(|cb| cb());
                            })
                            .unchecked_into(),
                        )
                        .unwrap_or(0);
                    raf_id.set(handle);
                },
            );

            let observer = web_sys::ResizeObserver::new(resize_callback.as_ref().unchecked_ref())
                .expect("ResizeObserver should be created.");
            observer.observe(el.unchecked_ref::<web_sys::Element>());

            // Prevent the closure from being dropped while the observer is active
            let resize_callback = SendWrapper::new(resize_callback);
            let observer = SendWrapper::new(observer);
            Owner::on_cleanup(move || {
                observer.disconnect();
                drop(resize_callback);
            });
        }
    });
}

/* -------------------------------------------------------------------------------------------------
 * Tests
 * -----------------------------------------------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use super::*;

    // ── to_int ──────────────────────────────────────────────

    #[test]
    fn to_int_pixels() {
        assert_eq!(to_int("8px"), 8.0);
    }

    #[test]
    fn to_int_leading_whitespace() {
        assert_eq!(to_int("  12rem"), 12.0);
    }

    #[test]
    fn to_int_negative() {
        assert_eq!(to_int("-5px"), -5.0);
    }

    #[test]
    fn to_int_positive_sign() {
        assert_eq!(to_int("+3"), 3.0);
    }

    #[test]
    fn to_int_non_numeric() {
        assert_eq!(to_int("abc"), 0.0);
    }

    #[test]
    fn to_int_empty_string() {
        assert_eq!(to_int(""), 0.0);
    }

    #[test]
    fn to_int_zero() {
        assert_eq!(to_int("0"), 0.0);
    }

    // ── get_thumb_ratio ─────────────────────────────────────

    #[test]
    fn thumb_ratio_normal() {
        assert_eq!(get_thumb_ratio(200.0, 400.0), 0.5);
    }

    #[test]
    fn thumb_ratio_viewport_equals_content() {
        assert_eq!(get_thumb_ratio(500.0, 500.0), 1.0);
    }

    #[test]
    fn thumb_ratio_content_zero() {
        // 0/0 is NaN, guard returns 0.0
        assert_eq!(get_thumb_ratio(0.0, 0.0), 0.0);
    }

    // ── get_thumb_size ──────────────────────────────────────

    #[test]
    fn thumb_size_normal() {
        let sizes = Sizes {
            content: 1000.0,
            viewport: 500.0,
            scrollbar: ScrollbarSizes {
                size: 500.0,
                padding_start: 0.0,
                padding_end: 0.0,
            },
        };
        // ratio = 500/1000 = 0.5, thumb = 500 * 0.5 = 250
        assert_eq!(get_thumb_size(&sizes), 250.0);
    }

    #[test]
    fn thumb_size_with_padding() {
        let sizes = Sizes {
            content: 1000.0,
            viewport: 500.0,
            scrollbar: ScrollbarSizes {
                size: 500.0,
                padding_start: 50.0,
                padding_end: 50.0,
            },
        };
        // ratio = 0.5, scrollbar usable = 500 - 100 = 400, thumb = 400 * 0.5 = 200
        assert_eq!(get_thumb_size(&sizes), 200.0);
    }

    #[test]
    fn thumb_size_minimum_enforced() {
        let sizes = Sizes {
            content: 10000.0,
            viewport: 100.0,
            scrollbar: ScrollbarSizes {
                size: 100.0,
                padding_start: 0.0,
                padding_end: 0.0,
            },
        };
        // ratio = 100/10000 = 0.01, thumb = 100 * 0.01 = 1.0, clamped to 18.0
        assert_eq!(get_thumb_size(&sizes), 18.0);
    }

    // ── is_scrolling_within_scrollbar_bounds ─────────────────

    #[test]
    fn scrolling_within_bounds_middle() {
        assert!(is_scrolling_within_scrollbar_bounds(50.0, 100.0));
    }

    #[test]
    fn scrolling_within_bounds_at_zero() {
        assert!(!is_scrolling_within_scrollbar_bounds(0.0, 100.0));
    }

    #[test]
    fn scrolling_within_bounds_at_max() {
        assert!(!is_scrolling_within_scrollbar_bounds(100.0, 100.0));
    }

    #[test]
    fn scrolling_within_bounds_negative() {
        assert!(!is_scrolling_within_scrollbar_bounds(-1.0, 100.0));
    }

    #[test]
    fn scrolling_within_bounds_just_above_zero() {
        assert!(is_scrolling_within_scrollbar_bounds(0.001, 100.0));
    }

    #[test]
    fn scrolling_within_bounds_just_below_max() {
        assert!(is_scrolling_within_scrollbar_bounds(99.999, 100.0));
    }

    // ── get_scroll_position_from_pointer ────────────────────

    fn standard_sizes() -> Sizes {
        Sizes {
            content: 1000.0,
            viewport: 500.0,
            scrollbar: ScrollbarSizes {
                size: 500.0,
                padding_start: 0.0,
                padding_end: 0.0,
            },
        }
    }

    #[test]
    fn scroll_position_from_pointer_ltr_start() {
        let sizes = standard_sizes();
        // thumb_size = 500 * (500/1000) = 250, center = 125
        // min_pointer = 0 + 125 = 125, max_pointer = 500 - 0 - 125 = 375
        // scroll_range = [0, 500], interpolate(125) = 0
        let pos = get_scroll_position_from_pointer(125.0, 0.0, &sizes, Direction::Ltr);
        assert!((pos - 0.0).abs() < 0.01);
    }

    #[test]
    fn scroll_position_from_pointer_ltr_middle() {
        let sizes = standard_sizes();
        // interpolate(250) = midpoint = 250
        let pos = get_scroll_position_from_pointer(250.0, 0.0, &sizes, Direction::Ltr);
        assert!((pos - 250.0).abs() < 0.01);
    }

    #[test]
    fn scroll_position_from_pointer_ltr_end() {
        let sizes = standard_sizes();
        // interpolate(375) = 500
        let pos = get_scroll_position_from_pointer(375.0, 0.0, &sizes, Direction::Ltr);
        assert!((pos - 500.0).abs() < 0.01);
    }

    #[test]
    fn scroll_position_from_pointer_rtl() {
        let sizes = standard_sizes();
        // RTL: scroll_range = [-500, 0]
        // interpolate(125) = -500, interpolate(375) = 0
        let pos = get_scroll_position_from_pointer(125.0, 0.0, &sizes, Direction::Rtl);
        assert!((pos - (-500.0)).abs() < 0.01);
    }

    // ── get_thumb_offset_from_scroll ────────────────────────

    #[test]
    fn thumb_offset_at_zero_scroll() {
        let sizes = standard_sizes();
        let offset = get_thumb_offset_from_scroll(0.0, &sizes, Direction::Ltr);
        assert!((offset - 0.0).abs() < 0.01);
    }

    #[test]
    fn thumb_offset_at_middle_scroll() {
        let sizes = standard_sizes();
        // max_scroll = 500, max_thumb = 500 - 250 = 250
        // interpolate(250) maps [0,500]->[0,250] = 125
        let offset = get_thumb_offset_from_scroll(250.0, &sizes, Direction::Ltr);
        assert!((offset - 125.0).abs() < 0.01);
    }

    #[test]
    fn thumb_offset_at_max_scroll() {
        let sizes = standard_sizes();
        let offset = get_thumb_offset_from_scroll(500.0, &sizes, Direction::Ltr);
        assert!((offset - 250.0).abs() < 0.01);
    }

    #[test]
    fn thumb_offset_rtl_negative_scroll() {
        let sizes = standard_sizes();
        // RTL: clamp_range = [-500, 0], scroll_pos = -250 clamped to -250
        // interpolate maps [0, 500] -> [0, 250], input = -250 → -125
        let offset = get_thumb_offset_from_scroll(-250.0, &sizes, Direction::Rtl);
        assert!((offset - (-125.0)).abs() < 0.01);
    }
}
