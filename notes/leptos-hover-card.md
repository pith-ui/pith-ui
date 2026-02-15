---
react_location: "[[reference/react-radix-primitives/packages/react/hover-card/src/hover-card.tsx|hover-card]]"
rust_location: "[[packages/primitives/leptos/hover-card/src/hover_card.rs|hover-card]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/hover-card.stories.tsx|hover-card]]"
rust_story: "[[stories/leptos/src/primitives/hover_card.rs|hover-card]]"
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-dismissable-layer]]"
  - "[[leptos-popper]]"
  - "[[leptos-portal]]"
  - "[[leptos-presence]]"
  - "[[leptos-primitive]]"
  - "[[leptos-use-controllable-state]]"
ported: true
tested: false
tested_story: true
---
## Intent

A non-modal popover that appears on hover/focus. Designed for previews (link previews, user profile cards). Excludes touch interactions — pointer-only.

## React API

```ts
// 5 sub-components:
HoverCard, HoverCardTrigger, HoverCardPortal, HoverCardContent, HoverCardArrow
```

Props: `open`, `defaultOpen`, `onOpenChange`, `openDelay` (default 700ms), `closeDelay` (default 300ms).

## Leptos API

```rust
// HoverCard (root)
pub fn HoverCard(
    open: MaybeProp<bool>,
    default_open: MaybeProp<bool>,
    on_open_change: Option<Callback<bool>>,
    open_delay: MaybeProp<f64>,     // default 700.0
    close_delay: MaybeProp<f64>,    // default 300.0
    children: ChildrenFn,
) -> impl IntoView;

// HoverCardTrigger
pub fn HoverCardTrigger(
    on_pointer_enter: Option<Callback<ev::PointerEvent>>,
    on_pointer_leave: Option<Callback<ev::PointerEvent>>,
    on_focus: Option<Callback<ev::FocusEvent>>,
    on_blur: Option<Callback<ev::FocusEvent>>,
    on_touch_start: Option<Callback<ev::TouchEvent>>,
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    children: Option<ChildrenFn>,
) -> impl IntoView;

// HoverCardPortal
pub fn HoverCardPortal(
    container: MaybeProp<SendWrapper<web_sys::Element>>,
    container_ref: AnyNodeRef,
    force_mount: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView;

// HoverCardContent
pub fn HoverCardContent(
    force_mount: MaybeProp<bool>,
    on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    on_pointer_down_outside: Option<Callback<web_sys::CustomEvent>>,
    on_focus_outside: Option<Callback<web_sys::CustomEvent>>,
    on_interact_outside: Option<Callback<web_sys::CustomEvent>>,
    on_pointer_enter: Option<Callback<ev::PointerEvent>>,
    on_pointer_leave: Option<Callback<ev::PointerEvent>>,
    // All PopperContent props forwarded (side, side_offset, align, etc.)
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    children: Option<ChildrenFn>,
) -> impl IntoView;

// HoverCardArrow
pub fn HoverCardArrow(
    width: MaybeProp<f64>,
    height: MaybeProp<f64>,
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    children: Option<ChildrenFn>,
) -> impl IntoView;
```

## React Implementation Notes

- ~435 lines.
- Touch-exclusive: `excludeTouch()` helper filters out touch pointer events.
- Open/close delays via `window.setTimeout` for hover intent detection.
- Content prevents user-select on body while open to avoid text selection during dismissal.
- `DismissableLayer` for escape key dismissal (no outside-click — hover handles that).
- Tracks whether pointer is down on content (`isPointerDownOnContentRef`) and whether text selection exists (`hasSelectionRef`) to avoid premature close.
- `getTabbableNodes()` TreeWalker for finding focusable elements.
- Uses `Popper` for positioning, `Presence` for animation.

## Leptos Implementation Notes

### Context structure

- `HoverCardContextValue`: Stores `open: Signal<bool>`, `on_open_change: Callback<bool>`, `on_open/on_close/on_dismiss: Callback<()>`, `has_selection_ref: RwSignal<bool>`, `is_pointer_down_on_content_ref: RwSignal<bool>`.
- `HoverCardPortalContextValue`: Stores `force_mount: Signal<bool>`. Content reads this for its own force_mount fallback (same pattern as Dialog).

### Timer management

- Uses `StoredValue<Option<i32>>` for open/close timer handles (setTimeout IDs).
- `set_timeout` helper uses `Closure::once_into_js` to schedule callbacks.
- `clear_timeout` helper reads the stored handle and calls `window.clear_timeout`.
- Cleanup on `on_cleanup` clears both timers.

### Touch exclusion

- React uses an `excludeTouch(handler)` higher-order function. In Leptos, the check is inlined at each event handler call site (`if event.pointer_type() != "touch"`), since Leptos event callbacks work differently from React's synthetic events.

### Text selection protection

- `contain_selection: RwSignal<bool>` tracks whether the user is actively selecting text in content.
- An `Effect` watches `contain_selection` and sets/restores `body.style.userSelect` (plus `-webkit-user-select` for Safari). The original value is stored in `StoredValue<Option<String>>`.
- React uses a module-level `let originalBodyUserSelect: string` variable. In Leptos, `StoredValue` provides equivalent per-instance storage.

### Pointerup document listener

- Uses `StoredValue<Option<SendWrapper<Closure<dyn Fn(web_sys::Event)>>>>` since `Closure` is not `Clone` and cannot be stored in `RwSignal`.
- The listener is added/removed via an `Effect` watching `content_ref.get()`, and cleaned up in `on_cleanup`.
- The listener's inner logic uses `Closure::once_into_js` + `set_timeout` to defer selection check by one frame (matching React's `requestAnimationFrame`-like pattern using `setTimeout(0)`).

### Tabbable node suppression

- `get_tabbable_nodes` uses `TreeWalker` with `NodeFilter` (constant `0x1` for `SHOW_ELEMENT`, since `web_sys::NodeFilter` doesn't expose `SHOW_ELEMENT` as a Rust constant).
- Follows the exact pattern from `focus_scope.rs`: `Closure<dyn Fn(web_sys::Node) -> u32>` for the accept callback.
- An `Effect` sets `tabindex="-1"` on all tabbable descendants whenever content mounts.

### DismissableLayer usage

- `disable_outside_pointer_events=false` — hover card should not block outside interactions.
- `on_focus_outside` always calls `event.prevent_default()` to prevent the hover card from dismissing when focus moves within it (matches React behavior).

### CSS custom property re-namespacing

- `PopperContent` exposes `--radix-popper-*` custom properties. The `HoverCardContentImpl` re-maps them to `--radix-hover-card-*` via inline styles (same approach as React).

### Context re-provision through portals (scope composition substitute)

React uses `createContextScope` with `createPopperScope` to compose isolated context instances per HoverCard. Each HoverCard passes `{...popperScope}` to its Popper children, ensuring nested HoverCards each have independent Popper contexts. Leptos has no equivalent to React's scope composition mechanism.

`HoverCardPortal` explicitly captures both `HoverCardContextValue` and `PopperScope` (via `use_popper_scope()`) before the portal boundary, then re-provides them inside the Portal's children. `PopperScope` is an opaque wrapper around `PopperContextValue` (which is private to the popper module), exposed through `use_popper_scope()` and `provide_popper_scope()` helper functions.

This pattern ensures nested HoverCards work correctly: each level's portal re-provides the correct Popper context for its own content, so inner `PopperContent` components find the inner anchor ref rather than the outer one.

### Omissions

- **`useCallbackRef`**: React wraps some callbacks in `useCallbackRef` for stable references. In Leptos, `Callback` types are inherently stable (they're `Copy` smart pointers), so this wrapper is unnecessary.

### Children optionality

- `HoverCardTrigger` and `HoverCardContent` accept `Option<ChildrenFn>` instead of required `ChildrenFn` to support self-closing tags in stories (e.g., Chromatic's `<HoverCardTrigger />`).

### Stories

Stories match the React reference: Basic, ContainTextSelection, AsyncUpdate, CustomDurations, Controlled, Layerable, Animated, ForcedMount, Nested, NonPortal, WithSlottedTrigger, WithSlottedContent, Chromatic.

### HoverCardPortal: no portal-level Presence

React wraps `Presence` inside `Portal` in `HoverCardPortal`. In Leptos, a portal-level Presence wrapper has no `node_ref` attached to a real DOM element, so it cannot detect exit animations and unmounts immediately — killing children before their animations complete. Following the Dialog pattern, `HoverCardPortal` always renders the Portal unconditionally and lets `HoverCardContent` handle mount/unmount via its own Presence wrapper with a properly composed `node_ref`.
