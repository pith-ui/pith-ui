# MenubarLabel

## React Signature

```typescript
const MenubarLabel = React.forwardRef<MenubarLabelElement, MenubarLabelProps>(...)

type MenubarLabelElement = React.ComponentRef<typeof MenuPrimitive.Label>;
type MenuLabelProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.Label>;

interface MenubarLabelProps extends MenuLabelProps {}

// Where MenuLabelProps extends:
type PrimitiveDivProps = React.ComponentPropsWithoutRef<typeof Primitive.div>;
interface MenuLabelProps extends PrimitiveDivProps {}
```

## Leptos Signature

```rust
pub fn MenubarLabel(
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

- Renders as a `<div>` with no ARIA role. Used as a visual label for a group of menu items.
- Pure pass-through to the underlying `MenuLabel` component.
