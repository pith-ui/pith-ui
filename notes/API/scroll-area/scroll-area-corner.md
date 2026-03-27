# ScrollAreaCorner

## React Signature

```typescript
const ScrollAreaCorner = React.forwardRef<ScrollAreaCornerElement, ScrollAreaCornerProps>(...)

type ScrollAreaCornerElement = ScrollAreaCornerImplElement;
interface ScrollAreaCornerProps extends ScrollAreaCornerImplProps {}

type ScrollAreaCornerImplElement = React.ComponentRef<typeof Primitive.div>;
interface ScrollAreaCornerImplProps extends PrimitiveDivProps {}
```

## Leptos Signature

```rust
pub fn ScrollAreaCorner(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the corner DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| *(spread)* | — | `...PrimitiveDivProps` | — | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- The corner is only rendered when **both** a horizontal and vertical scrollbar are present (mounted in the DOM) **and** the scroll area `type` is not `"scroll"`. When `type="scroll"`, no corner is rendered because the scrollbars show/hide independently based on scroll activity.
- The corner is further conditional on having a non-zero size: it only renders after `ResizeObserver` has measured the scrollbar dimensions. If either dimension is zero, the corner is not rendered.
- The corner element is positioned with `position: absolute; bottom: 0` and either `right: 0` (LTR) or `left: 0` (RTL).
- The corner's width is set to match the vertical scrollbar's `offsetWidth`, and its height is set to match the horizontal scrollbar's `offsetHeight`. These dimensions are measured via `ResizeObserver` and update dynamically.
- The corner reports its measured dimensions back to the parent `ScrollArea` context, which uses them to set `--scroll-area-corner-width` and `--scroll-area-corner-height` CSS custom properties on the root element. The scrollbars use these properties to offset themselves so they don't overlap the corner.
