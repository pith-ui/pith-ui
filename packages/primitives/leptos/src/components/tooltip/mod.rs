use std::cell::RefCell;
use std::rc::Rc;

use crate::support::compose_refs::use_composed_refs;
use crate::support::dismissable_layer::DismissableLayer;
use crate::support::id::use_id;
use crate::support::popper::{
    Align, Padding, Popper, PopperAnchor, PopperArrow, PopperContent, Side, Sticky,
    UpdatePositionStrategy, provide_popper_scope, use_popper_scope,
};
use crate::support::portal::{ScopedPortal, resolve_force_mount};
use crate::support::presence::Presence;
use crate::support::primitive::{Primitive, compose_callbacks, prop_or, prop_or_default};
use crate::support::use_controllable_state::{UseControllableStateParams, use_controllable_state};
use crate::support::visually_hidden::VisuallyHidden;
use leptos::{
    attribute_interceptor::AttributeInterceptor, context::Provider, ev, html, prelude::*,
};
use leptos_node_ref::AnyNodeRef;
use radix_utils::{Point, get_hull, is_point_in_polygon};
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::closure::Closure;

mod tooltip;
mod tooltip_content;
mod tooltip_provider;

pub use tooltip::*;
pub use tooltip_content::*;
pub use tooltip_provider::*;

/// Shared closure storage that outlives StoredValue disposal.
///
/// During Leptos scope disposal, `StoredValue` contents may be dropped before
/// `on_cleanup` callbacks run. If a `Closure` is dropped while a DOM listener
/// still references it, wasm_bindgen throws "closure invoked after being dropped".
///
/// By storing closures in `Rc<RefCell<...>>` (wrapped in SendWrapper for Send+Sync),
/// both the setup code and the cleanup callback hold Rc clones. Even if one clone is
/// dropped during disposal, the other keeps the Closure alive until cleanup removes
/// the listener.
type ClosureCell<T> = SendWrapper<Rc<RefCell<Option<Closure<T>>>>>;

fn closure_cell<T: ?Sized>() -> ClosureCell<T> {
    SendWrapper::new(Rc::new(RefCell::new(None)))
}

const DEFAULT_DELAY_DURATION: f64 = 700.0;
const TOOLTIP_OPEN: &str = "tooltip.open";

/* -------------------------------------------------------------------------------------------------
 * TooltipProviderContextValue
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy)]
struct TooltipProviderContextValue {
    is_open_delayed: RwSignal<bool>,
    delay_duration: Signal<f64>,
    on_open: Callback<()>,
    on_close: Callback<()>,
    on_pointer_in_transit_change: Callback<bool>,
    is_pointer_in_transit: RwSignal<bool>,
    disable_hoverable_content: Signal<bool>,
}

/* -------------------------------------------------------------------------------------------------
 * TooltipContextValue
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy)]
struct TooltipContextValue {
    content_id: Signal<String>,
    open: Signal<bool>,
    state_attribute: Signal<&'static str>,
    trigger: RwSignal<Option<SendWrapper<web_sys::HtmlElement>>>,
    on_trigger_enter: Callback<()>,
    on_trigger_leave: Callback<()>,
    on_open: Callback<()>,
    on_close: Callback<()>,
    disable_hoverable_content: Signal<bool>,
}

/* -------------------------------------------------------------------------------------------------
 * Shared Helpers
 * -----------------------------------------------------------------------------------------------*/

fn cleanup_grace_area_listeners(
    trigger_el: &Option<SendWrapper<web_sys::HtmlElement>>,
    content_el: &Option<web_sys::HtmlElement>,
    trigger_leave_closure: &ClosureCell<dyn Fn(web_sys::PointerEvent)>,
    content_leave_closure: &ClosureCell<dyn Fn(web_sys::PointerEvent)>,
) {
    if let Some(closure) = trigger_leave_closure.borrow().as_ref() {
        if let Some(trigger) = trigger_el.as_ref() {
            trigger
                .remove_event_listener_with_callback(
                    "pointerleave",
                    closure.as_ref().unchecked_ref(),
                )
                .ok();
        }
    }
    if let Some(closure) = content_leave_closure.borrow().as_ref() {
        if let Some(content) = content_el.as_ref() {
            content
                .remove_event_listener_with_callback(
                    "pointerleave",
                    closure.as_ref().unchecked_ref(),
                )
                .ok();
        }
    }
}

/* -------------------------------------------------------------------------------------------------
 * Geometry Utilities
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Debug)]
struct Rect {
    top: f64,
    bottom: f64,
    left: f64,
    right: f64,
}

impl From<&web_sys::DomRect> for Rect {
    fn from(r: &web_sys::DomRect) -> Self {
        Self {
            top: r.top(),
            bottom: r.bottom(),
            left: r.left(),
            right: r.right(),
        }
    }
}

fn get_exit_side_from_rect(point: &Point, rect: &Rect) -> Side {
    let top = (rect.top - point.y).abs();
    let bottom = (rect.bottom - point.y).abs();
    let right = (rect.right - point.x).abs();
    let left = (rect.left - point.x).abs();

    let min = top.min(bottom).min(right).min(left);

    if (min - left).abs() < f64::EPSILON {
        Side::Left
    } else if (min - right).abs() < f64::EPSILON {
        Side::Right
    } else if (min - top).abs() < f64::EPSILON {
        Side::Top
    } else {
        Side::Bottom
    }
}

fn get_padded_exit_points(exit_point: &Point, exit_side: &Side) -> Vec<Point> {
    let padding = 5.0;
    match exit_side {
        Side::Top => vec![
            Point {
                x: exit_point.x - padding,
                y: exit_point.y + padding,
            },
            Point {
                x: exit_point.x + padding,
                y: exit_point.y + padding,
            },
        ],
        Side::Bottom => vec![
            Point {
                x: exit_point.x - padding,
                y: exit_point.y - padding,
            },
            Point {
                x: exit_point.x + padding,
                y: exit_point.y - padding,
            },
        ],
        Side::Left => vec![
            Point {
                x: exit_point.x + padding,
                y: exit_point.y - padding,
            },
            Point {
                x: exit_point.x + padding,
                y: exit_point.y + padding,
            },
        ],
        Side::Right => vec![
            Point {
                x: exit_point.x - padding,
                y: exit_point.y - padding,
            },
            Point {
                x: exit_point.x - padding,
                y: exit_point.y + padding,
            },
        ],
    }
}

fn get_points_from_rect(rect: &Rect) -> Vec<Point> {
    vec![
        Point {
            x: rect.left,
            y: rect.top,
        },
        Point {
            x: rect.right,
            y: rect.top,
        },
        Point {
            x: rect.right,
            y: rect.bottom,
        },
        Point {
            x: rect.left,
            y: rect.bottom,
        },
    ]
}

/* -------------------------------------------------------------------------------------------------
 * Utils
 * -----------------------------------------------------------------------------------------------*/

fn tooltip_state_attribute(open: bool, was_delayed: bool) -> &'static str {
    match (open, was_delayed) {
        (true, true) => "delayed-open",
        (true, false) => "instant-open",
        (false, _) => "closed",
    }
}

fn set_timeout(f: impl Fn() + 'static, delay: i32) -> i32 {
    let closure = Closure::once_into_js(f);
    web_sys::window()
        .expect("Window should exist.")
        .set_timeout_with_callback_and_timeout_and_arguments_0(closure.unchecked_ref(), delay)
        .expect("setTimeout should succeed.")
}

fn clear_timeout(handle: StoredValue<Option<i32>>) {
    if let Some(id) = handle.get_value() {
        web_sys::window()
            .expect("Window should exist.")
            .clear_timeout_with_handle(id);
        handle.set_value(None);
    }
}

/// Schedules a closure to run in a microtask (after the current synchronous execution).
/// This is used to defer state updates that would cause synchronous DOM mutations during
/// Leptos's delegated event dispatch, matching React 18+'s automatic batching behavior.
fn queue_microtask(f: impl FnOnce() + 'static) {
    let cb = Closure::once_into_js(f);
    web_sys::window()
        .expect("Window should exist.")
        .queue_microtask(cb.unchecked_ref());
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── get_padded_exit_points ──────────────────────────────

    #[test]
    fn padded_exit_top() {
        let exit = Point { x: 50.0, y: 10.0 };
        let points = get_padded_exit_points(&exit, &Side::Top);
        assert_eq!(points.len(), 2);
        // Top exit: x ± 5, y + 5
        assert!((points[0].x - 45.0).abs() < f64::EPSILON);
        assert!((points[0].y - 15.0).abs() < f64::EPSILON);
        assert!((points[1].x - 55.0).abs() < f64::EPSILON);
        assert!((points[1].y - 15.0).abs() < f64::EPSILON);
    }

    #[test]
    fn padded_exit_bottom() {
        let exit = Point { x: 50.0, y: 90.0 };
        let points = get_padded_exit_points(&exit, &Side::Bottom);
        assert_eq!(points.len(), 2);
        // Bottom exit: x ± 5, y - 5
        assert!((points[0].x - 45.0).abs() < f64::EPSILON);
        assert!((points[0].y - 85.0).abs() < f64::EPSILON);
        assert!((points[1].x - 55.0).abs() < f64::EPSILON);
        assert!((points[1].y - 85.0).abs() < f64::EPSILON);
    }

    #[test]
    fn padded_exit_left() {
        let exit = Point { x: 10.0, y: 50.0 };
        let points = get_padded_exit_points(&exit, &Side::Left);
        assert_eq!(points.len(), 2);
        // Left exit: x + 5, y ± 5
        assert!((points[0].x - 15.0).abs() < f64::EPSILON);
        assert!((points[0].y - 45.0).abs() < f64::EPSILON);
        assert!((points[1].x - 15.0).abs() < f64::EPSILON);
        assert!((points[1].y - 55.0).abs() < f64::EPSILON);
    }

    #[test]
    fn padded_exit_right() {
        let exit = Point { x: 90.0, y: 50.0 };
        let points = get_padded_exit_points(&exit, &Side::Right);
        assert_eq!(points.len(), 2);
        // Right exit: x - 5, y ± 5
        assert!((points[0].x - 85.0).abs() < f64::EPSILON);
        assert!((points[0].y - 45.0).abs() < f64::EPSILON);
        assert!((points[1].x - 85.0).abs() < f64::EPSILON);
        assert!((points[1].y - 55.0).abs() < f64::EPSILON);
    }

    // ── get_exit_side_from_rect ─────────────────────────────

    fn rect(top: f64, right: f64, bottom: f64, left: f64) -> Rect {
        Rect {
            top,
            bottom,
            left,
            right,
        }
    }

    #[test]
    fn exit_side_closest_to_left() {
        let r = rect(0.0, 100.0, 100.0, 0.0);
        assert_eq!(
            get_exit_side_from_rect(&Point { x: 1.0, y: 50.0 }, &r),
            Side::Left
        );
    }

    #[test]
    fn exit_side_closest_to_right() {
        let r = rect(0.0, 100.0, 100.0, 0.0);
        assert_eq!(
            get_exit_side_from_rect(&Point { x: 99.0, y: 50.0 }, &r),
            Side::Right
        );
    }

    #[test]
    fn exit_side_closest_to_top() {
        let r = rect(0.0, 100.0, 100.0, 0.0);
        assert_eq!(
            get_exit_side_from_rect(&Point { x: 50.0, y: 1.0 }, &r),
            Side::Top
        );
    }

    #[test]
    fn exit_side_closest_to_bottom() {
        let r = rect(0.0, 100.0, 100.0, 0.0);
        assert_eq!(
            get_exit_side_from_rect(&Point { x: 50.0, y: 99.0 }, &r),
            Side::Bottom
        );
    }

    #[test]
    fn exit_side_corner_prefers_left() {
        // At top-left corner, left distance == top distance, left wins by priority
        let r = rect(0.0, 100.0, 100.0, 0.0);
        assert_eq!(
            get_exit_side_from_rect(&Point { x: 0.0, y: 0.0 }, &r),
            Side::Left
        );
    }

    // ── get_points_from_rect ────────────────────────────────

    #[test]
    fn points_from_rect_corners() {
        let r = rect(10.0, 110.0, 60.0, 10.0);
        let points = get_points_from_rect(&r);
        assert_eq!(points.len(), 4);
        // top-left
        assert!((points[0].x - 10.0).abs() < f64::EPSILON);
        assert!((points[0].y - 10.0).abs() < f64::EPSILON);
        // top-right
        assert!((points[1].x - 110.0).abs() < f64::EPSILON);
        assert!((points[1].y - 10.0).abs() < f64::EPSILON);
        // bottom-right
        assert!((points[2].x - 110.0).abs() < f64::EPSILON);
        assert!((points[2].y - 60.0).abs() < f64::EPSILON);
        // bottom-left
        assert!((points[3].x - 10.0).abs() < f64::EPSILON);
        assert!((points[3].y - 60.0).abs() < f64::EPSILON);
    }

    // ── tooltip_state_attribute ──────────────────────────────

    #[test]
    fn state_delayed_open() {
        assert_eq!(tooltip_state_attribute(true, true), "delayed-open");
    }

    #[test]
    fn state_instant_open() {
        assert_eq!(tooltip_state_attribute(true, false), "instant-open");
    }

    #[test]
    fn state_closed() {
        assert_eq!(tooltip_state_attribute(false, false), "closed");
        assert_eq!(tooltip_state_attribute(false, true), "closed");
    }
}
