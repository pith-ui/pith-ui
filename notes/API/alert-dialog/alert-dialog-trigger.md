# AlertDialogTrigger

## React Signature

```typescript
type AlertDialogTriggerElement = React.ComponentRef<typeof DialogPrimitive.Trigger>;
type DialogTriggerProps = React.ComponentPropsWithoutRef<typeof DialogPrimitive.Trigger>;
interface AlertDialogTriggerProps extends DialogTriggerProps {}

const AlertDialogTrigger = React.forwardRef<AlertDialogTriggerElement, AlertDialogTriggerProps>(
  (props, forwardedRef) => {
    return <DialogPrimitive.Trigger {...triggerProps} ref={forwardedRef} />;
  },
);
```

`DialogTriggerProps` extends `PrimitiveButtonProps` -- all standard `<button>` attributes are accepted.

## Leptos Signature

```rust
pub fn AlertDialogTrigger(
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `onClick` | `on_click` | `(event: MouseEvent) => void` | `Option<Callback<ev::MouseEvent>>` | Optional click handler composed with the internal toggle handler. The user's handler runs first; the built-in open-toggle runs after unless `preventDefault()` is called. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<button>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<button>`, merging props and refs. The child must be a focusable, clickable element for accessibility. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | The trigger's content (typically a text label). |
| *(spread)* | -- | `...DialogTriggerProps` | -- | React allows spreading any `<button>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"open" \| "closed"` | Reflects the alert dialog's open state. Inherited from `DialogTrigger`. |

### Implicit behavior

- Sets `type="button"` on the rendered element.
- Sets `aria-haspopup="dialog"`, `aria-expanded`, and `aria-controls` pointing to the content's auto-generated ID. All inherited from `DialogTrigger`.
- Clicking the trigger toggles the alert dialog open/closed.
