---
react_location: "[[reference/react-radix-primitives/packages/react/primitive/src/primitive.tsx|primitive]]"
rust_location: "[[packages/primitives/leptos/primitive/src/primitive.rs|primitive]]"
react_story: ""
rust_story: ""
dependencies:
  - "[[leptos-slot]]"
ported: true
tested: false
---
## Intent

The foundational rendering layer for all Radix primitives. Every Radix component ultimately renders through `Primitive`, which provides the `asChild` pattern — allowing consumers to replace the default rendered element with their own child element while merging props and refs.

## React API

```ts
// A Primitive component variant for each HTML element type
const Primitive: {
  a, button, div, form, h2, h3, img, input, label, li, nav, ol, p, select, span, svg, ul
}

// Each variant accepts:
interface PrimitivePropsWithRef<E> {
  asChild?: boolean;  // render child element instead of default
  // ...all native props and ref for the element type
}

function dispatchDiscreteCustomEvent<E extends CustomEvent>(target, event): void
```

When `asChild` is `true`, renders a `Slot` (from `@radix-ui/react-slot`) that merges props/ref onto the child. Otherwise renders the native HTML element. `dispatchDiscreteCustomEvent` wraps `ReactDOM.flushSync` to ensure custom events dispatched inside discrete events (e.g., `pointerdown`) are processed immediately rather than batched.

## Leptos API

```rust
#[component]
fn Primitive<E, C>(
    element: fn() -> HtmlElement<E, (), ()>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<C>,
) -> impl IntoView

#[component]
fn VoidPrimitive<E, C>(/* same props */) -> impl IntoView

fn compose_callbacks<E>(
    original_handler: Option<Callback<E>>,
    our_handler: Option<Callback<E>>,
    check_default_prevented: Option<bool>,
) -> impl Fn(E)
```

- `Primitive` — takes an `element` factory (e.g., `|| html::div()`) instead of a string tag name. Uses `TypedFallbackShow` to switch between rendering the element with children or passing children through directly when `as_child` is true. Ref forwarding via `AnyNodeRef` + `any_node_ref`.
- `VoidPrimitive` — same as `Primitive` but for void elements (e.g., `<input>`) that cannot have children. When `as_child` is false, renders the element without children.
- `compose_callbacks` — Leptos-specific version of `composeEventHandlers` from core. Uses `Callback<E>` instead of `fn(E)`.

## React Implementation Notes

- `NODES` array defines 17 supported HTML element types. `Primitive` is built by reducing over this array, creating a `forwardRef` component per element.
- `asChild` delegates to `Slot` from `@radix-ui/react-slot` which merges props and composes refs.
- Sets `window[Symbol.for('radix-ui')] = true` as a global marker.
- `dispatchDiscreteCustomEvent` uses `ReactDOM.flushSync` to work around React 18's batching of custom event types inside discrete event handlers.

## Leptos Implementation Notes

- Instead of a fixed set of element variants, the Leptos port is generic over `E: ElementType`. The caller passes an element constructor function.
- `TypedFallbackShow` replaces the `Slot` pattern: when `as_child` is true, children are rendered directly with `node_ref` attached; otherwise the element wraps the children.
- `compose_callbacks` lives here (not in `core/primitive`) and uses `Callback<E>` with `.run()` instead of bare function pointers. Combines the `let` guard with `if` for the default-prevented check.
- No equivalent of `dispatchDiscreteCustomEvent` — Leptos doesn't have React's batching semantics, so the flush workaround is unnecessary.
- Dependencies: `leptos`, `leptos-node-ref`, `leptos-typed-fallback-show`.
