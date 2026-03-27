# NavigationMenuViewport

## React Signature

```typescript
type NavigationMenuViewportElement = NavigationMenuViewportImplElement;
type NavigationMenuViewportImplElement = React.ComponentRef<typeof Primitive.div>;

interface NavigationMenuViewportProps extends PrimitiveDivProps {
  /**
   * Used to force mounting when more control is needed. Useful when
   * controlling animation with React animation libraries.
   */
  forceMount?: true;
}

const NavigationMenuViewport = React.forwardRef<
  NavigationMenuViewportElement,
  NavigationMenuViewportProps
>(...)
```

## Leptos Signature

```rust
pub fn NavigationMenuViewport(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` | Forces the viewport to stay mounted even when no content is active. Useful for controlling enter/exit animations. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the viewport DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| *(spread)* | -- | `...PrimitiveDivProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"open" \| "closed"` | Whether any content is currently active. `"open"` when the menu has an active item, `"closed"` when none is active. |
| `data-orientation` | `"horizontal" \| "vertical"` | Reflects the parent navigation menu's `orientation`. |

### Implicit behavior

- The viewport is a **shared rendering area** for all `NavigationMenuContent` components. When a viewport is present, content is not rendered inline next to triggers but instead portaled into the viewport.
- Registers itself in the parent context so `NavigationMenuContent` components detect its existence and register their content data for viewport rendering.
- **Content lifecycle:** The viewport renders all registered content items, each wrapped in `Presence`. Only the active content (or force-mounted content) is present. The last active content value is persisted so content remains mounted during close animations.
- **Pointer events:** When closed and inside a root menu, `pointer-events: none` is applied to prevent interaction during close animations. Also handles `pointerenter` (cancels close timer) and `pointerleave` (starts close timer, mouse only).
- **Size tracking:** Uses `ResizeObserver` on the active content element to measure its dimensions and sets CSS custom properties on the viewport element:
  - `--navigation-menu-viewport-width` -- the width of the active content in pixels.
  - `--navigation-menu-viewport-height` -- the height of the active content in pixels.
- These CSS custom properties enable smooth width/height transitions when switching between content panels of different sizes.
- Signals its existence synchronously during construction (via `has_viewport_component` signal), so content components know to use viewport rendering before any menu interaction occurs. This prevents a flash where content renders inline before the viewport effect fires.
