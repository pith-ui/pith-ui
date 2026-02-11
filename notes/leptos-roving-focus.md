---
react_location: "[[reference/react-radix-primitives/packages/react/roving-focus/src/roving-focus-group.tsx|roving-focus-group]]"
rust_location: "[[packages/primitives/leptos/roving-focus/src/roving_focus_group.rs|roving_focus_group]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/roving-focus-group.stories.tsx|roving-focus-group]]"
rust_story: ""
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-collection]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-direction]]"
  - "[[leptos-id]]"
  - "[[leptos-primitive]]"
  - "[[leptos-use-callback-ref]]"
  - "[[leptos-use-controllable-state]]"
ported: true
tested: false
tested_story: false
---
## Intent

Implements roving tabindex for composite widgets (toolbars, menus, radio groups). Only one item in the group is tabbable at a time; arrow keys move focus between items. Supports horizontal, vertical, and both orientations, with optional wrapping.

## React API

```ts
const RovingFocusGroup: React.ForwardRefExoticComponent<RovingFocusGroupProps>
const RovingFocusGroupItem: React.ForwardRefExoticComponent<RovingFocusItemProps>
```

Props: `orientation`, `dir`, `loop`, `currentTabStopId`, `defaultCurrentTabStopId`, `onCurrentTabStopIdChange`, `preventScrollOnEntryFocus`, `onEntryFocus`.

## Leptos API

```rust
#[component] fn RovingFocusGroup(...) -> impl IntoView
#[component] fn RovingFocusGroupItem(focusable: MaybeProp<bool>, active: MaybeProp<bool>, ...) -> impl IntoView
```

**Note:** Uses old Leptos API. Needs migration.

## React Implementation Notes

- Uses `Collection` to track items in DOM order.
- `currentTabStopId` tracks which item is tabbable (via `useControllableState`).
- Arrow key navigation uses `Collection.getItems()` to find the next focusable item.
- Dispatches custom `rovingFocusGroup.onEntryFocus` event on group focus.
- Respects `dir` (LTR/RTL) for horizontal arrow key mapping.

## Leptos Implementation Notes

- Uses `CollectionProvider`/`CollectionSlot`/`CollectionItemSlot` for item tracking.
- Arrow key handling mirrors React: filters focusable items, wraps if `loop` is set.
- Custom events via `web_sys::CustomEvent` for entry focus.
- Uses old Leptos API — needs migration.
- Dependencies: `leptos`, `radix-leptos-collection`, `radix-leptos-compose-refs`, `radix-leptos-direction`, `radix-leptos-id`, `radix-leptos-use-controllable-state`, `web-sys`.
