# DialogOverlay

## React Signature

```typescript
const DialogOverlay = React.forwardRef<DialogOverlayElement, DialogOverlayProps>(...)

type DialogOverlayElement = DialogOverlayImplElement; // React.ComponentRef<typeof Primitive.div>

interface DialogOverlayProps extends DialogOverlayImplProps {
  /**
   * Used to force mounting when more control is needed. Useful when
   * controlling animation with React animation libraries.
   */
  forceMount?: true;
}

interface DialogOverlayImplProps extends PrimitiveDivProps {}
```

## Leptos Signature

```rust
pub fn DialogOverlay(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` | Forces the overlay to stay mounted in the DOM even when closed. Falls back to the portal-level `forceMount` if not set. Useful for exit animations. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | Optional children. The overlay can wrap the content (for outer-scrollable patterns where the content scrolls inside the overlay). |
| *(spread)* | -- | `...PrimitiveDivProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"open" \| "closed"` | Reflects the dialog's current open state. |

### Implicit behavior

- **Only renders in modal mode.** When the dialog's `modal` prop is `false`, the overlay renders nothing.
- Sets `pointer-events: auto` as an inline style. This re-enables pointer events that `DialogContent` may prevent (via `DismissableLayer`'s `disableOutsidePointerEvents`), allowing the overlay itself to be scrollable/clickable.
- **Body scroll lock:** The internal `DialogOverlayImpl` activates `use_body_scroll_lock()` (Leptos) / wraps content in `RemoveScroll` (React) to prevent background page scrolling while the modal dialog is open.
- In React, `RemoveScroll` is configured with `allowPinchZoom` and `shards={[contentRef]}` so that the content element remains scrollable even when it is a sibling of the overlay rather than a descendant. The Leptos implementation uses `use_body_scroll_lock()` which sets `overflow: hidden` on the body.
