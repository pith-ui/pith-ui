# MenubarTrigger

## React Signature

```typescript
const MenubarTrigger = React.forwardRef<MenubarTriggerElement, MenubarTriggerProps>(...)

type MenubarTriggerElement = React.ComponentRef<typeof Primitive.button>;
type PrimitiveButtonProps = React.ComponentPropsWithoutRef<typeof Primitive.button>;

interface MenubarTriggerProps extends PrimitiveButtonProps {}
```

The `disabled` prop is destructured from `PrimitiveButtonProps` with a default of `false`.

## Leptos Signature

```rust
pub fn MenubarTrigger(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_pointer_down: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_enter: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_focus: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] on_blur: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `disabled` | `disabled` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | Disables the trigger. When disabled, it cannot be clicked, is skipped during keyboard navigation, and receives `data-disabled`. |
| -- | `on_pointer_down` | *(via spread)* | `Option<Callback<ev::PointerEvent>>` | User-provided pointer-down handler composed with the internal handler. In React this is passed via spread props. |
| -- | `on_pointer_enter` | *(via spread)* | `Option<Callback<ev::PointerEvent>>` | User-provided pointer-enter handler composed with the internal handler. |
| -- | `on_key_down` | *(via spread)* | `Option<Callback<ev::KeyboardEvent>>` | User-provided key-down handler composed with the internal handler. |
| -- | `on_focus` | *(via spread)* | `Option<Callback<ev::FocusEvent>>` | User-provided focus handler composed with the internal handler. |
| -- | `on_blur` | *(via spread)* | `Option<Callback<ev::FocusEvent>>` | User-provided blur handler composed with the internal handler. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<button>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<button>`, merging props and refs. |
| *(spread)* | -- | `...PrimitiveButtonProps` | -- | React allows spreading any `<button>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"open" \| "closed"` | Whether the associated menu is currently open. |
| `data-highlighted` | `""` (present/absent) | Present when the trigger has focus. |
| `data-disabled` | `""` (present/absent) | Present when the trigger is disabled. |

### Implicit behavior

- Renders as a `<button>` with `type="button"` and `role="menuitem"`.
- Sets `aria-haspopup="menu"`, `aria-expanded`, and `aria-controls` (pointing to the content's auto-generated ID when open).
- The trigger's `id` is auto-generated and used by `MenubarContent` for `aria-labelledby`.
- Participates in roving focus (via `RovingFocusGroupItem`) for left/right arrow key navigation between triggers.
- Participates in the collection system so `MenubarContent` can navigate between menus.
- Pointer-down on a non-disabled trigger opens the menu (and prevents trigger focus to allow content to receive focus).
- Pointer-enter while another menu is open opens this menu and focuses the trigger.
- `Enter`/`Space` toggles the menu; `ArrowDown` opens it. All three set the keyboard-trigger flag and call `preventDefault`.
