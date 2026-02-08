---
react_location: "[[reference/react-radix-primitives/packages/react/password-toggle-field/src/password-toggle-field.tsx|password-toggle-field]]"
rust_location:
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/password-toggle-field.stories.tsx|password-toggle-field]]"
rust_story: ""
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-id]]"
  - "[[leptos-primitive]]"
  - "[[leptos-use-controllable-state]]"
  - "[[leptos-use-is-hydrated]]"
ported: false
tested: false
---
## Intent

A password visibility toggle component that switches a password input between hidden and visible states while preserving cursor/selection position. Marked as `unstable` in React.

## React API

```ts
// 5 sub-components:
PasswordToggleField, PasswordToggleFieldInput, PasswordToggleFieldToggle,
PasswordToggleFieldSlot, PasswordToggleFieldIcon
```

Props: `visible`, `defaultVisible` (default false), `onVisibilityChange`, `id`.

## React Implementation Notes

- ~480 lines.
- Focus state tracking: Preserves `selectionStart`/`selectionEnd` across visibility toggle.
- Toggle button: Dynamic `aria-label` based on inner text content via `MutationObserver`.
- Hydration-aware: `aria-hidden`/`tabIndex=-1` before hydration, fully interactive after.
- Selection position restoration after type switch using `requestAnimationFrame`.
- Form event handling: Resets visibility to hidden on form submit/reset.
- Pointer event tracking for click detection vs keyboard activation.
- `requestIdleCallback` polyfill for cleanup scheduling.
- `flushSync` for immediate state updates on toggle.
- `PasswordToggleFieldSlot` supports both declarative (`visible`/`hidden` props) and render prop patterns.
- `PasswordToggleFieldIcon` wraps SVG with `aria-hidden` and `asChild`.
