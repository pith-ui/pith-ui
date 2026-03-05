---
react_location: "[[reference/react-radix-primitives/packages/react/menubar/src/menubar.tsx|menubar]]"
rust_location: "[[packages/primitives/leptos/src/menubar/mod.rs|menubar]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/menubar.stories.tsx|menubar]]"
rust_story: "[[stories/leptos/src/primitives/menubar.rs|menubar]]"
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
ported: true
tested: true
tested_story: true
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

## Leptos Implementation Notes

### Prerequisites added to Menu
- Added `on_entry_focus: Option<Callback<ev::Event>>` prop to `MenuContent`, `MenuRootContentModal`, and `MenuRootContentNonModal` — previously only on `MenuContentImpl`. Menubar needs this to prevent auto-focus when opened via pointer (only focus on keyboard open).
- Added `on_key_down: Option<Callback<ev::KeyboardEvent>>` prop through the same chain — Menubar needs this for arrow key inter-menu navigation.

### Ref patterns
- `was_keyboard_trigger_open_ref` uses `SendWrapper<Rc<Cell<bool>>>` (React `useRef(false)`). Wrapped in `StoredValue` in `MenubarTrigger` to avoid `FnOnce` closure issues.
- `has_interacted_outside_ref` in `MenubarContent` uses the same `SendWrapper<Rc<Cell<bool>>>` pattern.

### Collection scope
- `use_collection::<ItemData>()` is called in `MenubarContent` *before* rendering `<MenuContent>` (which provides its own different-typed Collection). The Leptos Collection system resolves by `ItemData` type, so the menubar's `ItemData` (with `value: String, disabled: bool`) is distinct from Menu's internal `ItemData`.
- Result wrapped in `StoredValue` (established project pattern) to allow access from multiple `Callback::new()` closures without `Send + Sync` issues.

### Data attributes via Effect
- `MenubarSubTrigger` sets `data-radix-menubar-subtrigger` via `Effect` on a composed node ref, since `MenuSubTrigger` doesn't support arbitrary data attribute passthrough.
- `MenubarSubContent` sets `data-radix-menubar-content` the same way.

### `wrapArray` helper
- Ported as `wrap_array<T: Clone>` — rotates a `Vec<T>` starting at a given index for circular navigation.

### `align_offset` omission
- `MenubarSubContent` does not expose `align_offset` because the Leptos `MenuSubContent` component does not have this prop. The React version passes it through to Popper, but the Leptos Menu port doesn't thread it through `MenuSubContent`. This is a minor omission — `align_offset` on sub-content is rarely used in practice.
