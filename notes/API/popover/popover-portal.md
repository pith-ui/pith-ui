# PopoverPortal

## React Signature

```typescript
const PopoverPortal: React.FC<PopoverPortalProps>

type PortalProps = React.ComponentPropsWithoutRef<typeof PortalPrimitive>;

interface PopoverPortalProps {
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

`PopoverPortal` is a plain `React.FC` -- it does not forward a ref or render its own DOM element.

## Leptos Signature

```rust
pub fn PopoverPortal(
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
| -- | `container_ref` | -- | `AnyNodeRef` | Leptos-only alternative to `container`. A node ref that resolves to the portal target element. |
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` | Forces the content to stay mounted in the DOM even when the popover is closed. Useful when controlling open/close animations with CSS or animation libraries -- without this, the element is removed from the DOM when closed and exit animations cannot run. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | Should contain a `PopoverContent`. |

### Implicit behavior

- In React, wraps children in `Presence` (for mount/unmount animation) and `PortalPrimitive`. The Leptos version uses `ScopedPortal` which handles both portalling and context bridging (re-providing `PopoverContextValue` and popper scope inside the portal).
- When `forceMount` is set, the content is always present in the DOM regardless of the popover's open state.
- The `forceMount` value is passed down via context so that `PopoverContent` can also read it.
