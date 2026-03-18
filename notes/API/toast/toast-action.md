# ToastAction

## React Signature

```typescript
const ToastAction = React.forwardRef<ToastActionElement, ToastActionProps>(...)

type ToastActionElement = ToastCloseElement;  // React.ComponentRef<typeof Primitive.button>

interface ToastActionProps extends ToastCloseProps {
  /**
   * A short description for an alternate way to carry out the action. For screen reader users
   * who will not be able to navigate to the button easily/quickly.
   * @example <ToastAction altText="Goto account settings to upgrade">Upgrade</ToastAction>
   * @example <ToastAction altText="Undo (Alt+U)">Undo</ToastAction>
   */
  altText: string;
}
```

`ToastActionProps` extends `ToastCloseProps` which extends `PrimitiveButtonProps` -- all standard `<button>` attributes are accepted.

## Leptos Signature

```rust
pub fn ToastAction(
    /// A short description for an alternate way to carry out the action. For screen reader users
    /// who will not be able to navigate to the button easily/quickly.
    #[prop(into)]
    alt_text: String,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `altText` | `alt_text` | `string` (required) | `String` (required, `#[prop(into)]`) | A short description of an alternative way to perform this action, for screen reader users who may not be able to navigate to the button easily. Must be non-empty. In React, an empty string logs a console error and renders nothing. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<button>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<button>`, merging props and refs. The child must be a focusable, clickable element. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | The action button label. |
| *(spread)* | -- | `...ToastCloseProps` (which extends `PrimitiveButtonProps`) | -- | React allows spreading any `<button>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- `ToastAction` wraps a `ToastClose` inside a `ToastAnnounceExclude`. This means:
  1. Clicking the action button dismisses the toast (same as `ToastClose`).
  2. The action button's visible text is excluded from the screen reader announcement. Instead, the `altText` is announced as an alternative description.
- The `altText` prop is set as `data-radix-toast-announce-alt` on the exclude wrapper element, which the announce system reads instead of the button's DOM text content.
- In React, if `altText` is an empty string, the component logs a console error and returns `null` (renders nothing). The Leptos version does not currently replicate this validation.

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-radix-toast-announce-exclude` | `""` | Present on the wrapper element. Marks this subtree as excluded from the announce text extraction. |
| `data-radix-toast-announce-alt` | `string` | The `altText` value, read by the announce system as the alternative text for this action. |
