# ProgressIndicator

## React Signature

```typescript
const ProgressIndicator = React.forwardRef<ProgressIndicatorElement, ProgressIndicatorProps>(...)

type ProgressIndicatorElement = React.ComponentRef<typeof Primitive.div>;

interface ProgressIndicatorProps extends PrimitiveDivProps {}
```

`ProgressIndicatorProps` inherits from `PrimitiveDivProps` -- all standard `<div>` attributes are accepted. The component itself defines no additional props.

## Leptos Signature

```rust
pub fn ProgressIndicator(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | Optional content to render inside the indicator. Often left empty, with the indicator styled purely via CSS (e.g., a colored bar whose width reflects progress). |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| *(spread)* | -- | `...PrimitiveDivProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"indeterminate" \| "loading" \| "complete"` | Mirrors the parent `Progress` root's state. `"indeterminate"` when `value` is `null`/`None`, `"complete"` when `value === max`, `"loading"` otherwise. |
| `data-value` | number string or absent | The current `value` from the parent context, as a string. Absent when indeterminate. |
| `data-max` | number string | The `max` value from the parent context, as a string. |

### Implicit behavior

- Reads `value` and `max` from the `ProgressContextValue` provided by the parent `Progress` component. Must be a descendant of `Progress`.
- Does not set any ARIA attributes itself -- all accessibility semantics are on the parent `Progress` root element.
- Typically styled with CSS to visually represent the progress (e.g., a bar whose `width` is set to the completion percentage). The `data-state` attribute can be used to style different visual states (different colors for loading vs. complete vs. indeterminate).
