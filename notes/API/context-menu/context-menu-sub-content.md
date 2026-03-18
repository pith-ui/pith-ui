# ContextMenuSubContent

## React Signature

```typescript
const ContextMenuSubContent = React.forwardRef<
  ContextMenuSubContentElement,
  ContextMenuSubContentProps
>(...)

type ContextMenuSubContentElement = React.ComponentRef<typeof MenuPrimitive.Content>;
type MenuSubContentProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.SubContent>;

interface ContextMenuSubContentProps extends MenuSubContentProps {}

// MenuSubContentProps:
interface MenuSubContentProps
  extends Omit<
    MenuContentImplProps,
    keyof MenuContentImplPrivateProps | 'onCloseAutoFocus' | 'onEntryFocus' | 'side' | 'align'
  > {
  forceMount?: true;
}

// After omissions, the inherited props include:
// - forceMount?: true
// - loop?: boolean
// - onEscapeKeyDown?: (event: KeyboardEvent) => void
// - onPointerDownOutside?: (event: PointerDownOutsideEvent) => void
// - onFocusOutside?: (event: FocusOutsideEvent) => void
// - onInteractOutside?: (event: PointerDownOutsideEvent | FocusOutsideEvent) => void
// - sideOffset?: number
// - alignOffset?: number
// - avoidCollisions?: boolean
// - collisionBoundary?: Element | Element[] | null
// - collisionPadding?: number | Padding
// - arrowPadding?: number
// - sticky?: 'partial' | 'always'
// - hideWhenDetached?: boolean
// - asChild?: boolean
// - ref
```

## Leptos Signature

```rust
pub fn ContextMenuSubContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] side_offset: MaybeProp<f64>,
    #[prop(into, optional)] align_offset: MaybeProp<f64>,
    #[prop(into, optional)] avoid_collisions: MaybeProp<bool>,
    #[prop(into, optional)] collision_boundary: MaybeProp<SendWrapper<Vec<web_sys::Element>>>,
    #[prop(into, optional)] collision_padding: MaybeProp<Padding>,
    #[prop(into, optional)] arrow_padding: MaybeProp<f64>,
    #[prop(into, optional)] sticky: MaybeProp<Sticky>,
    #[prop(into, optional)] hide_when_detached: MaybeProp<bool>,
    #[prop(into, optional)] r#loop: MaybeProp<bool>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<ev::CustomEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<ev::CustomEvent>>,
    #[prop(into, optional)] on_interact_outside: Option<Callback<ev::CustomEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` | Forces the subcontent to stay mounted in the DOM even when the submenu is closed. Useful for CSS animations. |
| `loop` | `r#loop` | `boolean` | `MaybeProp<bool>` | Whether keyboard navigation should loop from last item back to first and vice versa. |
| `onEscapeKeyDown` | `on_escape_key_down` | `(event: KeyboardEvent) => void` | `Option<Callback<ev::KeyboardEvent>>` | Called when the Escape key is pressed. Call `event.preventDefault()` to prevent the submenu from closing. |
| `onPointerDownOutside` | `on_pointer_down_outside` | `(event: PointerDownOutsideEvent) => void` | `Option<Callback<ev::CustomEvent>>` | Called when a pointer down occurs outside the content. |
| `onFocusOutside` | `on_focus_outside` | `(event: FocusOutsideEvent) => void` | `Option<Callback<ev::CustomEvent>>` | Called when focus moves outside the subcontent. |
| `onInteractOutside` | `on_interact_outside` | `(event: PointerDownOutsideEvent \| FocusOutsideEvent) => void` | `Option<Callback<ev::CustomEvent>>` | Called on any outside interaction (pointer or focus). |
| `sideOffset` | `side_offset` | `number` (default `0`) | `MaybeProp<f64>` | Offset in pixels from the sub-trigger along the side axis. |
| `alignOffset` | `align_offset` | `number` (default `0`) | `MaybeProp<f64>` | Offset in pixels from the alignment edge. |
| `avoidCollisions` | `avoid_collisions` | `boolean` (default `true`) | `MaybeProp<bool>` | Whether the content should shift to stay within the viewport. |
| `collisionBoundary` | `collision_boundary` | `Element \| Element[] \| null` | `MaybeProp<SendWrapper<Vec<web_sys::Element>>>` | Additional elements to use as collision boundaries. |
| `collisionPadding` | `collision_padding` | `number \| Padding` (default `0`) | `MaybeProp<Padding>` | Padding between the content and the collision boundary edges. |
| `arrowPadding` | `arrow_padding` | `number` (default `0`) | `MaybeProp<f64>` | Padding between the arrow and the content edges. |
| `sticky` | `sticky` | `'partial' \| 'always'` (default `'partial'`) | `MaybeProp<Sticky>` | How the content behaves when its trigger scrolls partially out of view. |
| `hideWhenDetached` | `hide_when_detached` | `boolean` (default `false`) | `MaybeProp<bool>` | Whether to hide the content when the trigger is fully scrolled out of view. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element. |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a container, merging props and refs. |
| *(spread)* | — | `...MenuSubContentProps` | — | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"open" \| "closed"` | Reflects whether the submenu is currently open. |

### Implicit behavior

- The `side` and `align` props are omitted from the public API — they are set internally by the underlying `MenuSubContent` to position the submenu adjacent to its trigger.
- The `onCloseAutoFocus` and `onEntryFocus` props are omitted from the public API.
- CSS custom properties for positioning metadata are set (same set as `ContextMenuContent`) — see the root file for the full list.
- Pressing `ArrowLeft` (LTR) or `ArrowRight` (RTL) while inside the subcontent closes the submenu and returns focus to the parent `ContextMenuSubTrigger`.
- Pressing `Escape` closes the submenu (and may bubble to close the entire menu).
