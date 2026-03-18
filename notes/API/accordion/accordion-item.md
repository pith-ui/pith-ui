# AccordionItem

## React Signature

```typescript
const AccordionItem = React.forwardRef<AccordionItemElement, AccordionItemProps>(...)

type AccordionItemElement = React.ComponentRef<typeof CollapsiblePrimitive.Root>;

interface AccordionItemProps
  extends Omit<CollapsibleProps, 'open' | 'defaultOpen' | 'onOpenChange'> {
  disabled?: boolean;
  value: string;
}
```

## Leptos Signature

```rust
pub fn AccordionItem(
    #[prop(into)] value: String,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `value` | `value` | `string` (required) | `String` (required) | A unique string identifying this item within the accordion. This value is passed to `onValueChange` callbacks and used to determine which item(s) are expanded. |
| `disabled` | `disabled` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | Disables this specific item. Inherits the parent accordion's `disabled` state — the effective disabled is `accordion.disabled \|\| item.disabled`. When disabled, the trigger cannot be clicked and is skipped during keyboard navigation. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the root DOM element. The underlying element is a `CollapsiblePrimitive.Root` (renders as `<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a container element, merging props and refs. |
| *(spread)* | — | `...Omit<CollapsibleProps, 'open' \| 'defaultOpen' \| 'onOpenChange'>` | — | React allows spreading Collapsible root props (except open-state props, which are managed internally). Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-orientation` | `"horizontal" \| "vertical"` | Reflects the parent accordion's `orientation`. |
| `data-state` | `"open" \| "closed"` | Whether this item's content is currently expanded. |
