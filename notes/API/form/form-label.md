# FormLabel

## React Signature

```typescript
const FormLabel = React.forwardRef<FormLabelElement, FormLabelProps>(...)

type FormLabelElement = React.ComponentRef<typeof LabelPrimitive>;
type LabelProps = React.ComponentPropsWithoutRef<typeof LabelPrimitive>;

interface FormLabelProps extends LabelProps {}
```

`LabelPrimitive` is `@radix-ui/react-label`, which renders a `<label>` element and accepts `htmlFor` among its props.

## Leptos Signature

```rust
pub fn FormLabel(
    #[prop(into, optional)] html_for: Option<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `htmlFor` | `html_for` | `string \| undefined` | `Option<String>` | The `for` attribute linking the label to a control. Defaults to the parent `FormField`'s auto-generated `id`, which matches the `FormControl`'s `id`. Override this when using a custom `id` on `FormControl`. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<label>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<label>`, merging props and refs. |
| *(spread)* | -- | `...LabelProps` | -- | React allows spreading any `<label>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-valid` | `"true"` / absent | Present when the field's validity is confirmed valid and `serverInvalid` is not `true`. |
| `data-invalid` | `"true"` / absent | Present when the field's validity is invalid or `serverInvalid` is `true`. |

### Implicit behavior

- Reads `FormFieldContext` to determine the default `htmlFor` value (the field's auto-generated `id`) and the field's `name` for validity lookup.
- Renders as a `Label` primitive (from `@radix-ui/react-label` / the Leptos `label` component), which provides click-to-focus behavior for the associated control.
