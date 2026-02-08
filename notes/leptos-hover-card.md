---
react_location: "[[reference/react-radix-primitives/packages/react/hover-card/src/hover-card.tsx|hover-card]]"
rust_location:
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/hover-card.stories.tsx|hover-card]]"
rust_story: ""
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-dismissable-layer]]"
  - "[[leptos-popper]]"
  - "[[leptos-portal]]"
  - "[[leptos-presence]]"
  - "[[leptos-primitive]]"
  - "[[leptos-use-controllable-state]]"
ported: false
tested: false
---
## Intent

A non-modal popover that appears on hover/focus. Designed for previews (link previews, user profile cards). Excludes touch interactions — pointer-only.

## React API

```ts
// 5 sub-components:
HoverCard, HoverCardTrigger, HoverCardPortal, HoverCardContent, HoverCardArrow
```

Props: `open`, `defaultOpen`, `onOpenChange`, `openDelay` (default 700ms), `closeDelay` (default 300ms).

## React Implementation Notes

- ~435 lines.
- Touch-exclusive: `excludeTouch()` helper filters out touch pointer events.
- Open/close delays via `window.setTimeout` for hover intent detection.
- Content prevents user-select on body while open to avoid text selection during dismissal.
- `DismissableLayer` for escape key dismissal (no outside-click — hover handles that).
- Tracks whether pointer is down on content (`isPointerDownOnContentRef`) and whether text selection exists (`hasSelectionRef`) to avoid premature close.
- `getTabbableNodes()` TreeWalker for finding focusable elements.
- Uses `Popper` for positioning, `Presence` for animation.
