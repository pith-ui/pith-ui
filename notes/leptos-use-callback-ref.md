---
react_location: "[[reference/react-radix-primitives/packages/react/use-callback-ref/src/use-callback-ref.tsx|use-callback-ref]]"
rust_location:
dependencies: []
ported: false
tested: false
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

## Leptos Equivalent

Partially relevant. Leptos doesn't have the same re-render optimization concerns, but stable callback references can be useful. Leptos's `Callback` type and closure-based reactivity may obviate the need for this pattern.
