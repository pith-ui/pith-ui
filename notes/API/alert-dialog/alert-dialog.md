# AlertDialog (Root)

## Anatomy

The expected component nesting structure:

```
AlertDialog
├── AlertDialogTrigger
└── AlertDialogPortal
    ├── AlertDialogOverlay
    └── AlertDialogContent
        ├── AlertDialogTitle
        ├── AlertDialogDescription
        ├── AlertDialogAction
        └── AlertDialogCancel
```

### React

```tsx
<AlertDialog.Root>
  <AlertDialog.Trigger>...</AlertDialog.Trigger>
  <AlertDialog.Portal>
    <AlertDialog.Overlay />
    <AlertDialog.Content>
      <AlertDialog.Title>...</AlertDialog.Title>
      <AlertDialog.Description>...</AlertDialog.Description>
      <AlertDialog.Action>...</AlertDialog.Action>
      <AlertDialog.Cancel>...</AlertDialog.Cancel>
    </AlertDialog.Content>
  </AlertDialog.Portal>
</AlertDialog.Root>
```

### Leptos

```rust
<AlertDialog>
  <AlertDialogTrigger>"..."</AlertDialogTrigger>
  <AlertDialogPortal>
    <AlertDialogOverlay />
    <AlertDialogContent>
      <AlertDialogTitle>"..."</AlertDialogTitle>
      <AlertDialogDescription>"..."</AlertDialogDescription>
      <AlertDialogAction>"..."</AlertDialogAction>
      <AlertDialogCancel>"..."</AlertDialogCancel>
    </AlertDialogContent>
  </AlertDialogPortal>
</AlertDialog>
```

## React Signature

AlertDialog wraps `Dialog.Root` with `modal` forced to `true`:

```typescript
type DialogProps = React.ComponentPropsWithoutRef<typeof DialogPrimitive.Root>;
interface AlertDialogProps extends Omit<DialogProps, 'modal'> {}

const AlertDialog: React.FC<AlertDialogProps> = (props) => {
  return <DialogPrimitive.Root {...alertDialogProps} modal={true} />;
};
```

The underlying `DialogProps`:

```typescript
interface DialogProps {
  children?: React.ReactNode;
  open?: boolean;
  defaultOpen?: boolean;
  onOpenChange?(open: boolean): void;
  modal?: boolean; // omitted by AlertDialog — always true
}
```

## Leptos Signature

```rust
pub fn AlertDialog(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `open` | `open` | `boolean \| undefined` | `MaybeProp<bool>` | The controlled open state of the dialog. When set, the component becomes controlled and `onOpenChange` should be used to respond to state changes. |
| `defaultOpen` | `default_open` | `boolean \| undefined` | `MaybeProp<bool>` | The open state on initial render. Use when you do not need to control the open state. |
| `onOpenChange` | `on_open_change` | `(open: boolean) => void` | `Option<Callback<bool>>` | Callback fired when the open state changes. Receives the new open value. |
| `modal` | — | `boolean` (always `true`) | — | Omitted from both React and Leptos `AlertDialogProps`. AlertDialog always forces `modal=true` on the underlying Dialog, meaning the overlay blocks interaction with the rest of the page. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | The alert dialog's child components (trigger, portal, etc.). |

### Implicit behavior

- AlertDialog always renders in modal mode (`modal=true` is hardcoded). Unlike `Dialog`, the `modal` prop is not exposed.
- The underlying `Dialog` context provides auto-generated IDs for content, title, and description, wiring up `aria-labelledby` and `aria-describedby` automatically.

## Usage Examples

### Basic (uncontrolled)

#### React

```tsx
<AlertDialog.Root>
  <AlertDialog.Trigger>Delete item</AlertDialog.Trigger>
  <AlertDialog.Portal>
    <AlertDialog.Overlay />
    <AlertDialog.Content>
      <AlertDialog.Title>Are you sure?</AlertDialog.Title>
      <AlertDialog.Description>
        This action cannot be undone.
      </AlertDialog.Description>
      <AlertDialog.Action>Yes, delete</AlertDialog.Action>
      <AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
    </AlertDialog.Content>
  </AlertDialog.Portal>
</AlertDialog.Root>
```

#### Leptos

```rust
<AlertDialog>
  <AlertDialogTrigger>"Delete item"</AlertDialogTrigger>
  <AlertDialogPortal>
    <AlertDialogOverlay />
    <AlertDialogContent>
      <AlertDialogTitle>"Are you sure?"</AlertDialogTitle>
      <AlertDialogDescription>
        "This action cannot be undone."
      </AlertDialogDescription>
      <AlertDialogAction>"Yes, delete"</AlertDialogAction>
      <AlertDialogCancel>"Cancel"</AlertDialogCancel>
    </AlertDialogContent>
  </AlertDialogPortal>
</AlertDialog>
```

### Controlled

#### React

```tsx
const [open, setOpen] = React.useState(false);

<AlertDialog.Root open={open} onOpenChange={setOpen}>
  <AlertDialog.Trigger>Delete item</AlertDialog.Trigger>
  <AlertDialog.Portal>
    <AlertDialog.Overlay />
    <AlertDialog.Content>
      <AlertDialog.Title>Are you sure?</AlertDialog.Title>
      <AlertDialog.Description>This action cannot be undone.</AlertDialog.Description>
      <AlertDialog.Action>Yes, delete</AlertDialog.Action>
      <AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
    </AlertDialog.Content>
  </AlertDialog.Portal>
</AlertDialog.Root>
```

#### Leptos

```rust
let (open, set_open) = signal(false);

<AlertDialog
  open=open
  on_open_change=Callback::new(move |value: bool| set_open.set(value))
>
  <AlertDialogTrigger>"Delete item"</AlertDialogTrigger>
  <AlertDialogPortal>
    <AlertDialogOverlay />
    <AlertDialogContent>
      <AlertDialogTitle>"Are you sure?"</AlertDialogTitle>
      <AlertDialogDescription>"This action cannot be undone."</AlertDialogDescription>
      <AlertDialogAction>"Yes, delete"</AlertDialogAction>
      <AlertDialogCancel>"Cancel"</AlertDialogCancel>
    </AlertDialogContent>
  </AlertDialogPortal>
</AlertDialog>
```

### Default open

#### React

```tsx
<AlertDialog.Root defaultOpen>
  {/* ...trigger, portal, etc. */}
</AlertDialog.Root>
```

#### Leptos

```rust
<AlertDialog default_open=true>
  // ...trigger, portal, etc.
</AlertDialog>
```

### Action with side effect

#### React

```tsx
<AlertDialog.Action onClick={() => performDangerousAction()}>
  Confirm
</AlertDialog.Action>
```

#### Leptos

```rust
<AlertDialogAction
  on_click=Callback::new(move |_: web_sys::MouseEvent| {
    perform_dangerous_action();
  })
>
  "Confirm"
</AlertDialogAction>
```

## Accessibility

Implements the [WAI-ARIA AlertDialog pattern](https://www.w3.org/WAI/ARIA/apd/patterns/alertdialog/).

### Keyboard Interactions

| Key | Description |
|---|---|
| `Tab` | Moves focus to the next tabbable element inside the content. Focus is trapped within the dialog. |
| `Shift+Tab` | Moves focus to the previous tabbable element inside the content. Focus is trapped within the dialog. |
| `Escape` | Closes the alert dialog and returns focus to the trigger. |

### ARIA Attributes

| Element | Attribute | Value | Notes |
|---|---|---|---|
| `AlertDialogContent` | `role` | `"alertdialog"` | Identifies the element as an alert dialog. Overrides Dialog's default `"dialog"` role. |
| `AlertDialogContent` | `aria-labelledby` | `string` | Auto-generated ID pointing to `AlertDialogTitle`. |
| `AlertDialogContent` | `aria-describedby` | `string` | Auto-generated ID pointing to `AlertDialogDescription`. |
| `AlertDialogTrigger` | `aria-haspopup` | `"dialog"` | Indicates that the trigger opens a dialog. Inherited from `DialogTrigger`. |
| `AlertDialogTrigger` | `aria-expanded` | `"true" \| "false"` | Reflects the open state. Inherited from `DialogTrigger`. |
| `AlertDialogTrigger` | `aria-controls` | `string` | Points to the content element's auto-generated `id`. Inherited from `DialogTrigger`. |
| `AlertDialogTitle` | `id` | `string` | Auto-generated. Referenced by content's `aria-labelledby`. |
| `AlertDialogDescription` | `id` | `string` | Auto-generated. Referenced by content's `aria-describedby`. |

### Behavioral Notes

- AlertDialog is always modal. Unlike Dialog, the `modal` prop is not configurable. The overlay blocks interaction with the rest of the page.
- Clicking outside the alert dialog does not dismiss it. Both `onPointerDownOutside` and `onInteractOutside` are internally prevented.
- On open, focus moves to the `AlertDialogCancel` button by default. In React, this is done by explicitly focusing `cancelRef` in `onOpenAutoFocus`. In Leptos, FocusScope's default `focus_first()` behavior focuses the first tabbable element, which by convention should be the Cancel button (place it before Action in the DOM, or place it after Action to focus Action first).
- Focus is trapped within the dialog content while open.
- On close, focus returns to the trigger element.
- In development mode, warnings are emitted if `AlertDialogTitle` or `AlertDialogDescription` are missing from the content.
