// Adapted from https://github.com/reach/observe-rect/tree
// MIT license, React Training

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{DomRect, Element};

/// A type that can report its bounding client rect.
pub trait Measurable {
    fn get_bounding_client_rect(&self) -> DomRect;
}

impl Measurable for Element {
    fn get_bounding_client_rect(&self) -> DomRect {
        self.get_bounding_client_rect()
    }
}

/// A plain Rust struct mirroring the six fields of `DOMRect`.
/// Deriving `PartialEq` replaces the React `rectEquals` helper.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Rect {
    pub width: f64,
    pub height: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

impl From<DomRect> for Rect {
    fn from(r: DomRect) -> Self {
        Self {
            width: r.width(),
            height: r.height(),
            top: r.top(),
            right: r.right(),
            bottom: r.bottom(),
            left: r.left(),
        }
    }
}

// ========================================================================
// module internals

type CallbackId = u64;
type CallbackEntry = (CallbackId, Rc<dyn Fn(Rect)>);
type ChangedEntry = (Rect, Vec<Rc<dyn Fn(Rect)>>);

struct ObservedData {
    rect: Rect,
    callbacks: Vec<CallbackEntry>,
}

#[derive(Default)]
struct LoopState {
    next_callback_id: u64,
    entries: Vec<(Element, ObservedData)>,
    raf_id: Option<i32>,
    raf_closure: Option<Closure<dyn FnMut()>>,
}

thread_local! {
    static LOOP_STATE: RefCell<LoopState> = RefCell::new(LoopState::default());
}

/// Observes an element's rectangle on screen (`getBoundingClientRect`).
///
/// This is useful to track elements on the screen and attach other elements
/// that might be in different layers, etc.
///
/// Returns a cleanup function that stops observing when called.
pub fn observe_element_rect<F: Fn(Rect) + 'static>(
    element: &Element,
    callback: F,
) -> impl FnOnce() + use<F> {
    let callback: Rc<dyn Fn(Rect)> = Rc::new(callback);
    let element = element.clone();

    let callback_id = LOOP_STATE.with(|state| {
        let mut state = state.borrow_mut();

        let id = state.next_callback_id;
        state.next_callback_id += 1;

        // Find existing entry for this element.
        let existing = state.entries.iter_mut().find(|(el, _)| el == &element);

        if let Some((_, data)) = existing {
            // Only add a callback for this element as it's already observed.
            data.callbacks.push((id, Rc::clone(&callback)));

            // Immediately invoke the callback with the current rect (matches React behavior
            // when registering an additional callback on an already-observed element).
            let rect = Rect::from(element.get_bounding_client_rect());
            // Clone callback out before dropping borrow to avoid re-entrancy issues.
            let cb = Rc::clone(&callback);
            drop(state);
            cb(rect);
        } else {
            // Add the element to the list of observed elements with its first callback,
            // because this is the first time this element is observed.
            let data = ObservedData {
                rect: Rect::default(),
                callbacks: vec![(id, callback)],
            };
            state.entries.push((element.clone(), data));

            if state.entries.len() == 1 {
                // Start the internal loop once at least 1 element is observed.
                start_loop(&mut state);
            }
        }

        id
    });

    // Return cleanup closure.
    move || {
        LOOP_STATE.with(|state| {
            let mut state = state.borrow_mut();

            let entry = state.entries.iter_mut().find(|(el, _)| *el == element);

            let Some((_, data)) = entry else {
                return;
            };

            // Remove the callback by ID.
            data.callbacks.retain(|(id, _)| *id != callback_id);

            if data.callbacks.is_empty() {
                // Stop observing this element because there are no
                // callbacks registered for it anymore.
                state.entries.retain(|(el, _)| *el != element);

                if state.entries.is_empty() {
                    // Stop the internal loop once no elements are observed anymore.
                    stop_loop(&mut state);
                }
            }
        });
    }
}

fn start_loop(state: &mut LoopState) {
    let closure = Closure::wrap(Box::new(run_loop) as Box<dyn FnMut()>);
    let window = web_sys::window().expect("window should be available");
    let id = window
        .request_animation_frame(closure.as_ref().unchecked_ref())
        .expect("requestAnimationFrame should succeed");

    state.raf_id = Some(id);
    state.raf_closure = Some(closure);
}

fn run_loop() {
    // Three-phase approach to avoid RefCell re-entrancy panics:
    // 1. Borrow state, batch-read rects, collect changes, release borrow.
    // 2. Fire callbacks (no borrow held).
    // 3. Borrow state, reschedule RAF.

    // Phase 1: Read all rects and find changes.
    let changes: Vec<ChangedEntry> = LOOP_STATE.with(|state| {
        let mut state = state.borrow_mut();
        let mut changes = Vec::new();

        for (element, data) in &mut state.entries {
            let new_rect = Rect::from(element.get_bounding_client_rect());

            if data.rect != new_rect {
                data.rect = new_rect.clone();
                let callbacks: Vec<_> =
                    data.callbacks.iter().map(|(_, cb)| Rc::clone(cb)).collect();
                changes.push((new_rect, callbacks));
            }
        }

        changes
    });

    // Phase 2: Fire callbacks outside of any borrow.
    for (rect, callbacks) in &changes {
        for callback in callbacks {
            callback(rect.clone());
        }
    }

    // Phase 3: Reschedule RAF if there are still entries.
    LOOP_STATE.with(|state| {
        let mut state = state.borrow_mut();

        if state.entries.is_empty() {
            return;
        }

        if let Some(ref closure) = state.raf_closure {
            let window = web_sys::window().expect("window should be available");
            let id = window
                .request_animation_frame(closure.as_ref().unchecked_ref())
                .expect("requestAnimationFrame should succeed");
            state.raf_id = Some(id);
        }
    });
}

fn stop_loop(state: &mut LoopState) {
    if let Some(id) = state.raf_id.take()
        && let Some(window) = web_sys::window()
    {
        window.cancel_animation_frame(id).ok();
    }
    state.raf_closure = None;
}
