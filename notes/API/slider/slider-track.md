# SliderTrack

## React Signature

```typescript
const SliderTrack = React.forwardRef<SliderTrackElement, SliderTrackProps>(...)

type SliderTrackElement = React.ComponentRef<typeof Primitive.span>;
type PrimitiveSpanProps = React.ComponentPropsWithoutRef<typeof Primitive.span>;

interface SliderTrackProps extends PrimitiveSpanProps {}
```

## Leptos Signature

```rust
pub fn SliderTrack(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the track DOM element (`<span>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<span>`, merging props and refs. |
| *(spread)* | -- | `...PrimitiveSpanProps` | -- | React allows spreading any `<span>` HTML attribute. Leptos uses `attr:` directives on the call site instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-disabled` | `""` (present/absent) | Present when the parent slider is disabled. Inherited from `SliderContextValue`. |
| `data-orientation` | `"horizontal" \| "vertical"` | Reflects the parent slider's `orientation`. Inherited from `SliderContextValue`. |

### Implicit behavior

- `SliderTrack` reads the `SliderContextValue` from context to obtain `disabled` and `orientation` for rendering data attributes.
- The track is a purely visual element with no interactive behavior of its own. Pointer events on the track bubble up to the `Slider` root, which handles them.
