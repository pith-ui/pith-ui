---
react_location: "[[reference/react-radix-primitives/packages/react/use-is-hydrated/src/use-is-hydrated.tsx|use-is-hydrated]]"
rust_location: ""
react_story: ""
rust_story: ""
dependencies: []
ported: true
tested: true
tested_story: false
---
## Intent

Determines whether the component tree has been hydrated (SSR → client-side rehydration complete).

## React API

```ts
function useIsHydrated(): boolean
```

## React Implementation Notes

- ~16 lines.
- Uses `useSyncExternalStore` shim for SSR safety.
- Returns `true` as server state, `false` as initial client state.
- After hydration, returns `true` on the client.

## Leptos Equivalent

Not needed for CSR (`wasm32-unknown-unknown` target). Leptos has its own hydration system; if SSR support is added in the future, a similar utility could check `leptos::is_server()` or use `create_effect` to detect client-side mounting. The existing avatar port omits this dependency without issue. May revisit for `PasswordToggleField` or `OneTimePasswordField` if SSR support is added.
