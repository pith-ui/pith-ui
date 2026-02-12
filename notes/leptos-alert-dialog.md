---
react_location: "[[reference/react-radix-primitives/packages/react/alert-dialog/src/alert-dialog.tsx|alert-dialog]]"
rust_location: "[[packages/primitives/leptos/alert-dialog/src/alert_dialog.rs|alert-dialog]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/alert-dialog.stories.tsx|alert-dialog]]"
rust_story: "[[stories/leptos/src/primitives/alert_dialog.rs|alert-dialog]]"
dependencies:
  - "[[leptos-compose-refs]]"
  - "[[leptos-dialog]]"
  - "[[leptos-primitive]]"
ported: true
tested: false
tested_story: true
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

- `AlertDialogProps` — same as `DialogProps` but with `modal` omitted (always `true`).
- `AlertDialogContentProps` — same as `DialogContentProps` but with `onPointerDownOutside` and `onInteractOutside` omitted (always `preventDefault`).
- `AlertDialogAction` / `AlertDialogCancel` — both wrap `DialogClose`.
- `AlertDialogCancel` registers its ref with `AlertDialogContentContext` for auto-focus on open.

Props match Dialog equivalents. `modal` is always `true`.

## Leptos API

```rust
// 9 components matching React API:
AlertDialog       // wraps Dialog with modal=true
AlertDialogTrigger       // wraps DialogTrigger
AlertDialogPortal        // wraps DialogPortal
AlertDialogOverlay       // wraps DialogOverlay
AlertDialogContent       // wraps DialogContent with role="alertdialog", cancel auto-focus, outside prevention
AlertDialogTitle         // wraps DialogTitle
AlertDialogDescription   // wraps DialogDescription
AlertDialogAction        // wraps DialogClose
AlertDialogCancel        // wraps DialogClose, registers cancel_ref for auto-focus
```

## React Implementation Notes

- ~309 lines. Thin wrapper over Dialog — reuses 95% of Dialog logic.
- Sets `role="alertdialog"` instead of `"dialog"`.
- Forces `modal={true}` (no modeless variant).
- Overrides `onOpenAutoFocus` to auto-focus cancel button via `cancelRef`.
- Prevents all outside interactions (pointer and interact events `preventDefault`).
- `AlertDialogAction` and `AlertDialogCancel` both close the dialog (both wrap `DialogClose`).
- `AlertDialogContentContext` stores `cancelRef` for focus management.
- Uses `createContextScope` for scope isolation — not needed in Leptos (uses Leptos context directly).
- React's `Slottable` wrapper avoids premature `DescriptionWarning` rendering — not applicable in Leptos.
- Development-only `DescriptionWarning` checks for `aria-describedby` — omitted in Leptos port.

## Leptos Implementation Notes

### Key decisions

- **Callback passthrough pattern**: Leptos's `#[prop(into, optional)]` cannot pass `Option<Callback<T>>` through the view! macro to another component's `Option<Callback<T>>` prop. A `wrap_callback` helper converts `Option<Callback<T>>` into a concrete `Callback<T>` (no-op if `None`). This is functionally equivalent because Dialog treats `None` the same as a no-op callback.
- **Context**: Uses a simple Leptos `Provider`/`expect_context` for `AlertDialogContentContextValue` containing `cancel_ref: AnyNodeRef`, matching the React `AlertDialogContentContext` pattern.
- **role override**: A `role` prop was added to `DialogContent` → `DialogContentModal`/`DialogContentNonModal` → `DialogContentImpl`, threaded via a `ContentOptions` struct. AlertDialogContent passes `role="alertdialog"` to override the default `"dialog"` role. Using `attr:role` directly doesn't work because `DialogContentImpl` hardcodes the role attribute on `DismissableLayer`.
- **Deferred cancel auto-focus**: In React, `cancelRef.current` is synchronously available when `onOpenAutoFocus` fires, so the cancel button can be focused immediately with `preventDefault()` (preventing FocusScope's `focus_first`). In Leptos, `use_composed_refs` propagates refs via an Effect that hasn't run yet at callback time (cancel_ref is still `None`). The fix: don't prevent default (let FocusScope focus the first tabbable element, moving focus into the dialog), then use `requestAnimationFrame` to focus the cancel button once the ref is available. This matches the final behavior of React.
- **Deferred `hide_others` in Dialog**: React's `hideOthers` uses `useEffect` (runs after layout/paint), while FocusScope auto-focus uses `useLayoutEffect` (runs before paint). In Leptos, both use `Effect::new` with the same priority. The `hide_others` call in `DialogContentModal` was wrapped in `requestAnimationFrame` to emulate React's `useEffect` timing, ensuring FocusScope moves focus into the dialog before `hide_others` sets `aria-hidden` on outside elements. Without this, a browser warning ("Blocked aria-hidden on an element because its descendant retained focus") occurs.

### Omissions

- **`createContextScope` / `__scopeAlertDialog`**: React's scope isolation system is not needed in Leptos; standard Leptos context providers handle component isolation.
- **`DescriptionWarning`**: React's dev-time warning that checks for `aria-describedby` is omitted. This is a development-only convenience; WASM has no equivalent `process.env.NODE_ENV` check, and the warning provides no runtime behavior.
- **`Slottable` wrapper**: React uses `createSlottable('AlertDialogContent')` to defer `DescriptionWarning` rendering until content is mounted. Since we omit `DescriptionWarning`, the `Slottable` wrapper is also unnecessary.
- **`composeEventHandlers`**: React uses this for `onOpenAutoFocus`. In Leptos, the composition is done manually in the `alert_on_open_auto_focus` callback (call user callback first, check `defaultPrevented`, then schedule deferred cancel focus). The `composeEventHandlers` "skip if user prevented default" behavior is preserved.
