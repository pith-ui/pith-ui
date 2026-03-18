# TooltipPortal

## React Signature

```typescript
const TooltipPortal: React.FC<TooltipPortalProps> = (props) => { ... }

type PortalProps = React.ComponentPropsWithoutRef<typeof PortalPrimitive>;

interface TooltipPortalProps {
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

Note: `TooltipPortal` is an `FC` (not `forwardRef`) because it does not render a DOM element itself.

## Leptos Signature

```rust
pub fn TooltipPortal(
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
| -- | `container_ref` | -- | `AnyNodeRef` | Leptos-only alternative to `container`. A node ref that resolves to the container element. Useful when the container is a Leptos-managed element. |
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` | Forces the content to stay mounted in the DOM even when the tooltip is closed. Useful when controlling animation with animation libraries. When set on the portal, `TooltipContent` inherits this value. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | Typically a single `TooltipContent` element. |

### Implicit behavior

- Wraps children in a `Presence` component that controls mounting based on `forceMount || context.open`.
- Portals the content into the specified container (defaults to `document.body`) using `PortalPrimitive` (React) / `ScopedPortal` (Leptos).
- The Leptos `ScopedPortal` bridges context across the portal boundary, re-providing `TooltipContextValue`, `TooltipProviderContextValue`, and the popper scope so that portalled content can still access parent context.
- Provides a `PortalContext` (React) to child `TooltipContent` so it can inherit the portal's `forceMount` value.
