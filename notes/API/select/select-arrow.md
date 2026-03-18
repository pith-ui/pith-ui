# SelectArrow

## React Signature

```typescript
const SelectArrow = React.forwardRef<SelectArrowElement, SelectArrowProps>(...)

type SelectArrowElement = React.ComponentRef<typeof PopperPrimitive.Arrow>;
type PopperArrowProps = React.ComponentPropsWithoutRef<typeof PopperPrimitive.Arrow>;

interface SelectArrowProps extends PopperArrowProps {}
```

## Leptos Signature

```rust
pub fn SelectArrow(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional, default = 10.0.into())] width: Signal<f64>,
    #[prop(into, optional, default = 5.0.into())] height: Signal<f64>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `width` | `width` | `number` (default `10`) | `Signal<f64>` (default `10.0`) | The width of the arrow in pixels. |
| `height` | `height` | `number` (default `5`) | `Signal<f64>` (default `5.0`) | The height of the arrow in pixels. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered arrow DOM element. |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of the default SVG arrow. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | Custom arrow content. |
| *(spread)* | -- | `...PopperArrowProps` | -- | React allows spreading Popper arrow props. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Only rendered when the select is open AND `position="popper"`. When `position="item-aligned"` (the default), the arrow is never shown.
- Delegates to `PopperArrow` for the actual SVG rendering and positioning.
