# MenuSubContent

## React Signature

```typescript
const MenuSubContent = React.forwardRef<MenuSubContentElement, MenuSubContentProps>(...)

type MenuSubContentElement = MenuContentImplElement;

interface MenuSubContentProps
  extends Omit<
    MenuContentImplProps,
    keyof MenuContentImplPrivateProps | 'onCloseAutoFocus' | 'onEntryFocus' | 'side' | 'align'
  > {
  /** Used to force mounting when more control is needed. */
  forceMount?: true;
}
```

`MenuSubContent` omits `side`, `align`, `onCloseAutoFocus`, `onEntryFocus`, and all private props (`trapFocus`, `disableOutsidePointerEvents`, `disableOutsideScroll`, `onOpenAutoFocus`, `onDismiss`) from the public interface — those are managed internally.

## Leptos Signature

```rust
pub fn MenuSubContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] side_offset: MaybeProp<f64>,
    #[prop(into, optional)] align_offset: MaybeProp<f64>,
    #[prop(into, optional)] avoid_collisions: MaybeProp<bool>,
    #[prop(into, optional)] collision_boundary: MaybeProp<SendWrapper<Vec<web_sys::Element>>>,
    #[prop(into, optional)] collision_padding: MaybeProp<Padding>,
    #[prop(into, optional)] arrow_padding: MaybeProp<f64>,
    #[prop(into, optional)] sticky: MaybeProp<Sticky>,
    #[prop(into, optional)] hide_when_detached: MaybeProp<bool>,
    /// Whether keyboard navigation should loop around.
    #[prop(into, optional)]
    r#loop: MaybeProp<bool>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<ev::CustomEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<ev::CustomEvent>>,
    #[prop(into, optional)] on_interact_outside: Option<Callback<ev::CustomEvent>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

### Positioning props

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `sideOffset` | `side_offset` | `number` (default `0`) | `MaybeProp<f64>` | Distance in pixels from the trigger along the side axis. |
| `alignOffset` | `align_offset` | `number` (default `0`) | `MaybeProp<f64>` | Offset in pixels from the alignment edge. |
| `avoidCollisions` | `avoid_collisions` | `boolean` (default `true`) | `MaybeProp<bool>` | When `true`, overrides positioning to prevent boundary collisions. |
| `collisionBoundary` | `collision_boundary` | `Element \| Element[]` | `MaybeProp<SendWrapper<Vec<web_sys::Element>>>` | The collision boundary element(s). |
| `collisionPadding` | `collision_padding` | `number \| Padding` (default `0`) | `MaybeProp<Padding>` | Padding between boundary edges and the content. |
| `arrowPadding` | `arrow_padding` | `number` (default `0`) | `MaybeProp<f64>` | Padding between the arrow and content edges. |
| `sticky` | `sticky` | `'partial' \| 'always'` (default `'partial'`) | `MaybeProp<Sticky>` | Sticky behavior on the align axis. |
| `hideWhenDetached` | `hide_when_detached` | `boolean` (default `false`) | `MaybeProp<bool>` | Whether to hide content when detached from its anchor. |

### Behavior props

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` | Forces the content to stay mounted even when the submenu is closed. |
| `loop` | `r#loop` | `boolean` (default `false`) | `MaybeProp<bool>` | Whether keyboard navigation loops within the submenu. |
| `onEscapeKeyDown` | `on_escape_key_down` | `(event: KeyboardEvent) => void` | `Option<Callback<ev::KeyboardEvent>>` | Called when Escape is pressed. Composed with internal handler that closes the entire menu tree. |
| `onPointerDownOutside` | `on_pointer_down_outside` | `(event: CustomEvent) => void` | `Option<Callback<ev::CustomEvent>>` | Called when a pointer down event occurs outside the sub-content. |
| `onFocusOutside` | `on_focus_outside` | `(event: CustomEvent) => void` | `Option<Callback<ev::CustomEvent>>` | Called when focus moves outside. Composed with internal handler that closes the submenu (unless focus moved to the trigger). |
| `onInteractOutside` | `on_interact_outside` | `(event: CustomEvent) => void` | `Option<Callback<ev::CustomEvent>>` | Called when any interaction (pointer or focus) occurs outside the sub-content. |
| `onKeyDown` | `on_key_down` | `(event: KeyboardEvent) => void` | `Option<Callback<ev::KeyboardEvent>>` | Called on keydown. Composed with internal handler for sub-close keys. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the content DOM element. |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping. |
| *(spread)* | -- | `...Omit<PopperContentProps, ...>` | -- | React allows spreading remaining PopperContent props. Leptos uses `attr:` directives. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"open" \| "closed"` | Whether the submenu is currently open. |
| `data-radix-menu-content` | `""` | Marker for scoping keyboard events to this menu level. |

### Implicit behavior

- Automatically positions to the `right` side in LTR and `left` side in RTL, with `align="start"`. These are not user-configurable — `side` and `align` are omitted from the public API.
- Sets `id` from `MenuSubContextValue.contentId` and `aria-labelledby` from `MenuSubContextValue.triggerId`.
- Focus is not trapped (`trapFocus=false`), outside pointer events are not disabled, and outside scroll is not prevented — submenus are always non-modal.
- On open auto-focus: for keyboard users, focuses the first non-disabled item via `requestAnimationFrame`. For pointer users, prevents auto-focus.
- On close auto-focus: always prevented — focus is managed manually to avoid refocusing the trigger when switching between submenus.
- On focus outside: if focus moves to a target that is *not* the sub-trigger, the submenu closes. If focus moves to the sub-trigger, the close is suppressed to avoid a re-open animation.
- On Escape: closes the entire menu tree (calls `rootContext.onClose`), not just the submenu.
- Sub-close keys (`ArrowLeft` in LTR / `ArrowRight` in RTL) close the submenu and focus the trigger.
