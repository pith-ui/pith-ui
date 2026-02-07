---
react_location: "[[reference/react-radix-primitives/packages/react/dialog/src/dialog.tsx|dialog]]"
rust_location:
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-dismissable-layer]]"
  - "[[leptos-focus-guards]]"
  - "[[leptos-focus-scope]]"
  - "[[leptos-id]]"
  - "[[leptos-portal]]"
  - "[[leptos-presence]]"
  - "[[leptos-primitive]]"
  - "[[leptos-slot]]"
  - "[[leptos-use-controllable-state]]"
ported: false
tested: false
---
## Intent

A fully-featured modal dialog with dual modes (modal and non-modal), focus management, scroll prevention, keyboard handling, and accessibility. Supports animated transitions and flexible portal rendering.

## React API

```ts
// 8 sub-components:
Dialog, DialogTrigger, DialogPortal, DialogOverlay, DialogContent,
DialogTitle, DialogDescription, DialogClose
```

Props: `open`, `defaultOpen`, `onOpenChange`, `modal` (default true).

## React Implementation Notes

- ~592 lines.
- Two implementations: `DialogContentModal` and `DialogContentNonModal`.
- Modal mode: Uses `hideOthers()` from `aria-hidden` library, `FocusScope` with trapping, disables outside pointer events, prevents right-click from closing, auto-focuses trigger on close.
- Non-modal mode: No focus trap, tracks outside interactions to decide whether to focus trigger, special Safari pointerdown/focusin edge case handling.
- Uses `DismissableLayer` for escape key and outside-click dismissal.
- `RemoveScroll` from `react-remove-scroll` to prevent body scroll in modal mode.
- Warning system for missing title (accessibility compliance).
- External dependencies: `react-remove-scroll`, `aria-hidden`.
