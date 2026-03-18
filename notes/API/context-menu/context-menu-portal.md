# ContextMenuPortal

## React Signature

```typescript
const ContextMenuPortal: React.FC<ContextMenuPortalProps> = (props) => { ... }

type MenuPortalProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.Portal>;

interface ContextMenuPortalProps extends MenuPortalProps {}

// MenuPortalProps:
interface MenuPortalProps {
  children?: React.ReactNode;
  /** Specify a container element to portal the content into. */
  container?: Element | DocumentFragment | null;
  /** Used to force mounting when more control is needed. */
  forceMount?: true;
}
```

`ContextMenuPortal` is not a `forwardRef` component — it does not render a ref-able DOM element. It portals its children into a specified container.

## Leptos Signature

```rust
pub fn ContextMenuPortal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    #[prop(optional)] container_ref: AnyNodeRef,
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `container` | `container` | `Element \| DocumentFragment \| null` | `MaybeProp<SendWrapper<web_sys::Element>>` | The DOM element to portal the content into. Defaults to `document.body`. |
| — | `container_ref` | — | `AnyNodeRef` | Leptos-only alternative to `container`. A node ref that resolves to the container element. Useful when the container is rendered by Leptos. |
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` | Forces the portal content to stay mounted in the DOM even when the menu is closed. Useful for controlling open/close animations with CSS or animation libraries. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | The content to portal (typically `ContextMenuContent` or `ContextMenuSubContent`). |
