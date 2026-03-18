# AccordionContent

## React Signature

```typescript
const AccordionContent = React.forwardRef<AccordionContentElement, AccordionContentProps>(...)

type AccordionContentElement = React.ComponentRef<typeof CollapsiblePrimitive.Content>;
type CollapsibleContentProps = React.ComponentPropsWithoutRef<typeof CollapsiblePrimitive.Content>;

interface AccordionContentProps extends CollapsibleContentProps {}
```

`CollapsibleContentProps` extends `CollapsibleContentImplProps` (which extends `PrimitiveDivProps`) and adds:

```typescript
interface CollapsibleContentProps extends Omit<CollapsibleContentImplProps, 'present'> {
  forceMount?: true;
}
```

## Leptos Signature

```rust
pub fn AccordionContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` | Forces the content to stay mounted in the DOM even when collapsed. Useful when controlling open/close animations with CSS or animation libraries — without this, the element is removed from the DOM when closed and animations cannot run. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | The collapsible content. Leptos wraps this in `Option` to allow an empty content region. |
| *(spread)* | — | `...CollapsibleContentProps` | — | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Renders with `role="region"` and `aria-labelledby` pointing to the sibling `AccordionTrigger`'s auto-generated `id`.
- Exposes CSS custom properties for animation sizing:
  - `--radix-accordion-content-height` (aliased from `--radix-collapsible-content-height`)
  - `--radix-accordion-content-width` (aliased from `--radix-collapsible-content-width`)

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-orientation` | `"horizontal" \| "vertical"` | Reflects the parent accordion's `orientation`. |
| `data-state` | `"open" \| "closed"` | Inherited from `CollapsibleContent`. Whether the content is currently visible. |
| `data-disabled` | `""` (present/absent) | Inherited from `CollapsibleContent`. Present when the item is disabled. |
