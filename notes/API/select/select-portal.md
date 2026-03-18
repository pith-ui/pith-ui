# SelectPortal

## React Signature

```typescript
type PortalProps = React.ComponentPropsWithoutRef<typeof PortalPrimitive>;

interface SelectPortalProps {
  children?: React.ReactNode;
  /** Specify a container element to portal the content into. */
  container?: PortalProps['container'];
}

const SelectPortal: React.FC<SelectPortalProps> = (props) => { ... };
```

`SelectPortal` is a functional component (not `forwardRef`), it does not render its own DOM element.

## Leptos Signature

```rust
pub fn SelectPortal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    #[prop(optional)] container_ref: AnyNodeRef,
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `container` | `container` | `PortalProps['container']` | `MaybeProp<SendWrapper<web_sys::Element>>` | The DOM element to portal the content into. Defaults to `document.body`. |
| -- | `container_ref` | -- | `AnyNodeRef` | Leptos-only. A `NodeRef` pointing to the container element, as an alternative to passing a raw element. |
| -- | `force_mount` | -- | `MaybeProp<bool>` | Leptos-only. Forces the portal content to stay mounted even when the select is closed. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | The content to render inside the portal (typically `SelectContent`). |

### Implicit behavior

- The Leptos implementation uses `ScopedPortal` which bridges the `SelectContext`, `PopperScope`, and `CollectionScope` contexts across the portal boundary so that nested components (items, viewport, etc.) can access them.
- React's implementation renders the portal with `asChild` so it does not create a wrapper element.
