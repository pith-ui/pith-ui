# DropdownMenuArrow

## React Signature

```typescript
const DropdownMenuArrow = React.forwardRef<DropdownMenuArrowElement, DropdownMenuArrowProps>(...)

type DropdownMenuArrowElement = React.ComponentRef<typeof MenuPrimitive.Arrow>;
type MenuArrowProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.Arrow>;

interface DropdownMenuArrowProps extends MenuArrowProps {}
```

`MenuArrowProps` extends `PopperArrowProps`, which extends `ArrowProps` (from `@radix-ui/react-arrow`). The arrow renders an SVG element.

## Leptos Signature

```rust
pub fn DropdownMenuArrow(
    #[prop(into, optional)] width: MaybeProp<f64>,
    #[prop(into, optional)] height: MaybeProp<f64>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `width` | `width` | `number` (default `10`) | `MaybeProp<f64>` | The width of the arrow in pixels. |
| `height` | `height` | `number` (default `5`) | `MaybeProp<f64>` | The height of the arrow in pixels. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element. |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of the default SVG arrow, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | Optional custom arrow content. Wrapped in `Option` in Leptos. |
| *(spread)* | -- | `...ArrowProps` | -- | React allows spreading SVG attributes. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Renders an SVG arrow (triangle) pointing toward the trigger, positioned by the Popper.
- Must be placed as a child of `DropdownMenuContent` (or `DropdownMenuSubContent`).
- The arrow's position is controlled by the `arrowPadding` prop on `DropdownMenuContent`.
