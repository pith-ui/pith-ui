# DialogTrigger

## React Signature

```typescript
const DialogTrigger = React.forwardRef<DialogTriggerElement, DialogTriggerProps>(...)

type DialogTriggerElement = React.ComponentRef<typeof Primitive.button>;
type PrimitiveButtonProps = React.ComponentPropsWithoutRef<typeof Primitive.button>;

interface DialogTriggerProps extends PrimitiveButtonProps {}
```

## Leptos Signature

```rust
pub fn DialogTrigger(
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `onClick` | `on_click` | `MouseEventHandler` | `Option<Callback<ev::MouseEvent>>` | Optional click handler composed with the internal toggle handler. The user's handler runs first; the dialog toggle runs after (unless the event's default is prevented). |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<button>`). Composed with the internal trigger ref stored in context. |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<button>`, merging props and refs. The child must be a focusable, clickable element for accessibility. |
| *(spread)* | -- | `...PrimitiveButtonProps` | -- | React allows spreading any `<button>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"open" \| "closed"` | Reflects the dialog's current open state. |

### Implicit behavior

- Renders `type="button"` to prevent form submission when used inside a form.
- Sets `aria-haspopup="dialog"` to indicate the button opens a dialog.
- Sets `aria-expanded` reflecting the dialog's open state (`"true"` or `"false"`).
- Sets `aria-controls` pointing to the content element's auto-generated `id`.
- Click toggles the dialog open/closed via `on_open_toggle` from context.
- The trigger's node ref is stored in context so that `DialogContent` can return focus to it on close.
