# DropdownMenuLabel

## React Signature

```typescript
const DropdownMenuLabel = React.forwardRef<DropdownMenuLabelElement, DropdownMenuLabelProps>(...)

type DropdownMenuLabelElement = React.ComponentRef<typeof MenuPrimitive.Label>;
type MenuLabelProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.Label>;

interface DropdownMenuLabelProps extends MenuLabelProps {}
```

`MenuLabelProps` extends `PrimitiveDivProps` -- all standard `<div>` attributes are accepted.

## Leptos Signature

```rust
pub fn DropdownMenuLabel(
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

- Renders as a plain `<div>` with no implicit ARIA role. It is a visual label, not a focusable item.
- Typically used as the first child of a `DropdownMenuGroup` to label the group.
- Not included in keyboard navigation (not a roving focus item).
