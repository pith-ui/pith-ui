# NavigationMenuTrigger

## React Signature

```typescript
type NavigationMenuTriggerElement = React.ComponentRef<typeof Primitive.button>;
type PrimitiveButtonProps = React.ComponentPropsWithoutRef<typeof Primitive.button>;

interface NavigationMenuTriggerProps extends PrimitiveButtonProps {}

const NavigationMenuTrigger = React.forwardRef<
  NavigationMenuTriggerElement,
  NavigationMenuTriggerProps
>(...)
```

## Leptos Signature

```rust
pub fn NavigationMenuTrigger(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_pointer_enter: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_move: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_leave: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `disabled` | `disabled` | `boolean` | `MaybeProp<bool>` | Disables the trigger. When `true`, hover and click interactions are ignored, and the trigger is excluded from keyboard navigation. In React this is part of the spread button props; Leptos surfaces it as an explicit prop. |
| -- | `on_pointer_enter` | *(via spread)* | `Option<Callback<ev::PointerEvent>>` | Composed with the internal pointer-enter handler. In React, passed via `onPointerEnter` spread prop. |
| -- | `on_pointer_move` | *(via spread)* | `Option<Callback<ev::PointerEvent>>` | Composed with the internal pointer-move handler (hover-intent logic). In React, passed via `onPointerMove` spread prop. |
| -- | `on_pointer_leave` | *(via spread)* | `Option<Callback<ev::PointerEvent>>` | Composed with the internal pointer-leave handler. In React, passed via `onPointerLeave` spread prop. |
| -- | `on_click` | *(via spread)* | `Option<Callback<ev::MouseEvent>>` | Composed with the internal click handler (toggle on click). In React, passed via `onClick` spread prop. |
| -- | `on_key_down` | *(via spread)* | `Option<Callback<ev::KeyboardEvent>>` | Composed with the internal keydown handler (arrow-key entry into content). In React, passed via `onKeyDown` spread prop. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<button>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<button>`, merging props and refs. The child must be a focusable, clickable element for accessibility. |
| *(spread)* | -- | `...PrimitiveButtonProps` | -- | React allows spreading any `<button>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"open" \| "closed"` | Whether the associated content is currently visible. |
| `data-disabled` | `""` (present/absent) | Present when the trigger is disabled. |

### Implicit behavior

- The trigger's `id` is auto-generated as `{baseId}-trigger-{itemValue}` and used by `NavigationMenuContent` for `aria-labelledby`.
- Sets `aria-expanded` reflecting the open state.
- Sets `aria-controls` pointing to the associated content element's auto-generated ID.
- Hover-intent: only mouse pointer events (`pointerType === 'mouse'`) trigger open/close via the delay system. Touch events are ignored for hover behavior (touch users use click/tap instead).
- Click toggles the item open/closed. A "was click close" flag prevents the menu from re-opening on the same pointer-move after a click-close.
- When the trigger is focused and the content is open, pressing the entry key (ArrowDown for horizontal, ArrowRight/ArrowLeft for vertical depending on `dir`) moves focus into the content area.
- When open, a visually hidden focus proxy element is rendered after the trigger to enable natural tab-order flow between trigger and content. When a viewport exists, a visually hidden `<span aria-owns={contentId}>` restructures the a11y tree so screen readers associate the trigger with the viewport-rendered content.
