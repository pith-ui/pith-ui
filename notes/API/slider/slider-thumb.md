# SliderThumb

## React Signature

```typescript
const SliderThumb = React.forwardRef<SliderThumbElement, SliderThumbProps>(...)

type SliderThumbElement = SliderThumbImplElement; // React.ComponentRef<typeof Primitive.span>

interface SliderThumbProps extends Omit<SliderThumbImplProps, 'index'> {}

// Internal (not publicly exposed, but defines accepted props):
interface SliderThumbImplProps extends PrimitiveSpanProps {
  index: number;
  name?: string;
}
```

## Leptos Signature

```rust
pub fn SliderThumb(
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `name` | `name` | `string \| undefined` | `MaybeProp<String>` | Overrides the form input name for this specific thumb. When omitted, the name is derived from the root slider's `name` prop (with `[]` appended for multi-thumb sliders). |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the thumb DOM element (`<span>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<span>`, merging props and refs. |
| *(spread)* | -- | `...PrimitiveSpanProps` | -- | React allows spreading any `<span>` HTML attribute (including `aria-label`). Leptos uses `attr:` directives on the call site instead. |

Note: `children` is `Option<ChildrenFn>` in Leptos (optional), since the thumb is typically a leaf element.

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-disabled` | `""` (present/absent) | Present when the parent slider is disabled. Inherited from `SliderContextValue`. |
| `data-orientation` | `"horizontal" \| "vertical"` | Reflects the parent slider's `orientation`. Inherited from `SliderContextValue`. |

### Implicit behavior

- **Index detection:** The thumb determines its own index within the slider by using the collection system. It finds its position among all registered `SliderThumb` elements. The index maps to the corresponding value in the slider's `values` array.

- **ARIA attributes:** The thumb automatically sets:
  - `role="slider"`
  - `aria-valuemin` (from root `min`)
  - `aria-valuenow` (the thumb's current value)
  - `aria-valuemax` (from root `max`)
  - `aria-orientation` (from root `orientation`)
  - `aria-label` -- auto-generated for multi-thumb sliders:
    - 2 thumbs: "Minimum" / "Maximum"
    - 3+ thumbs: "Value N of M"
    - Single thumb: no auto-generated label (use `attr:aria-label` to provide one)
  - `tabindex="0"` when not disabled, absent when disabled

- **Positioning:** The thumb is absolutely positioned using percentage-based offsets on the appropriate CSS edge (`left`, `right`, `top`, or `bottom` depending on orientation and direction). A bounds-correction offset (`getThumbInBoundsOffset`) keeps the thumb visually within the track at the 0% and 100% extremes.

- **Hidden when no value:** If the thumb's index does not correspond to a value (e.g., during SSR before hydration), the thumb is hidden with `display: none` to prevent visual jank.

- **Focus tracking:** When a thumb receives focus, it updates `value_index_to_change` in the slider context so keyboard interactions adjust the correct value.

- **Registration/cleanup:** Thumbs register themselves in the context's `thumbs` set on mount and unregister on cleanup. This set is used by the root to determine if a pointer-down target is a thumb (which should be focused rather than triggering a slide).

- **Form integration:** Each thumb renders a hidden `<input>` element for form submission when inside a `<form>` or when the root has a `form` prop. The input's name is either the thumb's own `name` prop or derived from the root's `name` (with `[]` suffix for multi-thumb sliders). The hidden input uses a bubble-input pattern to dispatch `input` events when the value changes programmatically.
