---
react_location: "[[reference/react-radix-primitives/packages/core/number/src/number.ts|number]]"
rust_location: "[[packages/primitives/core/number/src/number.rs|number]]"
react_story: ""
rust_story: ""
dependencies: []
ported: true
tested: false
tested_story: false
---
## Intent

Utility for clamping a numeric value within a `[min, max]` range. Used by higher-level primitives that need to constrain values (e.g., sliders, progress bars).

## React API

```ts
function clamp(value: number, [min, max]: [number, number]): number
```

Single export: `clamp`. Takes a value and a destructured `[min, max]` tuple.

## Rust API

```rust
pub fn clamp(value: f64, [min, max]: [f64; 2]) -> f64
```

Identical signature adapted to Rust types. Uses `f64` for all parameters. This is framework-agnostic (lives in `core/`), so there is no Leptos/Yew/Dioxus distinction.

## React Implementation Notes

Uses `Math.min(max, Math.max(min, value))` — standard min/max nesting pattern.

## Rust Implementation Notes

Uses method chaining: `value.max(min).min(max)`. Functionally equivalent to the React version. No dependencies. Crate name: `radix-number`.

Note: No tests exist yet. The function is straightforward but should have edge-case coverage (e.g., `min > max`, `NaN` inputs, `value == min`, `value == max`).
