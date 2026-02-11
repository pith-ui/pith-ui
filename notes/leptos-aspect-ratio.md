---
react_location: "[[reference/react-radix-primitives/packages/react/aspect-ratio/src/aspect-ratio.tsx|aspect-ratio]]"
rust_location: "[[packages/primitives/leptos/aspect-ratio/src/aspect_ratio.rs|aspect_ratio]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/aspect-ratio.stories.tsx|aspect-ratio]]"
rust_story: "[[stories/leptos/src/primitives/aspect_ratio.rs|aspect_ratio]]"
dependencies:
  - "[[leptos-primitive]]"
ported: true
tested: false
tested_story: true
---
## Intent

Displays content within a desired aspect ratio using the padding-bottom percentage trick. Wraps children in a relatively-positioned container with an absolutely-positioned inner element.

## React API

```ts
interface AspectRatioProps extends PrimitiveDivProps {
  ratio?: number;  // defaults to 1/1
}
const AspectRatio: React.ForwardRefExoticComponent<AspectRatioProps>
```

Renders a wrapper `<div>` with `position: relative`, `width: 100%`, and `paddingBottom: (100/ratio)%`. The inner `Primitive.div` is absolutely positioned to fill the wrapper.

## Leptos API

```rust
#[component]
fn AspectRatio(
    #[prop(into, optional, default = 1.0.into())] ratio: Signal<f64>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView
```

## React Implementation Notes

- Two-element structure: outer `<div>` (wrapper with padding trick) + inner `Primitive.div` (absolutely positioned, fills wrapper).
- Wrapper has `data-radix-aspect-ratio-wrapper` attribute.
- Consumer styles merge onto the inner div.

## Leptos Implementation Notes

- `ratio` is a reactive `Signal<f64>` (React uses a static number).
- Uses `AttributeInterceptor` to capture and forward attributes to the inner `Primitive` — the outer wrapper div doesn't receive consumer attributes.
- `padding-bottom` is computed reactively via `format!("{}%", 100.0 / ratio.get())`.
- Dependencies: `leptos`, `leptos-node-ref`, `radix-leptos-primitive`.
