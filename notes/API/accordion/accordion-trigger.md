# AccordionTrigger

## React Signature

```typescript
const AccordionTrigger = React.forwardRef<AccordionTriggerElement, AccordionTriggerProps>(...)

type AccordionTriggerElement = React.ComponentRef<typeof CollapsiblePrimitive.Trigger>;
type CollapsibleTriggerProps = React.ComponentPropsWithoutRef<typeof CollapsiblePrimitive.Trigger>;

interface AccordionTriggerProps extends CollapsibleTriggerProps {}
```

`CollapsibleTriggerProps` inherits from `PrimitiveButtonProps` — all standard `<button>` attributes are accepted.

## Leptos Signature

```rust
pub fn AccordionTrigger(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

The underlying `CollapsibleTrigger` also accepts `on_click: Option<Callback<ev::MouseEvent>>`, but `AccordionTrigger` does not re-expose it — click handling is wired internally.

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<button>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<button>`, merging props and refs. The child must be a focusable, clickable element for accessibility. |
| *(spread)* | — | `...CollapsibleTriggerProps` | — | React allows spreading any `<button>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- The trigger's `id` is auto-generated and used by `AccordionContent` for `aria-labelledby`.
- When the item is open and the accordion is not collapsible (single mode, `collapsible=false`), `aria-disabled="true"` is set to indicate the trigger cannot collapse its section.

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-orientation` | `"horizontal" \| "vertical"` | Reflects the parent accordion's `orientation`. |
| `data-state` | `"open" \| "closed"` | Inherited from `CollapsibleTrigger`. Whether the parent item is expanded. |
| `data-disabled` | `""` (present/absent) | Inherited from `CollapsibleTrigger`. Present when the item is disabled. |
