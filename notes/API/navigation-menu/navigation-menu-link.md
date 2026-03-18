# NavigationMenuLink

## React Signature

```typescript
type NavigationMenuLinkElement = React.ComponentRef<typeof Primitive.a>;
type PrimitiveLinkProps = React.ComponentPropsWithoutRef<typeof Primitive.a>;

interface NavigationMenuLinkProps extends Omit<PrimitiveLinkProps, 'onSelect'> {
  active?: boolean;
  onSelect?: (event: Event) => void;
}

const NavigationMenuLink = React.forwardRef<NavigationMenuLinkElement, NavigationMenuLinkProps>(...)
```

## Leptos Signature

```rust
pub fn NavigationMenuLink(
    #[prop(into, optional)] active: MaybeProp<bool>,
    #[prop(into, optional)] on_select: Option<Callback<web_sys::Event>>,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `active` | `active` | `boolean` | `MaybeProp<bool>` | Marks this link as the currently active page. When `true`, sets `data-active` and `aria-current="page"` on the element. |
| `onSelect` | `on_select` | `(event: Event) => void` | `Option<Callback<web_sys::Event>>` | Called when the link is selected (clicked). Receives a custom `navigationMenu.linkSelect` event. Call `event.preventDefault()` to prevent the default behavior of closing the menu after selection. |
| -- | `on_click` | *(via spread)* | `Option<Callback<ev::MouseEvent>>` | Composed with the internal click handler. In React, passed via `onClick` spread prop. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<a>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in an `<a>`, merging props and refs. Useful for integrating with a router's link component. |
| *(spread)* | -- | `...Omit<PrimitiveLinkProps, 'onSelect'>` | -- | React allows spreading any `<a>` HTML attribute (e.g., `href`). Leptos uses `attr:` directives instead (e.g., `attr:href`). |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-active` | `""` (present/absent) | Present when the `active` prop is `true`. |

### Implicit behavior

- On click, dispatches a custom `navigationMenu.linkSelect` event on the element. The `onSelect` callback, if provided, is attached as a one-time listener for this event.
- After the `linkSelect` event fires, if it was not `defaultPrevented` and the click was not a meta-click (Cmd/Ctrl+click for new tab), a `navigationMenu.rootContentDismiss` event is dispatched to close the navigation menu.
- Meta-clicks (holding Cmd/Ctrl) do not close the menu, allowing the user to open links in new tabs while keeping the menu open.
- The link is wrapped in a `FocusGroupItem`, making it participate in arrow-key navigation alongside triggers.
