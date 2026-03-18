# MenubarPortal

## React Signature

```typescript
type MenuPortalProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.Portal>;

interface MenubarPortalProps extends MenuPortalProps {}

// Where MenuPortalProps is:
interface MenuPortalProps {
  children?: React.ReactNode;
  /** Specify a container element to portal the content into. */
  container?: PortalProps['container'];
  /** Used to force mounting when more control is needed. Useful when controlling animation with React animation libraries. */
  forceMount?: true;
}

const MenubarPortal: React.FC<MenubarPortalProps> = (props) => { ... }
```

`MenubarPortal` is a plain functional component (not `forwardRef`) -- it does not render a DOM element directly.

## Leptos Signature

```rust
pub fn MenubarPortal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    #[prop(optional)] container_ref: AnyNodeRef,
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `container` | `container` | `HTMLElement` | `MaybeProp<SendWrapper<web_sys::Element>>` | The DOM element to portal the content into. Defaults to `document.body`. |
| -- | `container_ref` | -- | `AnyNodeRef` | Leptos-only alternative to `container` that accepts a node ref instead of a raw element. Not present in React. |
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` | Forces the content to stay mounted in the DOM even when closed. Useful when controlling open/close animations with CSS or animation libraries. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | The content to portal (typically `MenubarContent`). |

### Implicit behavior

- Portals its children into the specified container element (or `document.body` by default).
- The `forceMount` propagates to the underlying `Presence` component, keeping the content mounted even when the menu is closed.
