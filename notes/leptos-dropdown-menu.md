---
react_location: "[[reference/react-radix-primitives/packages/react/dropdown-menu/src/dropdown-menu.tsx|dropdown-menu]]"
rust_location:
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/dropdown-menu.stories.tsx|dropdown-menu]]"
rust_story: ""
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-menu]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-id]]"
  - "[[leptos-primitive]]"
  - "[[leptos-use-controllable-state]]"
ported: false
tested: false
---
## Intent

A dropdown menu triggered by a button. Wraps the `Menu` primitive with trigger toggle behavior, open/close state management, and keyboard support.

## React API

```ts
// 16 sub-components (mirrors Menu):
DropdownMenu, DropdownMenuTrigger, DropdownMenuPortal, DropdownMenuContent,
DropdownMenuGroup, DropdownMenuLabel, DropdownMenuItem, DropdownMenuCheckboxItem,
DropdownMenuRadioGroup, DropdownMenuRadioItem, DropdownMenuItemIndicator,
DropdownMenuSeparator, DropdownMenuArrow, DropdownMenuSub,
DropdownMenuSubTrigger, DropdownMenuSubContent
```

Props: `open`, `defaultOpen`, `onOpenChange`, `modal` (default true), `dir`.

## React Implementation Notes

- ~566 lines. Thin wrapper over `Menu` primitive.
- Trigger toggles open state on click.
- Trigger supports Space/Enter/ArrowDown to open.
- Content auto-focuses first item when opened via keyboard.
- Most sub-components are direct pass-throughs to Menu equivalents.
- Sets `aria-haspopup="menu"`, `aria-expanded`, `aria-controls` on trigger.
