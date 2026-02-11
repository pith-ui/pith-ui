---
react_location: "[[reference/react-radix-primitives/packages/react/use-effect-event/src/use-effect-event.tsx|use-effect-event]]"
rust_location:
react_story: ""
rust_story: ""
dependencies: []
ported: false
tested: false
tested_story: false
---
## Intent

Shim for React's experimental `useEffectEvent` hook. Ensures event handlers can't be called during render.

## React API

```ts
function useEffectEvent<T>(callback?: T): T
```

## React Implementation Notes

- ~36 lines.
- Delegates to native `React.useEffectEvent` if available.
- Falls back to `useInsertionEffect` → `useLayoutEffect` for ref updating.
- Throws error if handler called during render.
- Returns memoized wrapper for stable identity.

## Leptos Equivalent

Not needed. This is React-specific — addresses React's render cycle and effect timing semantics, which don't apply to Leptos's reactive system.
