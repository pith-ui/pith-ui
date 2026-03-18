# MenubarArrow

## React Signature

```typescript
const MenubarArrow = React.forwardRef<MenubarArrowElement, MenubarArrowProps>(...)

type MenubarArrowElement = React.ComponentRef<typeof MenuPrimitive.Arrow>;
type MenuArrowProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.Arrow>;

interface MenubarArrowProps extends MenuArrowProps {}

// Where MenuArrowProps extends PopperArrowProps (from PopperPrimitive.Arrow).
```

## Leptos Signature

```rust
pub fn MenubarArrow(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered SVG element. |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of the default arrow SVG, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | Optional custom arrow content. Leptos wraps in `Option`. |
| `width` | -- | `number` (default `10`) | -- | Width of the arrow in pixels. Not directly exposed in the Leptos menubar wrapper; available on the underlying `MenuArrow`/`PopperArrow`. |
| `height` | -- | `number` (default `5`) | -- | Height of the arrow in pixels. Not directly exposed in the Leptos menubar wrapper. |
| *(spread)* | -- | `...PopperArrowProps` | -- | React allows spreading SVG attributes. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Renders an SVG arrow element positioned by the popper. Must be rendered inside `MenubarContent` or `MenubarSubContent`.
- Pure pass-through to the underlying `MenuArrow` component.
