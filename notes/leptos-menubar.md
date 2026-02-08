---
react_location: "[[reference/react-radix-primitives/packages/react/menubar/src/menubar.tsx|menubar]]"
rust_location:
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/menubar.stories.tsx|menubar]]"
rust_story: ""
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-collection]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-direction]]"
  - "[[leptos-id]]"
  - "[[leptos-menu]]"
  - "[[leptos-primitive]]"
  - "[[leptos-roving-focus]]"
  - "[[leptos-use-controllable-state]]"
ported: false
tested: false
---
## Intent

A horizontal menu bar (like a desktop app menu bar). Multiple menus share a roving focus group, with arrow key navigation between top-level triggers and automatic open-on-hover when a menu is already open.

## React API

```ts
// 17 sub-components:
Menubar, MenubarMenu, MenubarTrigger, MenubarPortal, MenubarContent,
MenubarGroup, MenubarLabel, MenubarItem, MenubarCheckboxItem,
MenubarRadioGroup, MenubarRadioItem, MenubarItemIndicator,
MenubarSeparator, MenubarArrow, MenubarSub, MenubarSubTrigger, MenubarSubContent
```

Props: `value`, `defaultValue`, `onValueChange`, `dir`, `loop` (default true).

## React Implementation Notes

- ~765 lines.
- Uses `Collection` to track trigger elements for arrow key navigation.
- `RovingFocusGroup` handles horizontal focus movement between triggers.
- `value` tracks which menu is currently open (by menu name).
- When a menu is open, hovering other triggers opens them immediately.
- Each `MenubarMenu` is a full `Menu` primitive with scope passthrough.
- Arrow key on trigger opens the corresponding menu's content.
- Focus wraps between triggers when `loop` is enabled.
