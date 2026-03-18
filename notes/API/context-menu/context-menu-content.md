# ContextMenuContent

## React Signature

```typescript
const ContextMenuContent = React.forwardRef<ContextMenuContentElement, ContextMenuContentProps>(...)

type ContextMenuContentElement = React.ComponentRef<typeof MenuPrimitive.Content>;
type MenuContentProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.Content>;

interface ContextMenuContentProps
  extends Omit<MenuContentProps, 'onEntryFocus' | 'side' | 'sideOffset' | 'align'> {}

// The inherited MenuContentProps include (after omissions):
// - forceMount?: true
// - loop?: boolean
// - onCloseAutoFocus?: (event: Event) => void
// - onEscapeKeyDown?: (event: KeyboardEvent) => void
// - onPointerDownOutside?: (event: PointerDownOutsideEvent) => void
// - onFocusOutside?: (event: FocusOutsideEvent) => void
// - onInteractOutside?: (event: PointerDownOutsideEvent | FocusOutsideEvent) => void
// - alignOffset?: number
// - avoidCollisions?: boolean
// - collisionBoundary?: Element | Element[] | null
// - collisionPadding?: number | Padding
// - arrowPadding?: number
// - sticky?: 'partial' | 'always'
// - hideWhenDetached?: boolean
// - asChild?: boolean
// - ref
// Plus all PrimitiveDivProps via spread
```

## Leptos Signature

```rust
pub fn ContextMenuContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] align_offset: MaybeProp<f64>,
    #[prop(into, optional)] avoid_collisions: MaybeProp<bool>,
    #[prop(into, optional)] collision_boundary: MaybeProp<SendWrapper<Vec<web_sys::Element>>>,
    #[prop(into, optional)] collision_padding: MaybeProp<Padding>,
    #[prop(into, optional)] arrow_padding: MaybeProp<f64>,
    #[prop(into, optional)] sticky: MaybeProp<Sticky>,
    #[prop(into, optional)] hide_when_detached: MaybeProp<bool>,
    #[prop(into, optional)] r#loop: MaybeProp<bool>,
    #[prop(into, optional)] on_close_auto_focus: Option<Callback<ev::Event>>,
    #[prop(into, optional)] on_interact_outside: Option<Callback<ev::CustomEvent>>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<ev::CustomEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<ev::CustomEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` | Forces the content to stay mounted in the DOM even when closed. Useful for CSS animations on open/close. |
| `loop` | `r#loop` | `boolean` | `MaybeProp<bool>` | Whether keyboard navigation should loop from last item back to first and vice versa. |
| `onCloseAutoFocus` | `on_close_auto_focus` | `(event: Event) => void` | `Option<Callback<ev::Event>>` | Called when auto-focus fires after the menu closes. Call `event.preventDefault()` to prevent the default focus behavior. |
| `onEscapeKeyDown` | `on_escape_key_down` | `(event: KeyboardEvent) => void` | `Option<Callback<ev::KeyboardEvent>>` | Called when the Escape key is pressed while the menu is open. Call `event.preventDefault()` to prevent the menu from closing. |
| `onPointerDownOutside` | `on_pointer_down_outside` | `(event: PointerDownOutsideEvent) => void` | `Option<Callback<ev::CustomEvent>>` | Called when a pointer down event occurs outside the content. Call `event.preventDefault()` to prevent the menu from closing. |
| `onFocusOutside` | `on_focus_outside` | `(event: FocusOutsideEvent) => void` | `Option<Callback<ev::CustomEvent>>` | Called when focus moves outside the content. Call `event.preventDefault()` to prevent the menu from closing. |
| `onInteractOutside` | `on_interact_outside` | `(event: PointerDownOutsideEvent \| FocusOutsideEvent) => void` | `Option<Callback<ev::CustomEvent>>` | Called when any interaction (pointer or focus) occurs outside the content. Fires in addition to the more specific handlers above. |
| `alignOffset` | `align_offset` | `number` (default `0`) | `MaybeProp<f64>` | Offset in pixels from the `"start"` alignment edge. |
| `avoidCollisions` | `avoid_collisions` | `boolean` (default `true`) | `MaybeProp<bool>` | Whether the content should shift to stay within the viewport when it would otherwise overflow. |
| `collisionBoundary` | `collision_boundary` | `Element \| Element[] \| null` | `MaybeProp<SendWrapper<Vec<web_sys::Element>>>` | Elements to use as collision boundaries for overflow detection, in addition to the viewport. |
| `collisionPadding` | `collision_padding` | `number \| Padding` (default `0`) | `MaybeProp<Padding>` | Padding between the content and the collision boundary edges. Accepts a number (all sides) or per-side object. |
| `arrowPadding` | `arrow_padding` | `number` (default `0`) | `MaybeProp<f64>` | Padding between the arrow and the content edges. Prevents the arrow from reaching the very corner of the content. |
| `sticky` | `sticky` | `'partial' \| 'always'` (default `'partial'`) | `MaybeProp<Sticky>` | How the content behaves when its trigger scrolls partially out of view. `'partial'` keeps it anchored as long as the trigger is partially visible; `'always'` keeps it anchored even when fully scrolled past. |
| `hideWhenDetached` | `hide_when_detached` | `boolean` (default `false`) | `MaybeProp<bool>` | Whether to hide the content when the trigger is fully scrolled out of view. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| *(spread)* | — | `...Omit<MenuContentProps, ...>` | — | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"open" \| "closed"` | Reflects whether the menu is currently open. Inherited from `MenuContent`. |

### Implicit behavior

- The `side`, `sideOffset`, and `align` props are hardcoded internally: `side="right"`, `sideOffset={2}`, `align="start"`. These are omitted from the public API because context menus always open at the right-click point with consistent positioning.
- The `onEntryFocus` prop is omitted from the public API (it is used internally by the underlying `MenuContent`).
- On close, if the user interacted outside the content in non-modal mode, auto-focus back to the trigger is suppressed (the `onCloseAutoFocus` handler calls `event.preventDefault()`). This prevents disorienting focus jumps after a context menu dismissal.
- CSS custom properties are set for positioning metadata — see the root file for the full list.
