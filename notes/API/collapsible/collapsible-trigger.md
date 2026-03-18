# CollapsibleTrigger

## React Signature

```typescript
const CollapsibleTrigger = React.forwardRef<CollapsibleTriggerElement, CollapsibleTriggerProps>(...)

type CollapsibleTriggerElement = React.ComponentRef<typeof Primitive.button>;
type PrimitiveButtonProps = React.ComponentPropsWithoutRef<typeof Primitive.button>;

interface CollapsibleTriggerProps extends PrimitiveButtonProps {}
```

## Leptos Signature

```rust
pub fn CollapsibleTrigger(
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `onClick` | `on_click` | `React.MouseEventHandler` | `Option<Callback<ev::MouseEvent>>` | Optional click handler. Composed with the internal toggle handler -- your handler runs first, then the collapsible toggles. In React this is passed via spread props; in Leptos it is an explicit prop. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<button>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<button>`, merging props and refs. The child must be a focusable, clickable element for accessibility. |
| *(spread)* | — | `...PrimitiveButtonProps` | — | React allows spreading any `<button>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"open" \| "closed"` | Reflects the parent collapsible's open state. |
| `data-disabled` | `""` (present/absent) | Present when the parent collapsible's `disabled` prop is `true`. |

### Implicit behavior

- Renders as `<button type="button">` by default.
- `aria-controls` is automatically set to the auto-generated `content_id` from the parent `Collapsible` context, linking the trigger to its content region.
- `aria-expanded` is automatically set to `"true"` or `"false"` based on the parent collapsible's open state.
- The native `disabled` attribute is set when the parent collapsible is disabled, preventing focus and click at the browser level.
- Clicking the trigger toggles the collapsible open/closed by calling the context's `onOpenToggle` callback. In the Leptos implementation, an explicit guard checks `!context.disabled.get()` before toggling, providing a defense-in-depth layer alongside the native `disabled` attribute.
