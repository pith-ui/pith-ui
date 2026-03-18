# SelectItemIndicator

## React Signature

```typescript
const SelectItemIndicator = React.forwardRef<SelectItemIndicatorElement, SelectItemIndicatorProps>(...)

type SelectItemIndicatorElement = React.ComponentRef<typeof Primitive.span>;

interface SelectItemIndicatorProps extends PrimitiveSpanProps {}
```

## Leptos Signature

```rust
pub fn SelectItemIndicator(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<span>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<span>`, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | The indicator content (e.g., a checkmark icon). |
| *(spread)* | -- | `...PrimitiveSpanProps` | -- | React allows spreading any `<span>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Only rendered when the parent `SelectItem` is selected (`is_selected` is `true`). When not selected, nothing is rendered.
- Renders with `aria-hidden="true"` since the indicator is a visual-only affordance. Screen readers already know which item is selected via `aria-selected` on the item.
