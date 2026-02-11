---
react_location: "[[reference/react-radix-primitives/packages/react/use-rect/src/use-rect.tsx|use-rect]]"
rust_location:
react_story: ""
rust_story: ""
dependencies:
  - "[[core-rect]]"
ported: false
tested: false
tested_story: false
---
## Intent

Hook to get an element's bounding rectangle and observe changes over time. Wraps `observeElementRect` from `@radix-ui/rect`.

## React API

```ts
function useRect(measurable: Measurable | null): DOMRect | undefined
```

## React Implementation Notes

- ~25 lines. Thin hook wrapper.
- Uses `React.useState` to store `DOMRect`.
- Sets up observation via `observeElementRect()` from `@radix-ui/rect` (the core rect package).
- Cleans up observer and clears rect on unmount.
- Returns `undefined` while element is null/unmounted.
- Depends on `core-rect` being ported for the underlying observation logic.
