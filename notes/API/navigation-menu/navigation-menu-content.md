# NavigationMenuContent

## React Signature

```typescript
type NavigationMenuContentElement = NavigationMenuContentImplElement;
type DismissableLayerProps = React.ComponentPropsWithoutRef<typeof DismissableLayer>;

interface NavigationMenuContentProps
  extends Omit<DismissableLayerProps, 'onDismiss' | 'disableOutsidePointerEvents'> {
  /**
   * Used to force mounting when more control is needed. Useful when
   * controlling animation with React animation libraries.
   */
  forceMount?: true;
}

const NavigationMenuContent = React.forwardRef<
  NavigationMenuContentElement,
  NavigationMenuContentProps
>(...)
```

`NavigationMenuContentProps` inherits from `DismissableLayerProps` (minus `onDismiss` and `disableOutsidePointerEvents`), which includes `onEscapeKeyDown`, `onPointerDownOutside`, `onFocusOutside`, and `onInteractOutside`.

## Leptos Signature

```rust
pub fn NavigationMenuContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] on_pointer_enter: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_leave: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] on_interact_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` | Forces the content to stay mounted in the DOM even when its item is not active. Useful when controlling open/close animations with CSS or animation libraries. |
| `onEscapeKeyDown` | `on_escape_key_down` | `(event: KeyboardEvent) => void` | `Option<Callback<web_sys::KeyboardEvent>>` | Called when the Escape key is pressed while content has focus. Called before the content is dismissed. Call `event.preventDefault()` to prevent dismissal. |
| `onPointerDownOutside` | `on_pointer_down_outside` | `(event: CustomEvent) => void` | `Option<Callback<web_sys::CustomEvent>>` | Called when a pointer-down occurs outside the content. By default, clicks on triggers or the viewport are prevented from dismissing. |
| `onFocusOutside` | `on_focus_outside` | `(event: CustomEvent) => void` | `Option<Callback<web_sys::CustomEvent>>` | Called when focus moves outside the content. By default, focus moving within the root navigation menu does not dismiss (prevented internally). |
| `onInteractOutside` | `on_interact_outside` | `(event: CustomEvent) => void` | `Option<Callback<web_sys::CustomEvent>>` | Called for any interaction outside the content (union of pointer-down-outside and focus-outside). |
| -- | `on_pointer_enter` | *(via spread)* | `Option<Callback<ev::PointerEvent>>` | Composed with the internal handler that cancels the close timer. In React, passed via `onPointerEnter` spread prop. |
| -- | `on_pointer_leave` | *(via spread)* | `Option<Callback<ev::PointerEvent>>` | Composed with the internal handler that starts the close timer (mouse only). In React, passed via `onPointerLeave` spread prop. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`, wrapped by `DismissableLayer`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| *(spread)* | -- | `...Omit<DismissableLayerProps, 'onDismiss' \| 'disableOutsidePointerEvents'>` | -- | React allows spreading DismissableLayer props. Leptos surfaces the key callbacks as explicit props. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"open" \| "closed"` | Whether this content is currently active/visible. |
| `data-motion` | `"from-start" \| "from-end" \| "to-start" \| "to-end" \| null` | Animation direction when transitioning between items. `from-*` is for entering content, `to-*` is for leaving content. `null` (absent) on initial open or when closing to nothing. |
| `data-orientation` | `"horizontal" \| "vertical"` | Reflects the parent navigation menu's `orientation`. |

### Implicit behavior

- Sets `id` to an auto-generated value `{baseId}-content-{itemValue}`.
- Sets `aria-labelledby` pointing to the associated trigger's auto-generated ID.
- **Rendering mode:** When a `NavigationMenuViewport` is present in the tree, content is registered with the viewport context and rendered there (portaled). When no viewport exists, content renders inline using `Presence`.
- **Pointer events:** When closed and inside a root menu, `pointer-events: none` is applied to prevent interaction during close animations.
- **Dismiss behavior:** Internally uses `DismissableLayer` with `disableOutsidePointerEvents=false`. On dismiss, dispatches a custom `navigationMenu.rootContentDismiss` event that bubbles up to close the menu and return focus to the trigger.
- **Escape handling:** When Escape is pressed, sets a `wasEscapeClose` flag on the parent item context, which prevents the trigger from immediately re-opening on the next pointer-move.
- **Tab handling:** Tab key within content cycles through tabbable elements; at the edges, focus is sent to the focus proxy element to allow natural tab flow out of the menu.
