# MenuArrow

## React Signature

```typescript
const MenuArrow = React.forwardRef<MenuArrowElement, MenuArrowProps>(...)

type MenuArrowElement = React.ComponentRef<typeof PopperPrimitive.Arrow>;
type PopperArrowProps = React.ComponentPropsWithoutRef<typeof PopperPrimitive.Arrow>;
interface MenuArrowProps extends PopperArrowProps {}
```

## Leptos Signature

```rust
pub fn MenuArrow(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the arrow DOM element. |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of the default arrow SVG. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | Custom arrow content. Leptos wraps in `Option`. |
| `width` | -- | `number` (default `10`) | -- | Width of the arrow in pixels. Inherited from `PopperArrowProps`. Not explicitly re-exposed in the Leptos `MenuArrow` signature — set via `attr:width` or PopperArrow props. |
| `height` | -- | `number` (default `5`) | -- | Height of the arrow in pixels. Same as above. |
| *(spread)* | -- | `...PopperArrowProps` | -- | React allows spreading PopperArrow props (SVG attributes). Leptos uses `attr:` directives. |

### Implicit behavior

- Delegates to `PopperArrow`, which renders an SVG arrow element positioned by the Popper positioning engine.
- Must be placed inside `MenuContent` or `MenuSubContent` to function correctly.
