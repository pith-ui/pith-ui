# PopoverArrow

## React Signature

```typescript
const PopoverArrow = React.forwardRef<PopoverArrowElement, PopoverArrowProps>(...)

type PopoverArrowElement = React.ComponentRef<typeof PopperPrimitive.Arrow>;
type PopperArrowProps = React.ComponentPropsWithoutRef<typeof PopperPrimitive.Arrow>;

interface PopoverArrowProps extends PopperArrowProps {}
```

`PopperArrowProps` extends `ArrowProps`, which extends `PrimitiveSvgProps` -- all standard `<svg>` attributes are accepted (including `width`, `height`, `fill`, etc.).

The underlying `Arrow` component defaults: `width = 10`, `height = 5`.

## Leptos Signature

```rust
pub fn PopoverArrow(
    #[prop(into, optional)] width: MaybeProp<f64>,
    #[prop(into, optional)] height: MaybeProp<f64>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `width` | `width` | `number` (default `10`) | `MaybeProp<f64>` | The width of the arrow in pixels. |
| `height` | `height` | `number` (default `5`) | `MaybeProp<f64>` | The height of the arrow in pixels. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered SVG element. |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of the default SVG arrow, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | Custom arrow content. When using `asChild`, the children replace the entire SVG. |
| *(spread)* | -- | `...PrimitiveSvgProps` | -- | React allows spreading any `<svg>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Positioned automatically by the Popper positioning engine based on the content's `side` placement.
- Renders an SVG triangle pointing toward the trigger by default.
- Hidden automatically (via `visibility: hidden`) when the arrow cannot be centered on the content edge (e.g., when the content is shifted far from the trigger due to collision avoidance).
- Rotated to match the placed side (e.g., rotated 180 degrees when the content appears above the trigger).
