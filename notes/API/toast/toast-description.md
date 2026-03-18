# ToastDescription

## React Signature

```typescript
const ToastDescription = React.forwardRef<ToastDescriptionElement, ToastDescriptionProps>(...)

type ToastDescriptionElement = React.ComponentRef<typeof Primitive.div>;
type PrimitiveDivProps = React.ComponentPropsWithoutRef<typeof Primitive.div>;

interface ToastDescriptionProps extends PrimitiveDivProps {}
```

## Leptos Signature

```rust
pub fn ToastDescription(
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | The description text content. |
| *(spread)* | -- | `...PrimitiveDivProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- `ToastDescription` is a purely presentational wrapper. It renders a `<div>` (via `Primitive`) with no additional ARIA attributes or behavior.
- The description text is included in the toast's screen reader announcement via the `ToastAnnounce` system (which reads text content from the DOM tree, excluding elements marked with `data-radix-toast-announce-exclude`).
