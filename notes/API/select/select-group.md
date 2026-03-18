# SelectGroup

## React Signature

```typescript
const SelectGroup = React.forwardRef<SelectGroupElement, SelectGroupProps>(...)

type SelectGroupElement = React.ComponentRef<typeof Primitive.div>;

interface SelectGroupProps extends PrimitiveDivProps {}
```

## Leptos Signature

```rust
pub fn SelectGroup(
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
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | The group children (typically `SelectLabel` + `SelectItem` entries). |
| *(spread)* | -- | `...PrimitiveDivProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Renders with `role="group"`.
- Auto-generates an `id` used for `aria-labelledby`, linking the group to its `SelectLabel` child.
- Provides a `SelectGroupContext` so that the child `SelectLabel` can read the group's `id`.
