---
react_location: "[[reference/react-radix-primitives/packages/react/toggle/src/toggle.tsx|toggle]]"
rust_location: "[[packages/primitives/leptos/toggle/src/toggle.rs|toggle]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/toggle.stories.tsx|toggle]]"
rust_story: "[[stories/leptos/src/primitives/toggle.rs|toggle]]"
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-primitive]]"
  - "[[leptos-use-controllable-state]]"
ported: true
tested: false
tested_story: false
---
## Intent

A two-state button that can be either on or off. Simpler than Switch — no form integration, no hidden input, just a `<button>` with `aria-pressed`.

## React API

```ts
interface ToggleProps extends PrimitiveButtonProps {
  pressed?: boolean;
  defaultPressed?: boolean;
  onPressedChange?(pressed: boolean): void;
}
```

## Leptos API

```rust
#[component]
fn Toggle(
    pressed: MaybeProp<bool>,
    default_pressed: MaybeProp<bool>,
    on_pressed_change: Option<Callback<bool>>,
    disabled: MaybeProp<bool>,
    ...
) -> impl IntoView
```

**Note:** Uses old Leptos API. Needs migration.

## React Implementation Notes

- Simple: `useControllableState` for pressed, `composeEventHandlers` for click.
- `data-state`: `on` | `off`.
- Respects `disabled` — won't toggle when disabled.

## Leptos Implementation Notes

- Faithful port. Uses `compose_callbacks` and `use_controllable_state`.
- Disabled check is in the click handler (same as React).
- Uses old Leptos API — needs migration.
- Dependencies: `leptos`, `radix-leptos-use-controllable-state`.
