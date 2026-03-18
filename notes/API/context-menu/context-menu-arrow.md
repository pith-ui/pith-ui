# ContextMenuArrow

## React Signature

```typescript
const ContextMenuArrow = React.forwardRef<ContextMenuArrowElement, ContextMenuArrowProps>(...)

type ContextMenuArrowElement = React.ComponentRef<typeof MenuPrimitive.Arrow>;
type MenuArrowProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.Arrow>;

interface ContextMenuArrowProps extends MenuArrowProps {}

// MenuArrowProps extends PopperArrowProps, which extends ArrowProps (PrimitiveSvgProps).
// ArrowProps includes:
//   width?: number (default 10)
//   height?: number (default 5)
//   Plus all SVG element attributes via spread.
```

## Leptos Signature

```rust
pub fn ContextMenuArrow(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `width` | — | `number` (default `10`) | — | The width of the arrow SVG in pixels. Not currently exposed in Leptos. |
| `height` | — | `number` (default `5`) | — | The height of the arrow SVG in pixels. Not currently exposed in Leptos. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered SVG element. |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of the default arrow SVG, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | Optional custom arrow content. Leptos wraps in `Option`. |
| *(spread)* | — | `...PrimitiveSvgProps` | — | React allows spreading any SVG attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Rendered as an SVG element positioned by the popper (floating-ui) along the edge of the content closest to the anchor point.
- The arrow automatically rotates to point toward the anchor based on which side of the anchor the content is placed on.
- Typically used inside `ContextMenuSubContent` rather than `ContextMenuContent`, since the main content is anchored to a point (the right-click location) where an arrow has less visual utility.
