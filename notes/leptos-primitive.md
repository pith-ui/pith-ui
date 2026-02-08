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

### Monomorphization and wasm linker pressure

Because `Primitive` is generic over `C` (the children type), every call site with a distinct `view!` fragment produces a unique monomorphization of `Primitive<E, C>`. Each `view!` macro invocation returns an anonymous type, so two calls that look identical at the source level still generate different concrete types.

This creates cumulative pressure on the wasm linker (`rust-lld`). In isolation each monomorphization is small, but across a full stories binary with many component demos the total can exceed `rust-lld`'s memory/resource limits, resulting in a **SIGBUS crash** during linking of the debug wasm target. Release builds with LTO may fare better because duplicate code is merged, but debug builds (the default for `trunk serve`) are the primary development workflow.

**Practical impact:** The progress Chromatic story originally instantiated 7 `<Progress>` / `<ProgressIndicator>` pairs (mirroring the React reference). This, combined with all other story components, pushed `rust-lld` past its limit. The workaround was to replace the Chromatic story with a placeholder and cover all states interactively in the Styled story instead.

**Potential fixes (not yet implemented):**
1. **Type-erase children** — Change `Primitive` to accept `ChildrenFn` (or `Box<dyn Fn() -> AnyView>`) instead of `TypedChildrenFn<C>`. This collapses all call sites into a single monomorphization per element type. Tradeoff: loses compile-time children type information and may require boxing.
2. **Reduce element type generics** — Use a single element type (e.g., always `html::div`) and set the tag name dynamically. Tradeoff: loses static element type guarantees.
3. **Split story binaries** — Build each primitive's stories as a separate wasm binary instead of one monolithic binary. Tradeoff: more complex build setup.

Until one of these is adopted, stories that instantiate many `Primitive`-based components should minimize the number of distinct `view!` call sites in a single binary.
