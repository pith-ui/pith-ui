# MenuPortal

## React Signature

```typescript
interface MenuPortalProps {
  children?: React.ReactNode;
  /** Specify a container element to portal the content into. */
  container?: PortalProps['container'];
  /** Used to force mounting when more control is needed. Useful when controlling animation with React animation libraries. */
  forceMount?: true;
}

const MenuPortal: React.FC<MenuPortalProps> = (props) => { ... }
```

`MenuPortal` is an `FC` (not `forwardRef`) — it does not accept a `ref`.

## Leptos Signature

```rust
pub fn MenuPortal(
    /// Specify a container element to portal the content into.
    #[prop(into, optional)]
    container: MaybeProp<SendWrapper<web_sys::Element>>,
    /// Optional ref for the container element.
    #[prop(optional)]
    container_ref: AnyNodeRef,
    /// Used to force mounting when more control is needed.
    #[prop(into, optional)]
    force_mount: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `container` | `container` | `PortalProps['container']` | `MaybeProp<SendWrapper<web_sys::Element>>` | The DOM element to portal children into. Defaults to `document.body`. |
| -- | `container_ref` | -- | `AnyNodeRef` | Leptos-only. An optional ref for the container element. Not present in React. |
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` | Forces the portal content to stay mounted in the DOM even when the menu is closed. Useful for controlling exit animations. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | The content to portal (typically `MenuContent` or `MenuSubContent`). |

### Implicit behavior

- In React, `MenuPortal` provides a `PortalContext` with `forceMount` that `MenuContent` and `MenuSubContent` inherit. In Leptos, `force_mount` is passed directly through the `ScopedPortal` component.
- The portal wraps its children in a `Presence` component keyed to `forceMount || context.open`, so content can animate out before being removed from the DOM.
