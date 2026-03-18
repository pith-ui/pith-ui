# PopoverContent

## React Signature

```typescript
const PopoverContent = React.forwardRef<PopoverContentTypeElement, PopoverContentProps>(...)

interface PopoverContentProps extends PopoverContentTypeProps {
  /**
   * Used to force mounting when more control is needed. Useful when
   * controlling animation with React animation libraries.
   */
  forceMount?: true;
}

interface PopoverContentTypeProps
  extends Omit<PopoverContentImplProps, 'trapFocus' | 'disableOutsidePointerEvents'> {}

interface PopoverContentImplProps
  extends Omit<PopperContentProps, 'onPlaced'>,
    Omit<DismissableLayerProps, 'onDismiss'> {
  /**
   * Whether focus should be trapped within the Popover
   * (default: false)
   */
  trapFocus?: FocusScopeProps['trapped'];

  /**
   * Event handler called when auto-focusing on open.
   * Can be prevented.
   */
  onOpenAutoFocus?: FocusScopeProps['onMountAutoFocus'];

  /**
   * Event handler called when auto-focusing on close.
   * Can be prevented.
   */
  onCloseAutoFocus?: FocusScopeProps['onUnmountAutoFocus'];
}

// From PopperContent:
interface PopperContentProps extends PrimitiveDivProps {
  side?: Side;                                           // default: 'bottom'
  sideOffset?: number;                                   // default: 0
  align?: Align;                                         // default: 'center'
  alignOffset?: number;                                  // default: 0
  arrowPadding?: number;                                 // default: 0
  avoidCollisions?: boolean;                             // default: true
  collisionBoundary?: Boundary | Boundary[];             // default: []
  collisionPadding?: number | Partial<Record<Side, number>>; // default: 0
  sticky?: 'partial' | 'always';                         // default: 'partial'
  hideWhenDetached?: boolean;                            // default: false
  updatePositionStrategy?: 'optimized' | 'always';       // default: 'optimized'
}
```

## Leptos Signature

```rust
pub fn PopoverContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] on_open_auto_focus: Option<Callback<web_sys::Event>>,
    #[prop(into, optional)] on_close_auto_focus: Option<Callback<web_sys::Event>>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] on_interact_outside: Option<Callback<web_sys::CustomEvent>>,
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

### Positioning props (from PopperContent)

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `side` | `side` | `'top' \| 'right' \| 'bottom' \| 'left'` (default `'bottom'`) | `Signal<Side>` (default `Side::Bottom`) | The preferred side of the trigger to render against. Will be reversed when collisions occur and `avoidCollisions` is enabled. |
| `sideOffset` | `side_offset` | `number` (default `0`) | `Signal<f64>` (default `0.0`) | Distance in pixels from the trigger along the side axis. |
| `align` | `align` | `'start' \| 'center' \| 'end'` (default `'center'`) | `Signal<Align>` (default `Align::Center`) | Alignment along the cross axis relative to the trigger. |
| `alignOffset` | `align_offset` | `number` (default `0`) | `Signal<f64>` (default `0.0`) | Offset in pixels from the aligned edge. |
| `arrowPadding` | `arrow_padding` | `number` (default `0`) | `Signal<f64>` (default `0.0`) | Padding between the arrow and the edges of the content. Prevents the arrow from overflowing the rounded corners. |
| `avoidCollisions` | `avoid_collisions` | `boolean` (default `true`) | `Signal<bool>` (default `true`) | When `true`, the popover flips to the opposite side and/or shifts along the axis to stay within the collision boundary. |
| `collisionBoundary` | `collision_boundary` | `Element \| Element[] \| null` (default `[]`) | `Signal<SendWrapper<Vec<web_sys::Element>>>` (default `vec![]`) | Element(s) used as the collision boundary. By default, the viewport is used. |
| `collisionPadding` | `collision_padding` | `number \| Partial<Record<Side, number>>` (default `0`) | `Signal<Padding>` (default `Padding::All(0.0)`) | Padding in pixels between the content and the collision boundary edge. Accepts a single number for all sides or per-side values. |
| `sticky` | `sticky` | `'partial' \| 'always'` (default `'partial'`) | `Signal<Sticky>` (default `Sticky::Partial`) | Controls the behavior when the content overflows along the alignment axis. `"partial"` keeps the content in the boundary as long as the trigger is at least partially visible. `"always"` keeps it in the boundary regardless. |
| `hideWhenDetached` | `hide_when_detached` | `boolean` (default `false`) | `Signal<bool>` (default `false`) | When `true`, hides the content (via `visibility: hidden` and `pointer-events: none`) when the trigger is fully occluded or detached from the viewport. |
| `updatePositionStrategy` | `update_position_strategy` | `'optimized' \| 'always'` (default `'optimized'`) | `Signal<UpdatePositionStrategy>` (default `UpdatePositionStrategy::Optimized`) | Strategy for updating the position. `"optimized"` only repositions when needed. `"always"` repositions on every animation frame. |

### Focus and dismissal props

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` | Forces the content to stay mounted even when closed. Falls back to the value from `PopoverPortal` context if not set. |
| `onOpenAutoFocus` | `on_open_auto_focus` | `(event: Event) => void` | `Option<Callback<web_sys::Event>>` | Called when focus is about to move into the content on open. Call `event.preventDefault()` to prevent auto-focus. |
| `onCloseAutoFocus` | `on_close_auto_focus` | `(event: Event) => void` | `Option<Callback<web_sys::Event>>` | Called when focus is about to return to the trigger on close. Call `event.preventDefault()` to prevent auto-focus. |
| `onEscapeKeyDown` | `on_escape_key_down` | `(event: KeyboardEvent) => void` | `Option<Callback<web_sys::KeyboardEvent>>` | Called when the Escape key is pressed. Call `event.preventDefault()` to prevent dismissal. |
| `onPointerDownOutside` | `on_pointer_down_outside` | `(event: CustomEvent) => void` | `Option<Callback<web_sys::CustomEvent>>` | Called when a pointer down event occurs outside the content. Call `event.preventDefault()` to prevent dismissal. |
| `onFocusOutside` | `on_focus_outside` | `(event: CustomEvent) => void` | `Option<Callback<web_sys::CustomEvent>>` | Called when focus moves outside the content. Call `event.preventDefault()` to prevent dismissal. |
| `onInteractOutside` | `on_interact_outside` | `(event: CustomEvent) => void` | `Option<Callback<web_sys::CustomEvent>>` | Called when any interaction (pointer or focus) occurs outside the content. Call `event.preventDefault()` to prevent dismissal. |

### Standard props

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | The popover content. |
| *(spread)* | -- | `...PrimitiveDivProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"open" \| "closed"` | Reflects the popover's open state. |
| `data-side` | `"top" \| "right" \| "bottom" \| "left"` | The actual rendered side after collision avoidance (set by PopperContent). |
| `data-align` | `"start" \| "center" \| "end"` | The actual rendered alignment after collision avoidance (set by PopperContent). |

### Implicit behavior

- Renders with `role="dialog"` and an auto-generated `id` (linked to the trigger's `aria-controls`).
- Wraps content in `FocusScope` (with `loop=true`) and `DismissableLayer`.
- In **modal** mode: traps focus, disables outside pointer events, locks body scroll, and hides other elements from assistive technology via `aria-hidden`.
- In **non-modal** mode: does not trap focus. Clicking the trigger while open correctly closes without a toggle race. Clicking outside closes the popover. Focus returns to the trigger unless the user interacted outside.
- Sets CSS custom properties aliasing the underlying Popper values (see root file for the full list).
