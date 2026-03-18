# HoverCardContent

## React Signature

```typescript
const HoverCardContent = React.forwardRef<HoverCardContentElement, HoverCardContentProps>(...)

type HoverCardContentElement = HoverCardContentImplElement;

interface HoverCardContentProps extends HoverCardContentImplProps {
  /**
   * Used to force mounting when more control is needed. Useful when
   * controlling animation with React animation libraries.
   */
  forceMount?: true;
}

type HoverCardContentImplElement = React.ComponentRef<typeof PopperPrimitive.Content>;
type DismissableLayerProps = React.ComponentPropsWithoutRef<typeof DismissableLayer>;
type PopperContentProps = React.ComponentPropsWithoutRef<typeof PopperPrimitive.Content>;

interface HoverCardContentImplProps extends Omit<PopperContentProps, 'onPlaced'> {
  /**
   * Event handler called when the escape key is down. Can be prevented.
   */
  onEscapeKeyDown?: DismissableLayerProps['onEscapeKeyDown'];
  /**
   * Event handler called when a pointerdown event happens outside of the HoverCard.
   * Can be prevented.
   */
  onPointerDownOutside?: DismissableLayerProps['onPointerDownOutside'];
  /**
   * Event handler called when the focus moves outside of the HoverCard.
   * Can be prevented.
   */
  onFocusOutside?: DismissableLayerProps['onFocusOutside'];
  /**
   * Event handler called when an interaction happens outside the HoverCard.
   * Specifically, when a pointerdown event happens outside or focus moves outside of it.
   * Can be prevented.
   */
  onInteractOutside?: DismissableLayerProps['onInteractOutside'];
}

// Inherited from PopperContentProps:
interface PopperContentProps extends PrimitiveDivProps {
  side?: Side;                        // default: 'bottom'
  sideOffset?: number;                // default: 0
  align?: Align;                      // default: 'center'
  alignOffset?: number;               // default: 0
  arrowPadding?: number;              // default: 0
  avoidCollisions?: boolean;          // default: true
  collisionBoundary?: Boundary | Boundary[];  // default: []
  collisionPadding?: number | Partial<Record<Side, number>>;  // default: 0
  sticky?: 'partial' | 'always';      // default: 'partial'
  hideWhenDetached?: boolean;         // default: false
  updatePositionStrategy?: 'optimized' | 'always';  // default: 'optimized'
  // onPlaced is omitted in HoverCardContentImplProps
}
```

## Leptos Signature

```rust
pub fn HoverCardContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<ev::CustomEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<ev::CustomEvent>>,
    #[prop(into, optional)] on_interact_outside: Option<Callback<ev::CustomEvent>>,
    #[prop(into, optional)] on_pointer_enter: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_leave: Option<Callback<ev::PointerEvent>>,
    // PopperContent forwarded props
    #[prop(into, optional, default = Side::Bottom.into())] side: Signal<Side>,
    #[prop(into, optional, default = 0.0.into())] side_offset: Signal<f64>,
    #[prop(into, optional, default = Align::Center.into())] align: Signal<Align>,
    #[prop(into, optional, default = 0.0.into())] align_offset: Signal<f64>,
    #[prop(into, optional, default = 0.0.into())] arrow_padding: Signal<f64>,
    #[prop(into, optional, default = true.into())] avoid_collisions: Signal<bool>,
    #[prop(into, optional, default = SendWrapper::new(vec![]).into())]
    collision_boundary: Signal<SendWrapper<Vec<web_sys::Element>>>,
    #[prop(into, optional, default = Padding::All(0.0).into())] collision_padding: Signal<Padding>,
    #[prop(into, optional, default = Sticky::Partial.into())] sticky: Signal<Sticky>,
    #[prop(into, optional, default = false.into())] hide_when_detached: Signal<bool>,
    #[prop(into, optional, default = UpdatePositionStrategy::Optimized.into())]
    update_position_strategy: Signal<UpdatePositionStrategy>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

### Dismiss / interaction event handlers

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` | Forces the content to stay mounted in the DOM even when the hover card is closed. When used inside a `HoverCardPortal` with its own `forceMount`, this prop inherits the portal's value unless explicitly overridden. |
| `onEscapeKeyDown` | `on_escape_key_down` | `(event: KeyboardEvent) => void` | `Option<Callback<ev::KeyboardEvent>>` | Called when the Escape key is pressed. Call `preventDefault()` on the event to prevent dismissal. |
| `onPointerDownOutside` | `on_pointer_down_outside` | `(event: CustomEvent) => void` | `Option<Callback<ev::CustomEvent>>` | Called when a pointer-down occurs outside the content. Call `preventDefault()` to prevent dismissal. |
| `onFocusOutside` | `on_focus_outside` | `(event: CustomEvent) => void` | `Option<Callback<ev::CustomEvent>>` | Called when focus moves outside the content. By default, `preventDefault()` is always called internally to prevent the hover card from closing on focus-out (focus should not dismiss a hover card). The user handler is called before this default prevention. |
| `onInteractOutside` | `on_interact_outside` | `(event: CustomEvent) => void` | `Option<Callback<ev::CustomEvent>>` | Called on any outside interaction (pointer-down or focus-out). Call `preventDefault()` to prevent dismissal. |

### Pointer event handlers

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `onPointerEnter` | `on_pointer_enter` | `(event: PointerEvent) => void` | `Option<Callback<ev::PointerEvent>>` | Composed with the internal handler that re-opens (cancels close timer) when the pointer enters the content. Touch events are excluded. |
| `onPointerLeave` | `on_pointer_leave` | `(event: PointerEvent) => void` | `Option<Callback<ev::PointerEvent>>` | Composed with the internal handler that starts the close timer when the pointer leaves the content. Touch events are excluded. |

### Positioning props (from PopperContent)

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `side` | `side` | `'top' \| 'right' \| 'bottom' \| 'left'` (default `'bottom'`) | `Signal<Side>` (default `Side::Bottom`) | The preferred side of the trigger to render against. Will be reversed when collisions occur and `avoidCollisions` is `true`. |
| `sideOffset` | `side_offset` | `number` (default `0`) | `Signal<f64>` (default `0.0`) | Distance in pixels from the trigger along the side axis. |
| `align` | `align` | `'start' \| 'center' \| 'end'` (default `'center'`) | `Signal<Align>` (default `Align::Center`) | The preferred alignment along the cross axis. |
| `alignOffset` | `align_offset` | `number` (default `0`) | `Signal<f64>` (default `0.0`) | Offset in pixels from the `align` edge. |
| `arrowPadding` | `arrow_padding` | `number` (default `0`) | `Signal<f64>` (default `0.0`) | Padding between the arrow and the content edges. Prevents the arrow from overflowing the corners when the content has `border-radius`. |
| `avoidCollisions` | `avoid_collisions` | `boolean` (default `true`) | `Signal<bool>` (default `true`) | When `true`, the content will flip to the opposite side and/or shift to stay within the collision boundary. |
| `collisionBoundary` | `collision_boundary` | `Element \| Element[] \| null` (default `[]`) | `Signal<SendWrapper<Vec<web_sys::Element>>>` (default `vec![]`) | Element(s) used as the collision boundary. Defaults to the viewport. |
| `collisionPadding` | `collision_padding` | `number \| Partial<Record<Side, number>>` (default `0`) | `Signal<Padding>` (default `Padding::All(0.0)`) | Padding from the collision boundary edges. A number applies to all sides; an object allows per-side values. |
| `sticky` | `sticky` | `'partial' \| 'always'` (default `'partial'`) | `Signal<Sticky>` (default `Sticky::Partial`) | How the content behaves when it overflows: `partial` keeps it in view as much as possible, `always` maintains alignment even when overflowing. |
| `hideWhenDetached` | `hide_when_detached` | `boolean` (default `false`) | `Signal<bool>` (default `false`) | When `true`, hides the content (via `visibility: hidden`) when the trigger is fully occluded. |
| `updatePositionStrategy` | `update_position_strategy` | `'optimized' \| 'always'` (default `'optimized'`) | `Signal<UpdatePositionStrategy>` (default `UpdatePositionStrategy::Optimized`) | How frequently the position is recalculated. `optimized` only updates when needed; `always` updates on every animation frame. |

### Standard props

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | The hover card content. |
| *(spread)* | -- | `...PopperContentProps` (excluding `onPlaced`) | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"open" \| "closed"` | Reflects the hover card's current open state. |
| `data-side` | `"top" \| "right" \| "bottom" \| "left"` | The side the content is actually rendered on (after collision avoidance). Inherited from PopperContent. |
| `data-align` | `"start" \| "center" \| "end"` | The alignment the content is actually rendered with (after collision avoidance). Inherited from PopperContent. |

### Implicit behavior

- The content is wrapped in a `DismissableLayer` with `disableOutsidePointerEvents=false`, meaning outside pointer events are not blocked (unlike modal overlays like Dialog).
- Focus-outside events are always prevented by default -- focus moving away from the hover card does not dismiss it. Only pointer-down-outside and Escape dismiss.
- All tabbable elements inside the content have their `tabindex` set to `-1` on mount, preventing keyboard navigation into the content.
- During a pointer-down inside the content, `user-select: none` is set on the body (with Safari `-webkit-` prefix), and `user-select: text` is set on the content itself, to contain text selection within the content area.
- When a text selection is active inside the content, the hover card will not close on pointer leave. The selection must be cleared or an outside interaction must occur.
- CSS custom properties for positioning are set as inline styles (see CSS Custom Properties table on the root part).
