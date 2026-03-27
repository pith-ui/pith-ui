# ScrollAreaThumb

## React Signature

```typescript
const ScrollAreaThumb = React.forwardRef<ScrollAreaThumbElement, ScrollAreaThumbProps>(...)

type ScrollAreaThumbElement = ScrollAreaThumbImplElement;

interface ScrollAreaThumbProps extends ScrollAreaThumbImplProps {
  /**
   * Used to force mounting when more control is needed. Useful when
   * controlling animation with React animation libraries.
   */
  forceMount?: true;
}

type ScrollAreaThumbImplElement = React.ComponentRef<typeof Primitive.div>;
interface ScrollAreaThumbImplProps extends PrimitiveDivProps {}
```

## Leptos Signature

```rust
pub fn ScrollAreaThumb(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` (default `false`) | Forces the thumb to remain mounted in the DOM even when it would normally be hidden (e.g., when viewport size equals or exceeds content size). Useful for controlling animations with CSS or animation libraries. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the thumb DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| *(spread)* | — | `...PrimitiveDivProps` | — | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"visible" \| "hidden"` | Reflects whether the thumb should be visible based on the viewport-to-content ratio. The thumb is `"visible"` when the ratio is between 0 and 1 (exclusive) — meaning content overflows the viewport — and `"hidden"` otherwise. |

### Implicit behavior

- The thumb's size is set via CSS custom properties: `width: var(--scroll-area-thumb-width)` and `height: var(--scroll-area-thumb-height)`. These properties are calculated and set by the parent scrollbar component based on the viewport-to-content size ratio. The minimum thumb size is 18px (matching macOS behavior).
- The thumb's position is updated via `transform: translate3d(...)` applied directly to the element's style. This avoids scroll-linked effects by using `requestAnimationFrame` to track scroll position changes.
- Listens for `scroll` events on the viewport and runs a `requestAnimationFrame` loop during active scrolling to update the thumb position smoothly.
- Handles `pointerdown` on the thumb to begin drag tracking: records the pointer offset within the thumb so dragging feels anchored to the initial click position.
- Handles `pointerup` on the thumb to end drag tracking (resets the pointer offset to zero).
- The thumb is conditionally mounted via `Presence` based on whether the scrollbar has a visible thumb (viewport/content ratio between 0 and 1). The `force_mount` prop bypasses this conditional rendering.
