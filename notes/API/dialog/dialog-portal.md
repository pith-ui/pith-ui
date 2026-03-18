# DialogPortal

## React Signature

```typescript
const DialogPortal: React.FC<DialogPortalProps>

type PortalProps = React.ComponentPropsWithoutRef<typeof PortalPrimitive>;

interface DialogPortalProps {
  children?: React.ReactNode;
  /**
   * Specify a container element to portal the content into.
   */
  container?: PortalProps['container'];
  /**
   * Used to force mounting when more control is needed. Useful when
   * controlling animation with React animation libraries.
   */
  forceMount?: true;
}
```

`DialogPortal` is a plain `React.FC` (not `forwardRef`) because it does not render its own DOM element — it portals children into a container.

## Leptos Signature

```rust
pub fn DialogPortal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    #[prop(optional)] container_ref: AnyNodeRef,
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `container` | `container` | `HTMLElement \| null` | `MaybeProp<SendWrapper<web_sys::Element>>` | The DOM element to portal the overlay and content into. Defaults to `document.body`. |
| -- | `container_ref` | -- | `AnyNodeRef` | Leptos-only alternative to `container`. Accepts a node ref that resolves to the container element. Useful when the container is rendered by Leptos and you have a `NodeRef` rather than a raw `web_sys::Element`. |
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` | Forces the portal contents to stay mounted in the DOM even when the dialog is closed. Useful for controlling open/close animations with CSS or animation libraries. Propagated to child `DialogOverlay` and `DialogContent` components via a scoped context. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | The overlay and content to portal. |

### Implicit behavior

- In React, each child is individually wrapped in a `<Presence><Portal>` pair, allowing each child to observe its own animation events independently. In Leptos, the portal always renders and each child (`DialogOverlay`, `DialogContent`) handles its own mount/unmount via its own `Presence` wrapper. This avoids premature unmounting during exit animations.
- The `force_mount` value is stored in a scoped context so that `DialogOverlay` and `DialogContent` can inherit it without requiring the prop to be passed directly.
