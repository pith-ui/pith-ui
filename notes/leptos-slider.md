---
react_location: "[[reference/react-radix-primitives/packages/react/slider/src/slider.tsx|slider]]"
rust_location:
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
ported: false
tested: false
---
## Intent

A range input for selecting numeric values with draggable thumbs. Supports single and multi-value ranges, keyboard control, orientation, form integration, and RTL.

## React API

```ts
// 4 sub-components:
Slider, SliderTrack, SliderRange, SliderThumb
```

Props: `value`, `defaultValue` (number[]), `onValueChange`, `onValueCommit`, `min`, `max`, `step`, `orientation`, `minStepsBetweenThumbs`, `inverted`, `disabled`, `name`, `form`, `dir`.

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
