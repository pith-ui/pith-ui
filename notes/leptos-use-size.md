---
react_location: "[[reference/react-radix-primitives/packages/react/use-size/src/use-size.tsx|use-size]]"
rust_location: "[[packages/primitives/leptos/use-size/src/use_size.rs|use_size]]"
dependencies: []
ported: true
tested: false
---
## Intent

Observes an element's size using `ResizeObserver` and returns reactive width/height values. Provides an initial measurement from `offsetWidth`/`offsetHeight` for immediate availability.

## React API

```ts
function useSize(element: HTMLElement | null): { width: number; height: number } | undefined
```

Takes a direct element reference. Returns `undefined` when element is null.

## Leptos API

```rust
pub struct Size { pub width: f64, pub height: f64 }

pub fn use_size(element_ref: AnyNodeRef) -> ReadSignal<Option<Size>>
```

Takes an `AnyNodeRef` instead of a raw element.

## React Implementation Notes

- Immediate measurement via `offsetWidth`/`offsetHeight` in `useLayoutEffect`.
- `ResizeObserver` with `box: 'border-box'` for ongoing updates.
- Prefers `borderBoxSize` from entries, falls back to `offsetWidth`/`offsetHeight` for older browsers.
- Returns `undefined` and cleanup when element becomes null.

## Leptos Implementation Notes

- `ResizeObserver` is stored in `Arc<Mutex<Option<SendWrapper<ResizeObserver>>>>` — the mutex and arc are for cleanup access across the effect and `on_cleanup`.
- Initial measurement via `offset_width()`/`offset_height()` inside the effect.
- Only reads `borderBoxSize` from observer entries (no `offsetWidth` fallback for older browsers).
- `Closure` for the resize callback is created inside the effect — may leak if the effect re-runs since the old closure isn't explicitly cleaned up.
- Resets to `None` when the element ref resolves to `None`.
- Dependencies: `leptos`, `leptos-node-ref`, `send_wrapper`, `web-sys` (with ResizeObserver features).
