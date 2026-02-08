---
react_location: "[[reference/react-radix-primitives/packages/react/context-menu/src/context-menu.tsx|context-menu]]"
rust_location:
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/context-menu.stories.tsx|context-menu]]"
rust_story: ""
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-menu]]"
  - "[[leptos-primitive]]"
  - "[[leptos-use-callback-ref]]"
  - "[[leptos-use-controllable-state]]"
ported: false
tested: false
---
## Intent

A right-click context menu. Wraps the `Menu` primitive with contextmenu event handling, virtual anchor positioning at the pointer location, and long-press support for touch devices.

## React API

```ts
// 16 sub-components (mirrors Menu):
ContextMenu, ContextMenuTrigger, ContextMenuPortal, ContextMenuContent,
ContextMenuGroup, ContextMenuLabel, ContextMenuItem, ContextMenuCheckboxItem,
ContextMenuRadioGroup, ContextMenuRadioItem, ContextMenuItemIndicator,
ContextMenuSeparator, ContextMenuArrow, ContextMenuSub,
ContextMenuSubTrigger, ContextMenuSubContent
```

Props: `onOpenChange`, `dir`, `modal` (default true).

## React Implementation Notes

- ~601 lines. Wrapper over `Menu` primitive.
- Trigger listens for `contextmenu` event and positions a virtual anchor at pointer coordinates.
- Long-press support for touch: Tracks pointer down/up with 700ms threshold.
- Uses `useCallbackRef` for stable callbacks.
- Prevents native context menu, opens custom menu at `{x, y}` point.
- Content positioned via `Menu.Content` with virtual anchor ref.
- Most sub-components are direct pass-throughs to Menu equivalents.
