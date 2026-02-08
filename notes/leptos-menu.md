---
react_location: "[[reference/react-radix-primitives/packages/react/menu/src/menu.tsx|menu]]"
rust_location: "[[packages/primitives/leptos/menu/src/menu.rs|menu]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/menu.stories.tsx|menu]]"
rust_story: "[[stories/leptos/src/primitives/menu.rs|menu]]"
dependencies:
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
  - "[[leptos-presence]]"
  - "[[leptos-primitive]]"
  - "[[leptos-roving-focus]]"
  - "[[leptos-slot]]"
  - "[[leptos-use-callback-ref]]"
ported: true
tested: false
---
## Intent

The foundational menu primitive. Not used directly — serves as the base for `DropdownMenu`, `ContextMenu`, and `Menubar`. Provides the full menu system: anchoring, portal rendering, content positioning, item navigation, checkbox/radio items, sub-menus, separators, and labels.

## React API

```ts
// 16 sub-components:
Menu, MenuAnchor, MenuPortal, MenuContent, MenuGroup, MenuLabel,
MenuItem, MenuCheckboxItem, MenuRadioGroup, MenuRadioItem,
MenuItemIndicator, MenuSeparator, MenuArrow, MenuSub, MenuSubTrigger, MenuSubContent
```

Largest primitive. ~1400 lines in React.

## Leptos API

Similar set of components (~1060 lines). Uses `Collection` for item tracking, `RovingFocusGroup` for keyboard navigation, `Popper` for positioning, `FocusScope` for focus trapping, `FocusGuards` for edge guards, `DismissableLayer` (types only) for outside interactions.

**Note:** Uses old Leptos API. Needs migration. Depends on `dismissable-layer` which is only type stubs.

## React Implementation Notes

- Complex layered architecture: Menu → Popper → FocusScope → DismissableLayer → RovingFocusGroup → Collection.
- Sub-menus use nested Menu contexts with `MenuSub`/`MenuSubTrigger`/`MenuSubContent`.
- Typeahead search: buffer key presses to jump to matching items.
- `MenuItem` handles pointer enter/leave for highlight state, click for selection.
- Checkbox/Radio items use `useControllableState` and `Presence` for indicator animation.

## Leptos Implementation Notes

- Ports the full React component set. ~1060 lines.
- Limited by `dismissable-layer` being unported — outside-click dismissal is incomplete.
- Uses old Leptos API — needs migration.
- Dependencies: `leptos`, `radix-leptos-collection`, `radix-leptos-compose-refs`, `radix-leptos-direction`, `radix-leptos-dismissable-layer`, `radix-leptos-focus-guards`, `radix-leptos-focus-scope`, `radix-leptos-popper`, `radix-leptos-roving-focus`, `web-sys`.
