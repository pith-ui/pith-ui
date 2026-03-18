# MenubarSeparator

## React Signature

```typescript
const MenubarSeparator = React.forwardRef<MenubarSeparatorElement, MenubarSeparatorProps>(...)

type MenubarSeparatorElement = React.ComponentRef<typeof MenuPrimitive.Separator>;
type MenuSeparatorProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.Separator>;

interface MenubarSeparatorProps extends MenuSeparatorProps {}

// Where MenuSeparatorProps extends:
type PrimitiveDivProps = React.ComponentPropsWithoutRef<typeof Primitive.div>;
interface MenuSeparatorProps extends PrimitiveDivProps {}
```

## Leptos Signature

```rust
pub fn MenubarSeparator(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>` with `role="separator"`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | Optional children. Leptos wraps in `Option`. |
| *(spread)* | -- | `...PrimitiveDivProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Renders as a `<div>` with `role="separator"` and `aria-orientation="horizontal"`.
- Pure pass-through to the underlying `MenuSeparator` component.
