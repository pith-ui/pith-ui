# SelectLabel

## React Signature

```typescript
const SelectLabel = React.forwardRef<SelectLabelElement, SelectLabelProps>(...)

type SelectLabelElement = React.ComponentRef<typeof Primitive.div>;

interface SelectLabelProps extends PrimitiveDivProps {}
```

## Leptos Signature

```rust
pub fn SelectLabel(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | The label text content. |
| *(spread)* | -- | `...PrimitiveDivProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Receives the `id` from the parent `SelectGroup`'s context and sets it as its own DOM `id`. This is what `SelectGroup`'s `aria-labelledby` points to, establishing the accessible label relationship.
- Must be a direct child of `SelectGroup`.
