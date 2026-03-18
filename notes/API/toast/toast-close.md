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
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<button>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<button>`, merging props and refs. The child must be a focusable, clickable element. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | The close button content (text or icon). |
| *(spread)* | -- | `...PrimitiveButtonProps` | -- | React allows spreading any `<button>` HTML attribute (e.g., `aria-label`, `onClick`). Leptos uses `attr:` directives instead. |

### Implicit behavior

- Renders a `<button type="button">` element.
- On click, calls the `onClose` handler from the `ToastInteractiveContext`, which dismisses the toast by setting `open` to `false`.
- In React, the click handler is composed with any user-provided `onClick` via `composeEventHandlers`. In Leptos, the click handler directly calls `interactive_context.on_close.run(())`.
- Wrapped in a `ToastAnnounceExclude` so the close button's text is excluded from the screen reader announcement of the toast content.

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-radix-toast-announce-exclude` | `""` | Present on the wrapper element. Marks this subtree as excluded from the announce text extraction. |
