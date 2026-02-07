---
react_location: "[[reference/react-radix-primitives/packages/react/arrow/src/arrow.tsx|arrow]]"
rust_location: "[[packages/primitives/leptos/arrow/src/arrow.rs|arrow]]"
dependencies:
  - "[[leptos-primitive]]"
ported: true
tested: false
---
## Intent

Renders a downward-pointing SVG arrow (triangle). Used internally by `Popper` to draw the arrow between the anchor and content. Supports `asChild` to replace the entire SVG.

## React API

```ts
interface ArrowProps extends PrimitiveSvgProps {
  // width and height default to 10 and 5
}
const Arrow: React.ForwardRefExoticComponent<ArrowProps>
```

Renders a `Primitive.svg` with `viewBox="0 0 30 10"` and `preserveAspectRatio="none"`. When `asChild` is false, renders a `<polygon points="0,0 30,0 15,10" />` inside.

## Leptos API

```rust
#[component]
fn Arrow(
    #[prop(into, optional, default=10.0.into())] width: MaybeProp<f64>,
    #[prop(into, optional, default=5.0.into())] height: MaybeProp<f64>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## React Implementation Notes

- Simple component: SVG wrapper with a polygon child.
- When `asChild` is true, renders children instead of the polygon (for custom arrow shapes).
- Default dimensions: width=10, height=5.

## Leptos Implementation Notes

- Uses `svg::svg` as the `Primitive` element type.
- SVG attributes (`viewBox`, `preserveAspectRatio`) applied via `custom_attribute` since Leptos doesn't have typed SVG attribute support for these.
- Uses `TypedFallbackShow` to switch between polygon and children when `as_child` is set.
- Dependencies: `leptos`, `leptos-node-ref`, `leptos-typed-fallback-show`, `radix-leptos-primitive`.
