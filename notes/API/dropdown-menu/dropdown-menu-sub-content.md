# DropdownMenuSubContent

## React Signature

```typescript
const DropdownMenuSubContent = React.forwardRef<
  DropdownMenuSubContentElement,
  DropdownMenuSubContentProps
>(...)

type DropdownMenuSubContentElement = React.ComponentRef<typeof MenuPrimitive.Content>;
type MenuSubContentProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.SubContent>;

interface DropdownMenuSubContentProps extends MenuSubContentProps {}
```

`MenuSubContentProps` extends `MenuContentImplProps` but omits private props, `onCloseAutoFocus`, `onEntryFocus`, `side`, and `align` (these are managed internally for submenus):

```typescript
interface MenuSubContentProps
  extends Omit<
    MenuContentImplProps,
    keyof MenuContentImplPrivateProps | 'onCloseAutoFocus' | 'onEntryFocus' | 'side' | 'align'
  > {
  forceMount?: true;
}
```

The remaining `PopperContentProps` include: `sideOffset`, `alignOffset`, `arrowPadding`, `avoidCollisions`, `collisionBoundary`, `collisionPadding`, `sticky`, `hideWhenDetached`.

## Leptos Signature

```rust
pub fn DropdownMenuSubContent(
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

### Positioning props

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `sideOffset` | `side_offset` | `number` (default `0`) | `MaybeProp<f64>` (default `0.0`) | The distance in pixels from the sub-trigger. |
| `alignOffset` | `align_offset` | `number` (default `0`) | `MaybeProp<f64>` (default `0.0`) | An offset in pixels from the alignment edge. |
| `avoidCollisions` | `avoid_collisions` | `boolean` (default `true`) | `MaybeProp<bool>` (default `true`) | When `true`, overrides position preferences to prevent collisions with viewport edges. |
| `collisionBoundary` | `collision_boundary` | `Element \| Element[]` | `MaybeProp<SendWrapper<Vec<web_sys::Element>>>` | The element(s) used as the collision boundary. |
| `collisionPadding` | `collision_padding` | `number \| Partial<Record<Side, number>>` (default `0`) | `MaybeProp<Padding>` (default `0.0`) | Padding from the boundary edges for collision detection. |
| `arrowPadding` | `arrow_padding` | `number` (default `0`) | `MaybeProp<f64>` (default `0.0`) | The padding between the arrow and the content edges. |
| `sticky` | `sticky` | `'partial' \| 'always'` (default `'partial'`) | `MaybeProp<Sticky>` (default `Partial`) | The sticky behavior on the align axis. |
| `hideWhenDetached` | `hide_when_detached` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | Whether to hide the content when fully detached from its reference. |

### Behavior props

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` | Forces the content to stay in the DOM when closed. Useful for controlling animations. |
| `loop` | `r#loop` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | Whether keyboard navigation should loop around within the submenu. |
| `onEscapeKeyDown` | `on_escape_key_down` | `(event: KeyboardEvent) => void` | `Option<Callback<ev::KeyboardEvent>>` | Called when the Escape key is pressed. The internal handler closes the entire menu tree and calls `preventDefault()` to avoid exiting fullscreen. |
| `onPointerDownOutside` | `on_pointer_down_outside` | `(event: CustomEvent) => void` | `Option<Callback<ev::CustomEvent>>` | Called when a pointer down event occurs outside the sub-content. |
| `onFocusOutside` | `on_focus_outside` | `(event: CustomEvent) => void` | `Option<Callback<ev::CustomEvent>>` | Called when focus moves outside the sub-content. The internal handler closes the submenu unless focus moved to the sub-trigger (to avoid a re-open animation loop). |
| `onInteractOutside` | `on_interact_outside` | `(event: CustomEvent) => void` | `Option<Callback<ev::CustomEvent>>` | Called when any interaction (pointer or focus) occurs outside the sub-content. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| *(spread)* | -- | remaining `PopperContentProps` | -- | React allows spreading popper content props. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"open" \| "closed"` | Reflects the submenu's open state. |
| `data-side` | `"top" \| "right" \| "bottom" \| "left"` | The side the content is rendered on. Set by Popper. |
| `data-align` | `"start" \| "center" \| "end"` | The alignment of the content. Set by Popper. |

### Implicit behavior

- The `side` is automatically set based on reading direction: `"right"` for LTR, `"left"` for RTL. The `align` is always `"start"`. These are not configurable on sub-content.
- Sets `id` and `aria-labelledby` from the sub-menu context (pointing to the sub-trigger).
- Sets `role="menu"` and `aria-orientation="vertical"` (from the underlying `MenuContentImpl`).
- Pressing `ArrowLeft` (LTR) or `ArrowRight` (RTL) inside the sub-content closes the submenu and returns focus to the sub-trigger.
- Pressing `Escape` closes the entire menu tree (not just the submenu).
- Focus is not trapped -- it is managed via roving focus within the submenu content.
- Exposes the same CSS custom properties as `DropdownMenuContent` (see root file for the full list).
