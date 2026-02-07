---
react_location: "[[reference/react-radix-primitives/packages/react/alert-dialog/src/alert-dialog.tsx|alert-dialog]]"
rust_location:
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-dialog]]"
  - "[[leptos-slot]]"
ported: false
tested: false
---
## Intent

A specialized dialog for critical user actions (confirmations, warnings). Enforces focus management and dismissal prevention. Thin wrapper over Dialog with `role="alertdialog"`.

## React API

```ts
// 9 sub-components:
AlertDialog, AlertDialogTrigger, AlertDialogPortal, AlertDialogOverlay,
AlertDialogContent, AlertDialogTitle, AlertDialogDescription,
AlertDialogAction, AlertDialogCancel
```

Props match Dialog equivalents. `modal` is always `true`.

## React Implementation Notes

- ~309 lines. Thin wrapper over Dialog — reuses 95% of Dialog logic.
- Sets `role="alertdialog"` instead of `"dialog"`.
- Forces `modal={true}` (no modeless variant).
- Overrides `onOpenAutoFocus` to auto-focus cancel button.
- Prevents all outside interactions (pointer and interact events `preventDefault`).
- `AlertDialogAction` and `AlertDialogCancel` both close the dialog.
- Context stores `cancelRef` for focus management.
