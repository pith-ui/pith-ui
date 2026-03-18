# PasswordToggleFieldInput

## React Signature

```typescript
const PasswordToggleFieldInput = React.forwardRef<
  HTMLInputElement,
  PasswordToggleFieldInputProps
>(...)

interface PasswordToggleFieldOwnProps {
  autoComplete?: 'current-password' | 'new-password';
}

interface PasswordToggleFieldInputProps
  extends PasswordToggleFieldOwnProps,
    Omit<PrimitiveInputProps, keyof PasswordToggleFieldOwnProps | 'type'> {
  autoComplete?: 'current-password' | 'new-password';
}
```

The `type` prop is omitted because it is managed internally (toggled between `"password"` and `"text"`).

## Leptos Signature

```rust
pub fn PasswordToggleFieldInput(
    #[prop(into, optional)] auto_complete: MaybeProp<AutoComplete>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] on_blur: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `autoComplete` | `auto_complete` | `'current-password' \| 'new-password'` (default `'current-password'`) | `MaybeProp<AutoComplete>` (default `CurrentPassword`) | The autocomplete hint. Use `"new-password"` for password creation fields and `"current-password"` for login fields. |
| `id` | `id` | `string \| undefined` | `MaybeProp<String>` | The input's `id` attribute. If provided, this overrides the auto-generated ID from the parent `PasswordToggleField`. The parent context is notified of the override so that `aria-controls` on the toggle points to the correct element. |
| `onBlur` | `on_blur` | `React.FocusEventHandler` | `Option<Callback<ev::FocusEvent>>` | Callback for blur events. The component saves the cursor selection on blur (for restoration after toggling); this callback runs before that logic. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered `<input>` DOM element. |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in an `<input>`, merging props and refs. |
| *(spread)* | -- | `...Omit<PrimitiveInputProps, ...>` | -- | React allows spreading any `<input>` HTML attribute (except `type`). Leptos uses `attr:` directives instead. |

### Leptos-only enum

```rust
pub enum AutoComplete {
    CurrentPassword,  // default
    NewPassword,
}
```

### Implicit behavior

- The `type` attribute is managed internally: it is `"password"` when visibility is `false` and `"text"` when visibility is `true`.
- Always renders with `autocapitalize="off"` and `spellcheck="false"` to prevent browser interference with password input.
- On blur, the input saves its `selectionStart` and `selectionEnd` to an internal ref. When the toggle is clicked, this saved position is restored after the input is re-focused.
- Listens for `reset` and `submit` events on the associated `<form>`. On either event, visibility is reset to `false` (password masked). For form reset, this only happens if the event's default action is not prevented.
- Syncs the user-provided `id` prop into the parent context so that the `Toggle`'s `aria-controls` attribute always matches the input's actual `id`.
