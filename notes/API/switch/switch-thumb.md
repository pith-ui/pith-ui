# SwitchThumb

## React Signature

```typescript
const SwitchThumb = React.forwardRef<SwitchThumbElement, SwitchThumbProps>(...)

type SwitchThumbElement = React.ComponentRef<typeof Primitive.span>;
type PrimitiveSpanProps = React.ComponentPropsWithoutRef<typeof Primitive.span>;

interface SwitchThumbProps extends PrimitiveSpanProps {}
```

`SwitchThumb` has no custom props beyond the inherited `PrimitiveSpanProps`.

## Leptos Signature

```rust
pub fn SwitchThumb(
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
| *(spread)* | -- | `...PrimitiveSpanProps` | -- | React allows spreading any `<span>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"checked" \| "unchecked"` | Reflects the parent switch's checked state. Read from `SwitchContextValue`. |
| `data-disabled` | `""` (present/absent) | Present when the parent switch is disabled. Read from `SwitchContextValue`. |

### Implicit behavior

- Reads `checked` and `disabled` from the parent `Switch` via Leptos context (`SwitchContextValue`). In React, this uses `useSwitchContext`.
- Renders no ARIA attributes itself -- it is a purely presentational element. All accessibility semantics are on the parent `Switch` button.
- `children` is optional in Leptos. The thumb is typically an empty `<span>` styled via CSS, but children can be provided if needed (e.g., for an icon inside the thumb).
