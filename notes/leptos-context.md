---
react_location: "[[reference/react-radix-primitives/packages/react/context/src/create-context.tsx|create-context]]"
rust_location:
dependencies: []
ported: false
tested: false
---
## Intent

Creates typed React contexts with error handling, optional defaults, and scoping support for nested component composition. Every Radix primitive uses this.

## React API

```ts
function createContext<T>(rootComponentName: string, defaultContext?: T): [Provider, useContext]
function createContextScope(scopeName: string, deps?: CreateScope[]): [createContext, composeContextScopes]
function composeContextScopes(...scopes: CreateScope[]): CreateScope
```

## React Implementation Notes

- ~139 lines.
- `createContext`: Factory returning typed Provider + error-throwing `useContext` hook.
- `createContextScope`: Advanced factory for scoped contexts that support component composition (e.g., Dialog using Dismissable Layer's scope).
- Context memoization based on prop value changes.
- Error messages when consumer used outside provider.

## Leptos Equivalent

Not needed as a separate package. Leptos has built-in `provide_context`/`use_context` which serves the same role. The scoping pattern is not directly applicable — Leptos contexts are type-based, not scope-based.
