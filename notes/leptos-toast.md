---
react_location: "[[reference/react-radix-primitives/packages/react/toast/src/toast.tsx|toast]]"
rust_location:
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/toast.stories.tsx|toast]]"
rust_story: ""
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-collection]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-dismissable-layer]]"
  - "[[leptos-portal]]"
  - "[[leptos-presence]]"
  - "[[leptos-primitive]]"
  - "[[leptos-use-callback-ref]]"
  - "[[leptos-use-controllable-state]]"
  - "[[leptos-visually-hidden]]"
ported: false
tested: false
tested_story: false
---
## Intent

A notification toast system with auto-dismiss timers, swipe-to-dismiss gestures, viewport-based focus management, and accessible screen reader announcements.

## React API

```ts
// 7 sub-components:
ToastProvider, ToastViewport, Toast, ToastTitle,
ToastDescription, ToastAction, ToastClose
```

Provider props: `label`, `duration` (default 5000ms), `swipeDirection`, `swipeThreshold` (default 50px).

## React Implementation Notes

- ~982 lines. One of the most complex primitives.
- `ToastProvider` manages toast count, viewport reference, and shared configuration.
- `ToastViewport` is the container — uses Collection to track toasts, handles focus management with E/F1 hotkey, pause-on-hover/focus.
- Auto-dismiss timer with pause/resume: Pauses when viewport hovered/focused, resumes with remaining time.
- Swipe gesture system: Tracks pointer start, calculates delta, dispatches custom swipe events (`toast.swipeStart`, `toast.swipeMove`, `toast.swipeEnd`, `toast.swipeCancel`), uses CSS variables for position.
- `ToastAnnounce` — renders content in `VisuallyHidden` + `Portal` for screen reader announcement with 1-frame delay for NVDA compatibility.
- `ToastAction` requires `altText` prop for screen reader users.
- `ToastAnnounceExclude` suppresses interactive elements from announcements.
- Uses `DismissableLayer` for escape key.
- Portals each toast into the viewport element via `ReactDOM.createPortal`.
