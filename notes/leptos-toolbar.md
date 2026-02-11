---
react_location: "[[reference/react-radix-primitives/packages/react/toolbar/src/toolbar.tsx|toolbar]]"
rust_location:
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/toolbar.stories.tsx|toolbar]]"
rust_story: ""
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-direction]]"
  - "[[leptos-primitive]]"
  - "[[leptos-roving-focus]]"
  - "[[leptos-separator]]"
  - "[[leptos-toggle-group]]"
ported: false
tested: false
tested_story: false
---
## Intent

A flexible container for grouping related interactive controls (buttons, links, toggle groups) with roving focus and optional visual separators. Commonly used for editor toolbars, formatting controls.

## React API

```ts
// 6 sub-components:
Toolbar, ToolbarButton, ToolbarLink, ToolbarSeparator,
ToolbarToggleGroup, ToolbarToggleItem
```

Props: `orientation` (default `'horizontal'`), `dir`, `loop` (default true).

## React Implementation Notes

- ~250 lines.
- Composition of `RovingFocusGroup` + `ToggleGroup` + `Separator` primitives.
- All children (Button, Link, ToggleGroup items) are `RovingFocusGroup.Item`s.
- `ToolbarLink` adds space key handling (converts to click).
- `ToolbarSeparator` auto-flips orientation perpendicular to toolbar.
- `ToolbarToggleGroup` sets `rovingFocus={false}` to prevent double focus handling (toolbar handles focus).
- `ToolbarToggleItem` bridges: wraps `ToggleGroupItem` in `ToolbarButton`.
- Keyboard navigation handled entirely by `RovingFocusGroup`.
