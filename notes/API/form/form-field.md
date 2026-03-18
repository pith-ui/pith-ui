# FormField

## React Signature

```typescript
const FormField = React.forwardRef<FormFieldElement, FormFieldProps>(...)

type FormFieldElement = React.ComponentRef<typeof Primitive.div>;
type PrimitiveDivProps = React.ComponentPropsWithoutRef<typeof Primitive.div>;

interface FormFieldProps extends PrimitiveDivProps {
  name: string;
  serverInvalid?: boolean;
}
```

## Leptos Signature

```rust
pub fn FormField(
    #[prop(into)] name: String,
    #[prop(into, optional)] server_invalid: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `name` | `name` | `string` (required) | `String` (required, `#[prop(into)]`) | The name of the form field. Must match the `name` attribute on the corresponding `FormControl` input. Used to look up validation state and wire ARIA descriptions. |
| `serverInvalid` | `server_invalid` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | When `true`, marks this field as invalid due to server-side validation. Triggers `data-invalid` on the field, label, and control, and sets `aria-invalid` on the control. Also auto-focuses the first server-invalid control. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the root DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| *(spread)* | -- | `...PrimitiveDivProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-valid` | `"true"` / absent | Present when the field's validity is confirmed valid and `serverInvalid` is not `true`. |
| `data-invalid` | `"true"` / absent | Present when the field's validity is invalid or `serverInvalid` is `true`. |

### Implicit behavior

- Provides a `FormFieldContext` to all descendants containing the field's auto-generated `id`, `name`, and `server_invalid` signal.
- `FormLabel`, `FormControl`, and `FormMessage` use this context to wire up ARIA relationships and validation behavior without explicit prop threading.
- The auto-generated `id` is used by `FormControl` as its default `id` attribute and by `FormLabel` as its default `for` attribute, establishing the label-control association.
