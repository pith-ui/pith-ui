# DropdownMenuPortal

## React Signature

```typescript
type PortalProps = React.ComponentPropsWithoutRef<typeof PortalPrimitive>;

interface MenuPortalProps {
  children?: React.ReactNode;
  /** Specify a container element to portal the content into. */
  container?: PortalProps['container'];
  /** Used to force mounting when more control is needed. Useful when controlling animation with React animation libraries. */
  forceMount?: true;
}

interface DropdownMenuPortalProps extends MenuPortalProps {}

const DropdownMenuPortal: React.FC<DropdownMenuPortalProps> = (props) => { ... };
```

`DropdownMenuPortal` is a plain `React.FC` (not `forwardRef`) since it is a utility wrapper with no DOM element of its own.

## Leptos Signature

```rust
pub fn DropdownMenuPortal(
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
| -- | `container_ref` | -- | `AnyNodeRef` | Leptos-only convenience: a `NodeRef` to the container element, as an alternative to passing a raw `web_sys::Element`. |
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` | Forces the content to stay mounted in the DOM even when closed. Useful when controlling open/close animations with CSS or animation libraries. The `forceMount` propagates to `MenuContent` via context. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | Typically a `DropdownMenuContent` or `DropdownMenuSubContent`. |

### Implicit behavior

- Wraps children in a `Presence` component that controls mount/unmount based on the menu's open state (or `forceMount`).
- Portals the content outside its DOM parent into `container` (or `document.body` by default), preventing CSS stacking context issues.
