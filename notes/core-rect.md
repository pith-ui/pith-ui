---
react_location: "[[reference/react-radix-primitives/packages/core/rect/src/observe-element-rect.ts|observe-element-rect]]"
rust_location: "[[packages/primitives/core/rect/src/observe_element_rect.rs|observe_element_rect]]"
dependencies: []
ported: false
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
pub fn observe_element_rect() {}
```

Stubbed only — empty function with no parameters or body.

## React Implementation Notes

- Uses a module-level `Map<Measurable, ObservedData>` to track all observed elements and their callbacks.
- Runs a single shared `requestAnimationFrame` loop (`runLoop`) that starts when the first element is observed and stops when the last is unobserved.
- Each tick: reads all `getBoundingClientRect()` first (batched DOM reads), then fires callbacks for changed rects (batched DOM writes). This read/write batching avoids layout thrashing.
- Rect equality is checked by comparing all six fields (`width`, `height`, `top`, `right`, `bottom`, `left`).
- Multiple callbacks can be registered per element; the cleanup function removes only its own callback.
- Adapted from `reach/observe-rect`.

## Rust Implementation Notes

Not yet ported. The Rust file contains only an empty stub function.

**Porting considerations:**
- Will need `web_sys` with features for `Window`, `DomRect`, `Element`, `HtmlElement` (or a `Measurable` trait)
- `requestAnimationFrame` is available via `web_sys::Window::request_animation_frame`
- The module-level `Map` could be a `thread_local! { RefCell<HashMap<...>> }` or similar, since WASM is single-threaded
- The cleanup/unsubscribe pattern maps to returning a closure or a guard struct implementing `Drop`
- Rect equality comparison is straightforward with a helper function or `PartialEq` impl
