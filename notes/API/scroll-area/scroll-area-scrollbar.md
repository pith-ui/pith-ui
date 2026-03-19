# ScrollAreaScrollbar

## React Signature

```typescript
const ScrollAreaScrollbar = React.forwardRef<ScrollAreaScrollbarElement, ScrollAreaScrollbarProps>(...)

type ScrollAreaScrollbarElement = ScrollAreaScrollbarVisibleElement;

interface ScrollAreaScrollbarProps extends ScrollAreaScrollbarVisibleProps {
  forceMount?: true;
}

interface ScrollAreaScrollbarVisibleProps
  extends Omit<ScrollAreaScrollbarAxisProps, keyof ScrollAreaScrollbarAxisPrivateProps> {
  orientation?: 'horizontal' | 'vertical';
}
```

The public surface is `ScrollAreaScrollbarProps`. Internally, the component delegates to one of four sub-components (`ScrollAreaScrollbarHover`, `ScrollAreaScrollbarScroll`, `ScrollAreaScrollbarAuto`, `ScrollAreaScrollbarVisible`) based on the parent `ScrollArea`'s `type` value.

## Leptos Signature

```rust
pub fn ScrollAreaScrollbar(
    #[prop(into, optional)] orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `orientation` | `orientation` | `'horizontal' \| 'vertical'` (default `'vertical'`) | `MaybeProp<Orientation>` (default `Vertical`) | Which scroll axis this scrollbar controls. You typically mount two `ScrollAreaScrollbar` instances — one vertical and one horizontal — if the content can scroll in both directions. |
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` (default `false`) | Forces the scrollbar to remain mounted in the DOM even when it would normally be hidden. Useful when controlling show/hide animations with CSS or animation libraries. Does not apply when `type="always"` (the scrollbar is always mounted in that case). |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the scrollbar DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| *(spread)* | — | `...PrimitiveDivProps` | — | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-orientation` | `"horizontal" \| "vertical"` | Reflects the `orientation` prop. Set on the underlying scrollbar element. |
| `data-state` | `"visible" \| "hidden"` | Present when `type` is `"hover"`, `"scroll"`, or `"auto"`. Reflects whether the scrollbar is currently visible. Not present when `type="always"`. Useful for CSS animations on show/hide. |

### Implicit behavior

- Registers the scrollbar's axis (horizontal or vertical) with the parent `ScrollArea` context on mount, and unregisters on unmount. This controls the viewport's `overflow-x`/`overflow-y` values.
- The scrollbar element is positioned with `position: absolute` and inline styles that place it at the appropriate edge of the scroll area (e.g., vertical scrollbar on the right in LTR, left in RTL; horizontal scrollbar at the bottom).
- When `type="hover"`: the scrollbar is shown when the pointer enters the scroll area and hidden after `scrollHideDelay` ms when the pointer leaves.
- When `type="scroll"`: the scrollbar is shown when the user scrolls in the scrollbar's axis direction and hidden after `scrollHideDelay` ms of inactivity. Pointer interaction keeps it visible.
- When `type="auto"`: the scrollbar is shown when content overflows the viewport in the scrollbar's axis and hidden when it does not. Overflow is checked via `ResizeObserver` on both the viewport and content elements.
- When `type="always"`: the scrollbar is always visible. No `Presence` wrapper is used.
- Handles pointer drag scrolling: clicking on the scrollbar track scrolls to that position, and dragging performs continuous scroll tracking. During drag, text selection is suppressed (`webkitUserSelect: none` on `document.body`) and viewport `scroll-behavior` is set to `auto`.
- Handles wheel events on the scrollbar: a document-level wheel listener (passive: false) intercepts wheel events that target the scrollbar and translates them into viewport scroll position changes, preventing page scroll when within bounds.
- Provides a scrollbar context to `ScrollAreaThumb` with shared state: thumb visibility, element ref, and pointer event callbacks.
