---
react_location: "[[reference/react-radix-primitives/packages/react/one-time-password-field/src/one-time-password-field.tsx|one-time-password-field]]"
rust_location:
dependencies:
  - "[[core-number]]"
  - "[[core-primitive]]"
  - "[[leptos-collection]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-direction]]"
  - "[[leptos-primitive]]"
  - "[[leptos-roving-focus]]"
  - "[[leptos-use-controllable-state]]"
  - "[[leptos-use-is-hydrated]]"
ported: false
tested: false
---
## Intent

A multi-input OTP (one-time password) field component that handles character-by-character input, paste distribution, auto-submit, and validation. Marked as `unstable` in React.

## React API

```ts
// 3 sub-components:
OneTimePasswordField, OneTimePasswordFieldInput, OneTimePasswordFieldHiddenInput
```

Props: `value`, `defaultValue`, `onValueChange`, `onAutoSubmit`, `autoSubmit` (default false), `validationType` (`'numeric'` | `'alpha'` | `'alphanumeric'` | `'none'`), `type` (`'password'` | `'text'`), `autoComplete`, `disabled`, `readOnly`, `placeholder`, `orientation`, `dir`, `name`, `form`.

## React Implementation Notes

- ~965 lines.
- `Collection` + `RovingFocusGroup` for input management and keyboard navigation.
- Action system: `SET_CHAR`, `CLEAR_CHAR`, `PASTE` actions drive input state changes.
- Validation map with regex patterns and input modes per validation type.
- Paste event handling: Distributes pasted characters across inputs.
- Password manager integration detection for compatibility.
- Custom sanitization function support.
- Form submit on Enter key.
- Hidden input for form submission with concatenated value.
- Auto-focus movement: Moves to next input on character entry, previous on backspace.
