---
react_location: "[[reference/react-radix-primitives/packages/react/toggle-group/src/toggle-group.tsx|toggle-group]]"
rust_location:
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/toggle-group.stories.tsx|toggle-group]]"
rust_story: ""
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-direction]]"
  - "[[leptos-primitive]]"
  - "[[leptos-roving-focus]]"
  - "[[leptos-toggle]]"
  - "[[leptos-use-controllable-state]]"
ported: false
tested: false
tested_story: false
---
## Intent

A group of toggle buttons with single or multiple selection mode. Supports roving focus (keyboard navigation), disabled state, and visual/semantic state representation.

## React API

```ts
const ToggleGroup: React.ForwardRefExoticComponent<ToggleGroupProps>
const ToggleGroupItem: React.ForwardRefExoticComponent<ToggleGroupItemProps>
```

`type: 'single' | 'multiple'` (required). Shared props: `disabled`, `rovingFocus` (default true), `loop` (default true), `orientation`, `dir`.

## React Implementation Notes

- ~319 lines.
- Built on Toggle primitive for each item.
- `RovingFocusGroup` for keyboard navigation when `rovingFocus=true`.
- Single mode: Uses `role="radio"` and `aria-checked` for single-selection semantics.
- Multiple mode: Uses standard toggle semantics.
- `ToggleGroupItem` wraps `Toggle` in `RovingFocusGroup.Item` if roving focus is enabled.
- Calls `onItemActivate`/`onItemDeactivate` callbacks based on toggle state changes.
