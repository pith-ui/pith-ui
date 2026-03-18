# TooltipTrigger

## React Signature

```typescript
const TooltipTrigger = React.forwardRef<TooltipTriggerElement, TooltipTriggerProps>(...)

type TooltipTriggerElement = React.ComponentRef<typeof Primitive.button>;
type PrimitiveButtonProps = React.ComponentPropsWithoutRef<typeof Primitive.button>;

interface TooltipTriggerProps extends PrimitiveButtonProps {}
```

All standard `<button>` HTML attributes are accepted via spread.

## Leptos Signature

```rust
pub fn TooltipTrigger(
    #[prop(into, optional)] on_pointer_move: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_leave: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_down: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_focus: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] on_blur: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `onPointerMove` | `on_pointer_move` | `(event: PointerEvent) => void` | `Option<Callback<ev::PointerEvent>>` | Composed event handler for pointer move. The internal handler opens the tooltip on pointer movement (non-touch) unless the pointer is in transit to another tooltip. |
| `onPointerLeave` | `on_pointer_leave` | `(event: PointerEvent) => void` | `Option<Callback<ev::PointerEvent>>` | Composed event handler for pointer leave. The internal handler notifies the tooltip to leave the trigger. |
| `onPointerDown` | `on_pointer_down` | `(event: PointerEvent) => void` | `Option<Callback<ev::PointerEvent>>` | Composed event handler for pointer down. The internal handler closes an open tooltip and tracks pointer-down state to suppress focus-triggered open. |
| `onFocus` | `on_focus` | `(event: FocusEvent) => void` | `Option<Callback<ev::FocusEvent>>` | Composed event handler for focus. The internal handler opens the tooltip instantly (no delay) unless the focus was caused by a pointer down. |
| `onBlur` | `on_blur` | `(event: FocusEvent) => void` | `Option<Callback<ev::FocusEvent>>` | Composed event handler for blur. The internal handler closes the tooltip. |
| `onClick` | `on_click` | `(event: MouseEvent) => void` | `Option<Callback<ev::MouseEvent>>` | Composed event handler for click. The internal handler closes the tooltip. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<button>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<button>`, merging props and refs. Useful for rendering the tooltip trigger as an anchor tag or other element. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | The trigger content. |
| *(spread)* | -- | `...PrimitiveButtonProps` | -- | React allows spreading any `<button>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Rendered inside a `PopperAnchor` (with `asChild`), making this element the positioning anchor for the tooltip content.
- Sets `aria-describedby` to the tooltip content's auto-generated `id` when the tooltip is open; removes the attribute when closed.
- Sets `data-state` reflecting the tooltip's state attribute.
- The trigger intentionally does not set `type="button"` because tooltip triggers are commonly anchors, and the `type` attribute on anchors signifies MIME type.
- A one-time `pointerup` listener is attached to `document` on pointer down to track pointer-down state. The closure is leaked to JavaScript via `Closure::once_into_js` to avoid disposal issues.

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"closed" \| "delayed-open" \| "instant-open"` | Reflects how the tooltip was opened. `delayed-open` indicates the tooltip opened after the delay timer; `instant-open` indicates it opened immediately (skip-delay or focus). `closed` when the tooltip is not open. |
