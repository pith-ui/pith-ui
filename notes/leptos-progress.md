---
react_location: "[[reference/react-radix-primitives/packages/react/progress/src/progress.tsx|progress]]"
rust_location: "[[packages/primitives/leptos/progress/src/progress.rs|progress]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/progress.stories.tsx|progress]]"
rust_story: "[[stories/leptos/src/primitives/progress.rs|progress]]"
dependencies:
  - "[[leptos-primitive]]"
ported: true
tested: false
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

- `ProgressState` enum with `Display` and `IntoAttribute` impls (old API).
- Validation is simpler: clamps value to `0..=max`, treats `max == 0` as invalid.
- No console error logging for invalid props (React logs warnings).
- `get_value_label` accepts `Box<dyn Fn(f64, f64) -> String>` — non-reactive, set once.
- Uses old Leptos API — needs migration.
- Dependencies: `leptos` only.
