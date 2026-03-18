# SelectSeparator

## React Signature

```typescript
const SelectSeparator = React.forwardRef<SelectSeparatorElement, SelectSeparatorProps>(...)

type SelectSeparatorElement = React.ComponentRef<typeof Primitive.div>;

interface SelectSeparatorProps extends PrimitiveDivProps {}
```

## Leptos Signature

```rust
pub fn SelectSeparator(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | Optional content inside the separator. |
| *(spread)* | -- | `...PrimitiveDivProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Renders with `aria-hidden="true"` since it is a visual-only separator with no semantic meaning to assistive technology.
