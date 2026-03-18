# AccordionHeader

## React Signature

```typescript
const AccordionHeader = React.forwardRef<AccordionHeaderElement, AccordionHeaderProps>(...)

type AccordionHeaderElement = React.ComponentRef<typeof Primitive.h3>;
type PrimitiveHeading3Props = React.ComponentPropsWithoutRef<typeof Primitive.h3>;

interface AccordionHeaderProps extends PrimitiveHeading3Props {}
```

## Leptos Signature

```rust
pub fn AccordionHeader(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<h3>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in an `<h3>`, merging props and refs. Useful when you want a different heading level. |
| *(spread)* | — | `...PrimitiveHeading3Props` | — | React allows spreading any `<h3>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-orientation` | `"horizontal" \| "vertical"` | Reflects the parent accordion's `orientation`. |
| `data-state` | `"open" \| "closed"` | Whether the parent `AccordionItem`'s content is currently expanded. |
| `data-disabled` | `""` (present/absent) | Present when the parent `AccordionItem` is disabled. |
