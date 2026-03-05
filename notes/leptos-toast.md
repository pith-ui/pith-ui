---
react_location: "[[reference/react-radix-primitives/packages/react/toast/src/toast.tsx|toast]]"
rust_location: "[[packages/primitives/leptos/src/toast/mod.rs|toast]]"
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
  - "[[leptos-use-controllable-state]]"
  - "[[leptos-visually-hidden]]"
ported: true
tested: true
tested_story: true
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

## Leptos Implementation Notes

- Merged React's `Toast` and `ToastImpl` into a single `Toast` component. In React, the separation exists for ref-forwarding; in Leptos this is unnecessary since `AnyNodeRef` handles ref composition directly.
- `ToastAnnounce` (screen reader announcement) is not yet rendered. The announce text is computed (`get_announce_text_content`) but not displayed in a `VisuallyHidden`+`Portal` region. This omission affects screen reader announcements but not interactive behavior.
- Removed `leptos-use-callback-ref` from dependencies — not needed since Leptos callbacks are already stable references.
- Timer management uses `StoredValue<Option<i32>>` + `set_timeout`/`clear_timeout` helpers (same pattern as hover-card).
- Custom DOM events (`toast.viewportPause`/`toast.viewportResume`) use `web_sys::CustomEvent` dispatched on the viewport element.
- Swipe gesture detection uses `pointer_start_ref` and `swipe_delta_ref` as `StoredValue<Option<(f64, f64)>>`.
- Collection API uses `PhantomData<()>` constant (`ITEM_DATA_PHANTOM`) for generic type parameters in view macros.
- `ToastViewport` wraps content in `DismissableLayerBranch` (with `role="region"`) containing a `Primitive.ol`. User-facing attrs propagate to the `<ol>` via `node_ref`; the branch div receives internal attrs (`role`, `aria-label`, `tabindex`).
- `ToastViewport.children` is `Option<ChildrenFn>` since toasts are portaled into the viewport — the viewport itself typically has no direct children.
