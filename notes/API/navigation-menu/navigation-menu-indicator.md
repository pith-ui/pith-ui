# NavigationMenuIndicator

## React Signature

```typescript
type NavigationMenuIndicatorElement = NavigationMenuIndicatorImplElement;
type NavigationMenuIndicatorImplElement = React.ComponentRef<typeof Primitive.div>;

interface NavigationMenuIndicatorProps extends PrimitiveDivProps {
  /**
   * Used to force mounting when more control is needed. Useful when
   * controlling animation with React animation libraries.
   */
  forceMount?: true;
}

const NavigationMenuIndicator = React.forwardRef<
  NavigationMenuIndicatorElement,
  NavigationMenuIndicatorProps
>(...)
```

## Leptos Signature

```rust
pub fn NavigationMenuIndicator(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` | Forces the indicator to stay mounted even when no item is active. Useful for controlling enter/exit animations. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the indicator DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| *(spread)* | -- | `...PrimitiveDivProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"visible" \| "hidden"` | Whether any navigation menu item is currently active. `"visible"` when a content panel is open, `"hidden"` when none is open. |
| `data-orientation` | `"horizontal" \| "vertical"` | Reflects the parent navigation menu's `orientation`. |

### Implicit behavior

- The indicator is **portaled** into the `NavigationMenuList`'s wrapper `<div>` (the "indicator track"). This means it renders as a sibling of the `<ul>` rather than inside it, keeping it out of the list's flow.
- Uses `ResizeObserver` on both the active trigger element and the indicator track to update positioning when sizes change.
- The indicator is positioned absolutely and uses inline styles:
  - For horizontal orientation: `left: 0`, `width: {triggerWidth}px`, `transform: translateX({triggerOffset}px)`
  - For vertical orientation: `top: 0`, `height: {triggerHeight}px`, `transform: translateY({triggerOffset}px)`
- The indicator waits for position calculation before rendering (no flash at initial position).
- Rendered with `aria-hidden="true"` since it is purely decorative.
- Re-provides parent contexts across the portal boundary so collection-based trigger tracking works correctly.
