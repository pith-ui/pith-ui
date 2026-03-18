# HoverCardTrigger

## React Signature

```typescript
const HoverCardTrigger = React.forwardRef<HoverCardTriggerElement, HoverCardTriggerProps>(...)

type HoverCardTriggerElement = React.ComponentRef<typeof Primitive.a>;
type PrimitiveLinkProps = React.ComponentPropsWithoutRef<typeof Primitive.a>;
interface HoverCardTriggerProps extends PrimitiveLinkProps {}
```

The trigger renders as a `Primitive.a` (anchor element) by default.

## Leptos Signature

```rust
pub fn HoverCardTrigger(
    #[prop(into, optional)] on_pointer_enter: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_leave: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_focus: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] on_blur: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] on_touch_start: Option<Callback<ev::TouchEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `onPointerEnter` | `on_pointer_enter` | `(event: PointerEvent) => void` | `Option<Callback<ev::PointerEvent>>` | Event handler composed with the internal pointer-enter handler. The internal handler opens the hover card (touch events are excluded). |
| `onPointerLeave` | `on_pointer_leave` | `(event: PointerEvent) => void` | `Option<Callback<ev::PointerEvent>>` | Event handler composed with the internal pointer-leave handler. The internal handler starts the close timer (touch events are excluded). |
| `onFocus` | `on_focus` | `(event: FocusEvent) => void` | `Option<Callback<ev::FocusEvent>>` | Event handler composed with the internal focus handler that opens the hover card. |
| `onBlur` | `on_blur` | `(event: BlurEvent) => void` | `Option<Callback<ev::FocusEvent>>` | Event handler composed with the internal blur handler that starts the close timer. |
| `onTouchStart` | `on_touch_start` | `(event: TouchEvent) => void` | `Option<Callback<ev::TouchEvent>>` | Event handler composed with the internal handler that calls `preventDefault()` to suppress focus events on touch devices. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<a>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in an `<a>`, merging props and refs. Use this when the trigger should be a `<button>` or other element instead of a link. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | The trigger content. |
| *(spread)* | -- | `...PrimitiveLinkProps` | -- | React allows spreading any `<a>` HTML attribute (e.g., `href`, `className`). Leptos uses `attr:` directives instead (e.g., `attr:href="/"`). |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"open" \| "closed"` | Reflects the hover card's current open state. |

### Implicit behavior

- The trigger is wrapped in a `PopperAnchor` (with `asChild=true`), which registers the trigger as the anchor element for the popper positioning system. The trigger itself does not render any extra DOM wrapper for the anchor.
- Touch interactions are suppressed: the `touchstart` handler calls `preventDefault()` to prevent focus (and thus hover card opening) on touch devices.
- Pointer events with `pointerType === "touch"` are excluded from open/close logic.
