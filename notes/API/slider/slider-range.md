# SliderRange

## React Signature

```typescript
const SliderRange = React.forwardRef<SliderRangeElement, SliderRangeProps>(...)

type SliderRangeElement = React.ComponentRef<typeof Primitive.span>;

interface SliderRangeProps extends PrimitiveSpanProps {}
```

## Leptos Signature

```rust
pub fn SliderRange(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the range DOM element (`<span>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<span>`, merging props and refs. |
| *(spread)* | -- | `...PrimitiveSpanProps` | -- | React allows spreading any `<span>` HTML attribute. Leptos uses `attr:` directives on the call site instead. |

Note: `children` is `Option<ChildrenFn>` in Leptos (optional), since the range is typically a leaf element with no children.

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-disabled` | `""` (present/absent) | Present when the parent slider is disabled. Inherited from `SliderContextValue`. |
| `data-orientation` | `"horizontal" \| "vertical"` | Reflects the parent slider's `orientation`. Inherited from `SliderContextValue`. |

### Implicit behavior

- `SliderRange` computes its position and size from the slider's current values.
  - For a single thumb: the range spans from the start edge (0%) to the thumb's percentage.
  - For multiple thumbs: the range spans from the minimum thumb percentage to the maximum thumb percentage.
- Positioning is done via inline styles on the orientation-appropriate CSS edges:
  - Horizontal: `left` and `right` (or `right` and `left` in RTL/inverted mode).
  - Vertical: `bottom` and `top` (or `top` and `bottom` when inverted).
- The range reads both `SliderContextValue` (for values, min, max, orientation, disabled) and `SliderOrientationContextValue` (for start/end edges) from context.
