---
react_location: "[[reference/react-radix-primitives/packages/react/radio-group/src/radio-group.tsx|radio-group]]"
rust_location:
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/radio-group.stories.tsx|radio-group]]"
rust_story: ""
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-direction]]"
  - "[[leptos-presence]]"
  - "[[leptos-primitive]]"
  - "[[leptos-roving-focus]]"
  - "[[leptos-use-controllable-state]]"
  - "[[leptos-use-previous]]"
  - "[[leptos-use-size]]"
ported: false
tested: false
---
## Intent

A group of mutually exclusive radio buttons with roving focus keyboard navigation, form integration, and flexible composition. Enforces single-selection semantics.

## React API

```ts
// From radio-group.tsx + radio.tsx:
const RadioGroup: React.ForwardRefExoticComponent<RadioGroupProps>
const RadioGroupItem: React.ForwardRefExoticComponent<RadioGroupItemProps>
const RadioGroupIndicator: React.ForwardRefExoticComponent<RadioGroupIndicatorProps>
```

Props: `name`, `required`, `disabled`, `value`, `defaultValue`, `onValueChange`, `orientation`, `dir`, `loop`.

## React Implementation Notes

- ~441 lines across two files (`radio-group.tsx` + `radio.tsx`).
- Two-file structure: `radio-group.tsx` orchestrates, `radio.tsx` provides the Radio button primitive.
- `RovingFocusGroup` for keyboard navigation.
- Form integration via `RadioBubbleInput` — hidden input behind button, uses property descriptor trick to set checked state.
- Arrow key tracking: On focus from arrow key, auto-clicks the radio to check it.
- Enter key explicitly prevented per WAI-ARIA spec.
- Accessibility: `aria-checked`, `data-state`, `role="radio"`.
- `RadioGroupIndicator` supports `forceMount` via `Presence`.
