---
react_location: "[[reference/react-radix-primitives/packages/react/slot/src/slot.tsx|slot]]"
rust_location:
react_story: ""
rust_story: ""
dependencies:
  - "[[leptos-compose-refs]]"
ported: true
tested: true
tested_story: false
---
## Intent

Implements the "Slot" pattern — allows components to accept a child element and merge props from parent onto child. This is the foundation of `asChild` in React. In Leptos, this role is handled differently via `TypedFallbackShow` in the `Primitive` component.

## React API

```ts
const Slot: React.ForwardRefExoticComponent<SlotProps>
const Slottable: React.FC
function createSlot(ownerName: string): typeof Slot
function createSlottable(ownerName: string): typeof Slottable
```

## React Implementation Notes

- ~228 lines.
- Core of React Radix's `asChild` pattern: Detects `Slottable` children via symbol-based marker (`__radixId`).
- Merges props: Event handlers composed, styles/classes merged, child overrides parent.
- Handles `React.lazy` components via experimental `React.use()`.
- Clones elements while composing refs.
- React 18/19 ref compatibility handling.
- Special handling for `React.Fragment`.

## Leptos Equivalent

Not needed as a separate package. Leptos handles the `asChild` pattern via `TypedFallbackShow` in the `Primitive` component, which renders children instead of the default element when `as_child` is set. Marked as ported — no code needed.
