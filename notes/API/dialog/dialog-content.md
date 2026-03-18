# DialogContent

## React Signature

```typescript
const DialogContent = React.forwardRef<DialogContentElement, DialogContentProps>(...)

type DialogContentElement = DialogContentTypeElement; // React.ComponentRef<typeof DismissableLayer>

interface DialogContentProps extends DialogContentTypeProps {
  /**
   * Used to force mounting when more control is needed. Useful when
   * controlling animation with React animation libraries.
   */
  forceMount?: true;
}

interface DialogContentTypeProps
  extends Omit<DialogContentImplProps, 'trapFocus' | 'disableOutsidePointerEvents'> {}

interface DialogContentImplProps extends Omit<DismissableLayerProps, 'onDismiss'> {
  /**
   * When `true`, focus cannot escape the `Content` via keyboard,
   * pointer, or a programmatic focus.
   * @defaultValue false
   */
  trapFocus?: FocusScopeProps['trapped'];

  /**
   * Event handler called when auto-focusing on open.
   * Can be prevented.
   */
  onOpenAutoFocus?: FocusScopeProps['onMountAutoFocus'];

  /**
   * Event handler called when auto-focusing on close.
   * Can be prevented.
   */
  onCloseAutoFocus?: FocusScopeProps['onUnmountAutoFocus'];
}
```

Note: `trapFocus` and `disableOutsidePointerEvents` are stripped from the public props via `Omit` — they are set internally based on the dialog's `modal` state.

## Leptos Signature

```rust
pub fn DialogContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    /// The ARIA role for the content element. Defaults to `"dialog"`.
    /// AlertDialog overrides this to `"alertdialog"`.
    #[prop(into, optional)]
    role: Option<String>,
    #[prop(into, optional)] on_open_auto_focus: Option<Callback<ev::Event>>,
    #[prop(into, optional)] on_close_auto_focus: Option<Callback<ev::Event>>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<ev::CustomEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<ev::CustomEvent>>,
    #[prop(into, optional)] on_interact_outside: Option<Callback<ev::CustomEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` | Forces the content to stay mounted in the DOM even when closed. Falls back to the portal-level `forceMount` if not set. Useful for exit animations. |
| -- | `role` | -- | `Option<String>` (default `"dialog"`) | Leptos-only. The ARIA role for the content element. Defaults to `"dialog"`. Exposed so that `AlertDialog` can override it to `"alertdialog"`. Not part of the React public API (React hardcodes `role="dialog"` internally). |
| `onOpenAutoFocus` | `on_open_auto_focus` | `(event: Event) => void` | `Option<Callback<ev::Event>>` | Called when the content auto-focuses on mount. Call `event.preventDefault()` to prevent default auto-focus behavior and handle focus manually. |
| `onCloseAutoFocus` | `on_close_auto_focus` | `(event: Event) => void` | `Option<Callback<ev::Event>>` | Called when auto-focus returns on unmount. Call `event.preventDefault()` to prevent default focus-return behavior (which focuses the trigger). |
| `onEscapeKeyDown` | `on_escape_key_down` | `(event: KeyboardEvent) => void` | `Option<Callback<ev::KeyboardEvent>>` | Called when Escape is pressed. Call `event.preventDefault()` to prevent the dialog from closing. Inherited from `DismissableLayer`. |
| `onPointerDownOutside` | `on_pointer_down_outside` | `(event: CustomEvent) => void` | `Option<Callback<ev::CustomEvent>>` | Called when a pointer down occurs outside the content. Call `event.preventDefault()` to prevent dismissal. The event's `detail.originalEvent` contains the native pointer event. Inherited from `DismissableLayer`. |
| `onFocusOutside` | `on_focus_outside` | `(event: CustomEvent) => void` | `Option<Callback<ev::CustomEvent>>` | Called when focus moves outside the content. In modal mode, this is always prevented internally (focus is trapped). Inherited from `DismissableLayer`. |
| `onInteractOutside` | `on_interact_outside` | `(event: CustomEvent) => void` | `Option<Callback<ev::CustomEvent>>` | Called on any outside interaction (pointer or focus). Call `event.preventDefault()` to prevent dismissal. Inherited from `DismissableLayer`. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (the `DismissableLayer` root, typically a `<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a container element, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | The dialog content. Leptos wraps this in `Option` to allow an empty content region. |
| *(spread)* | -- | `...DismissableLayerProps` | -- | React allows spreading any `DismissableLayer` prop (which extends div props). Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"open" \| "closed"` | Reflects the dialog's current open state. |

### Implicit behavior

- Sets `role="dialog"` (or `"alertdialog"` when overridden via the `role` prop).
- Sets `id` to the auto-generated `content_id` from context.
- Sets `aria-labelledby` pointing to the `DialogTitle`'s auto-generated `id`.
- Sets `aria-describedby` pointing to the `DialogDescription`'s auto-generated `id`.
- **Modal mode:** Focus is trapped inside the content (`FocusScope` with `trapped=true`, `loop=true`). `disableOutsidePointerEvents` is set to `true`. Outside elements are hidden from screen readers via `aria-hidden` (`hide_others`). On close, `aria-hidden` is removed from outside elements before focus returns to the trigger (preventing browser warnings). Right-clicks outside the content do not dismiss.
- **Non-modal mode:** Focus is not trapped. Outside pointer events are not disabled. On close, focus returns to the trigger only if the user did not interact outside the dialog. Clicking the trigger while the dialog is open prevents double-toggling.
- **Focus guards:** `use_focus_guards()` is called to ensure focus-guard sentinel elements exist at the edges of the DOM tree, preventing focus from escaping the portal.
- **Dev-time warnings:** In debug builds (`cfg!(debug_assertions)`), console errors are emitted if no `DialogTitle` element is found matching the auto-generated `title_id`, and console warnings are emitted if no `DialogDescription` element is found matching `description_id`.
