# FormSubmit

## React Signature

```typescript
const FormSubmit = React.forwardRef<FormSubmitElement, FormSubmitProps>(...)

type FormSubmitElement = React.ComponentRef<typeof Primitive.button>;
type PrimitiveButtonProps = React.ComponentPropsWithoutRef<typeof Primitive.button>;

interface FormSubmitProps extends PrimitiveButtonProps {}
```

## Leptos Signature

```rust
pub fn FormSubmit(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<button>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<button>`, merging props and refs. |
| *(spread)* | -- | `...PrimitiveButtonProps` | -- | React allows spreading any `<button>` HTML attribute (e.g., `disabled`, `className`). Leptos uses `attr:` directives instead. |

### Implicit behavior

- Renders a `<button>` with `type="submit"` set automatically. The `type` attribute does not need to be specified by the consumer.
- No context is consumed -- this is a thin wrapper around a submit button.
