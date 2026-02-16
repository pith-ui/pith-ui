---
react_location: "[[reference/react-radix-primitives/packages/react/popover/src/popover.tsx|popover]]"
rust_location: "[[packages/primitives/leptos/popover/src/popover.rs|popover]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/popover.stories.tsx|popover]]"
rust_story: "[[stories/leptos/src/primitives/popover.rs|popover]]"
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-dismissable-layer]]"
  - "[[leptos-focus-guards]]"
  - "[[leptos-focus-scope]]"
  - "[[leptos-id]]"
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

A popover that displays rich content anchored to a trigger element. Supports focus trapping, outside-click dismissal, scroll prevention, and portal rendering. Similar architecture to Dialog but positioned relative to a trigger.

## React API

```ts
// 7 sub-components:
Popover, PopoverAnchor, PopoverTrigger, PopoverPortal,
PopoverContent, PopoverClose, PopoverArrow

// Popover (Root)
interface PopoverProps {
  open?: boolean;
  defaultOpen?: boolean;
  onOpenChange?: (open: boolean) => void;
  modal?: boolean; // default false
}

// PopoverAnchor — wraps PopperPrimitive.Anchor
// PopoverTrigger — button that toggles, wraps in PopperAnchor if no custom anchor
// PopoverPortal — container, forceMount, renders Presence + Portal
// PopoverContent — all PopperContent props + DismissableLayer props + FocusScope props
// PopoverClose — button that closes
// PopoverArrow — wraps PopperPrimitive.Arrow
```

## Leptos API

```rust
// 7 public components (matching React):
Popover, PopoverAnchor, PopoverTrigger, PopoverPortal,
PopoverContent, PopoverClose, PopoverArrow

// Popover (Root)
pub fn Popover(
    open: MaybeProp<bool>,          // controlled
    default_open: MaybeProp<bool>,  // uncontrolled
    on_open_change: Option<Callback<bool>>,
    modal: MaybeProp<bool>,         // default false
    children: ChildrenFn,
)

// PopoverContent forwards all PopperContent positioning props:
// side, side_offset, align, align_offset, arrow_padding,
// avoid_collisions, collision_boundary, collision_padding,
// sticky, hide_when_detached, update_position_strategy
```

## React Implementation Notes

- ~531 lines.
- Uses `Popper` primitive for positioning content relative to trigger/anchor.
- `DismissableLayer` for escape key and outside-click dismissal.
- `FocusScope` for focus trapping (only when `modal` is true).
- `useFocusGuards` prevents focus from escaping.
- `Presence` for animation-friendly mounting.
- `hideOthers()` from `aria-hidden` when modal.
- `RemoveScroll` prevents body scroll when modal.
- Custom anchor support: Can use `PopoverAnchor` instead of trigger for positioning.
- External dependencies: `react-remove-scroll`, `aria-hidden`.
- `PopoverContentModal` vs `PopoverContentNonModal` split with different focus/dismiss behavior.
- CSS custom properties re-namespaced from `--radix-popper-*` to `--radix-popover-*`.
- `createSlot('PopoverContent.RemoveScroll')` used for RemoveScroll composition in modal mode.

## Leptos Implementation Notes

### Structure
Follows the same component hierarchy as React: Root wraps Popper + context provider, Trigger conditionally wraps in PopperAnchor, Portal captures and re-provides contexts across mount boundary, Content delegates to Modal/NonModal variants, both rendered through a shared ContentImpl.

### Omissions

1. **`createSlot` / `RemoveScroll` composition** — React wraps modal content in `<RemoveScroll as={Slot} allowPinchZoom>`. Leptos uses a simplified `use_body_scroll_lock()` that sets `overflow: hidden` on body. Does not support scroll sharding (allowing scroll on specific elements) or pinch-zoom preservation. Same approach as `DialogContentModal`.

2. **`aria-hidden` library** — React uses the `aria-hidden` npm package (`hideOthers`) which walks the full tree. Leptos uses a simplified implementation that only hides body's direct children (same as Dialog). Sufficient for portalled content.

3. **`createContextScope` / `__scopePopover`** — React uses scoped contexts for component isolation. Leptos uses standard `Provider`/`expect_context` with explicit capture/re-provide across portal boundaries (same pattern as HoverCard/Dialog).

4. **`leptos-slot` dependency removed** — The React source uses `createSlot` for the RemoveScroll wrapper. Since we use the simplified `use_body_scroll_lock()` approach, the slot dependency is not needed.

### Key Decisions

1. **`PopoverTrigger` conditional anchor wrapping** — React renders the trigger once and conditionally wraps it in `PopperAnchor`. Leptos cannot share `impl IntoView` between branches, so we extract a `PopoverTriggerInner` component and render it in both branches of a `Show`. The composed click callback is created once and shared.

2. **`has_custom_anchor` reactivity** — Uses `RwSignal<bool>` set on mount of `PopoverAnchor` and cleared on cleanup, matching React's `useState` + `useEffect` pattern. The `Show` in `PopoverTrigger` reacts to changes.

3. **Portal context re-provision** — Captures `PopoverContextValue` and `PopperScope` before the Portal boundary and re-provides them inside, following the same pattern established in `HoverCardPortal`.

4. **CSS custom properties** — Applied via `Effect` + `setProperty()` on the content element rather than `attr:style`, to avoid conflicts with caller-provided style attributes. Re-maps popper variables to popover namespace.

5. **`PopoverContextValue` is `Copy`** — All fields (`AnyNodeRef`, `ReadSignal`, `Signal`, `Callback`, `RwSignal`) are `Copy` in Leptos, allowing efficient context sharing without cloning issues in closures.

### Stories
10 stories matching the React reference: Styled, Boundary, Modality, Controlled, Animated, ForcedMount, Nested, CustomAnchor, WithSlottedTrigger, Chromatic.
