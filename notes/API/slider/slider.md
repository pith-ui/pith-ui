# Slider (Root)

## Anatomy

The expected component nesting structure:

```
Slider
├── SliderTrack
│   └── SliderRange
└── SliderThumb (one per value)
```

### React

```tsx
<Slider.Root defaultValue={[50]}>
  <Slider.Track>
    <Slider.Range />
  </Slider.Track>
  <Slider.Thumb />
</Slider.Root>
```

### Leptos

```rust
<Slider default_value=vec![50.0]>
  <SliderTrack>
    <SliderRange />
  </SliderTrack>
  <SliderThumb />
</Slider>
```

## React Signature

```typescript
const Slider = React.forwardRef<SliderElement, SliderProps>(...)

type SliderElement = SliderHorizontalElement | SliderVerticalElement;

interface SliderProps
  extends Omit<
    SliderHorizontalProps | SliderVerticalProps,
    keyof SliderOrientationPrivateProps | 'defaultValue'
  > {
  name?: string;
  disabled?: boolean;
  orientation?: React.AriaAttributes['aria-orientation'];
  dir?: Direction; // 'ltr' | 'rtl'
  min?: number;
  max?: number;
  step?: number;
  minStepsBetweenThumbs?: number;
  value?: number[];
  defaultValue?: number[];
  onValueChange?(value: number[]): void;
  onValueCommit?(value: number[]): void;
  inverted?: boolean;
  form?: string;
}
```

## Leptos Signature

```rust
pub fn Slider(
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] min: MaybeProp<f64>,
    #[prop(into, optional)] max: MaybeProp<f64>,
    #[prop(into, optional)] step: MaybeProp<f64>,
    #[prop(into, optional)] min_steps_between_thumbs: MaybeProp<f64>,
    #[prop(into, optional)] value: MaybeProp<Vec<f64>>,
    #[prop(into, optional)] default_value: MaybeProp<Vec<f64>>,
    #[prop(into, optional)] on_value_change: Option<Callback<Vec<f64>>>,
    #[prop(into, optional)] on_value_commit: Option<Callback<Vec<f64>>>,
    #[prop(into, optional)] inverted: MaybeProp<bool>,
    #[prop(into, optional)] form: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `name` | `name` | `string \| undefined` | `MaybeProp<String>` | The name attribute for the hidden input element(s) used for form submission. When multiple thumbs exist, each hidden input gets `name[]` as its name (unless the thumb has its own `name` prop). |
| `disabled` | `disabled` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | When `true`, the slider ignores all user interaction. All thumbs lose their `tabindex` and the root renders `aria-disabled="true"`. |
| `orientation` | `orientation` | `'horizontal' \| 'vertical'` (default `'horizontal'`) | `MaybeProp<Orientation>` (default `Horizontal`) | The axis along which the slider operates. Controls which arrow keys adjust the value and which CSS dimension is used for positioning. |
| `dir` | `dir` | `'ltr' \| 'rtl'` | `MaybeProp<Direction>` | The reading direction. Affects which end of a horizontal slider is the minimum. In RTL, the minimum is on the right. |
| `min` | `min` | `number` (default `0`) | `MaybeProp<f64>` (default `0.0`) | The minimum allowed value. |
| `max` | `max` | `number` (default `100`) | `MaybeProp<f64>` (default `100.0`) | The maximum allowed value. |
| `step` | `step` | `number` (default `1`) | `MaybeProp<f64>` (default `1.0`) | The stepping interval. Values snap to the nearest multiple of `step` offset from `min`. Supports fractional steps (e.g., `0.003`). |
| `minStepsBetweenThumbs` | `min_steps_between_thumbs` | `number` (default `0`) | `MaybeProp<f64>` (default `0.0`) | The minimum number of steps between any two thumbs. Prevents thumbs from overlapping. For example, with `step=1` and `minStepsBetweenThumbs=3`, thumbs must be at least 3 apart. |
| `value` | `value` | `number[] \| undefined` | `MaybeProp<Vec<f64>>` | The controlled value(s). Each element corresponds to a `SliderThumb`. When set, the component is controlled and `onValueChange` must be used to update state. |
| `defaultValue` | `default_value` | `number[]` (default `[min]`) | `MaybeProp<Vec<f64>>` (default `[min]`) | The initial uncontrolled value(s). Defaults to a single-element array containing `min`. |
| `onValueChange` | `on_value_change` | `(value: number[]) => void` | `Option<Callback<Vec<f64>>>` | Callback fired continuously as the value changes during dragging or keyboard interaction. |
| `onValueCommit` | `on_value_commit` | `(value: number[]) => void` | `Option<Callback<Vec<f64>>>` | Callback fired when a drag interaction ends and the value has changed from when the drag started, or when a keyboard step commits. Useful for "final" value changes (e.g., saving to a server). |
| `inverted` | `inverted` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | When `true`, the slider direction is inverted. For horizontal LTR, the minimum moves to the right. For vertical, the minimum moves to the top. |
| `form` | `form` | `string \| undefined` | `MaybeProp<String>` | The `form` attribute for the hidden input elements, associating the slider with a form that is not its ancestor. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the root DOM element (`<span>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, the component renders its child directly instead of wrapping in a `<span>`, merging props and refs onto the child. |
| *(spread)* | -- | `...PrimitiveSpanProps` | -- | React allows spreading any `<span>` HTML attribute. Leptos uses `attr:` directives on the call site instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-disabled` | `""` (present/absent) | Present when the slider is disabled. |
| `data-orientation` | `"horizontal" \| "vertical"` | Reflects the `orientation` prop. |

### Implicit behavior

- The root renders as a `<span>` element (not a `<div>`).
- A `SliderContextValue` is provided to all descendants, carrying `name`, `disabled`, `min`, `max`, `values`, `orientation`, `form`, `thumbs`, and `value_index_to_change`.
- An orientation sub-context (`SliderOrientationContextValue`) is provided with `start_edge`, `end_edge`, `size`, and `direction` — computed based on `orientation`, `dir`, and `inverted`.
- The root handles all pointer and keyboard events. Pointer capture is used for drag tracking.
- The CSS custom property `--slider-thumb-transform` is set on the root to `translateX(-50%)` (horizontal) or `translateY(50%)` (vertical).
- When a value changes, the corresponding thumb is automatically focused.

## Usage Examples

### Basic uncontrolled

#### React

```tsx
<Slider.Root defaultValue={[50]}>
  <Slider.Track>
    <Slider.Range />
  </Slider.Track>
  <Slider.Thumb />
</Slider.Root>
```

#### Leptos

```rust
<Slider default_value=vec![50.0]>
  <SliderTrack>
    <SliderRange />
  </SliderTrack>
  <SliderThumb />
</Slider>
```

### Controlled

#### React

```tsx
const [value, setValue] = React.useState([50]);

<Slider.Root value={value} onValueChange={setValue}>
  <Slider.Track>
    <Slider.Range />
  </Slider.Track>
  <Slider.Thumb />
</Slider.Root>
```

#### Leptos

```rust
let (value, set_value) = signal(vec![50.0]);

<Slider
  value=value
  on_value_change=Callback::new(move |v: Vec<f64>| set_value.set(v))
>
  <SliderTrack>
    <SliderRange />
  </SliderTrack>
  <SliderThumb />
</Slider>
```

### Range slider (two thumbs)

#### React

```tsx
<Slider.Root defaultValue={[25, 75]} minStepsBetweenThumbs={1}>
  <Slider.Track>
    <Slider.Range />
  </Slider.Track>
  <Slider.Thumb />
  <Slider.Thumb />
</Slider.Root>
```

#### Leptos

```rust
<Slider default_value=vec![25.0, 75.0] min_steps_between_thumbs=1.0>
  <SliderTrack>
    <SliderRange />
  </SliderTrack>
  <SliderThumb />
  <SliderThumb />
</Slider>
```

### Vertical orientation

#### React

```tsx
<Slider.Root defaultValue={[50]} orientation="vertical">
  <Slider.Track>
    <Slider.Range />
  </Slider.Track>
  <Slider.Thumb />
</Slider.Root>
```

#### Leptos

```rust
<Slider default_value=vec![50.0] orientation=Orientation::Vertical>
  <SliderTrack>
    <SliderRange />
  </SliderTrack>
  <SliderThumb />
</Slider>
```

### With form submission

#### React

```tsx
<form onSubmit={handleSubmit}>
  <Slider.Root name="volume" defaultValue={[50]}>
    <Slider.Track>
      <Slider.Range />
    </Slider.Track>
    <Slider.Thumb />
  </Slider.Root>
  <button type="submit">Submit</button>
</form>
```

#### Leptos

```rust
<form on:submit=handle_submit>
  <Slider name="volume" default_value=vec![50.0]>
    <SliderTrack>
      <SliderRange />
    </SliderTrack>
    <SliderThumb />
  </Slider>
  <button type="submit">"Submit"</button>
</form>
```

### Inverted with RTL

#### React

```tsx
<Slider.Root defaultValue={[20]} dir="rtl" inverted>
  <Slider.Track>
    <Slider.Range />
  </Slider.Track>
  <Slider.Thumb />
</Slider.Root>
```

#### Leptos

```rust
<Slider default_value=vec![20.0] dir=Direction::Rtl inverted=true>
  <SliderTrack>
    <SliderRange />
  </SliderTrack>
  <SliderThumb />
</Slider>
```

### Small fractional steps

#### React

```tsx
<Slider.Root value={value} onValueChange={setValue} min={0.1} max={0.2} step={0.003}>
  <Slider.Track>
    <Slider.Range />
  </Slider.Track>
  <Slider.Thumb />
</Slider.Root>
```

#### Leptos

```rust
<Slider
  value=value
  on_value_change=Callback::new(move |v: Vec<f64>| set_value.set(v))
  min=0.1
  max=0.2
  step=0.003
>
  <SliderTrack>
    <SliderRange />
  </SliderTrack>
  <SliderThumb />
</Slider>
```

## Accessibility

Implements the [WAI-ARIA Slider pattern](https://www.w3.org/WAI/ARIA/apd/patterns/slider/).

Each `SliderThumb` receives `role="slider"` with the appropriate ARIA attributes. The root element itself does not carry a specific ARIA role -- it serves as the interaction container.

### Keyboard Interactions

| Key | Description |
|---|---|
| `ArrowRight` | For horizontal LTR sliders: increases the focused thumb's value by one step. For RTL, decreases. |
| `ArrowLeft` | For horizontal LTR sliders: decreases the focused thumb's value by one step. For RTL, increases. |
| `ArrowUp` | For vertical sliders: increases the focused thumb's value by one step. For horizontal sliders, also acts as an increase. |
| `ArrowDown` | For vertical sliders: decreases the focused thumb's value by one step. For horizontal sliders, also acts as a decrease. |
| `PageUp` | Increases the focused thumb's value by 10 steps (step * 10). |
| `PageDown` | Decreases the focused thumb's value by 10 steps (step * 10). |
| `Shift` + Arrow | Same as PageUp/PageDown -- increases or decreases by 10 steps. |
| `Home` | Sets the first thumb to the minimum value. |
| `End` | Sets the last thumb to the maximum value. |

All keyboard interactions prevent default browser scrolling behavior.

### ARIA Attributes

| Element | Attribute | Value | Notes |
|---|---|---|---|
| Root (`<span>`) | `aria-disabled` | `"true"` / absent | Present when the slider is disabled. |
| `SliderThumb` | `role` | `"slider"` | Each thumb is the ARIA slider element. |
| `SliderThumb` | `aria-valuemin` | number | The slider's minimum value (from root `min`). |
| `SliderThumb` | `aria-valuenow` | number | The thumb's current value. |
| `SliderThumb` | `aria-valuemax` | number | The slider's maximum value (from root `max`). |
| `SliderThumb` | `aria-orientation` | `"horizontal" \| "vertical"` | The slider's orientation. |
| `SliderThumb` | `aria-label` | string / absent | Auto-generated for multi-thumb sliders: "Minimum"/"Maximum" for 2 thumbs, "Value N of M" for 3+. Single-thumb sliders have no auto-generated label. |
| `SliderThumb` | `tabindex` | `"0"` / absent | Present when the slider is not disabled. Removed when disabled. |

### Behavioral Notes

- Pointer interactions use pointer capture for smooth drag tracking across the entire viewport.
- Clicking on the track (not on a thumb) snaps the closest thumb to the click position.
- Touch devices: thumbs are focused immediately on pointer down to avoid focus-delay issues during sliding.
- When multiple thumbs exist, the `SliderRange` spans from the minimum to the maximum thumb value.
- Thumbs are repositioned using `position: absolute` with percentage-based offsets and a bounds correction (`getThumbInBoundsOffset`) that keeps the thumb visually within the track at the extremes.
- The `onValueCommit` callback fires only at the end of a drag interaction if the value actually changed, or immediately on keyboard steps with `commit: true`.

## CSS Custom Properties

| Property | Source | Description |
|---|---|---|
| `--slider-thumb-transform` | Set on the root element | `translateX(-50%)` for horizontal, `translateY(50%)` for vertical. Applied to each thumb's wrapper to center it on its position. |
