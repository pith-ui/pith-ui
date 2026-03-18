# SelectIcon

## React Signature

```typescript
const SelectIcon = React.forwardRef<SelectIconElement, SelectIconProps>(...)

type SelectIconElement = React.ComponentRef<typeof Primitive.span>;

interface SelectIconProps extends PrimitiveSpanProps {}
```

## Leptos Signature

```rust
pub fn SelectIcon(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<span>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<span>`, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | Custom icon content. Both React and Leptos default to the "down triangle" character (`▼`) when no children are provided. |
| *(spread)* | -- | `...PrimitiveSpanProps` | -- | React allows spreading any `<span>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Renders with `aria-hidden="true"` since the icon is decorative.
