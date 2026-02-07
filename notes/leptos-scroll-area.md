---
react_location: "[[reference/react-radix-primitives/packages/react/scroll-area/src/scroll-area.tsx|scroll-area]]"
rust_location:
dependencies:
  - "[[core-number]]"
  - "[[core-primitive]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-direction]]"
  - "[[leptos-presence]]"
  - "[[leptos-primitive]]"
  - "[[leptos-use-callback-ref]]"
ported: false
tested: false
---
## Intent

A custom scrollbar component that hides native scrollbars while providing styled scroll thumb and track elements. Supports multiple visibility modes.

## React API

```ts
// 5 sub-components:
ScrollArea, ScrollAreaViewport, ScrollAreaScrollbar, ScrollAreaThumb, ScrollAreaCorner
```

Props: `type` (`'hover'` | `'scroll'` | `'auto'` | `'always'`), `dir`, `scrollHideDelay` (default 600ms).

## React Implementation Notes

- ~1040 lines.
- Multiple scrollbar visibility modes: `hover` (show on hover), `scroll` (show while scrolling), `auto` (native-like), `always` (always visible).
- State machine for scroll state: hidden → idle → scrolling → interacting.
- CSS to hide native scrollbars while keeping scroll functionality.
- `ResizeObserver` for dynamic viewport size changes.
- Pointer capture for drag interactions on thumb.
- Wheel event handling with `passive: false`.
- Direction-aware positioning for RTL.
- Debounced resize handling.
- `ScrollAreaCorner` fills the gap between horizontal and vertical scrollbars.
- Scroll button management with auto-scroll intervals.
