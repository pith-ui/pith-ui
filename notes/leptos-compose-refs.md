---
react_location: "[[reference/react-radix-primitives/packages/react/compose-refs/src/compose-refs.tsx|compose-refs]]"
rust_location: "[[packages/primitives/leptos/compose-refs/src/compose_refs.rs|compose_refs]]"
react_story: ""
rust_story: ""
dependencies: []
ported: true
tested: false
tested_story: false
---
## Intent

Composes multiple refs into one. When the composed ref is attached to a DOM node, all input refs receive that node. Used internally when a component needs to forward a ref while also maintaining its own.

## React API

```ts
function composeRefs<T>(...refs: PossibleRef<T>[]): React.RefCallback<T>
function useComposedRefs<T>(...refs: PossibleRef<T>[]): React.RefCallback<T>
```

Handles both callback refs and `RefObject`s. `useComposedRefs` memoizes the composed ref via `useCallback`. Supports React 19 ref cleanup functions.

## Leptos API

```rust
fn compose_refs(refs: Vec<AnyNodeRef>) -> AnyNodeRef
pub fn use_composed_refs(refs: Vec<AnyNodeRef>) -> AnyNodeRef
```

Returns a new `AnyNodeRef`. Only `use_composed_refs` is public.

## React Implementation Notes

- `setRef` handles both function refs and `RefObject`s.
- Supports React 19 cleanup: if any ref callback returns a cleanup function, the composed ref also returns a cleanup that nullifies all refs.
- `useComposedRefs` wraps `composeRefs` in `useCallback` keyed on the refs array.

## Leptos Implementation Notes

- Creates a new `AnyNodeRef` and uses an `Effect` to propagate — when the composed ref gets a DOM node, it loads that node into each input ref via `NodeRefContainer::<Div>::load`.
- The `Div` type parameter in `NodeRefContainer::<Div>` may limit ref composition to div-like elements. There's a commented-out `r#ref.load_any(&node)` that suggests a more generic approach was attempted.
- `use_composed_refs` is a trivial wrapper around `compose_refs`.
- Dependencies: `leptos`, `leptos-node-ref`.
