---
react_location: "[[reference/react-radix-primitives/packages/react/tooltip/src/tooltip.tsx|tooltip]]"
rust_location:
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/tooltip.stories.tsx|tooltip]]"
rust_story: ""
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-dismissable-layer]]"
  - "[[leptos-id]]"
  - "[[leptos-popper]]"
  - "[[leptos-portal]]"
  - "[[leptos-presence]]"
  - "[[leptos-primitive]]"
  - "[[leptos-slot]]"
  - "[[leptos-use-controllable-state]]"
  - "[[leptos-visually-hidden]]"
ported: false
tested: false
---
## Intent

A tooltip popup triggered by hover/focus on a trigger element. Manages delays, pointer-in-transit detection (safe triangle), and provider-level coordination between multiple tooltips.

## React API

```ts
// 6 sub-components:
TooltipProvider, Tooltip, TooltipTrigger, TooltipPortal, TooltipContent, TooltipArrow
```

`TooltipProvider` props: `delayDuration` (default 700ms), `skipDelayDuration` (default 300ms), `disableHoverableContent`.

## React Implementation Notes

- ~780 lines.
- `TooltipProvider` coordinates delay behavior across multiple tooltips — once one opens, subsequent ones skip delay.
- Pointer-in-transit detection: Uses convex hull algorithm to detect if pointer is moving toward content, preventing premature close.
- Custom event `tooltip.open` dispatched on `window` for cross-tooltip coordination.
- `DismissableLayer` for escape key (but not outside-click — hover handles that).
- `VisuallyHidden` used for the `aria-label` approach.
- `data-state`: `delayed-open` | `instant-open` | `closed`.
- Touch support: Shows on long press, dismisses on another tap.
