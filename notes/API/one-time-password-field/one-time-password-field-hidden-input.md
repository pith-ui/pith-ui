# OneTimePasswordFieldHiddenInput

## React Signature

```typescript
const OneTimePasswordFieldHiddenInput = React.forwardRef<
  HTMLInputElement,
  OneTimePasswordFieldHiddenInputProps
>(...)

interface OneTimePasswordFieldHiddenInputProps
  extends Omit<
    React.ComponentProps<'input'>,
    | keyof 'value'
    | 'defaultValue'
    | 'type'
    | 'onChange'
    | 'readOnly'
    | 'disabled'
    | 'autoComplete'
    | 'autoFocus'
  > {}
```

## Leptos Signature

```rust
pub fn OneTimePasswordFieldHiddenInput(
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `name` | `name` | `string \| undefined` | `MaybeProp<String>` | The name for form submission. If not provided, falls back to the `name` prop on the parent `OneTimePasswordField`. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the hidden `<input>` DOM element. Also used internally by the parent to locate the associated form. |
| *(spread)* | -- | `...Omit<InputProps, ...>` | -- | React allows spreading additional `<input>` attributes (though most are omitted). Leptos does not expose additional props on this component. |

### Implicit behavior

- Renders as `<input type="hidden" readonly>`. The value is the joined and trimmed string of all input slot values.
- Always has `autocomplete="off"`, `autocapitalize="off"`, and `spellcheck="false"` to prevent browser interference.
- The parent `OneTimePasswordField` uses this element's ref to locate the associated `<form>` element (via `input.form`) when no explicit `form` prop is provided.
- The `name` prop on this element takes precedence over the parent's `name` prop. If neither is set, the hidden input has no name and will not be included in form submission data.
