# AlertDialogPortal

## React Signature

```typescript
type DialogPortalProps = React.ComponentPropsWithoutRef<typeof DialogPrimitive.Portal>;
interface AlertDialogPortalProps extends DialogPortalProps {}

const AlertDialogPortal: React.FC<AlertDialogPortalProps> = (props) => {
  return <DialogPrimitive.Portal {...portalProps} />;
};
```

The underlying `DialogPortalProps`:

```typescript
interface DialogPortalProps {
  children?: React.ReactNode;
  /** Specify a container element to portal the content into. */
  container?: PortalProps['container'];
  /** Used to force mounting when more control is needed. Useful when controlling animation with React animation libraries. */
  forceMount?: true;
}
```

## Leptos Signature

```rust
pub fn AlertDialogPortal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    #[prop(optional)] container_ref: AnyNodeRef,
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `container` | `container` | `PortalProps['container']` | `MaybeProp<SendWrapper<web_sys::Element>>` | A DOM element to portal the content into. Defaults to `document.body`. |
| -- | `container_ref` | -- | `AnyNodeRef` | Leptos-only alternative to `container`. A node ref that resolves to the portal target element. Useful when the target is a Leptos-managed element. |
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` | Forces the portal children to stay mounted in the DOM even when the dialog is closed. Useful for controlling animations with external animation libraries. When set on the portal, child components (Overlay, Content) inherit this value unless they override it. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | The portal's children (typically Overlay and Content). |

### Implicit behavior

- AlertDialogPortal is a pure pass-through to `DialogPortal`. It does not add any behavior beyond scoping.
- In React, each portal child is individually wrapped in a `Presence` + `Portal`. In Leptos, the portal wraps all children together, and each child (Overlay, Content) manages its own presence/mount lifecycle.
