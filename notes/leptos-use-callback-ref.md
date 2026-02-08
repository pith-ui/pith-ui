---
react_location: "[[reference/react-radix-primitives/packages/react/use-callback-ref/src/use-callback-ref.tsx|use-callback-ref]]"
rust_location:
dependencies: []
ported: true
tested: true
---
## Intent

Converts a callback to a ref-based stable reference to avoid triggering re-renders or re-executing effects when passed as a dependency. Addresses React issue #19240.

## React API

```ts
function useCallbackRef<T>(callback: T | undefined): T
```

## React Implementation Notes

- ~18 lines.
- Stores callback in `React.useRef`, updates on every render via `useEffect`.
- Returns a memoized wrapper function with stable identity.

## Leptos Implementation Notes

**Intentionally omitted — no package or code needed.**

This hook exists solely to work around React's render-cycle semantics: when a callback is passed as a prop or dependency, React may re-execute effects or re-render children because the callback's identity changes on every render. `useCallbackRef` stabilizes the identity by storing the latest callback in a ref.

Leptos does not have this problem. Its fine-grained reactivity model means:
- Closures passed as props do not cause re-execution of effects unless the *signals they read* change.
- `Callback<T>` wraps a closure with a stable identity already.
- There is no dependency array that would trigger re-runs when a callback reference changes.

All React consumers of `useCallbackRef` in the Radix codebase (e.g., `scroll-area`, `use-escape-keydown`, `dismissable-layer`) can simply use the callback directly in Leptos without any wrapper. The already-ported Leptos primitives (`use-escape-keydown`, `focus-scope`, `avatar`) confirm this — none needed an equivalent.
