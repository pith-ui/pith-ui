# DropdownMenuTrigger

## React Signature

```typescript
const DropdownMenuTrigger = React.forwardRef<DropdownMenuTriggerElement, DropdownMenuTriggerProps>(...)

type DropdownMenuTriggerElement = React.ComponentRef<typeof Primitive.button>;
type PrimitiveButtonProps = React.ComponentPropsWithoutRef<typeof Primitive.button>;

interface DropdownMenuTriggerProps extends PrimitiveButtonProps {}
```

The `disabled` prop is destructured from spread props with a default of `false`.

## Leptos Signature

```rust
pub fn DropdownMenuTrigger(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_pointer_down: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `disabled` | `disabled` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | When `true`, the trigger cannot be interacted with. The button is disabled in the DOM and pointer/keyboard handlers are no-ops. |
| `onPointerDown` | `on_pointer_down` | `(event: PointerEvent) => void` | `Option<Callback<ev::PointerEvent>>` | Composable event handler called before the internal pointer-down logic. |
| `onKeyDown` | `on_key_down` | `(event: KeyboardEvent) => void` | `Option<Callback<ev::KeyboardEvent>>` | Composable event handler called before the internal key-down logic. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<button>`). Composed with the internal trigger ref used for focus restoration. |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<button>`, merging props and refs. The child must be focusable for accessibility. |
| *(spread)* | -- | `...PrimitiveButtonProps` | -- | React allows spreading any `<button>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"open" \| "closed"` | Reflects the dropdown menu's open state. |
| `data-disabled` | `""` (present/absent) | Present when the trigger is disabled. |

### Implicit behavior

- Renders `type="button"` to prevent form submission.
- Sets `aria-haspopup="menu"` to indicate it opens a menu.
- Sets `aria-expanded` to reflect the menu's open state.
- Sets `aria-controls` pointing to the content's auto-generated `id` (only when open).
- The trigger's `id` is auto-generated and used by `DropdownMenuContent` for `aria-labelledby`.
- Left-click (button 0, no Ctrl) toggles the menu. When opening, `preventDefault()` is called to prevent the trigger from stealing focus from the content.
- `Enter` / `Space` toggles the menu; `ArrowDown` opens the menu. All three keys call `preventDefault()`.
- The trigger is wrapped in a `MenuAnchor` so the Popper positions the content relative to it.
