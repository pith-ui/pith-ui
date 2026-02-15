---
react_location: "[[reference/react-radix-primitives/packages/react/slider/src/slider.tsx|slider]]"
rust_location: "[[packages/primitives/leptos/slider/src/slider.rs|slider]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/slider.stories.tsx|slider]]"
rust_story: "[[stories/leptos/src/primitives/slider.rs|slider]]"
dependencies:
  - "[[core-number]]"
  - "[[core-primitive]]"
  - "[[leptos-collection]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-direction]]"
  - "[[leptos-primitive]]"
  - "[[leptos-use-controllable-state]]"
  - "[[leptos-use-previous]]"
  - "[[leptos-use-size]]"
ported: true
tested: false
tested_story: true
---
## Intent

A range input for selecting numeric values with draggable thumbs. Supports single and multi-value ranges, keyboard control, orientation, form integration, and RTL.

## React API

```ts
// 4 public sub-components:
Slider, SliderTrack, SliderRange, SliderThumb
```

Props: `value`, `defaultValue` (number[]), `onValueChange`, `onValueCommit`, `min`, `max`, `step`, `orientation`, `minStepsBetweenThumbs`, `inverted`, `disabled`, `name`, `form`, `dir`.

## Leptos API

```rust
// 4 public components:
Slider        // Root component (renders horizontal or vertical variant)
SliderTrack   // Track element
SliderRange   // Filled range element
SliderThumb   // Draggable thumb element

// Plus Orientation enum:
Orientation::Horizontal | Orientation::Vertical
```

Props mirror React: `value: Vec<f64>`, `default_value: Vec<f64>`, `on_value_change: Callback<Vec<f64>>`, `on_value_commit: Callback<Vec<f64>>`, `min: f64`, `max: f64`, `step: f64`, `orientation: Orientation`, `min_steps_between_thumbs: f64`, `inverted: bool`, `disabled: bool`, `name: String`, `form: String`, `dir: Direction`.

## React Implementation Notes

- ~811 lines.
- Sophisticated pointer event handling for drag: `pointerdown` starts, `pointermove` updates value, `pointerup` commits.
- Keyboard: Home/End (min/max), PageUp/PageDown (10x step), Arrow keys (1 step).
- Multi-thumb: Values auto-sorted, `minStepsBetweenThumbs` enforced.
- Direction-aware: RTL inverts horizontal slider behavior.
- CSS custom properties for thumb positioning via inline styles.
- Form integration with hidden bubble input per thumb.
- Math utilities: `linearScale`, `getDecimalCount`, `roundValue` for precise stepping.
- `data-orientation`, `data-disabled` attributes on track/range/thumb.
- Internal components: `SliderHorizontal`, `SliderVertical`, `SliderImpl`, `SliderThumbImpl`, `SliderBubbleInput`.

## Leptos Implementation Notes

- **Thumbs set**: React uses `useRef<Set<HTMLElement>>` for tracking thumbs. In Leptos, uses `RwSignal<Vec<SendWrapper<HtmlElement>>>` because `SendWrapper` doesn't implement `Hash`/`Eq` (needed for `HashSet`). Uses `is_same_node` for equality checks when adding/removing.
- **Value index to change**: React uses `useRef<number>`. Leptos uses `RwSignal<usize>` since it's read/written imperatively in callbacks.
- **Orientation context**: Uses `Signal<SliderOrientationContextValue>` (reactive) rather than a simple struct, so it reacts to direction/inversion changes.
- **Collection usage**: Uses `CollectionProvider<ItemData>` with an empty `ItemData` struct and `PhantomData` constant (`ITEM_DATA_PHANTOM`) to work around Leptos view macro limitations with generic turbofish syntax.
- **BubbleInput**: Follows the same pattern as `RadioBubbleInput` in the radio-group package — hidden `<input>` that dispatches native "input" events using `Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value').set` via `js_sys::Reflect`.
- **`update_values` closure**: Stored as `StoredValue<SendWrapper<dyn Fn(f64, usize, bool)>>` to share across pointer event handlers without lifetime issues.
- **`values_before_slide_start`**: Stored as `StoredValue<Vec<f64>>` for snapshotting values at pointer-down.
- **Thumb positioning**: Uses `style=wrapper_style` closure on the `<span>` wrapper (rather than `attr:style`) because Leptos view macro doesn't support `attr:style` on plain HTML elements cleanly.
- **`compose_callbacks`**: Used from `radix-leptos-primitive` for composing user-provided event handlers with internal handlers on `SliderImpl`.
