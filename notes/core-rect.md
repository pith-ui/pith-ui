---
react_location: "[[reference/react-radix-primitives/packages/core/rect/src/observe-element-rect.ts|observe-element-rect]]"
rust_location: "[[packages/primitives/core/rect/src/observe_element_rect.rs|observe_element_rect]]"
dependencies: []
ported: true
tested: false
---
## Intent

Efficiently observes an element's bounding rectangle (`getBoundingClientRect`) over time, calling back when it changes. Used by primitives that need to track element position/size (e.g., popper, tooltip positioning).

## React API

```ts
type Measurable = { getBoundingClientRect(): DOMRect }

function observeElementRect(
    elementToObserve: Measurable,
    callback: (rect: DOMRect) => void,
): () => void   // returns unsubscribe function
```

Single export. Returns a cleanup function to stop observing.

## Rust API

```rust
pub trait Measurable {
    fn get_bounding_client_rect(&self) -> DomRect;
}

impl Measurable for Element { ... }

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Rect {
    pub width: f64,
    pub height: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

impl From<DomRect> for Rect { ... }

pub fn observe_element_rect(
    element: &Element,
    callback: impl Fn(Rect) + 'static,
) -> impl FnOnce()
```

## React Implementation Notes

- Uses a module-level `Map<Measurable, ObservedData>` to track all observed elements and their callbacks.
- Runs a single shared `requestAnimationFrame` loop (`runLoop`) that starts when the first element is observed and stops when the last is unobserved.
- Each tick: reads all `getBoundingClientRect()` first (batched DOM reads), then fires callbacks for changed rects (batched DOM writes). This read/write batching avoids layout thrashing.
- Rect equality is checked by comparing all six fields (`width`, `height`, `top`, `right`, `bottom`, `left`).
- Multiple callbacks can be registered per element; the cleanup function removes only its own callback.
- Adapted from `reach/observe-rect`.

## Rust Implementation Notes

**Key design decisions:**

1. **`Rect` struct instead of `DomRect`** — Callbacks receive a plain `Rect { width, height, top, right, bottom, left: f64 }` with derived `PartialEq`. This avoids cloning live JS objects and makes comparison trivial (replaces React's `rectEquals` helper).

2. **`Measurable` trait exported but not used in function signature** — `observe_element_rect` takes `&web_sys::Element` directly (the only WASM type with `getBoundingClientRect`). The `Measurable` trait is exported for downstream use (e.g., virtual refs in Popper).

3. **`Vec` not `HashMap` for entries** — `web_sys::Element` doesn't implement `Hash`. Uses `Vec<(Element, ObservedData)>` with linear scan (typical count: 1–5 elements). Element equality uses JS `===` via `JsValue::eq`.

4. **`CallbackId: u64` for removal** — Rust closures can't be compared by identity. Each registration gets a unique monotonic ID; the cleanup closure captures it for targeted removal.

5. **`Rc<dyn Fn(Rect)>` for callbacks** — Enables cloning callback references out of the `RefCell` borrow before invoking them. Prevents re-entrancy panics if callbacks call `observe_element_rect`.

6. **Three-phase `run_loop`** — (1) Borrow state, batch-read rects, collect changes + clone `Rc` callbacks, release borrow. (2) Fire callbacks (no borrow held). (3) Borrow state, reschedule RAF. This avoids `RefCell` double-borrow panics that would occur if callbacks access `LOOP_STATE`.

7. **`thread_local!` for module state** — WASM is single-threaded, so `thread_local! { RefCell<LoopState> }` replaces the React module-level `Map` and `rafId` variables.

**Omissions:** None — all React exports (`observeElementRect`, `Measurable`) are ported.
