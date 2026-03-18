# SelectContent

## React Signature

```typescript
const SelectContent = React.forwardRef<SelectContentElement, SelectContentProps>(...)

type SelectContentElement = SelectContentImplElement;

interface SelectContentProps extends SelectContentImplProps {}

interface SelectContentImplProps
  extends Omit<SelectPopperPositionProps, 'onPlaced'>,
    Omit<SelectItemAlignedPositionProps, 'onPlaced'> {
  /**
   * Event handler called when auto-focusing on close. Can be prevented.
   */
  onCloseAutoFocus?: FocusScopeProps['onUnmountAutoFocus'];
  /**
   * Event handler called when the escape key is down. Can be prevented.
   */
  onEscapeKeyDown?: DismissableLayerProps['onEscapeKeyDown'];
  /**
   * Event handler called when a `pointerdown` event happens outside. Can be prevented.
   */
  onPointerDownOutside?: DismissableLayerProps['onPointerDownOutside'];

  position?: 'item-aligned' | 'popper';
}
```

When `position="popper"`, the following `PopperContent` props are also accepted: `side`, `sideOffset`, `align`, `alignOffset`, `arrowPadding`, `collisionBoundary`, `collisionPadding`, `sticky`, `hideWhenDetached`, `avoidCollisions`.

## Leptos Signature

```rust
pub fn SelectContent(
    #[prop(into, optional)] position: MaybeProp<String>,
    #[prop(into, optional)] on_close_auto_focus: Option<Callback<web_sys::Event>>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<web_sys::CustomEvent>>,
    // PopperContent forwarded props
    #[prop(into, optional, default = Side::Bottom.into())] side: Signal<Side>,
    #[prop(into, optional, default = 0.0.into())] side_offset: Signal<f64>,
    #[prop(into, optional, default = Align::Start.into())] align: Signal<Align>,
    #[prop(into, optional, default = 0.0.into())] align_offset: Signal<f64>,
    #[prop(into, optional, default = 0.0.into())] arrow_padding: Signal<f64>,
    #[prop(into, optional, default = true.into())] avoid_collisions: Signal<bool>,
    #[prop(into, optional, default = SendWrapper::new(vec![]).into())]
        collision_boundary: Signal<SendWrapper<Vec<web_sys::Element>>>,
    #[prop(into, optional, default = Padding::All(10.0).into())]
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

### Core props

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `position` | `position` | `'item-aligned' \| 'popper'` (default `'item-aligned'`) | `MaybeProp<String>` (default `"item-aligned"`) | The positioning mode. `"item-aligned"` aligns the selected item with the trigger. `"popper"` uses floating-UI-based positioning. |
| `onCloseAutoFocus` | `on_close_auto_focus` | `(event: Event) => void` | `Option<Callback<web_sys::Event>>` | Called when auto-focusing on close. Call `event.preventDefault()` to prevent default focus restoration to the trigger. |
| `onEscapeKeyDown` | `on_escape_key_down` | `(event: KeyboardEvent) => void` | `Option<Callback<web_sys::KeyboardEvent>>` | Called when the Escape key is pressed. Call `event.preventDefault()` to prevent closing. |
| `onPointerDownOutside` | `on_pointer_down_outside` | `(event: CustomEvent) => void` | `Option<Callback<web_sys::CustomEvent>>` | Called when a pointer down occurs outside the content. Call `event.preventDefault()` to prevent closing. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered content DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | The content children (viewport, scroll buttons, etc.). |
| *(spread)* | -- | `...PrimitiveDivProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Popper positioning props (only effective when `position="popper"`)

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `side` | `side` | `'top' \| 'right' \| 'bottom' \| 'left'` (default `'bottom'`) | `Signal<Side>` (default `Side::Bottom`) | The preferred side of the trigger to render against. |
| `sideOffset` | `side_offset` | `number` (default `0`) | `Signal<f64>` (default `0.0`) | Distance in pixels from the trigger edge. |
| `align` | `align` | `'start' \| 'center' \| 'end'` (default `'start'`) | `Signal<Align>` (default `Align::Start`) | Alignment along the trigger edge. |
| `alignOffset` | `align_offset` | `number` (default `0`) | `Signal<f64>` (default `0.0`) | Offset from the alignment edge in pixels. |
| `arrowPadding` | `arrow_padding` | `number` (default `0`) | `Signal<f64>` (default `0.0`) | Padding between the arrow and content edges. |
| `avoidCollisions` | `avoid_collisions` | `boolean` (default `true`) | `Signal<bool>` (default `true`) | Whether to flip/shift the content to avoid viewport overflow. |
| `collisionBoundary` | `collision_boundary` | `Element \| Element[]` | `Signal<SendWrapper<Vec<web_sys::Element>>>` | Custom boundary element(s) for collision detection. |
| `collisionPadding` | `collision_padding` | `number \| Partial<Record<Side, number>>` (default `10`) | `Signal<Padding>` (default `Padding::All(10.0)`) | Padding from the collision boundary edges. |
| `sticky` | `sticky` | `'partial' \| 'always'` (default `'partial'`) | `Signal<Sticky>` (default `Sticky::Partial`) | Whether the content should stay attached to the trigger even when partially overflowing. |
| `hideWhenDetached` | `hide_when_detached` | `boolean` (default `false`) | `Signal<bool>` (default `false`) | Whether to hide the content when the trigger is fully occluded. |
| -- | `update_position_strategy` | -- | `Signal<UpdatePositionStrategy>` (default `Optimized`) | Leptos-only. Controls how aggressively position is recalculated. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"open" \| "closed"` | Reflects the open state of the select. |

### Implicit behavior

- When closed, children are rendered in a hidden container (before first open) to allow `SelectItemText` to copy the selected item's text into `SelectValue`. After the first open, closed content is unmounted entirely.
- The content uses `FocusScope` with `trapped=true` to keep keyboard focus inside while open.
- The content uses `DismissableLayer` with `disableOutsidePointerEvents=true` to block interactions outside.
- Focus-outside events are prevented (the select does not close on focus loss while focus is trapped).
- `aria-hidden` is applied to all sibling DOM elements while the content is open.
- Focus is automatically moved to the selected item (or first enabled item) after the content is positioned.
- The content closes on window `blur` and `resize` events.
- Context menu is prevented on the content.
- Renders with inline styles: `display: flex`, `flex-direction: column`, `outline: none`.
- In `"item-aligned"` mode, the content is wrapped in a fixed-position wrapper `<div>` for item-aligned positioning. In `"popper"` mode, it uses `PopperContent` with the CSS custom properties listed in the root doc.
