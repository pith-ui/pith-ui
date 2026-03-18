# AlertDialogAction

## React Signature

```typescript
type AlertDialogActionElement = React.ComponentRef<typeof DialogPrimitive.Close>;
type DialogCloseProps = React.ComponentPropsWithoutRef<typeof DialogPrimitive.Close>;
interface AlertDialogActionProps extends DialogCloseProps {}

const AlertDialogAction = React.forwardRef<AlertDialogActionElement, AlertDialogActionProps>(
  (props, forwardedRef) => {
    return <DialogPrimitive.Close {...actionProps} ref={forwardedRef} />;
  },
);
```

`DialogCloseProps` extends `PrimitiveButtonProps` -- all standard `<button>` attributes are accepted.

## Leptos Signature

```rust
pub fn AlertDialogAction(
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `onClick` | `on_click` | `(event: MouseEvent) => void` | `Option<Callback<ev::MouseEvent>>` | Optional click handler composed with the internal close handler. The user's handler runs first; the built-in close handler runs after. Use this to perform the destructive/confirming action before the dialog closes. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<button>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<button>`, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | The action button's label (e.g., "Delete", "Confirm"). |
| *(spread)* | -- | `...DialogCloseProps` | -- | React allows spreading any `<button>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Wraps `DialogClose` -- clicking this button closes the alert dialog.
- Sets `type="button"` on the rendered element. Inherited from `DialogClose`.
