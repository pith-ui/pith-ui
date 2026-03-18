# PopoverTrigger

## React Signature

```typescript
const PopoverTrigger = React.forwardRef<PopoverTriggerElement, PopoverTriggerProps>(...)

type PopoverTriggerElement = React.ComponentRef<typeof Primitive.button>;
type PrimitiveButtonProps = React.ComponentPropsWithoutRef<typeof Primitive.button>;

interface PopoverTriggerProps extends PrimitiveButtonProps {}
```

## Leptos Signature

```rust
pub fn PopoverTrigger(
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<button>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<button>`, merging props and refs. The child must be a focusable, clickable element for accessibility. |
| `onClick` | `on_click` | `MouseEventHandler` | `Option<Callback<ev::MouseEvent>>` | Optional click handler composed with the internal toggle handler. Your handler runs first; the popover toggle always runs after. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | The trigger content. |
| *(spread)* | -- | `...PrimitiveButtonProps` | -- | React allows spreading any `<button>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"open" \| "closed"` | Reflects the popover's open state. |

### Implicit behavior

- Renders `type="button"` to prevent form submission.
- Sets `aria-haspopup="dialog"` to indicate the trigger opens a dialog-type popup.
- Sets `aria-expanded` reflecting the popover's open state.
- Sets `aria-controls` pointing to the content element's auto-generated `id`.
- When no custom `PopoverAnchor` is present, the trigger automatically wraps itself in a `PopperAnchor` so the popover is positioned relative to the trigger.
- When a custom `PopoverAnchor` is present, the trigger renders without a `PopperAnchor` wrapper, and the popover is positioned relative to the custom anchor instead.
- Clicking the trigger toggles the popover open/closed.
