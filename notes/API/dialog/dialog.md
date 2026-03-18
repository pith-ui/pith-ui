# Dialog (Root)

## Anatomy

The expected component nesting structure:

```
Dialog
├── DialogTrigger
└── DialogPortal
    ├── DialogOverlay
    └── DialogContent
        ├── DialogTitle
        ├── DialogDescription
        ├── DialogClose
        └── (user content)
```

### React

```tsx
<Dialog.Root>
  <Dialog.Trigger>...</Dialog.Trigger>
  <Dialog.Portal>
    <Dialog.Overlay />
    <Dialog.Content>
      <Dialog.Title>...</Dialog.Title>
      <Dialog.Description>...</Dialog.Description>
      <Dialog.Close>...</Dialog.Close>
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>
```

### Leptos

```rust
<Dialog>
    <DialogTrigger>"..."</DialogTrigger>
    <DialogPortal>
        <DialogOverlay />
        <DialogContent>
            <DialogTitle>"..."</DialogTitle>
            <DialogDescription>"..."</DialogDescription>
            <DialogClose>"..."</DialogClose>
        </DialogContent>
    </DialogPortal>
</Dialog>
```

## React Signature

```typescript
const Dialog: React.FC<DialogProps>

interface DialogProps {
  children?: React.ReactNode;
  open?: boolean;
  defaultOpen?: boolean;
  onOpenChange?(open: boolean): void;
  modal?: boolean; // default: true
}
```

`Dialog` is a plain `React.FC` (not `forwardRef`) because it does not render a DOM element — it is a context provider only.

## Leptos Signature

```rust
pub fn Dialog(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] modal: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `open` | `open` | `boolean \| undefined` | `MaybeProp<bool>` | The controlled open state. When set, the dialog becomes controlled and `onOpenChange` must be used to update the state. |
| `defaultOpen` | `default_open` | `boolean \| undefined` | `MaybeProp<bool>` | The initial open state when uncontrolled. Defaults to `false`. |
| `onOpenChange` | `on_open_change` | `(open: boolean) => void` | `Option<Callback<bool>>` | Callback fired when the open state changes. Receives the new boolean value. |
| `modal` | `modal` | `boolean` (default `true`) | `MaybeProp<bool>` (default `true`) | Whether the dialog is modal. Modal dialogs trap focus, hide outside content from screen readers via `aria-hidden`, and lock body scroll. Non-modal dialogs allow interaction with content outside the dialog. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | The dialog's child components (Trigger, Portal, etc.). |

### Implicit behavior

- Provides a `DialogContextValue` to all descendants containing: `trigger_ref`, `content_ref`, `content_id`, `title_id`, `description_id`, `open`, `on_open_change`, `on_open_toggle`, and `modal`.
- Auto-generates unique IDs for `content_id`, `title_id`, and `description_id` via `use_id()`. These IDs wire up ARIA relationships between DialogContent, DialogTitle, and DialogDescription automatically.

## Usage Examples

### Basic (uncontrolled)

#### React

```tsx
<Dialog.Root>
  <Dialog.Trigger>Open Dialog</Dialog.Trigger>
  <Dialog.Portal>
    <Dialog.Overlay className="overlay" />
    <Dialog.Content className="content">
      <Dialog.Title>Edit Profile</Dialog.Title>
      <Dialog.Description>Make changes to your profile.</Dialog.Description>
      <Dialog.Close>Save</Dialog.Close>
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>
```

#### Leptos

```rust
<Dialog>
    <DialogTrigger>"Open Dialog"</DialogTrigger>
    <DialogPortal>
        <DialogOverlay attr:class="overlay" />
        <DialogContent attr:class="content">
            <DialogTitle>"Edit Profile"</DialogTitle>
            <DialogDescription>"Make changes to your profile."</DialogDescription>
            <DialogClose>"Save"</DialogClose>
        </DialogContent>
    </DialogPortal>
</Dialog>
```

### Controlled

#### React

```tsx
const [open, setOpen] = React.useState(false);

<Dialog.Root open={open} onOpenChange={setOpen}>
  <Dialog.Trigger>{open ? 'close' : 'open'}</Dialog.Trigger>
  <Dialog.Portal>
    <Dialog.Overlay />
    <Dialog.Content>
      <Dialog.Title>Title</Dialog.Title>
      <Dialog.Description>Description</Dialog.Description>
      <Dialog.Close>close</Dialog.Close>
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>
```

#### Leptos

```rust
let (open, set_open) = signal(false);

<Dialog
    open=open
    on_open_change=Callback::new(move |value: bool| set_open.set(value))
>
    <DialogTrigger>
        {move || if open.get() { "close" } else { "open" }}
    </DialogTrigger>
    <DialogPortal>
        <DialogOverlay />
        <DialogContent>
            <DialogTitle>"Title"</DialogTitle>
            <DialogDescription>"Description"</DialogDescription>
            <DialogClose>"close"</DialogClose>
        </DialogContent>
    </DialogPortal>
</Dialog>
```

### Non-modal

#### React

```tsx
<Dialog.Root modal={false}>
  <Dialog.Trigger>open (non-modal)</Dialog.Trigger>
  <Dialog.Portal>
    <Dialog.Overlay />
    <Dialog.Content
      onInteractOutside={(event) => event.preventDefault()}
    >
      <Dialog.Title>Title</Dialog.Title>
      <Dialog.Description>Description</Dialog.Description>
      <Dialog.Close>close</Dialog.Close>
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>
```

#### Leptos

```rust
<Dialog modal=false>
    <DialogTrigger>"open (non-modal)"</DialogTrigger>
    <DialogPortal>
        <DialogOverlay />
        <DialogContent
            on_interact_outside=Callback::new(|event: ev::CustomEvent| {
                event.prevent_default();
            })
        >
            <DialogTitle>"Title"</DialogTitle>
            <DialogDescription>"Description"</DialogDescription>
            <DialogClose>"close"</DialogClose>
        </DialogContent>
    </DialogPortal>
</Dialog>
```

### Custom focus management

#### React

```tsx
const firstNameRef = React.useRef(null);
const searchFieldRef = React.useRef(null);

<Dialog.Root>
  <Dialog.Trigger>open</Dialog.Trigger>
  <Dialog.Portal>
    <Dialog.Overlay />
    <Dialog.Content
      onOpenAutoFocus={(event) => {
        event.preventDefault();
        firstNameRef.current?.focus();
      }}
      onCloseAutoFocus={(event) => {
        event.preventDefault();
        searchFieldRef.current?.focus();
      }}
    >
      <Dialog.Title>Title</Dialog.Title>
      <Dialog.Description>Description</Dialog.Description>
      <input ref={firstNameRef} placeholder="First name" />
      <Dialog.Close>close</Dialog.Close>
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>
<input ref={searchFieldRef} placeholder="Search..." />
```

#### Leptos

```rust
let first_name_ref = AnyNodeRef::new();
let search_field_ref = AnyNodeRef::new();

<Dialog>
    <DialogTrigger>"open"</DialogTrigger>
    <DialogPortal>
        <DialogOverlay />
        <DialogContent
            on_open_auto_focus=Callback::new(move |event: ev::Event| {
                event.prevent_default();
                if let Some(el) = first_name_ref.get_untracked() {
                    use web_sys::wasm_bindgen::JsCast;
                    let el: &web_sys::HtmlElement = el.unchecked_ref();
                    el.focus().ok();
                }
            })
            on_close_auto_focus=Callback::new(move |event: ev::Event| {
                event.prevent_default();
                if let Some(el) = search_field_ref.get_untracked() {
                    use web_sys::wasm_bindgen::JsCast;
                    let el: &web_sys::HtmlElement = el.unchecked_ref();
                    el.focus().ok();
                }
            })
        >
            <DialogTitle>"Title"</DialogTitle>
            <DialogDescription>"Description"</DialogDescription>
            <input placeholder="First name" node_ref=first_name_ref />
            <DialogClose>"close"</DialogClose>
        </DialogContent>
    </DialogPortal>
</Dialog>
<input placeholder="Search..." node_ref=search_field_ref />
```

### Animated content (CSS keyframes)

Use `data-state` attributes on the overlay and content to animate open/close transitions:

```css
.overlay[data-state='open'],
.content[data-state='open'] {
  animation: fadeIn 150ms ease-out;
}

.overlay[data-state='closed'],
.content[data-state='closed'] {
  animation: fadeOut 150ms ease-in;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to   { opacity: 1; }
}

@keyframes fadeOut {
  from { opacity: 1; }
  to   { opacity: 0; }
}
```

## Accessibility

Implements the [WAI-ARIA Dialog (Modal) pattern](https://www.w3.org/WAI/ARIA/apd/patterns/dialog-modal/).

### Keyboard Interactions

| Key | Description |
|---|---|
| `Space` / `Enter` | When focus is on the trigger, opens the dialog. |
| `Escape` | Closes the dialog and returns focus to the trigger. Can be prevented via `on_escape_key_down`. When a text-editing element inside has focus, the first Escape moves focus to the content container; the second Escape dismisses (two-escapes pattern from DismissableLayer). |
| `Tab` | In modal mode, focus is trapped within the dialog content — Tab cycles through focusable elements and wraps from last to first. In non-modal mode, Tab can move focus outside the dialog. |
| `Shift+Tab` | In modal mode, moves focus to the previous focusable element within the dialog, wrapping from first to last. |

### ARIA Attributes

| Element | Attribute | Value | Notes |
|---|---|---|---|
| `DialogTrigger` | `aria-haspopup` | `"dialog"` | Indicates the trigger opens a dialog. |
| `DialogTrigger` | `aria-expanded` | `"true"` / `"false"` | Reflects the open state of the dialog. |
| `DialogTrigger` | `aria-controls` | `string` | Points to the content element's auto-generated `id`. |
| `DialogContent` | `role` | `"dialog"` | Identifies the content as a dialog. |
| `DialogContent` | `aria-labelledby` | `string` | Points to the `DialogTitle` element's auto-generated `id`. |
| `DialogContent` | `aria-describedby` | `string` | Points to the `DialogDescription` element's auto-generated `id`. |

### Behavioral Notes

- **Modal vs non-modal:** When `modal=true` (default), the dialog traps focus, locks body scroll via the overlay, and uses `aria-hidden` on all elements outside the dialog content. When `modal=false`, none of these behaviors apply.
- **Focus on open:** By default, focus moves to the first focusable element inside the content. Use `on_open_auto_focus` with `event.prevent_default()` to customize which element receives focus.
- **Focus on close:** In modal mode, focus returns to the trigger element by default. Use `on_close_auto_focus` with `event.prevent_default()` to redirect focus elsewhere. In non-modal mode, focus returns to the trigger only if the user did not interact outside the dialog.
- **Right-click on overlay:** In modal mode, right-clicking outside the content (e.g., on the overlay) does not dismiss the dialog — only left-clicks dismiss.
- **Overlay renders only in modal mode:** `DialogOverlay` renders nothing when `modal=false`.
- **Dev-time warnings:** In debug builds, console errors/warnings are emitted if `DialogTitle` or `DialogDescription` are missing from the content, since they are required for screen reader accessibility.
- **Body scroll lock:** In modal mode, the `DialogOverlayImpl` component activates `use_body_scroll_lock()` to prevent background scrolling while the dialog is open.
