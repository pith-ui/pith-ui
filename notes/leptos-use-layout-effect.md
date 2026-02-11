---
react_location: "[[reference/react-radix-primitives/packages/react/use-layout-effect/src/use-layout-effect.tsx|use-layout-effect]]"
rust_location: ""
react_story: ""
rust_story: ""
dependencies: []
ported: true
tested: true
tested_story: false
---
## Intent

Safe wrapper around React's `useLayoutEffect` that suppresses server warnings by replacing it with a no-op on the server.

## React API

```ts
const useLayoutEffect: typeof React.useLayoutEffect
```

## React Implementation Notes

- ~12 lines.
- Detects if `document` exists (client-side check).
- Returns `React.useLayoutEffect` on client, empty no-op on server.
- Prevents SSR warnings about `useLayoutEffect` on server.

## Leptos Equivalent

Not needed. This is a React-specific SSR warning suppression. Leptos has different server/client execution patterns that don't require this workaround.
