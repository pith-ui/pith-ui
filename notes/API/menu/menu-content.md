# MenuContent

## React Signature

```typescript
const MenuContent = React.forwardRef<MenuContentElement, MenuContentProps>(...)

type MenuContentElement = MenuRootContentTypeElement; // = MenuContentImplElement = PopperContent element

interface MenuContentProps extends MenuRootContentTypeProps {
  /** Used to force mounting when more control is needed. */
  forceMount?: true;
}

// MenuRootContentTypeProps omits the internal-only props:
interface MenuRootContentTypeProps
  extends Omit<MenuContentImplProps, keyof MenuContentImplPrivateProps> {}

// The full implementation interface (internal):
interface MenuContentImplProps extends MenuContentImplPrivateProps,
  Omit<PopperContentProps, 'dir' | 'onPlaced'> {
  onCloseAutoFocus?: FocusScopeProps['onUnmountAutoFocus'];
  loop?: RovingFocusGroupProps['loop'];
  onEntryFocus?: RovingFocusGroupProps['onEntryFocus'];
  onEscapeKeyDown?: DismissableLayerProps['onEscapeKeyDown'];
  onPointerDownOutside?: DismissableLayerProps['onPointerDownOutside'];
  onFocusOutside?: DismissableLayerProps['onFocusOutside'];
  onInteractOutside?: DismissableLayerProps['onInteractOutside'];
}
```

## Leptos Signature

```rust
pub fn MenuContent(
    /// Used to force mounting when more control is needed.
    #[prop(into, optional)]
    force_mount: MaybeProp<bool>,
    /// Event handler called when auto-focusing on close. Can be prevented.
    #[prop(into, optional)]
    on_close_auto_focus: Option<Callback<ev::Event>>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<CustomEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<CustomEvent>>,
    #[prop(into, optional)] on_interact_outside: Option<Callback<CustomEvent>>,
    /// Event handler called when the content receives initial focus. Can be prevented.
    #[prop(into, optional)]
    on_entry_focus: Option<Callback<ev::Event>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    /// The preferred side of the trigger to render against when open.
    #[prop(into, optional)]
    side: MaybeProp<PopperSide>,
    /// The distance in pixels from the trigger.
    #[prop(into, optional)]
    side_offset: MaybeProp<f64>,
    /// The preferred alignment against the trigger.
    #[prop(into, optional)]
    align: MaybeProp<Align>,
    /// An offset in pixels from the "start" or "end" alignment options.
    #[prop(into, optional)]
    align_offset: MaybeProp<f64>,
    /// When true, overrides side and align preferences to prevent collisions.
    #[prop(into, optional)]
    avoid_collisions: MaybeProp<bool>,
    /// The element(s) used as the collision boundary.
    #[prop(into, optional)]
    collision_boundary: MaybeProp<SendWrapper<Vec<web_sys::Element>>>,
    /// The padding between the boundary edges and the content.
    #[prop(into, optional)]
    collision_padding: MaybeProp<Padding>,
    /// The padding between the arrow and the edges of the content.
    #[prop(into, optional)]
    arrow_padding: MaybeProp<f64>,
    /// The sticky behavior on the align axis.
    #[prop(into, optional)]
    sticky: MaybeProp<Sticky>,
    /// Whether the content should be hidden when detached from its reference element.
    #[prop(into, optional)]
    hide_when_detached: MaybeProp<bool>,
    /// Whether keyboard navigation should loop around.
    #[prop(into, optional)]
    r#loop: MaybeProp<bool>,
    /// The id of the content element.
    #[prop(into, optional)]
    id: MaybeProp<String>,
    /// The id of the element that labels the content.
    #[prop(into, optional)]
    aria_labelledby: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

### Positioning props (forwarded to PopperContent)

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `side` | `side` | `'top' \| 'right' \| 'bottom' \| 'left'` (default `'bottom'`) | `MaybeProp<PopperSide>` (default `Bottom`) | The preferred side of the anchor to render against. |
| `sideOffset` | `side_offset` | `number` (default `0`) | `MaybeProp<f64>` (default `0.0`) | Distance in pixels from the anchor. |
| `align` | `align` | `'start' \| 'center' \| 'end'` (default `'center'`) | `MaybeProp<Align>` (default `Center`) | The preferred alignment against the anchor. |
| `alignOffset` | `align_offset` | `number` (default `0`) | `MaybeProp<f64>` (default `0.0`) | Offset in pixels from the alignment edge. |
| `avoidCollisions` | `avoid_collisions` | `boolean` (default `true`) | `MaybeProp<bool>` (default `true`) | When `true`, overrides `side` and `align` to prevent boundary collisions. |
| `collisionBoundary` | `collision_boundary` | `Element \| Element[]` | `MaybeProp<SendWrapper<Vec<web_sys::Element>>>` | The element(s) used as the collision boundary. Defaults to the viewport. |
| `collisionPadding` | `collision_padding` | `number \| Padding` (default `0`) | `MaybeProp<Padding>` (default `Padding::All(0.0)`) | Padding between boundary edges and the content. |
| `arrowPadding` | `arrow_padding` | `number` (default `0`) | `MaybeProp<f64>` (default `0.0`) | Padding between the arrow and the content edges. |
| `sticky` | `sticky` | `'partial' \| 'always'` (default `'partial'`) | `MaybeProp<Sticky>` (default `Partial`) | The sticky behavior on the align axis. |
| `hideWhenDetached` | `hide_when_detached` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | Whether to hide content when it detaches from its anchor. |

### Behavior props

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` | Forces the content to stay mounted in the DOM even when closed. Useful for exit animations. |
| `loop` | `r#loop` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | Whether keyboard navigation loops from the last item back to the first (and vice versa). |
| `onCloseAutoFocus` | `on_close_auto_focus` | `(event: Event) => void` | `Option<Callback<ev::Event>>` | Called when focus returns to the trigger after closing. Call `event.preventDefault()` to prevent default focus restoration. |
| `onEntryFocus` | `on_entry_focus` | `(event: Event) => void` | `Option<Callback<ev::Event>>` | Called when the roving focus group receives entry focus. Preventable. |
| `onEscapeKeyDown` | `on_escape_key_down` | `(event: KeyboardEvent) => void` | `Option<Callback<ev::KeyboardEvent>>` | Called when the Escape key is pressed. Preventable. |
| `onPointerDownOutside` | `on_pointer_down_outside` | `(event: CustomEvent) => void` | `Option<Callback<CustomEvent>>` | Called when a pointer down event occurs outside the content. Preventable. |
| `onFocusOutside` | `on_focus_outside` | `(event: CustomEvent) => void` | `Option<Callback<CustomEvent>>` | Called when focus moves outside the content. Preventable. |
| `onInteractOutside` | `on_interact_outside` | `(event: CustomEvent) => void` | `Option<Callback<CustomEvent>>` | Called when any interaction (pointer or focus) occurs outside the content. Preventable. |
| `onKeyDown` | `on_key_down` | `(event: KeyboardEvent) => void` | `Option<Callback<ev::KeyboardEvent>>` | Called on keydown events on the content element. |

### Standard props

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| -- | `id` | (via spread) | `MaybeProp<String>` | The `id` attribute for the content element. In React this is passed via spread. |
| -- | `aria_labelledby` | (via spread) | `MaybeProp<String>` | The `aria-labelledby` attribute. In React this is passed via spread. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the content DOM element. |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping. |
| *(spread)* | -- | `...PopperContentProps` | -- | React allows spreading HTML attributes. Leptos uses `attr:` directives. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"open" \| "closed"` | Whether the menu is currently open. |
| `data-radix-menu-content` | `""` | Marker attribute used internally for scoping keyboard events to the correct menu level. |

### Implicit behavior

- In modal mode (`modal=true`), the content traps focus, disables outside pointer events, hides all other DOM elements from ARIA using `aria-hidden`, and prevents body scroll.
- In non-modal mode, none of those constraints apply.
- The content element receives `role="menu"`, `aria-orientation="vertical"`, and `outline: none` style.
- Typeahead search is handled internally: single printable characters focus the next matching item. The search buffer resets after 1 second.
- `Tab` key is intercepted and prevented to enforce menu navigation via arrow keys.
- When the content element itself has focus and `Home`/`End`/`PageUp`/`PageDown`/`ArrowUp`/`ArrowDown` is pressed, focus jumps to the first or last non-disabled item.
