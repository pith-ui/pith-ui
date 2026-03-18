# MenubarContent

## React Signature

```typescript
const MenubarContent = React.forwardRef<MenubarContentElement, MenubarContentProps>(...)

type MenubarContentElement = React.ComponentRef<typeof MenuPrimitive.Content>;
type MenuContentProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.Content>;

interface MenubarContentProps extends Omit<MenuContentProps, 'onEntryFocus'> {}

// MenuContentProps ultimately extends MenuContentImplProps which extends PopperContentProps:
interface MenuContentImplProps
  extends MenuContentImplPrivateProps,
    Omit<PopperContentProps, 'dir' | 'onPlaced'> {
  onCloseAutoFocus?: FocusScopeProps['onUnmountAutoFocus'];
  loop?: boolean; // default false
  onEntryFocus?: RovingFocusGroupProps['onEntryFocus'];
  onEscapeKeyDown?: DismissableLayerProps['onEscapeKeyDown'];
  onPointerDownOutside?: DismissableLayerProps['onPointerDownOutside'];
  onFocusOutside?: DismissableLayerProps['onFocusOutside'];
  onInteractOutside?: DismissableLayerProps['onInteractOutside'];
  forceMount?: true;
}
```

The `align` prop defaults to `'start'` in `MenubarContent` (overriding the popper default of `'center'`).

## Leptos Signature

```rust
pub fn MenubarContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] align: MaybeProp<Align>,
    #[prop(into, optional)] align_offset: MaybeProp<f64>,
    #[prop(into, optional)] avoid_collisions: MaybeProp<bool>,
    #[prop(into, optional)] collision_boundary: MaybeProp<SendWrapper<Vec<web_sys::Element>>>,
    #[prop(into, optional)] collision_padding: MaybeProp<Padding>,
    #[prop(into, optional)] arrow_padding: MaybeProp<f64>,
    #[prop(into, optional)] sticky: MaybeProp<Sticky>,
    #[prop(into, optional)] hide_when_detached: MaybeProp<bool>,
    #[prop(into, optional)] r#loop: MaybeProp<bool>,
    #[prop(into, optional)] side: MaybeProp<PopperSide>,
    #[prop(into, optional)] side_offset: MaybeProp<f64>,
    #[prop(into, optional)] on_close_auto_focus: Option<Callback<ev::Event>>,
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

### Positioning props (from Popper)

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `side` | `side` | `'top' \| 'right' \| 'bottom' \| 'left'` (default `'bottom'`) | `MaybeProp<PopperSide>` | The preferred side of the trigger to render against. |
| `sideOffset` | `side_offset` | `number` (default `0`) | `MaybeProp<f64>` | Distance in pixels from the trigger. |
| `align` | `align` | `'start' \| 'center' \| 'end'` (default `'start'`) | `MaybeProp<Align>` (default `Align::Start`) | The preferred alignment against the trigger. Menubar overrides the popper default from `'center'` to `'start'`. |
| `alignOffset` | `align_offset` | `number` (default `0`) | `MaybeProp<f64>` | Offset in pixels from the `align` edge. |
| `avoidCollisions` | `avoid_collisions` | `boolean` (default `true`) | `MaybeProp<bool>` | Whether the content should avoid colliding with viewport edges by flipping/shifting. |
| `collisionBoundary` | `collision_boundary` | `Element \| Element[]` | `MaybeProp<SendWrapper<Vec<web_sys::Element>>>` | Elements to use as collision boundaries (in addition to the viewport). |
| `collisionPadding` | `collision_padding` | `number \| Partial<Record<Side, number>>` | `MaybeProp<Padding>` | Padding between the content and the collision boundaries. |
| `arrowPadding` | `arrow_padding` | `number` (default `0`) | `MaybeProp<f64>` | Padding between the arrow and the content edges. Prevents the arrow from overflowing the corners. |
| `sticky` | `sticky` | `'partial' \| 'always'` (default `'partial'`) | `MaybeProp<Sticky>` | The sticky behavior on the align axis. |
| `hideWhenDetached` | `hide_when_detached` | `boolean` (default `false`) | `MaybeProp<bool>` | Whether to hide the content when it becomes detached from the trigger (e.g., due to scrolling). |

### Content behavior props

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` | Forces the content to stay mounted in the DOM even when closed. Useful for animation control. |
| `loop` | `r#loop` | `boolean` (default `false`) | `MaybeProp<bool>` | Whether keyboard navigation within the menu should loop from the last item back to the first. |
| `onCloseAutoFocus` | `on_close_auto_focus` | `(event: Event) => void` | `Option<Callback<ev::Event>>` | Called when auto-focusing on close. Call `event.preventDefault()` to prevent default focus behavior. |
| `onEscapeKeyDown` | `on_escape_key_down` | `(event: KeyboardEvent) => void` | `Option<Callback<ev::KeyboardEvent>>` | Called when the Escape key is pressed. Call `event.preventDefault()` to prevent the menu from closing. |
| `onPointerDownOutside` | `on_pointer_down_outside` | `(event: CustomEvent) => void` | `Option<Callback<ev::CustomEvent>>` | Called when a pointer-down event occurs outside the content. Call `event.preventDefault()` to prevent the menu from closing. |
| `onFocusOutside` | `on_focus_outside` | `(event: CustomEvent) => void` | `Option<Callback<ev::CustomEvent>>` | Called when focus moves outside the content. Call `event.preventDefault()` to prevent the menu from closing. |
| `onInteractOutside` | `on_interact_outside` | `(event: CustomEvent) => void` | `Option<Callback<ev::CustomEvent>>` | Called when any interaction occurs outside the content (superset of pointer-down and focus). |
| -- | `on_key_down` | *(via spread)* | `Option<Callback<ev::KeyboardEvent>>` | User-provided key-down handler composed with the internal handler that manages cross-menu navigation. In React this is passed via spread props. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element. |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly, merging props and refs. |
| *(spread)* | -- | `...MenuContentProps` | -- | React allows spreading additional HTML attributes. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-radix-menubar-content` | `""` | Marker attribute used internally for submenu arrow-key navigation detection. |

Plus all data attributes inherited from the underlying `MenuContent` (e.g., `data-state`, `data-side`, `data-align`).

### Implicit behavior

- Sets `id` to the auto-generated content ID from `MenubarMenuContext` and `aria-labelledby` to the trigger's auto-generated ID.
- When closing without an outside interaction, focus is returned to the trigger.
- Focus-outside events targeting other menubar triggers are prevented (so the menu doesn't dismiss when switching menus).
- `onEntryFocus` is handled internally: if the menu was opened via pointer (not keyboard), the default entry focus is prevented so the first item is not focused.
- `ArrowLeft`/`ArrowRight` key-down events within content navigate between menubar menus. This respects `dir` (LTR/RTL), skips disabled items, and respects the menubar's `loop` prop.
- CSS custom properties are set for positioning-aware styling (see root file).
