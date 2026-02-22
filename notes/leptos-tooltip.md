---
react_location: "[[reference/react-radix-primitives/packages/react/tooltip/src/tooltip.tsx|tooltip]]"
rust_location: "[[packages/primitives/leptos/tooltip/src/tooltip.rs|tooltip]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/tooltip.stories.tsx|tooltip]]"
rust_story: "[[stories/leptos/src/primitives/tooltip.rs|tooltip]]"
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
ported: true
tested: true
tested_story: true
---
## Intent

A tooltip popup triggered by hover/focus on a trigger element. Manages delays, pointer-in-transit detection (safe triangle), and provider-level coordination between multiple tooltips.

## React API

```ts
// 6 sub-components:
TooltipProvider, Tooltip, TooltipTrigger, TooltipPortal, TooltipContent, TooltipArrow
```

`TooltipProvider` props: `delayDuration` (default 700ms), `skipDelayDuration` (default 300ms), `disableHoverableContent`.

`Tooltip` props: `open`, `defaultOpen`, `onOpenChange`, `delayDuration`, `disableHoverableContent`.

`TooltipTrigger` props: `asChild`.

`TooltipPortal` props: `forceMount`, `container`.

`TooltipContent` props: `forceMount`, `side` (default `top`), `sideOffset`, `align`, `alignOffset`, `avoidCollisions`, `collisionBoundary`, `collisionPadding`, `sticky`, `hideWhenDetached`, `aria-label`, `onEscapeKeyDown`, `onPointerDownOutside`.

`TooltipArrow` props: `width`, `height`.

## Leptos API

```rust
// TooltipProvider
pub fn TooltipProvider(
    delay_duration: MaybeProp<f64>,       // default 700.0
    skip_delay_duration: MaybeProp<f64>,  // default 300.0
    disable_hoverable_content: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView

// Tooltip
pub fn Tooltip(
    open: MaybeProp<bool>,
    default_open: MaybeProp<bool>,
    on_open_change: Option<Callback<bool>>,
    delay_duration: MaybeProp<f64>,
    disable_hoverable_content: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView

// TooltipTrigger
pub fn TooltipTrigger(
    on_pointer_move: Option<Callback<ev::PointerEvent>>,
    on_pointer_leave: Option<Callback<ev::PointerEvent>>,
    on_pointer_down: Option<Callback<ev::PointerEvent>>,
    on_focus: Option<Callback<ev::FocusEvent>>,
    on_blur: Option<Callback<ev::FocusEvent>>,
    on_click: Option<Callback<ev::MouseEvent>>,
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    children: Option<ChildrenFn>,
) -> impl IntoView

// TooltipPortal
pub fn TooltipPortal(
    container: MaybeProp<SendWrapper<web_sys::Element>>,
    container_ref: AnyNodeRef,
    force_mount: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView

// TooltipContent
pub fn TooltipContent(
    force_mount: MaybeProp<bool>,
    aria_label: MaybeProp<String>,
    on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    on_pointer_down_outside: Option<Callback<web_sys::CustomEvent>>,
    side: Signal<Side>,           // default Side::Top
    side_offset: Signal<f64>,
    align: Signal<Align>,
    align_offset: Signal<f64>,
    arrow_padding: Signal<f64>,
    avoid_collisions: Signal<bool>,
    collision_boundary: Signal<SendWrapper<Vec<web_sys::Element>>>,
    collision_padding: Signal<Padding>,
    sticky: Signal<Sticky>,
    hide_when_detached: Signal<bool>,
    update_position_strategy: Signal<UpdatePositionStrategy>,
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    children: Option<ChildrenFn>,
) -> impl IntoView

// TooltipArrow
pub fn TooltipArrow(
    width: MaybeProp<f64>,
    height: MaybeProp<f64>,
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    children: Option<ChildrenFn>,
) -> impl IntoView
```

## React Implementation Notes

- ~780 lines.
- `TooltipProvider` coordinates delay behavior across multiple tooltips — once one opens, subsequent ones skip delay.
- Pointer-in-transit detection: Uses convex hull algorithm to detect if pointer is moving toward content, preventing premature close.
- Custom event `tooltip.open` dispatched on `document` for cross-tooltip coordination.
- `DismissableLayer` for escape key (but not outside-click — hover handles that).
- `VisuallyHidden` used for the `aria-label` approach.
- `data-state`: `delayed-open` | `instant-open` | `closed`.
- Touch support: Shows on long press, dismisses on another tap.
- Trigger uses `onPointerMove` (not `onPointerEnter`) so it doesn't re-trigger when moving within the element while pointer is in transit.
- `TooltipArrow` checks if it's inside the `VisuallyHidden` context and renders nothing if so, preventing positioning issues from the duplicate arrow in the accessibility copy.

## Leptos Implementation Notes

### Structure

Follows the React source structure closely with these components:
- `TooltipProvider` — provider context with `is_open_delayed`, `skip_delay_timer`, `is_pointer_in_transit` signals
- `Tooltip` — individual tooltip state, wraps children in `Popper`
- `TooltipTrigger` — renders `Primitive` element (button) inside `PopperAnchor`
- `TooltipPortal` — `Portal` wrapper that re-provides `TooltipContextValue`, `TooltipProviderContextValue`, and `PopperScope`
- `TooltipContent` — branches between `TooltipContentHoverable` (with grace area) and `TooltipContentImpl` (without) based on `disable_hoverable_content`
- `TooltipContentHoverable` — manages pointer-in-transit grace area polygon using convex hull algorithm
- `TooltipContentImpl` — wraps `DismissableLayer` + `PopperContent`, handles `tooltip.open` event coordination, scroll-to-dismiss, `VisuallyHidden` for accessibility
- `TooltipArrow` — delegates to `PopperArrow`, skips render inside `VisuallyHiddenContentContextValue`

### Key Decisions

1. **Timer management**: Uses `StoredValue<Option<i32>>` for timer IDs with `set_timeout`/`clear_timeout` helpers, same pattern as HoverCard.

2. **Trigger ref sync**: The trigger element is synced to context via an `Effect` watching the `AnyNodeRef`. React uses `onTriggerChange` callback on `useComposedRefs`; Leptos uses a reactive Effect instead.

3. **Custom DOM events**: `tooltip.open` dispatched on `document` (matching React) via `CustomEvent::new_with_event_init_dict`. Listened for via `add_event_listener_with_callback` on `document` in `TooltipContentImpl`.

4. **Grace area event listeners**: `pointerleave` listeners on trigger and content elements are managed manually via `Closure` + `StoredValue`, with cleanup in both `Effect` re-runs and `on_cleanup`. The `pointermove` listener on `document` tracks whether the pointer is within the convex hull grace polygon.

5. **CSS custom properties**: Applied via `Effect` using `style.set_property()` (not `attr:style`) to avoid conflicts with caller-supplied `attr:style`, same pattern as HoverCard.

6. **Portal context re-provision**: `TooltipPortal` re-provides `TooltipContextValue`, `TooltipProviderContextValue`, and `PopperScope` inside the portal boundary.

### Omissions

1. **Slottable (createSlottable)**: React wraps children in a `Slottable` for `asChild` pass-through in `TooltipContentImpl`. Leptos's `Primitive` component handles child forwarding differently, so `Slottable` is not needed.

2. **Scope composition (createContextScope/usePopperScope)**: React uses `createContextScope` for isolated context per component instance. Leptos uses standard `Provider`/`expect_context` with explicit re-provision across portal boundaries.

3. **Touch-specific behavior**: React has touch-specific tooltip behavior (long press to show, tap elsewhere to dismiss). The current Leptos port handles touch events through the standard pointer event path. The `when_mouse` filter (`pointer_type == "touch"` check) is implemented on `TooltipTrigger`'s `pointermove` event, matching the React behavior for mouse-only hover triggering.

### Stories

16 stories implemented: Styled, Controlled, CustomDurations, Positions, CustomContent, AriaLabel, WithText, WithExternalRef, Unmount, Animated, SlottableContent, WithinDialog, KeepOpenOnActivation, WithinScrollable, DisableHoverableContent, Chromatic. Chromatic includes: Uncontrolled, Controlled, Positioning (side/align grid, sideOffset positive/negative, alignOffset positive/negative), Collisions, Relative parent (non-portalled), Slotted trigger, Slotted content.
