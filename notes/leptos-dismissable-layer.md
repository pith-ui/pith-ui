---
react_location: "[[reference/react-radix-primitives/packages/react/dismissable-layer/src/dismissable-layer.tsx|dismissable-layer]]"
rust_location: "[[packages/primitives/leptos/dismissable-layer/src/dismissable_layer.rs|dismissable_layer]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/dismissable-layer.stories.tsx|dismissable-layer]]"
rust_story: "[[stories/leptos/src/primitives/dismissable_layer.rs|dismissable_layer]]"
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-primitive]]"
  - "[[leptos-use-escape-keydown]]"
ported: true
tested: false
tested_story: true
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
#[component]
pub fn DismissableLayer(
    #[prop(into, optional)] disable_outside_pointer_events: MaybeProp<bool>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<PointerDownOutsideEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<FocusOutsideEvent>>,
    #[prop(into, optional)] on_interact_outside: Option<Callback<CustomEvent>>,
    #[prop(into, optional)] on_dismiss: Option<Callback<()>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView

#[component]
pub fn DismissableLayerBranch(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView

pub type PointerDownOutsideEvent = web_sys::CustomEvent;
pub type FocusOutsideEvent = web_sys::CustomEvent;
```

## React Implementation Notes

- Module-level context holds sets of `layers`, `layersWithOutsidePointerEventsDisabled`, and `branches`.
- Layer stacking: tracks insertion order, only the highest layer responds to Escape.
- `usePointerDownOutside`: listens for `pointerdown` on the document (with a `setTimeout(0)` delay to avoid catching the mount event). Uses a `pointerDownCapture` prop to track if the pointer is inside the React tree. Special handling for touch devices (waits for `click` event).
- `useFocusOutside`: listens for `focusin` on the document, tracks via `focusCapture`/`blurCapture`.
- `disableOutsidePointerEvents`: sets `body.style.pointerEvents = 'none'`, restores on cleanup.
- Uses `dispatchDiscreteCustomEvent` for custom event dispatch.
- Context update broadcasting via `CustomEvent("dismissableLayer.update")` on `document`.

## Leptos Implementation Notes

### Architecture

- **Global layer context**: `Lazy<Mutex<DismissableLayerContextValue>>` tracks layers, layers with outside pointer events disabled, and branches. This must be truly global (not `provide_context`) because portaled layers in different DOM locations must share stacking context.
- **Update broadcasting**: DOM `CustomEvent("dismissableLayer.update")` dispatched on `document`, matching React exactly. Each `DismissableLayer` listens and bumps a local `RwSignal<u64>` to trigger reactivity. Cannot use a global `RwSignal` because `Lazy` statics don't have a reactive owner.
- **Body pointer events**: `ORIGINAL_BODY_POINTER_EVENTS: Lazy<Mutex<String>>` stores the original `pointer-events` value. First layer with `disable_outside_pointer_events=true` saves and sets `"none"`; last such layer unmounting restores the original value.

### Omissions

1. **`dispatchDiscreteCustomEvent`**: Omitted because it wraps React's `ReactDOM.flushSync`, which ensures synchronous re-renders. In Leptos (WASM), signal updates are already synchronous. All custom events are dispatched directly via `target.dispatchEvent()`.

2. **`useCallbackRef`**: Omitted because Leptos `Callback` already has stable identity. React's `useCallbackRef` prevents stale closure captures; Leptos doesn't have this problem since callbacks are passed as `Callback` values that don't change identity.

3. **`InteractOutsideEvent` enum**: The existing type stubs had an `InteractOutsideEvent` enum. This was removed in favor of `Option<Callback<CustomEvent>>` for `on_interact_outside`, since both `PointerDownOutsideEvent` and `FocusOutsideEvent` are `CustomEvent` aliases. The consumer just needs `prevent_default()`.

4. **`ownerDocument` parameter**: React derives `ownerDocument` from the node (for cross-document/popup scenarios). The Leptos port uses `document()` directly, since WASM apps run in a single document context.

5. **`RemoveScroll` in stories**: The React stories' `DummyDialog` and `DummyPopover` wrap content in `RemoveScroll` (from `react-remove-scroll`) to lock body scrolling. No Leptos equivalent exists, so scroll locking is omitted from the story helpers. This does not affect the `DismissableLayer` component itself.

### Key Decisions

- **`Callback<web_sys::CustomEvent>` for outside events**: Both pointer-down-outside and focus-outside events are `CustomEvent` with the original event in `detail`. The user accesses the original event via `event.detail().dyn_into::<web_sys::Event>()`.
- **Manual `addEventListener` with `capture: true`**: Leptos `on:` event bindings don't support capture phase. Capture-phase handlers for `pointerdown`, `focusin`, and `blur` are attached manually via `addEventListener` with `AddEventListenerOptions` and cleaned up on unmount.
- **`StoredValue<SendWrapper<Box<dyn Fn()>>>` for capture callbacks**: The `Box<dyn Fn()>` closures from `use_pointer_down_outside`/`use_focus_outside` are not `Send + Sync`, so they're wrapped in `SendWrapper` for storage in `StoredValue`.
- **Two separate effects for layer registration and cleanup**: Matches React's deliberate separation. Effect 1 handles `disableOutsidePointerEvents` changes. Effect 2 is cleanup-only on unmount. This prevents re-ordering layers when `disableOutsidePointerEvents` toggles.
- **`AttributeInterceptor` + `Primitive` view pattern**: Follows the pattern established by `focus-scope` for forwarding attributes to the underlying `Primitive` component.
- **Layer containment check for portaled nested layers**: React's synthetic event system propagates events through the React component tree, not just the DOM tree. This means a parent DismissableLayer's `onFocusCapture`/`onPointerDownCapture` handlers fire for events inside portaled child layers (since they're React tree descendants). In Leptos/DOM, capture handlers only fire on DOM ancestors, and portaled layers are DOM siblings (both children of `<body>`). To compensate, the `pointer_down_outside` and `focus_outside` callbacks additionally check whether the event target is inside any registered layer (`layers_contain`), preventing false "outside" detections when focus or pointer events occur in portaled child layers.

### Deferred Stories

- `InPopupWindow`: Not meaningful for WASM single-document context.

### Implemented Stories

- `Basic`: Toggle layer with checkboxes for dismiss-on-escape, dismiss-on-pointer-down-outside, dismiss-on-focus-outside, disable-outside-pointer-events.
- `Nested`: Truly recursive `DismissableBox` testing infinite layer nesting and selective dismissal. Uses a plain function returning `AnyView` to break Rust's recursive opaque type limitation.
- `WithFocusScope`: `DismissableLayer` wrapping `FocusScope` with `trapped=true`. Adds `on_escape_key_down` handler that blurs focused inputs on first Escape (preventing dismissal) and dismisses on second Escape.
- `DialogExample`: `DummyDialog` helper composing `FocusGuards` + `Portal` + `DismissableLayer` + `FocusScope` for a fully modal dialog. `RemoveScroll` omitted (no Leptos equivalent); scroll locking is not applied.
- `PopoverFullyModal`: `DummyPopover` with `disable_outside_pointer_events=true`, focus trapping, and `Popper` positioning.
- `PopoverSemiModal`: `DummyPopover` with reactive color prop and a "Change color" button excluded from outside-click dismissal.
- `PopoverNonModal`: `DummyPopover` with `trapped=false`, allowing focus to leave the popover naturally.
- `PopoverInDialog`: `DummyPopover` nested inside `DummyDialog`, testing that Escape dismisses only the innermost layer.
- `PopoverNested`: Three nested `DummyPopover` instances (black → red → blue) testing layered pointer-events and stacking dismissal.

### `leptos-use-callback-ref` dependency removed

The `leptos-use-callback-ref` dependency listed in the original research note has been removed. This package doesn't exist in the workspace (it was planned but never needed, since Leptos `Callback` already provides stable identity).
