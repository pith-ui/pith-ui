# AlertDialogCancel

## React Signature

```typescript
type AlertDialogCancelElement = React.ComponentRef<typeof DialogPrimitive.Close>;
type DialogCloseProps = React.ComponentPropsWithoutRef<typeof DialogPrimitive.Close>;
interface AlertDialogCancelProps extends DialogCloseProps {}

const AlertDialogCancel = React.forwardRef<AlertDialogCancelElement, AlertDialogCancelProps>(
  (props, forwardedRef) => {
    const { cancelRef } = useAlertDialogContentContext(CANCEL_NAME, __scopeAlertDialog);
    const ref = useComposedRefs(forwardedRef, cancelRef);
    return <DialogPrimitive.Close {...cancelProps} ref={ref} />;
  },
);
```

`DialogCloseProps` extends `PrimitiveButtonProps` -- all standard `<button>` attributes are accepted.

## Leptos Signature

```rust
pub fn AlertDialogCancel(
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `onClick` | `on_click` | `(event: MouseEvent) => void` | `Option<Callback<ev::MouseEvent>>` | Optional click handler composed with the internal close handler. The user's handler runs first; the built-in close handler runs after. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<button>`). Composed with the internal `cancelRef` from `AlertDialogContentContext`. |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<button>`, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | The cancel button's label (e.g., "Cancel", "Never mind"). |
| *(spread)* | -- | `...DialogCloseProps` | -- | React allows spreading any `<button>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Wraps `DialogClose` -- clicking this button closes the alert dialog.
- Sets `type="button"` on the rendered element. Inherited from `DialogClose`.
- Registers itself with `AlertDialogContentContext` via a composed ref (`cancelRef`). This ref is used by `AlertDialogContent` to auto-focus the cancel button when the dialog opens.
- In React, `AlertDialogContent` explicitly focuses `cancelRef.current` in `onOpenAutoFocus`. In Leptos, FocusScope's `focus_first()` focuses the first tabbable element in DOM order. Place the Cancel button before the Action button in the DOM to ensure it receives focus on open (matching React's behavior), or place it after to focus the Action button first.
