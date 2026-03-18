# HoverCardArrow

## React Signature

```typescript
const HoverCardArrow = React.forwardRef<HoverCardArrowElement, HoverCardArrowProps>(...)

type HoverCardArrowElement = React.ComponentRef<typeof PopperPrimitive.Arrow>;
type PopperArrowProps = React.ComponentPropsWithoutRef<typeof PopperPrimitive.Arrow>;
interface HoverCardArrowProps extends PopperArrowProps {}
```

`PopperArrowProps` extends `ArrowProps` (from `@radix-ui/react-arrow`), which extends `PrimitiveSvgProps`. The arrow renders as an `<svg>` element by default. `width` defaults to `10` and `height` defaults to `5`.

## Leptos Signature

```rust
pub fn HoverCardArrow(
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
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<svg>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of the default SVG arrow, merging props and refs. Use this to provide a custom arrow element. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | Custom arrow content. When not provided, a default SVG triangle is rendered. |
| *(spread)* | -- | `...PopperArrowProps` | -- | React allows spreading any `<svg>` HTML attribute (e.g., `style`, `className`). Leptos uses `attr:` directives instead. |

### Implicit behavior

- The arrow is automatically positioned by the popper system based on the content's placement relative to the trigger. Its position updates when the content flips sides due to collision avoidance.
- The arrow is rendered inside a `<span>` wrapper (managed by `PopperArrow`) that handles rotation transforms based on the content's placed side.
- When the arrow would overflow the content's edges (e.g., when the content is shifted due to collisions), it is hidden via `visibility: hidden`.
