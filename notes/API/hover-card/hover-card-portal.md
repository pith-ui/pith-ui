# HoverCardPortal

## React Signature

```typescript
const HoverCardPortal: React.FC<HoverCardPortalProps>

type PortalProps = React.ComponentPropsWithoutRef<typeof PortalPrimitive>;

interface HoverCardPortalProps {
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

`HoverCardPortal` is not a `forwardRef` component -- it renders no DOM element of its own (it delegates to `PortalPrimitive`).

## Leptos Signature

```rust
pub fn HoverCardPortal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    #[prop(optional)] container_ref: AnyNodeRef,
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `container` | `container` | `HTMLElement \| null` | `MaybeProp<SendWrapper<web_sys::Element>>` | The DOM element to portal the content into. Defaults to `document.body`. |
| -- | `container_ref` | -- | `AnyNodeRef` | Leptos-only alternative to `container`. Accepts a `NodeRef` that resolves to the portal container element. Useful when the container is created reactively. |
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` | Forces the content to stay mounted in the DOM even when the hover card is closed. Useful when controlling open/close animations with CSS or animation libraries. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | Should contain `HoverCardContent`. |

### Implicit behavior

- In React, the portal wraps children in a `Presence` component that conditionally mounts based on `forceMount || context.open`. In Leptos, this presence logic is handled by the `ScopedPortal` internal component.
- The portal bridges context across the portal boundary: `HoverCardContextValue` and popper scope are re-provided inside the portal so that `HoverCardContent` can access them.
