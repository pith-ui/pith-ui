# ContextMenuLabel

## React Signature

```typescript
const ContextMenuLabel = React.forwardRef<ContextMenuLabelElement, ContextMenuLabelProps>(...)

type ContextMenuLabelElement = React.ComponentRef<typeof MenuPrimitive.Label>;
type MenuLabelProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.Label>;

interface ContextMenuLabelProps extends MenuLabelProps {}

// MenuLabelProps extends PrimitiveDivProps {}
```

## Leptos Signature

```rust
pub fn ContextMenuLabel(
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
| *(spread)* | — | `...PrimitiveDivProps` | — | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Used as a non-interactive label within a `ContextMenuGroup`. It is not focusable and does not receive `role="menuitem"`.
- The label is purely visual — it does not create an ARIA relationship with its sibling items automatically. Place it as the first child of a `ContextMenuGroup` for a clear visual grouping.
