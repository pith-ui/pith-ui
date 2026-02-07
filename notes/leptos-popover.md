---
react_location: "[[reference/react-radix-primitives/packages/react/popover/src/popover.tsx|popover]]"
rust_location:
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-dismissable-layer]]"
  - "[[leptos-focus-guards]]"
  - "[[leptos-focus-scope]]"
  - "[[leptos-id]]"
  - "[[leptos-popper]]"
  - "[[leptos-portal]]"
  - "[[leptos-presence]]"
  - "[[leptos-primitive]]"
  - "[[leptos-slot]]"
  - "[[leptos-use-controllable-state]]"
ported: false
tested: false
---
## Intent

A popover that displays rich content anchored to a trigger element. Supports focus trapping, outside-click dismissal, scroll prevention, and portal rendering. Similar architecture to Dialog but positioned relative to a trigger.

## React API

```ts
// 7 sub-components:
Popover, PopoverAnchor, PopoverTrigger, PopoverPortal,
PopoverContent, PopoverClose, PopoverArrow
```

Props: `open`, `defaultOpen`, `onOpenChange`, `modal` (default false).

## React Implementation Notes

- ~531 lines.
- Uses `Popper` primitive for positioning content relative to trigger/anchor.
- `DismissableLayer` for escape key and outside-click dismissal.
- `FocusScope` for focus trapping (only when `modal` is true).
- `useFocusGuards` prevents focus from escaping.
- `Presence` for animation-friendly mounting.
- `hideOthers()` from `aria-hidden` when modal.
- `RemoveScroll` prevents body scroll when modal.
- Custom anchor support: Can use `PopoverAnchor` instead of trigger for positioning.
- External dependencies: `react-remove-scroll`, `aria-hidden`.
