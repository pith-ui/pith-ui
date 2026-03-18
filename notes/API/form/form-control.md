# FormControl

## React Signature

```typescript
const FormControl = React.forwardRef<FormControlElement, FormControlProps>(...)

type FormControlElement = React.ComponentRef<typeof Primitive.input>;
type PrimitiveInputProps = React.ComponentPropsWithoutRef<typeof Primitive.input>;

interface FormControlProps extends PrimitiveInputProps {}
```

All standard `<input>` HTML attributes are accepted via `PrimitiveInputProps`.

## Leptos Signature

```rust
pub fn FormControl(
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] name: Option<String>,
    #[prop(into, optional)] on_invalid: Option<Callback<ev::Event>>,
    #[prop(into, optional)] on_change: Option<Callback<ev::Event>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `id` | `id` | `string \| undefined` | `Option<String>` | The `id` attribute for the input. Defaults to the parent `FormField`'s auto-generated `id`. Override when you need a specific `id` (and update `FormLabel`'s `htmlFor` to match). |
| `name` | `name` | `string \| undefined` | `Option<String>` | The `name` attribute for the input. Defaults to the parent `FormField`'s `name` prop. |
| `onInvalid` | `on_invalid` | `React.FormEventHandler` | `Option<Callback<ev::Event>>` | Called when the control's `invalid` event fires (during form submission if the control fails validation). The component also runs its own validity update logic after this callback. |
| `onChange` | `on_change` | `React.ChangeEventHandler` | `Option<Callback<ev::Event>>` | Called when the user modifies the control's value. In React, this fires on the `input` event (React's synthetic `onChange`). In Leptos, this is wired to the native `input` event. The component clears existing validation after this callback runs. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<input>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in an `<input>`, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | Optional children. Typically unused since `<input>` is a void element, but supported for `asChild` usage. |
| *(spread)* | -- | `...PrimitiveInputProps` | -- | React allows spreading any `<input>` attribute (`type`, `required`, `min`, `max`, `pattern`, etc.). Leptos uses `attr:` directives (e.g., `attr:r#type="email"`, `attr:required=""`). |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-valid` | `"true"` / absent | Present when the field's validity is confirmed valid and `serverInvalid` is not `true`. |
| `data-invalid` | `"true"` / absent | Present when the field's validity is invalid or `serverInvalid` is `true`. |

### Implicit behavior

- Sets `title=""` on the `<input>` to disable the browser's default behavior of showing built-in validation tooltips on hover.
- Reads `FormFieldContext` for default `id` and `name` values and the field's `serverInvalid` state.
- Sets `aria-invalid="true"` when the field is server-invalid.
- Sets `aria-describedby` to a space-separated list of IDs from all rendered `FormMessage` components for this field (managed via `AriaDescriptionContext`).
- Registers a native `change` event listener (not `input`) to trigger validation when the user finishes editing. This is a UX decision: validation runs on blur/commit, not on every keystroke.
- Registers a native `input` event listener to clear existing validation when the user starts editing again.
- Registers a `reset` event listener on the parent `<form>` to clear validation and custom validity when the form is reset.
- When `serverInvalid` becomes `true`, checks if this control is the first invalid control in the form and auto-focuses it if so.
- Runs the full validation pipeline on `invalid` events:
  1. Checks built-in HTML5 constraint violations first.
  2. If no built-in errors, runs sync custom matchers.
  3. If no sync custom errors, runs async custom matchers.
  4. Updates the field's validity state and custom errors in the `ValidationContext`.
