# TooltipContent

## React Signature

```typescript
const TooltipContent = React.forwardRef<TooltipContentElement, TooltipContentProps>(...)

type TooltipContentElement = TooltipContentImplElement;
type TooltipContentImplElement = React.ComponentRef<typeof PopperPrimitive.Content>;

interface TooltipContentProps extends TooltipContentImplProps {
  /**
   * Used to force mounting when more control is needed. Useful when
   * controlling animation with React animation libraries.
   */
  forceMount?: true;
}

type DismissableLayerProps = React.ComponentPropsWithoutRef<typeof DismissableLayer>;
type PopperContentProps = React.ComponentPropsWithoutRef<typeof PopperPrimitive.Content>;

interface TooltipContentImplProps extends Omit<PopperContentProps, 'onPlaced'> {
  /**
   * A more descriptive label for accessibility purpose
   */
  'aria-label'?: string;
  /**
   * Event handler called when the escape key is down.
   * Can be prevented.
   */
  onEscapeKeyDown?: DismissableLayerProps['onEscapeKeyDown'];
  /**
   * Event handler called when the a `pointerdown` event happens outside of the `Tooltip`.
   * Can be prevented.
   */
  onPointerDownOutside?: DismissableLayerProps['onPointerDownOutside'];
}
```

`PopperContentProps` includes the positioning props: `side`, `sideOffset`, `align`, `alignOffset`, `arrowPadding`, `avoidCollisions`, `collisionBoundary`, `collisionPadding`, `sticky`, `hideWhenDetached`, and `updatePositionStrategy`, plus all `<div>` HTML attributes via spread.

## Leptos Signature

```rust
pub fn TooltipContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<web_sys::CustomEvent>>,
    // PopperContent forwarded props
    #[prop(into, optional, default = Side::Top.into())] side: Signal<Side>,
    #[prop(into, optional, default = 0.0.into())] side_offset: Signal<f64>,
    #[prop(into, optional, default = Align::Center.into())] align: Signal<Align>,
    #[prop(into, optional, default = 0.0.into())] align_offset: Signal<f64>,
    #[prop(into, optional, default = 0.0.into())] arrow_padding: Signal<f64>,
    #[prop(into, optional, default = true.into())] avoid_collisions: Signal<bool>,
    #[prop(into, optional, default = SendWrapper::new(vec![]).into())]
    collision_boundary: Signal<SendWrapper<Vec<web_sys::Element>>>,
    #[prop(into, optional, default = Padding::All(0.0).into())]
    collision_padding: Signal<Padding>,
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

### Tooltip-specific props

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` | Forces the content to stay mounted in the DOM even when closed. Inherits from `TooltipPortal`'s `forceMount` if not set directly. Useful for CSS or JS animations that need the element present in the DOM for exit transitions. |
| `aria-label` | `aria_label` | `string \| undefined` | `MaybeProp<String>` | An accessible label for the tooltip. When provided, the visually hidden `role="tooltip"` element uses this string instead of the visible content text. Use when the visual content is rich/complex but the accessible description should be a simple string. |
| `onEscapeKeyDown` | `on_escape_key_down` | `(event: KeyboardEvent) => void` | `Option<Callback<web_sys::KeyboardEvent>>` | Called when the Escape key is pressed while the tooltip is open. Call `event.preventDefault()` to prevent the tooltip from closing. |
| `onPointerDownOutside` | `on_pointer_down_outside` | `(event: CustomEvent) => void` | `Option<Callback<web_sys::CustomEvent>>` | Called when a pointer down occurs outside the tooltip content. Call `event.preventDefault()` to prevent the tooltip from closing. Useful for keeping the tooltip open when clicking the trigger. |

### Positioning props (forwarded to PopperContent)

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `side` | `side` | `'top' \| 'right' \| 'bottom' \| 'left'` (default `'top'`) | `Signal<Side>` (default `Side::Top`) | The preferred side of the trigger to render against. Will be reversed if collisions are detected and `avoidCollisions` is `true`. |
| `sideOffset` | `side_offset` | `number` (default `0`) | `Signal<f64>` (default `0.0`) | Distance in pixels from the trigger along the side axis. |
| `align` | `align` | `'start' \| 'center' \| 'end'` (default `'center'`) | `Signal<Align>` (default `Align::Center`) | Alignment along the cross axis relative to the trigger. |
| `alignOffset` | `align_offset` | `number` (default `0`) | `Signal<f64>` (default `0.0`) | Offset in pixels from the `start` or `end` alignment options. |
| `arrowPadding` | `arrow_padding` | `number` (default `0`) | `Signal<f64>` (default `0.0`) | Padding between the arrow and the content edges. Prevents the arrow from overflowing rounded corners. |
| `avoidCollisions` | `avoid_collisions` | `boolean` (default `true`) | `Signal<bool>` (default `true`) | When `true`, the content flips to the opposite side and/or shifts along the axis to avoid overflowing the viewport or collision boundary. |
| `collisionBoundary` | `collision_boundary` | `Element \| Element[] \| null` | `Signal<SendWrapper<Vec<web_sys::Element>>>` | The element(s) used as the collision boundary. Defaults to the viewport. |
| `collisionPadding` | `collision_padding` | `number \| Padding` (default `0`) | `Signal<Padding>` (default `Padding::All(0.0)`) | Padding from the collision boundary edges where collision detection triggers. Accepts a number (all sides) or per-side padding. |
| `sticky` | `sticky` | `'partial' \| 'always'` (default `'partial'`) | `Signal<Sticky>` (default `Sticky::Partial`) | The sticky behavior on the cross axis. `partial` keeps the content visible as long as the trigger is partially in the boundary. `always` keeps it visible regardless. |
| `hideWhenDetached` | `hide_when_detached` | `boolean` (default `false`) | `Signal<bool>` (default `false`) | When `true`, hides the content when the trigger is fully occluded. |
| `updatePositionStrategy` | `update_position_strategy` | `'optimized' \| 'always'` (default `'optimized'`) | `Signal<UpdatePositionStrategy>` (default `UpdatePositionStrategy::Optimized`) | The strategy for updating the content position. `optimized` only updates when necessary; `always` updates on every animation frame. |

### Common props

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>` from PopperContent). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | The tooltip content. |
| *(spread)* | -- | `...PopperContentProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Wrapped in a `DismissableLayer` with `disableOutsidePointerEvents=false` and `onFocusOutside` prevented (focus leaving the tooltip does not dismiss it; only escape or pointer-down-outside do).
- Listens for the `tooltip.open` custom event on `document` to close when another tooltip opens.
- Listens for `scroll` events (capture phase) on `window` and closes the tooltip if the trigger element is scrolled.
- When `disableHoverableContent` is `false`, the content is rendered via `TooltipContentHoverable`, which sets up convex-hull grace area tracking between trigger and content to allow hovering the content without dismissal.
- Contains a `VisuallyHidden` element with `role="tooltip"` and an auto-generated `id`. If `aria-label` is provided, the visually hidden element uses that text; otherwise, it duplicates the children. This visually hidden element is what the trigger's `aria-describedby` references.
- A `VisuallyHiddenContentContext` prevents `TooltipArrow` from rendering inside the visually hidden copy (where it would cause positioning issues).
- CSS custom properties are applied via Effect (see the root file's CSS Custom Properties section).

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"closed" \| "delayed-open" \| "instant-open"` | Reflects how the tooltip was opened. Useful for CSS animation selectors. |
| `data-side` | `"top" \| "right" \| "bottom" \| "left"` | Inherited from `PopperContent`. The actual rendered side after collision avoidance. |
| `data-align` | `"start" \| "center" \| "end"` | Inherited from `PopperContent`. The actual rendered alignment after collision avoidance. |
