---
react_location: "[[reference/react-radix-primitives/packages/react/select/src/select.tsx|select]]"
rust_location:
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/select.stories.tsx|select]]"
rust_story: ""
dependencies:
  - "[[core-number]]"
  - "[[core-primitive]]"
  - "[[leptos-collection]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-direction]]"
  - "[[leptos-dismissable-layer]]"
  - "[[leptos-focus-guards]]"
  - "[[leptos-focus-scope]]"
  - "[[leptos-id]]"
  - "[[leptos-popper]]"
  - "[[leptos-portal]]"
  - "[[leptos-primitive]]"
  - "[[leptos-slot]]"
  - "[[leptos-use-callback-ref]]"
  - "[[leptos-use-controllable-state]]"
  - "[[leptos-use-previous]]"
  - "[[leptos-visually-hidden]]"
ported: false
tested: false
---
## Intent

A dropdown select component with typeahead search, keyboard navigation, flexible positioning (item-aligned or popper mode), and native form integration. The largest Radix primitive.

## React API

```ts
// 16 sub-components:
Select, SelectTrigger, SelectValue, SelectIcon, SelectPortal,
SelectContent, SelectViewport, SelectGroup, SelectLabel, SelectItem,
SelectItemText, SelectItemIndicator, SelectScrollUpButton,
SelectScrollDownButton, SelectSeparator, SelectArrow
```

Props: `value`, `defaultValue`, `onValueChange`, `open`, `defaultOpen`, `onOpenChange`, `dir`, `name`, `form`, `disabled`, `required`. Content: `position` (`'item-aligned'` | `'popper'`).

## React Implementation Notes

- ~1842 lines — the largest single primitive.
- Two positioning modes: `item-aligned` (selected item overlaps trigger) and `popper` (Floating UI positioning).
- Typeahead search: Buffers key presses with 1-second timeout to jump to matching items.
- Scroll buttons for long lists with auto-scroll on hover.
- Pointer move delta tracking to prevent accidental selection on open.
- Native `<select>` rendered for form submission.
- `FocusScope` for focus management, `DismissableLayer` for dismiss.
- `aria-hidden` others integration.
- Pointer up debouncing: On touch, waits for pointer up before activating selection to prevent accidental taps.
