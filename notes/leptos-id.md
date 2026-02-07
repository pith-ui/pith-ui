---
react_location: "[[reference/react-radix-primitives/packages/react/id/src/id.tsx|id]]"
rust_location: "[[packages/primitives/leptos/id/src/id.rs|id]]"
dependencies: []
ported: true
tested: false
---
## Intent

Generates unique, deterministic-friendly IDs for accessible component associations (e.g., `aria-labelledby`). Supports an optional consumer-provided ID that takes precedence.

## React API

```ts
function useId(deterministicId?: string): string
```

Tries React's built-in `useId` first (React 18+), falls back to a module-level counter. Returns `deterministicId` if provided, otherwise `"radix-{id}"`.

## Leptos API

```rust
pub fn use_id(deterministic_id: Option<String>) -> ReadSignal<String>
```

Returns a `ReadSignal<String>`.

## React Implementation Notes

- Accesses `React.useId` via string indirection (`' useId '.trim()`) to prevent bundler optimization issues.
- Has two-phase initialization: SSR-safe via `useId`, then client-side fallback via `useLayoutEffect` + counter.
- Returns empty string while waiting for client-side ID in pre-React 18.

## Leptos Implementation Notes

- Simpler: uses a global `AtomicUsize` counter (`COUNT`). No SSR/hydration considerations.
- ID is computed immediately at signal creation time — no layout effect needed.
- Returns `ReadSignal` rather than a plain string, so the ID is reactive (though in practice it never changes after creation).
- Format: `"radix-{n}"` where `n` is the counter value.
- No dependencies beyond `leptos`.
