# MenuLabel

## React Signature

```typescript
const MenuLabel = React.forwardRef<MenuLabelElement, MenuLabelProps>(...)

type MenuLabelElement = React.ComponentRef<typeof Primitive.div>;
interface MenuLabelProps extends PrimitiveDivProps {}
```

## Leptos Signature

```rust
pub fn MenuLabel(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| *(spread)* | -- | `...PrimitiveDivProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Renders as a plain `<div>` with no ARIA role. It serves as a visual label for a `MenuGroup`. Typically placed as the first child inside a `MenuGroup`.
- Not focusable and not part of the roving focus group — it is skipped during keyboard navigation.
