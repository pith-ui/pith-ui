---
react_location: "[[reference/react-radix-primitives/packages/react/use-rect/src/use-rect.tsx|use-rect]]"
rust_location: "[[packages/primitives/leptos/use-rect/src/use_rect.rs|use-rect]]"
react_story: ""
rust_story: ""
dependencies:
  - "[[core-rect]]"
ported: true
tested: false
tested_story: false
---
## Intent

Hook to get an element's bounding rectangle and observe changes over time. Wraps `observeElementRect` from `@radix-ui/rect`.

## React API

```ts
function useRect(measurable: Measurable | null): DOMRect | undefined
```

## Leptos API

```rust
pub fn use_rect(element_ref: AnyNodeRef) -> ReadSignal<Option<Rect>>
```

## React Implementation Notes

- ~25 lines. Thin hook wrapper.
- Uses `React.useState` to store `DOMRect`.
- Sets up observation via `observeElementRect()` from `@radix-ui/rect` (the core rect package).
- Cleans up observer and clears rect on unmount.
- Returns `undefined` while element is null/unmounted.
- Depends on `core-rect` being ported for the underlying observation logic.

## Leptos Implementation Notes

### Parameter type

React takes `Measurable | null`. Leptos takes `AnyNodeRef` — follows the `use_size` pattern. The element is obtained via `element_ref.get()` and cast to `web_sys::Element`.

### Cleanup storage

The cleanup function returned by `observe_element_rect` captures a cloned `Element` (`!Send`). Stored as `Arc<Mutex<Option<SendWrapper<Box<dyn FnOnce()>>>>>` — same pattern as `use_size` uses for its `ResizeObserver`.

### `observe_element_rect` signature fix

The `observe_element_rect` function in `core-rect` required a `+ use<F>` precise capture bound on its return type. Under Rust 2024 lifetime capture rules, the `impl FnOnce()` return type implicitly captured the `&Element` lifetime, even though the function clones the element internally. Added explicit named type parameter `F` and `use<F>` to exclude the reference lifetime from the return type.

### Omissions

None — the Leptos implementation covers the full React API surface.
