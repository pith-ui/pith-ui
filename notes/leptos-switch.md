---
react_location: "[[reference/react-radix-primitives/packages/react/switch/src/switch.tsx|switch]]"
rust_location: "[[packages/primitives/leptos/switch/src/switch.rs|switch]]"
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-primitive]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-use-controllable-state]]"
  - "[[leptos-use-previous]]"
  - "[[leptos-use-size]]"
ported: true
tested: false
---
## Intent

A toggle switch (`on`/`off`) with form integration. Renders a `<button>` with `role="switch"` plus a hidden `<input type="checkbox">` for native form submission.

## React API

```ts
const Switch: React.ForwardRefExoticComponent<SwitchProps>
const SwitchThumb: React.ForwardRefExoticComponent<SwitchThumbProps>
```

Very similar to Checkbox but boolean-only (no indeterminate state).

## Leptos API

```rust
#[component] fn Switch(...) -> impl IntoView
#[component] fn SwitchThumb(...) -> impl IntoView
```

**Note:** Uses old Leptos API. Needs migration.

## React Implementation Notes

- Same `BubbleInput` pattern as Checkbox: hidden input dispatches `click` events for form integration.
- Uses `usePrevious` + `useSize` for the bubble input.
- `data-state`: `checked` | `unchecked`.

## Leptos Implementation Notes

- Nearly identical structure to Checkbox, minus indeterminate state handling.
- `BubbleInput` is the same pattern: hidden `<input>`, dispatches `click` event on change.
- Uses old Leptos API — needs migration.
- Dependencies: `leptos`, `radix-leptos-compose-refs`, `radix-leptos-use-controllable-state`, `radix-leptos-use-previous`, `radix-leptos-use-size`, `web-sys`.
