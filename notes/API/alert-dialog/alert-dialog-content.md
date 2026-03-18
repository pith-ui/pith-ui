# AlertDialogContent

## React Signature

```typescript
type AlertDialogContentElement = React.ComponentRef<typeof DialogPrimitive.Content>;
type DialogContentProps = React.ComponentPropsWithoutRef<typeof DialogPrimitive.Content>;

interface AlertDialogContentProps
  extends Omit<DialogContentProps, 'onPointerDownOutside' | 'onInteractOutside'> {}

const AlertDialogContent = React.forwardRef<AlertDialogContentElement, AlertDialogContentProps>(
  (props, forwardedRef) => {
    return (
      <DialogPrimitive.Content
        role="alertdialog"
        {...contentProps}
        ref={composedRefs}
        onOpenAutoFocus={composeEventHandlers(contentProps.onOpenAutoFocus, (event) => {
          event.preventDefault();
          cancelRef.current?.focus({ preventScroll: true });
        })}
        onPointerDownOutside={(event) => event.preventDefault()}
        onInteractOutside={(event) => event.preventDefault()}
      />
    );
  },
);
```

The underlying `DialogContentProps` (before the `Omit`):

```typescript
interface DialogContentProps extends DialogContentTypeProps {
  forceMount?: true;
}

interface DialogContentTypeProps
  extends Omit<DialogContentImplProps, 'trapFocus' | 'disableOutsidePointerEvents'> {}

interface DialogContentImplProps extends Omit<DismissableLayerProps, 'onDismiss'> {
  trapFocus?: FocusScopeProps['trapped'];
  onOpenAutoFocus?: FocusScopeProps['onMountAutoFocus'];
  onCloseAutoFocus?: FocusScopeProps['onUnmountAutoFocus'];
}
```

## Leptos Signature

```rust
pub fn AlertDialogContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] on_open_auto_focus: Option<Callback<ev::Event>>,
    #[prop(into, optional)] on_close_auto_focus: Option<Callback<ev::Event>>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<ev::CustomEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` | Forces the content to stay mounted in the DOM even when closed. Inherits from `AlertDialogPortal`'s `forceMount` if not set. Useful for controlling enter/exit animations. |
| `onOpenAutoFocus` | `on_open_auto_focus` | `(event: Event) => void` | `Option<Callback<ev::Event>>` | Callback fired when focus is about to move into the content on open. Call `event.preventDefault()` to prevent the default focus behavior. In React, the default behavior focuses the Cancel button via `cancelRef`. In Leptos, FocusScope's `focus_first()` focuses the first tabbable element instead. |
| `onCloseAutoFocus` | `on_close_auto_focus` | `(event: Event) => void` | `Option<Callback<ev::Event>>` | Callback fired when focus is about to move back to the trigger on close. Call `event.preventDefault()` to prevent default focus restoration. |
| `onEscapeKeyDown` | `on_escape_key_down` | `(event: KeyboardEvent) => void` | `Option<Callback<ev::KeyboardEvent>>` | Callback fired when the Escape key is pressed. Call `event.preventDefault()` to prevent the dialog from closing. |
| `onFocusOutside` | `on_focus_outside` | *(omitted via `Omit<>` in React)* | `Option<Callback<ev::CustomEvent>>` | Called when focus moves outside the content. In Leptos, this is exposed for user-space handling even though AlertDialog internally prevents outside interactions. |
| `onPointerDownOutside` | -- | omitted via `Omit<>` | -- | Not exposed. AlertDialog always prevents outside pointer interactions. Internally calls `event.preventDefault()`. |
| `onInteractOutside` | -- | omitted via `Omit<>` | -- | Not exposed. AlertDialog always prevents outside interactions. Internally calls `event.preventDefault()`. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element. |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a container element, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | The content of the alert dialog (title, description, action buttons). Leptos wraps in `Option` to allow empty content. |
| *(spread)* | -- | `...Omit<DialogContentProps, 'onPointerDownOutside' \| 'onInteractOutside'>` | -- | React allows spreading remaining `DialogContent` props. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"open" \| "closed"` | Reflects the alert dialog's open state. Inherited from `DialogContent`. |

### Implicit behavior

- Sets `role="alertdialog"` on the rendered element, overriding Dialog's default `role="dialog"`.
- Auto-generates an `id` for the content element, used by the trigger's `aria-controls`.
- Sets `aria-labelledby` pointing to `AlertDialogTitle`'s auto-generated `id`.
- Sets `aria-describedby` pointing to `AlertDialogDescription`'s auto-generated `id`.
- Provides an `AlertDialogContentContext` containing a `cancelRef`, used by `AlertDialogCancel` to register itself for auto-focus on open.
- Prevents all outside interactions: both `onPointerDownOutside` and `onInteractOutside` are internally prevented, so the dialog cannot be dismissed by clicking outside.
- Focus is trapped within the content while the dialog is open.
- Uses `aria-hidden` on all elements outside the content (via `hideOthers` / `aria-hidden` library) for better screen reader support.
- In React, `onOpenAutoFocus` is overridden to focus the Cancel button. In Leptos, FocusScope's default `focus_first()` behavior handles initial focus (see Behavioral Notes in root file).
