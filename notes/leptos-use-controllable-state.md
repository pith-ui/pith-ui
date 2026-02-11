---
react_location: "[[reference/react-radix-primitives/packages/react/use-controllable-state/src/use-controllable-state.tsx|use-controllable-state]]"
rust_location: "[[packages/primitives/leptos/use-controllable-state/src/use_controllable_state.rs|use_controllable_state]]"
react_story: ""
rust_story: ""
dependencies: []
ported: true
tested: false
tested_story: false
---
## Intent

Manages state that can be either controlled (parent owns the value) or uncontrolled (component owns the value). The standard React pattern for form-like components. Returns a `[value, setValue]` pair that works correctly in both modes.

## React API

```ts
function useControllableState<T>(params: {
  prop?: T;
  defaultProp: T;
  onChange?: (state: T) => void;
  caller?: string;
}): [T, SetStateFn<T>]
```

Also exports `useControllableStateReducer` for more complex state management (reducer-based variant).

## Leptos API

```rust
pub fn use_controllable_state<T: Clone + PartialEq + Send + Sync>(
    params: UseControllableStateParams<T>,
) -> (Signal<Option<T>>, Callback<Option<T>>)

pub struct UseControllableStateParams<T: Send + Sync + 'static> {
    pub prop: MaybeProp<T>,
    pub default_prop: MaybeProp<T>,
    pub on_change: Option<Callback<Option<T>>>,
}
```

Returns `(Signal<Option<T>>, Callback<Option<T>>)` — both value and setter are wrapped in `Option`.

## React Implementation Notes

- Controlled mode: `setValue` calls `onChange` when the new value differs from the prop. Does not update internal state.
- Uncontrolled mode: uses `useState`. An `Effect` fires `onChange` when the internal value changes.
- Dev-mode warning when switching between controlled/uncontrolled (via `caller` parameter).
- `useInsertionEffect` keeps the `onChange` ref current without triggering re-renders.
- Also exports a reducer variant (`useControllableStateReducer`) for complex state with actions.

## Leptos Implementation Notes

- Similar structure: `use_controllable_state` delegates to `use_uncontrolled_state` for the uncontrolled path.
- `is_controlled` is a derived signal checking if `prop` is `Some`.
- The setter `Callback` checks `is_controlled` at call time to decide whether to invoke `on_change` directly or update internal state.
- Uncontrolled state uses a `signal` + `Effect` that fires `on_change` when the value changes, tracked via `prev_value` `RwSignal`.
- No dev-mode controlled/uncontrolled switching warning.
- No reducer variant ported.
- Values are `Option<T>` throughout (React uses `T | undefined`).
- Dependencies: `leptos` only.
