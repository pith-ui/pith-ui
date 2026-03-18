# DialogClose

## React Signature

```typescript
const DialogClose = React.forwardRef<DialogCloseElement, DialogCloseProps>(...)

type DialogCloseElement = React.ComponentRef<typeof Primitive.button>;

interface DialogCloseProps extends PrimitiveButtonProps {}
```

## Leptos Signature

```rust
pub fn DialogClose(
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `onClick` | `on_click` | `MouseEventHandler` | `Option<Callback<ev::MouseEvent>>` | Optional click handler composed with the internal close handler. The user's handler runs first; the dialog closes after (unless the event's default is prevented). |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<button>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<button>`, merging props and refs. |
| *(spread)* | -- | `...PrimitiveButtonProps` | -- | React allows spreading any `<button>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Renders `type="button"` to prevent form submission when used inside a form.
- Click calls `context.on_open_change(false)` to close the dialog.
