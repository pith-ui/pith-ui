# MenubarGroup

## React Signature

```typescript
const MenubarGroup = React.forwardRef<MenubarGroupElement, MenubarGroupProps>(...)

type MenubarGroupElement = React.ComponentRef<typeof MenuPrimitive.Group>;
type MenuGroupProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.Group>;

interface MenubarGroupProps extends MenuGroupProps {}

// Where MenuGroupProps extends:
type PrimitiveDivProps = React.ComponentPropsWithoutRef<typeof Primitive.div>;
interface MenuGroupProps extends PrimitiveDivProps {}
```

## Leptos Signature

```rust
pub fn MenubarGroup(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>` with `role="group"`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| *(spread)* | -- | `...PrimitiveDivProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Renders as a `<div>` with `role="group"`.
- Pure pass-through to the underlying `MenuGroup` component.
