# ToastTitle

## React Signature

```typescript
const ToastTitle = React.forwardRef<ToastTitleElement, ToastTitleProps>(...)

type ToastTitleElement = React.ComponentRef<typeof Primitive.div>;
type PrimitiveDivProps = React.ComponentPropsWithoutRef<typeof Primitive.div>;

interface ToastTitleProps extends PrimitiveDivProps {}
```

## Leptos Signature

```rust
pub fn ToastTitle(
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
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | The title text content. |
| *(spread)* | -- | `...PrimitiveDivProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- `ToastTitle` is a purely presentational wrapper. It renders a `<div>` (via `Primitive`) with no additional ARIA attributes or behavior.
- The title text is included in the toast's screen reader announcement via the `ToastAnnounce` system (which reads text content from the DOM tree, excluding elements marked with `data-radix-toast-announce-exclude`).
