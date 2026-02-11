---
react_location: "[[reference/react-radix-primitives/packages/react/progress/src/progress.tsx|progress]]"
rust_location: "[[packages/primitives/leptos/progress/src/progress.rs|progress]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/progress.stories.tsx|progress]]"
rust_story: "[[stories/leptos/src/primitives/progress.rs|progress]]"
dependencies:
  - "[[leptos-primitive]]"
ported: true
tested: false
tested_story: true
---
## Intent

Displays a progress bar with proper ARIA attributes. Supports determinate (value/max) and indeterminate (null value) states. Two sub-components: `Progress` (root) and `ProgressIndicator` (visual fill).

## React API

```ts
interface ProgressProps extends PrimitiveDivProps {
  value?: number | null;
  max?: number;           // default 100
  getValueLabel?(value: number, max: number): string;
}
const Progress: React.ForwardRefExoticComponent<ProgressProps>
const ProgressIndicator: React.ForwardRefExoticComponent<ProgressIndicatorProps>
```

## Leptos API

```rust
#[component] fn Progress(value: MaybeProp<f64>, max: MaybeProp<f64>, ...) -> impl IntoView
#[component] fn ProgressIndicator(...) -> impl IntoView
```

**Note:** Uses old Leptos API. Needs migration.

## React Implementation Notes

- Validates `max` (must be > 0) and `value` (must be 0..=max) with console errors, falls back to defaults.
- Sets `role="progressbar"`, `aria-valuemin=0`, `aria-valuemax`, `aria-valuenow`, `aria-valuetext`.
- `data-state`: `indeterminate` | `loading` | `complete`.
- Default label: `"${Math.round((value / max) * 100)}%"`.

## Leptos Implementation Notes

- `ProgressState` enum with `Display` impl.
- Validation is simpler: clamps value to `0..=max`, treats `max == 0` as invalid.
- No console error logging for invalid props (React logs warnings).
- `get_value_label` accepts `Callback<(f64, f64), String>`.
- Context is provided via `<Provider value=context>` wrapper (not bare `provide_context()`). This is required in Leptos to properly scope context per component instance — without it, sibling Progress components all share the last instance's context.
- Story updated to use stylance + CSS modules (matching accordion pattern) instead of TwClass/tailwind_fuse.

### Bugs fixed
- **Context scoping**: Changed `provide_context(context_value)` to `<Provider value=context_value>` wrapping the Primitive. Without this, multiple Progress instances on the same page all had their ProgressIndicators reading from the last instance's context.
- **Value label rounding**: Fixed `(value / max).round() * 100.0` to `((value / max) * 100.0).round()` — the original rounded before multiplying, causing incorrect `aria-valuetext` for non-boundary values (e.g., 30% showed as 0%).
