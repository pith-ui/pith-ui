# MenubarSubContent

## React Signature

```typescript
const MenubarSubContent = React.forwardRef<MenubarSubContentElement, MenubarSubContentProps>(...)

type MenubarSubContentElement = React.ComponentRef<typeof MenuPrimitive.Content>;
type MenuSubContentProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.SubContent>;

interface MenubarSubContentProps extends MenuSubContentProps {}

// Where MenuSubContentProps extends:
interface MenuSubContentProps
  extends Omit<
    MenuContentImplProps,
    keyof MenuContentImplPrivateProps | 'onCloseAutoFocus' | 'onEntryFocus' | 'side' | 'align'
  > {
  forceMount?: true;
}
```

Note: `side` and `align` are not user-configurable on sub-content -- the side is determined by `dir` (right in LTR, left in RTL) and align is always `'start'`.

## Leptos Signature

```rust
pub fn MenubarSubContent(
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

### Positioning props (from Popper)

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `sideOffset` | `side_offset` | `number` (default `0`) | `MaybeProp<f64>` | Distance in pixels from the sub-trigger. |
| `alignOffset` | `align_offset` | `number` (default `0`) | `MaybeProp<f64>` | Offset in pixels from the align edge. |
| `avoidCollisions` | `avoid_collisions` | `boolean` (default `true`) | `MaybeProp<bool>` | Whether the content should avoid colliding with viewport edges. |
| `collisionBoundary` | `collision_boundary` | `Element \| Element[]` | `MaybeProp<SendWrapper<Vec<web_sys::Element>>>` | Elements to use as collision boundaries. |
| `collisionPadding` | `collision_padding` | `number \| Partial<Record<Side, number>>` | `MaybeProp<Padding>` | Padding between the content and the collision boundaries. |
| `arrowPadding` | `arrow_padding` | `number` (default `0`) | `MaybeProp<f64>` | Padding between the arrow and the content edges. |
| `sticky` | `sticky` | `'partial' \| 'always'` (default `'partial'`) | `MaybeProp<Sticky>` | The sticky behavior on the align axis. |
| `hideWhenDetached` | `hide_when_detached` | `boolean` (default `false`) | `MaybeProp<bool>` | Whether to hide the content when detached from the trigger. |

### Content behavior props

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` | Forces the content to stay mounted in the DOM even when closed. |
| `loop` | `r#loop` | `boolean` (default `false`) | `MaybeProp<bool>` | Whether keyboard navigation within the submenu should loop. |
| `onEscapeKeyDown` | `on_escape_key_down` | `(event: KeyboardEvent) => void` | `Option<Callback<ev::KeyboardEvent>>` | Called when the Escape key is pressed. |
| `onPointerDownOutside` | `on_pointer_down_outside` | `(event: CustomEvent) => void` | `Option<Callback<ev::CustomEvent>>` | Called when a pointer down event occurs outside the sub-content. |
| `onFocusOutside` | `on_focus_outside` | `(event: CustomEvent) => void` | `Option<Callback<ev::CustomEvent>>` | Called when focus moves outside the sub-content. |
| `onInteractOutside` | `on_interact_outside` | `(event: CustomEvent) => void` | `Option<Callback<ev::CustomEvent>>` | Called when any interaction (pointer or focus) occurs outside the sub-content. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element. |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly, merging props and refs. |
| *(spread)* | -- | `...MenuSubContentProps` | -- | React allows spreading additional HTML attributes. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-radix-menubar-content` | `""` | Marker attribute used internally for submenu arrow-key navigation detection. |

Plus all data attributes inherited from the underlying `MenuSubContent` (e.g., `data-state`, `data-side`, `data-align`).

### Implicit behavior

- The `side` is automatically set based on `dir`: `'right'` in LTR, `'left'` in RTL. The `align` is always `'start'`. These are not user-configurable.
- CSS custom properties are set for positioning-aware styling (same as `MenubarContent`).
- `data-radix-menubar-content` is set so the parent `MenubarContent`'s key handler can detect when a key event comes from within a submenu.
