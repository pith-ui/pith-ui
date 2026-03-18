# DropdownMenuContent

## React Signature

```typescript
const DropdownMenuContent = React.forwardRef<DropdownMenuContentElement, DropdownMenuContentProps>(...)

type DropdownMenuContentElement = React.ComponentRef<typeof MenuPrimitive.Content>;
type MenuContentProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.Content>;

interface DropdownMenuContentProps extends Omit<MenuContentProps, 'onEntryFocus'> {}
```

The underlying `MenuContentProps` extends `MenuRootContentTypeProps` which extends `MenuContentImplProps` (minus private props), which includes `PopperContentProps` (minus `dir` and `onPlaced`), plus dismissable layer and roving focus callbacks:

```typescript
interface MenuContentImplProps
  extends Omit<PopperContentProps, 'dir' | 'onPlaced'> {
  onCloseAutoFocus?: FocusScopeProps['onUnmountAutoFocus'];
  loop?: boolean; // default false
  onEntryFocus?: RovingFocusGroupProps['onEntryFocus'];
  onEscapeKeyDown?: DismissableLayerProps['onEscapeKeyDown'];
  onPointerDownOutside?: DismissableLayerProps['onPointerDownOutside'];
  onFocusOutside?: DismissableLayerProps['onFocusOutside'];
  onInteractOutside?: DismissableLayerProps['onInteractOutside'];
}

interface PopperContentProps extends PrimitiveDivProps {
  side?: 'top' | 'right' | 'bottom' | 'left'; // default 'bottom'
  sideOffset?: number; // default 0
  align?: 'start' | 'center' | 'end'; // default 'center'
  alignOffset?: number; // default 0
  arrowPadding?: number; // default 0
  avoidCollisions?: boolean; // default true
  collisionBoundary?: Boundary | Boundary[];
  collisionPadding?: number | Partial<Record<Side, number>>; // default 0
  sticky?: 'partial' | 'always'; // default 'partial'
  hideWhenDetached?: boolean; // default false
  updatePositionStrategy?: 'optimized' | 'always';
}
```

Note: `DropdownMenuContent` omits `onEntryFocus` from the public API -- entry focus behavior is managed internally by the menu.

## Leptos Signature

```rust
pub fn DropdownMenuContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] side: MaybeProp<PopperSide>,
    #[prop(into, optional)] side_offset: MaybeProp<f64>,
    #[prop(into, optional)] align: MaybeProp<Align>,
    #[prop(into, optional)] align_offset: MaybeProp<f64>,
    #[prop(into, optional)] avoid_collisions: MaybeProp<bool>,
    #[prop(into, optional)] collision_boundary: MaybeProp<SendWrapper<Vec<web_sys::Element>>>,
    #[prop(into, optional)] collision_padding: MaybeProp<Padding>,
    #[prop(into, optional)] arrow_padding: MaybeProp<f64>,
    #[prop(into, optional)] sticky: MaybeProp<Sticky>,
    #[prop(into, optional)] hide_when_detached: MaybeProp<bool>,
    #[prop(into, optional)] r#loop: MaybeProp<bool>,
    #[prop(into, optional)] on_close_auto_focus: Option<Callback<ev::Event>>,
    #[prop(into, optional)] on_interact_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

### Positioning props (from Popper)

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `side` | `side` | `'top' \| 'right' \| 'bottom' \| 'left'` (default `'bottom'`) | `MaybeProp<PopperSide>` (default `Bottom`) | The preferred side of the trigger to render against. |
| `sideOffset` | `side_offset` | `number` (default `0`) | `MaybeProp<f64>` (default `0.0`) | The distance in pixels from the trigger. |
| `align` | `align` | `'start' \| 'center' \| 'end'` (default `'center'`) | `MaybeProp<Align>` (default `Center`) | The preferred alignment against the trigger. |
| `alignOffset` | `align_offset` | `number` (default `0`) | `MaybeProp<f64>` (default `0.0`) | An offset in pixels from the `"start"` or `"end"` alignment options. |
| `avoidCollisions` | `avoid_collisions` | `boolean` (default `true`) | `MaybeProp<bool>` (default `true`) | When `true`, overrides `side` and `align` preferences to prevent collisions with viewport edges. |
| `collisionBoundary` | `collision_boundary` | `Element \| Element[]` | `MaybeProp<SendWrapper<Vec<web_sys::Element>>>` | The element(s) used as the collision boundary. Defaults to the viewport. |
| `collisionPadding` | `collision_padding` | `number \| Partial<Record<Side, number>>` (default `0`) | `MaybeProp<Padding>` (default `0.0`) | The distance in pixels from the boundary edges where collision detection starts. Accepts a number for all sides or a per-side object. |
| `arrowPadding` | `arrow_padding` | `number` (default `0`) | `MaybeProp<f64>` (default `0.0`) | The padding between the arrow and the content edges. Prevents the arrow from overflowing rounded corners. |
| `sticky` | `sticky` | `'partial' \| 'always'` (default `'partial'`) | `MaybeProp<Sticky>` (default `Partial`) | The sticky behavior on the align axis. `"partial"` keeps the content in the boundary as long as the trigger is at least partially in the boundary. `"always"` keeps the content in the boundary regardless. |
| `hideWhenDetached` | `hide_when_detached` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | Whether the content should be hidden (via `visibility: hidden`) when it is fully detached from the trigger (the trigger has scrolled out of view). |

### Behavior props

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` | Forces the content to stay in the DOM when closed. Useful for controlling animations. Inherits from portal's `forceMount` if not set directly. |
| `loop` | `r#loop` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | Whether keyboard navigation should loop from the last item to the first and vice versa. |
| `onCloseAutoFocus` | `on_close_auto_focus` | `(event: Event) => void` | `Option<Callback<ev::Event>>` | Called when focus would move back to the trigger after closing. Call `event.preventDefault()` to prevent focus from moving. The internal handler always prevents default and manually focuses the trigger (unless the user interacted outside). |
| `onEscapeKeyDown` | `on_escape_key_down` | `(event: KeyboardEvent) => void` | `Option<Callback<ev::KeyboardEvent>>` | Called when the Escape key is pressed. Call `event.preventDefault()` to prevent the menu from closing. |
| `onPointerDownOutside` | `on_pointer_down_outside` | `(event: CustomEvent) => void` | `Option<Callback<web_sys::CustomEvent>>` | Called when a pointer down event occurs outside the content. Call `event.preventDefault()` to prevent the menu from closing. |
| `onFocusOutside` | `on_focus_outside` | `(event: CustomEvent) => void` | `Option<Callback<web_sys::CustomEvent>>` | Called when focus moves outside the content. Call `event.preventDefault()` to prevent the menu from closing. In modal mode, this is automatically prevented. |
| `onInteractOutside` | `on_interact_outside` | `(event: CustomEvent) => void` | `Option<Callback<web_sys::CustomEvent>>` | Called when any interaction (pointer or focus) occurs outside the content. The internal handler tracks whether the interaction was outside to control focus restoration behavior. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| *(spread)* | -- | `...Omit<MenuContentProps, 'onEntryFocus'>` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"open" \| "closed"` | Reflects the menu's open state. |
| `data-side` | `"top" \| "right" \| "bottom" \| "left"` | The side the content is rendered on after collision avoidance. Set by Popper. |
| `data-align` | `"start" \| "center" \| "end"` | The alignment of the content after collision avoidance. Set by Popper. |

### Implicit behavior

- Sets `id` to an auto-generated content ID from the `DropdownMenu` context.
- Sets `aria-labelledby` to the trigger's auto-generated ID.
- Sets `role="menu"` and `aria-orientation="vertical"` (from the underlying `MenuContent`).
- On close, focus returns to the trigger unless the user interacted outside the menu.
- Exposes CSS custom properties that re-namespace Popper values (see root file for the full list).
- Sets `outline: none` as an inline style (inherited from `MenuContentImpl`).
