# Progress (Root)

## Anatomy

The expected component nesting structure:

```
Progress
└── ProgressIndicator
```

### React

```tsx
<Progress.Root value={50}>
  <Progress.Indicator />
</Progress.Root>
```

### Leptos

```rust
<Progress value=50.0>
  <ProgressIndicator />
</Progress>
```

## React Signature

```typescript
const Progress = React.forwardRef<ProgressElement, ProgressProps>(...)

type ProgressElement = React.ComponentRef<typeof Primitive.div>;
type PrimitiveDivProps = React.ComponentPropsWithoutRef<typeof Primitive.div>;

interface ProgressProps extends PrimitiveDivProps {
  value?: number | null | undefined;
  max?: number;
  getValueLabel?(value: number, max: number): string;
}
```

## Leptos Signature

```rust
pub fn Progress(
    #[prop(into, optional)] value: MaybeProp<f64>,
    #[prop(into, optional)] max: MaybeProp<f64>,
    #[prop(into, optional)] get_value_label: Option<Callback<(f64, f64), String>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `value` | `value` | `number \| null \| undefined` (default `null`) | `MaybeProp<f64>` (default `None`) | The current progress value. Set to a number between `0` and `max` for determinate progress, or omit / set to `null`/`None` for indeterminate progress. Values outside the valid range are clamped to `null`/`None` with a console error. |
| `max` | `max` | `number` (default `100`) | `MaybeProp<f64>` (default `100.0`) | The maximum progress value. Must be a positive number greater than 0. Invalid values fall back to `100`/`100.0` with a console error. |
| `getValueLabel` | `get_value_label` | `(value: number, max: number) => string` | `Option<Callback<(f64, f64), String>>` | Custom function to generate the accessible label for the current value. Receives `(value, max)` and should return a human-readable string. Defaults to `"${Math.round((value / max) * 100)}%"` (e.g., `"50%"`). The returned string is set as `aria-valuetext`. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the root DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, the component renders its child directly instead of wrapping in a `<div>`, merging props and refs onto the child. |
| *(spread)* | -- | `...PrimitiveDivProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives on the call site instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"indeterminate" \| "loading" \| "complete"` | Reflects the progress state: `"indeterminate"` when `value` is `null`/`None`, `"complete"` when `value === max`, `"loading"` otherwise. |
| `data-value` | number string or absent | The current `value` as a string. Absent when indeterminate. |
| `data-max` | number string | The `max` value as a string (e.g. `"100"`). |

### Implicit behavior

- Sets `role="progressbar"` on the root element.
- Sets `aria-valuemin="0"`, `aria-valuemax={max}`, `aria-valuenow={value}` (omitted when indeterminate), and `aria-valuetext={valueLabel}` (omitted when indeterminate).
- Provides a context (`ProgressContextValue`) containing `value` and `max` signals to child components (consumed by `ProgressIndicator`).
- Validates `value` and `max` props at runtime, logging errors for invalid values.

## Usage Examples

### Basic determinate progress

#### React

```tsx
<Progress.Root value={50}>
  <Progress.Indicator style={{ width: '50%' }} />
</Progress.Root>
```

#### Leptos

```rust
<Progress value=50.0>
  <ProgressIndicator attr:style="width: 50%" />
</Progress>
```

### Indeterminate progress

Omit the `value` prop (or set it to `null`/`None`) for indeterminate state:

#### React

```tsx
<Progress.Root>
  <Progress.Indicator />
</Progress.Root>
```

#### Leptos

```rust
<Progress>
  <ProgressIndicator />
</Progress>
```

### Custom max value

#### React

```tsx
<Progress.Root value={75} max={150}>
  <Progress.Indicator />
</Progress.Root>
```

#### Leptos

```rust
<Progress value=75.0 max=150.0>
  <ProgressIndicator />
</Progress>
```

### Custom value label

#### React

```tsx
<Progress.Root
  value={3}
  max={10}
  getValueLabel={(value, max) => `${value} of ${max} tasks complete`}
>
  <Progress.Indicator />
</Progress.Root>
```

#### Leptos

```rust
<Progress
  value=3.0
  max=10.0
  get_value_label=Callback::new(|(value, max): (f64, f64)| {
      format!("{} of {} tasks complete", value, max)
  })
>
  <ProgressIndicator />
</Progress>
```

### Dynamic progress with state

#### React

```tsx
const [value, setValue] = React.useState(0);

<Progress.Root value={value}>
  <Progress.Indicator style={{ width: `${(value / 100) * 100}%` }} />
</Progress.Root>

<input
  type="range"
  value={value}
  max={100}
  onChange={(e) => setValue(Number(e.target.value))}
/>
```

#### Leptos

```rust
let (value, set_value) = signal(Some(0.0));

<Progress value=value>
  <ProgressIndicator
    attr:style=move || value.get().map(|v| format!("width: {}%", v))
  />
</Progress>
```

## Accessibility

Implements the [WAI-ARIA Progressbar pattern](https://www.w3.org/WAI/ARIA/apd/patterns/meter/).

The Progress component uses `role="progressbar"` with appropriate ARIA attributes to communicate progress state to assistive technologies.

### Keyboard Interactions

Progress is a non-interactive component and does not define any keyboard interactions.

### ARIA Attributes

| Element | Attribute | Value | Notes |
|---|---|---|---|
| `Progress` (root) | `role` | `"progressbar"` | Identifies the element as a progress indicator. |
| `Progress` (root) | `aria-valuemin` | `0` | The minimum value (always 0). |
| `Progress` (root) | `aria-valuemax` | number (default `100`) | The maximum value, reflecting the `max` prop. |
| `Progress` (root) | `aria-valuenow` | number or absent | The current value. Absent when indeterminate (`value` is `null`/`None`), signaling to assistive technologies that progress is unknown. |
| `Progress` (root) | `aria-valuetext` | string or absent | A human-readable string for the current value (e.g. `"50%"`). Generated by `getValueLabel`/`get_value_label`. Absent when indeterminate. |

### Behavioral Notes

- When `value` is `null`/`None`, `aria-valuenow` and `aria-valuetext` are both omitted, which communicates an indeterminate state to screen readers.
- The default value label function produces a percentage string (e.g., `"50%"`). Provide a custom `getValueLabel`/`get_value_label` for non-percentage semantics (e.g., `"3 of 10 tasks"`).
- The `data-state` attribute transitions through three states: `"indeterminate"` -> `"loading"` -> `"complete"`, which can be used for CSS-based visual changes (e.g., changing indicator color on completion).
