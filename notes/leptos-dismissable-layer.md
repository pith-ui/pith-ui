---
react_location: "[[reference/react-radix-primitives/packages/react/dismissable-layer/src/dismissable-layer.tsx|dismissable-layer]]"
rust_location: "[[packages/primitives/leptos/dismissable-layer/src/dismissable_layer.rs|dismissable_layer]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/dismissable-layer.stories.tsx|dismissable-layer]]"
rust_story: ""
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-primitive]]"
  - "[[leptos-use-callback-ref]]"
  - "[[leptos-use-escape-keydown]]"
ported: false
tested: false
tested_story: false
---
## Intent

A layer that can be dismissed by clicking outside, focusing outside, or pressing Escape. Manages a stack of layers so only the topmost one is dismissable. Used by dialogs, popovers, dropdowns. Supports disabling outside pointer events and stacking multiple layers.

## React API

```ts
const DismissableLayer: React.ForwardRefExoticComponent<DismissableLayerProps>
const DismissableLayerBranch: React.ForwardRefExoticComponent<DismissableLayerBranchProps>
```

Props: `disableOutsidePointerEvents`, `onEscapeKeyDown`, `onPointerDownOutside`, `onFocusOutside`, `onInteractOutside`, `onDismiss`.

## Leptos API

```rust
// Only type stubs exist:
pub type PointerDownOutsideEvent = CustomEvent;
pub type FocusOutsideEvent = CustomEvent;
pub enum InteractOutsideEvent { ... }
```

**Not ported.** Only event type definitions exist.

## React Implementation Notes

- Module-level context holds sets of `layers`, `layersWithOutsidePointerEventsDisabled`, and `branches`.
- Layer stacking: tracks insertion order, only the highest layer responds to Escape.
- `usePointerDownOutside`: listens for `pointerdown` on the document (with a `setTimeout(0)` delay to avoid catching the mount event). Uses a `pointerDownCapture` prop to track if the pointer is inside the React tree. Special handling for touch devices (waits for `click` event).
- `useFocusOutside`: listens for `focusin` on the document, tracks via `focusCapture`/`blurCapture`.
- `disableOutsidePointerEvents`: sets `body.style.pointerEvents = 'none'`, restores on cleanup.
- Uses `dispatchDiscreteCustomEvent` for custom event dispatch.

## Leptos Implementation Notes

Only type stubs exist — the actual component, layer stack, outside-click detection, focus-outside detection, and Escape handling are all unimplemented. This is a critical missing primitive that blocks Dialog, Popover, and other overlay components.
