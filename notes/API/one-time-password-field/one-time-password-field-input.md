# OneTimePasswordFieldInput

## React Signature

```typescript
const OneTimePasswordFieldInput = React.forwardRef<
  HTMLInputElement,
  OneTimePasswordFieldInputProps
>(...)

interface OneTimePasswordFieldInputProps
  extends Omit<
    Primitive.PrimitivePropsWithRef<'input'>,
    | 'value'
    | 'defaultValue'
    | 'disabled'
    | 'readOnly'
    | 'autoComplete'
    | 'autoFocus'
    | 'form'
    | 'name'
    | 'placeholder'
    | 'type'
  > {
  /** Callback fired when the user input fails native HTML input validation. */
  onInvalidChange?: (character: string) => void;
  /**
   * User-provided index to determine the order of the inputs. Useful for
   * preventing flickering after hydration.
   */
  index?: number;
}
```

Many standard `<input>` props (`value`, `disabled`, `readOnly`, `autoComplete`, `autoFocus`, `form`, `name`, `placeholder`, `type`) are explicitly omitted because they are managed by the parent `OneTimePasswordField` context.

## Leptos Signature

```rust
pub fn OneTimePasswordFieldInput(
    #[prop(into, optional)] on_invalid_change: Option<Callback<String>>,
    #[prop(into, optional)] index: MaybeProp<usize>,
    #[prop(into, optional)] on_focus: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] on_cut: Option<Callback<ev::ClipboardEvent>>,
    #[prop(into, optional)] on_input: Option<Callback<ev::Event>>,
    #[prop(into, optional)] on_change: Option<Callback<ev::Event>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `onInvalidChange` | `on_invalid_change` | `(character: string) => void` | `Option<Callback<String>>` | Callback fired when a user enters a character that fails native HTML input validation (based on the `pattern` attribute set by `validationType`). Receives the invalid character string. |
| `index` | `index` | `number \| undefined` | `MaybeProp<usize>` | Explicit index for this input's position in the sequence. When omitted, the index is determined automatically: React uses collection ordering (after hydration), Leptos uses an auto-incrementing counter at component creation time. Providing an explicit index prevents placeholder flickering during hydration. |
| `onFocus` | `on_focus` | `React.FocusEventHandler` | `Option<Callback<ev::FocusEvent>>` | Callback for focus events. The component always selects the input value on focus; this callback runs before that behavior. |
| `onCut` | `on_cut` | `React.ClipboardEventHandler` | `Option<Callback<ev::ClipboardEvent>>` | Callback for cut events. |
| `onInput` | `on_input` | `React.FormEventHandler` | `Option<Callback<ev::Event>>` | Callback for input events. The component intercepts input to handle character distribution; this callback runs before that logic. |
| `onChange` | `on_change` | `React.ChangeEventHandler` | `Option<Callback<ev::Event>>` | Callback for change events. In React, the internal logic runs in `onChange`; in Leptos, the primary logic runs in `on:input` and `on:change` is a simple passthrough. |
| `onKeyDown` | `on_key_down` | `React.KeyboardEventHandler` | `Option<Callback<ev::KeyboardEvent>>` | Callback for keydown events. The component intercepts keydown for navigation and editing; this callback runs before that logic. |
| `onPointerDown` | `on_pointer_down` | `React.PointerEventHandler` | `Option<Callback<ev::PointerEvent>>` | Callback for pointer down events. The component intercepts this to redirect focus to the correct input; this callback runs before that logic. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered `<input>` DOM element. |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in an `<input>`, merging props and refs. |
| *(spread)* | -- | `...Omit<PrimitiveInputProps, ...>` | -- | React allows spreading any remaining `<input>` attributes. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-radix-otp-input` | `""` | Marker attribute identifying this as an OTP input element. |
| `data-radix-index` | `"0"`, `"1"`, ... | The zero-based index of this input within the field. |

### Implicit behavior

- The input's `type`, `disabled`, `readOnly`, `autoComplete`, `form`, `name`, `placeholder`, and `value` are all inherited from the parent `OneTimePasswordField` context. They cannot be overridden per-input.
- `aria-label` is auto-generated as `"Character N of M"` where N is 1-indexed and M is the total number of inputs.
- `maxLength` is set to 1 for most inputs, but the input that holds the current tab stop gets `maxLength` equal to the total number of inputs to allow password-manager auto-fill.
- `inputMode` and `pattern` are set automatically based on the parent's `validationType` (e.g., `inputMode="numeric"` and `pattern="\d{1}"` for numeric validation).
- Password manager ignore attributes (`data-1p-ignore`, `data-lpignore`, `data-protonpass-ignore`, `data-bwignore`) are set to `"true"` on all inputs except the one that currently holds the tab stop, to prevent multiple password manager popups.
- The input is wrapped in `RovingFocusGroupItem` and `CollectionItemSlot` internally. Only inputs up to and including the first empty position are focusable via roving focus.
- On focus, the input value is automatically selected.
- On pointer down, focus is redirected to the last selectable (fillable) input position to prevent clicking on unfilled future slots.
