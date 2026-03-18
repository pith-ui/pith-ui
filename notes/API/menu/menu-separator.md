# MenuSeparator

## React Signature

```typescript
const MenuSeparator = React.forwardRef<MenuSeparatorElement, MenuSeparatorProps>(...)

type MenuSeparatorElement = React.ComponentRef<typeof Primitive.div>;
interface MenuSeparatorProps extends PrimitiveDivProps {}
```

## Leptos Signature

```rust
pub fn MenuSeparator(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | Optional children. Separators are typically empty. Leptos wraps in `Option`. |
| *(spread)* | -- | `...PrimitiveDivProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives. |

### Implicit behavior

- Renders with `role="separator"` and `aria-orientation="horizontal"`.
- Not focusable — skipped during keyboard navigation.
