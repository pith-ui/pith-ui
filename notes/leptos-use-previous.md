---
react_location: "[[reference/react-radix-primitives/packages/react/use-previous/src/use-previous.tsx|use-previous]]"
rust_location: "[[packages/primitives/leptos/use-previous/src/use_previous.rs|use_previous]]"
react_story: ""
rust_story: ""
dependencies: []
ported: true
tested: false
---
## Intent

Tracks the previous value of a reactive value. Returns the value from before the most recent change. Used by components that need to compare current and previous state (e.g., animation direction).

## React API

```ts
function usePrevious<T>(value: T): T
```

Uses a ref holding `{ value, previous }`. On each render, if `value` changed, shifts current to previous.

## Leptos API

```rust
pub fn use_previous<T: Clone + PartialEq + Send + Sync + 'static>(
    value: Signal<T>,
) -> Memo<T>
```

Returns a `Memo<T>` that recomputes when `value` changes.

## React Implementation Notes

- Stores both current and previous in a single ref: `{ value, previous }`.
- Wrapped in `useMemo` keyed on `value` — comparison uses `!==` (referential equality).
- On first render, both current and previous are the initial value.

## Leptos Implementation Notes

- Uses `StoredValue` holding a `(current, previous)` tuple.
- `Memo::new` recomputes when the input signal changes: if the new value differs from current, shifts current to previous and returns the old current.
- Uses `PartialEq` for comparison (value equality, not referential).
- Initial state: both tuple elements are `value.get_untracked()`.
- Dependencies: `leptos` only.
