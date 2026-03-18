# TooltipArrow

## React Signature

```typescript
const TooltipArrow = React.forwardRef<TooltipArrowElement, TooltipArrowProps>(...)

type TooltipArrowElement = React.ComponentRef<typeof PopperPrimitive.Arrow>;
type PopperArrowProps = React.ComponentPropsWithoutRef<typeof PopperPrimitive.Arrow>;

interface TooltipArrowProps extends PopperArrowProps {}
```

`PopperArrowProps` extends `ArrowProps`, which extends `PrimitiveSvgProps` -- all standard `<svg>` HTML attributes are accepted via spread. The underlying `Arrow` component accepts `width` (default `10`) and `height` (default `5`).

## Leptos Signature

```rust
pub fn TooltipArrow(
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
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the arrow SVG element. |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of the default SVG arrow, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | Custom arrow content. When provided, replaces the default SVG polygon arrow. |
| *(spread)* | -- | `...PopperArrowProps` | -- | React allows spreading any `<svg>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Renders nothing when inside the `VisuallyHidden` copy of `TooltipContent`. A `VisuallyHiddenContentContext` is used to detect this and return `null` / `None`, preventing a duplicate arrow from causing positioning issues.
- The arrow is positioned automatically by the Popper positioning engine based on the content's placed side.
