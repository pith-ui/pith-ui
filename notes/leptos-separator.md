---
react_location: "[[reference/react-radix-primitives/packages/react/separator/src/separator.tsx|separator]]"
rust_location: "[[packages/primitives/leptos/separator/src/separator.rs|separator]]"
dependencies:
  - "[[leptos-primitive]]"
ported: true
tested: false
---
## Intent

Visually or semantically separates content. Renders a `<div>` with proper ARIA attributes for `separator` role. Supports horizontal/vertical orientation and a decorative mode that removes it from the accessibility tree.

## React API

```ts
interface SeparatorProps extends PrimitiveDivProps {
  orientation?: 'horizontal' | 'vertical';  // default 'horizontal'
  decorative?: boolean;
}
```

When `decorative` is true: `role="none"`, no `aria-orientation`. Otherwise: `role="separator"`, `aria-orientation` set only when vertical (since `horizontal` is the default for `separator` role).

## Leptos API

```rust
#[component]
fn Separator(
    #[prop(into, optional)] orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] decorative: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

`Orientation` is a Rust enum (`Horizontal`, `Vertical`) with `Display` impl.

## React Implementation Notes

- Validates `orientation` with `isValidOrientation()`, falling back to `horizontal` if invalid.
- Sets `data-orientation` always, `aria-orientation` only when vertical and non-decorative.

## Leptos Implementation Notes

- The Rust `Orientation` enum eliminates the need for runtime validation.
- ARIA logic matches React: `aria-orientation` is `None` when decorative or horizontal.
- `data-orientation` is always set.
- Dependencies: `leptos`, `leptos-node-ref`, `radix-leptos-primitive`.
