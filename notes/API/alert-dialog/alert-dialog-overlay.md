# AlertDialogOverlay

## React Signature

```typescript
type AlertDialogOverlayElement = React.ComponentRef<typeof DialogPrimitive.Overlay>;
type DialogOverlayProps = React.ComponentPropsWithoutRef<typeof DialogPrimitive.Overlay>;
interface AlertDialogOverlayProps extends DialogOverlayProps {}

const AlertDialogOverlay = React.forwardRef<AlertDialogOverlayElement, AlertDialogOverlayProps>(
  (props, forwardedRef) => {
    return <DialogPrimitive.Overlay {...overlayProps} ref={forwardedRef} />;
  },
);
```

The underlying `DialogOverlayProps`:

```typescript
interface DialogOverlayProps extends DialogOverlayImplProps {
  /** Used to force mounting when more control is needed. Useful when controlling animation with React animation libraries. */
  forceMount?: true;
}

interface DialogOverlayImplProps extends PrimitiveDivProps {}
```

## Leptos Signature

```rust
pub fn AlertDialogOverlay(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` | Forces the overlay to stay mounted in the DOM even when the dialog is closed. Inherits from `AlertDialogPortal`'s `forceMount` if not set. Useful for controlling enter/exit animations. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | Optional children. The overlay typically has no children (it is a backdrop). Leptos wraps in `Option` to allow an empty overlay. |
| *(spread)* | -- | `...PrimitiveDivProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"open" \| "closed"` | Reflects the alert dialog's open state. Inherited from `DialogOverlay`. |

### Implicit behavior

- The overlay only renders when the dialog is modal. Since AlertDialog always forces `modal=true`, the overlay always renders (when the dialog is open or force-mounted).
- Sets `pointer-events: auto` on the rendered element to allow scrolling the overlay even when outside pointer events are disabled on the content.
- Activates body scroll lock (via `RemoveScroll` in React, `use_body_scroll_lock` in Leptos) while mounted, preventing page scrolling behind the overlay.
