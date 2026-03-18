# MenuSubTrigger

## React Signature

```typescript
const MenuSubTrigger = React.forwardRef<MenuSubTriggerElement, MenuSubTriggerProps>(...)

type MenuSubTriggerElement = MenuItemImplElement; // = Primitive.div element

interface MenuSubTriggerProps extends MenuItemImplProps {}

// MenuItemImplProps:
interface MenuItemImplProps extends PrimitiveDivProps {
  disabled?: boolean;
  textValue?: string;
}
```

## Leptos Signature

```rust
pub fn MenuSubTrigger(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] text_value: MaybeProp<String>,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] on_pointer_move: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_leave: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `disabled` | `disabled` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | When `true`, the trigger cannot open its submenu and is skipped during keyboard navigation. |
| `textValue` | `text_value` | `string` | `MaybeProp<String>` | Text for typeahead matching. Defaults to `.textContent`. |
| `onClick` | `on_click` | `(event: MouseEvent) => void` | `Option<Callback<ev::MouseEvent>>` | Click handler, composed with internal submenu open logic. |
| `onPointerMove` | `on_pointer_move` | `(event: PointerEvent) => void` | `Option<Callback<ev::PointerEvent>>` | Pointer move handler, composed with internal hover-to-open logic. |
| `onPointerLeave` | `on_pointer_leave` | `(event: PointerEvent) => void` | `Option<Callback<ev::PointerEvent>>` | Pointer leave handler, composed with internal grace area logic. |
| `onKeyDown` | `on_key_down` | `(event: KeyboardEvent) => void` | `Option<Callback<ev::KeyboardEvent>>` | Keydown handler, composed with internal submenu open key logic. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping. |
| *(spread)* | -- | `...MenuItemImplProps` | -- | React allows spreading MenuItemImpl props. Leptos uses `attr:` directives. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"open" \| "closed"` | Whether the submenu is currently open. |
| `data-highlighted` | `""` (present/absent) | Present when the trigger is focused. Inherited from `MenuItemImpl`. |
| `data-disabled` | `""` (present/absent) | Present when the trigger is disabled. Inherited from `MenuItemImpl`. |

### Implicit behavior

- Wraps itself in a `MenuAnchor` (with `as_child=true`) so the submenu positions relative to this trigger.
- Renders with auto-generated `id` (from `MenuSubContextValue.triggerId`) and ARIA attributes: `aria-haspopup="menu"`, `aria-expanded`, and `aria-controls` pointing to the submenu content's id.
- On click: focuses itself (for iOS Safari) and opens the submenu if not already open.
- On pointer move (mouse only): after a 100ms delay, opens the submenu. Clears any pending grace area intent.
- On pointer leave (mouse only): cancels the open timer and computes a grace area polygon between the trigger and the submenu content, allowing the user to move the pointer diagonally toward the submenu without it closing. The grace area times out after 300ms.
- On keydown: sub-open keys (`Enter`, `Space`, `ArrowRight` in LTR / `ArrowLeft` in RTL) open the submenu and focus its first non-disabled item via `requestAnimationFrame`.
- Cleans up grace area timers on unmount.
