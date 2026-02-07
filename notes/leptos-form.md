---
react_location: "[[reference/react-radix-primitives/packages/react/form/src/form.tsx|form]]"
rust_location:
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-id]]"
  - "[[leptos-label]]"
  - "[[leptos-primitive]]"
ported: false
tested: false
---
## Intent

A form validation component with support for both native HTML validation and custom validation matchers (sync and async). Provides per-field validity state tracking and accessible error messaging.

## React API

```ts
// 7 sub-components:
Form, FormField, FormLabel, FormControl, FormMessage, FormValidityState, FormSubmit
```

Props: `onClearServerErrors`. FormMessage: `match` (native ValidityState key or custom function), `forceMatch`, `name`.

## React Implementation Notes

- ~730 lines.
- Native HTML form validation integration.
- Custom matcher system: Sync and async validation functions.
- Per-field validity state tracking via context.
- Custom error messages with generated IDs for `aria-describedby`.
- First invalid control auto-focused on form submission.
- Server-side error tracking with `data-invalid`/`data-valid` attributes.
- `FormValidityState` render prop exposes `ValidityState` for custom UI.
- Message ID tracking for `aria-describedby` generation on `FormControl`.
- Cross-field validation possible via `FormData` in async matchers.
