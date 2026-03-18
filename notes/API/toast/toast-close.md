# ToastClose

## React Signature

```typescript
const ToastClose = React.forwardRef<ToastCloseElement, ToastCloseProps>(...)

type ToastCloseElement = React.ComponentRef<typeof Primitive.button>;
type PrimitiveButtonProps = React.ComponentPropsWithoutRef<typeof Primitive.button>;

interface ToastCloseProps extends PrimitiveButtonProps {}
```

## Leptos Signature

```rust
pub fn ToastClose(
    #[prop(into, optional)] on_click: Option<Option<Callback<ev::MouseEvent>>>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| *(via spread)* | `on_click` | `onClick` via `...PrimitiveButtonProps` | `Option<Option<Callback<ev::MouseEvent>>>` | Click handler composed with the internal close handler. In React this comes through the spread; Leptos exposes it as an explicit prop. The double `Option` allows distinguishing "not provided" from "explicitly set to `None`". |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<button>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<button>`, merging props and refs. The child must be a focusable, clickable element. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | The close button content (text or icon). |
| *(spread)* | -- | `...PrimitiveButtonProps` | -- | React allows spreading any `<button>` HTML attribute (e.g., `aria-label`, `onClick`). Leptos uses `attr:` directives instead. |

### Implicit behavior

- Renders a `<button type="button">` element.
- On click, calls the `onClose` handler from the `ToastInteractiveContext`, which dismisses the toast by setting `open` to `false`.
- The click handler is composed with the user-provided `on_click` callback — the user callback runs first, and then the internal close handler runs (unless the event's default is prevented).
- Wrapped in a `ToastAnnounceExclude` so the close button's text is excluded from the screen reader announcement of the toast content.

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-radix-toast-announce-exclude` | `""` | Present on the wrapper element. Marks this subtree as excluded from the announce text extraction. |
