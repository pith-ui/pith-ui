---
react_location: "[[reference/react-radix-primitives/packages/react/visually-hidden/src/visually-hidden.tsx|visually-hidden]]"
rust_location: "[[stories/leptos/src/primitives/visually_hidden.rs|visually_hidden]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/visually-hidden.stories.tsx|visually-hidden]]"
rust_story: "[[stories/leptos/src/primitives/visually_hidden.rs|visually_hidden]]"
dependencies:
  - "[[leptos-primitive]]"
ported: true
tested: false
tested_story: false
---
## Intent

Hides content visually while keeping it accessible to screen readers. Used by other primitives (e.g., `AccessibleIcon`) to provide screen-reader-only text.

## React API

```ts
interface VisuallyHiddenProps extends PrimitiveSpanProps {}

const VisuallyHidden: React.ForwardRefExoticComponent<VisuallyHiddenProps>
const VISUALLY_HIDDEN_STYLES: React.CSSProperties
```

Renders a `Primitive.span` with a frozen style object applied. Supports `asChild` and ref forwarding via `Primitive`. Consumer styles are merged after the hidden styles (`...VISUALLY_HIDDEN_STYLES, ...props.style`).

## Leptos API

```rust
#[component]
fn VisuallyHidden(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView
```

Renders a `Primitive` with `html::span` and inline `style:` attributes.

## React Implementation Notes

- Styles are based on [Bootstrap's visually-hidden mixin](https://github.com/twbs/bootstrap/blob/main/scss/mixins/_visually-hidden.scss): `position: absolute`, 1x1px, `clip: rect(0,0,0,0)`, `overflow: hidden`, `margin: -1`.
- `VISUALLY_HIDDEN_STYLES` is exported as a frozen object for reuse.
- Consumer `style` prop merges on top, so individual properties can be overridden.

## Leptos Implementation Notes

- Applies the same CSS properties as individual `style:` directives on the `Primitive` component.
- Does not export a standalone styles constant — the values are inline only.
- Does not expose a mechanism for consumers to merge/override individual style properties (React allows this via `...props.style` spread).
- Dependencies: `leptos`, `leptos-node-ref`, `radix-leptos-primitive`.
