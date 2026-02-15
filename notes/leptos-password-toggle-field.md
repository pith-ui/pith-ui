---
react_location: "[[reference/react-radix-primitives/packages/react/password-toggle-field/src/password-toggle-field.tsx|password-toggle-field]]"
rust_location: "[[packages/primitives/leptos/password-toggle-field/src/password_toggle_field.rs|password-toggle-field]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/password-toggle-field.stories.tsx|password-toggle-field]]"
rust_story: "[[stories/leptos/src/primitives/password_toggle_field.rs|password-toggle-field]]"
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-id]]"
  - "[[leptos-primitive]]"
  - "[[leptos-use-controllable-state]]"
  - "[[leptos-use-is-hydrated]]"
ported: true
tested: false
tested_story: true
---
## Intent

A password visibility toggle component that switches a password input between hidden and visible states while preserving cursor/selection position. Marked as `unstable` in React.

## React API

```ts
// 5 sub-components:
PasswordToggleField, PasswordToggleFieldInput, PasswordToggleFieldToggle,
PasswordToggleFieldSlot, PasswordToggleFieldIcon
```

Props: `visible`, `defaultVisible` (default false), `onVisibilityChange`, `id`.

## Leptos API

```rust
// PasswordToggleField (Root)
pub fn PasswordToggleField(
    id: MaybeProp<String>,
    visible: MaybeProp<bool>,
    default_visible: MaybeProp<bool>,
    on_visibility_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView

// PasswordToggleFieldInput
pub fn PasswordToggleFieldInput(
    auto_complete: MaybeProp<AutoComplete>,  // CurrentPassword | NewPassword
    id: MaybeProp<String>,
    on_blur: Option<Callback<ev::FocusEvent>>,
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    children: Option<ChildrenFn>,
) -> impl IntoView

// PasswordToggleFieldToggle
pub fn PasswordToggleFieldToggle(
    on_click: Option<Callback<ev::MouseEvent>>,
    on_pointer_down: Option<Callback<ev::PointerEvent>>,
    on_pointer_cancel: Option<Callback<ev::PointerEvent>>,
    on_pointer_up: Option<Callback<ev::PointerEvent>>,
    aria_label: MaybeProp<String>,
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView

// PasswordToggleFieldSlot
pub fn PasswordToggleFieldSlot(
    render: Option<Callback<bool, AnyView>>,
    visible_content: Option<ChildrenFn>,
    hidden_content: Option<ChildrenFn>,
) -> impl IntoView

// PasswordToggleFieldIcon
pub fn PasswordToggleFieldIcon(
    visible_icon: ViewFn,
    hidden_icon: ViewFn,
    node_ref: AnyNodeRef,
) -> impl IntoView
```

## React Implementation Notes

- ~480 lines.
- Focus state tracking: Preserves `selectionStart`/`selectionEnd` across visibility toggle.
- Toggle button: Dynamic `aria-label` based on inner text content via `MutationObserver`.
- Hydration-aware: `aria-hidden`/`tabIndex=-1` before hydration, fully interactive after.
- Selection position restoration after type switch using `requestAnimationFrame`.
- Form event handling: Resets visibility to hidden on form submit/reset.
- Pointer event tracking for click detection vs keyboard activation.
- `requestIdleCallback` polyfill for cleanup scheduling.
- `flushSync` for immediate state updates on toggle.
- `PasswordToggleFieldSlot` supports both declarative (`visible`/`hidden` props) and render prop patterns.
- `PasswordToggleFieldIcon` wraps SVG with `aria-hidden` and `asChild`.

## Leptos Implementation Notes

### Omissions

1. **`flushSync`** — Omitted. Leptos CSR signals are synchronous; state updates in event handlers apply immediately without needing React's `flushSync`.

2. **`useEffectEvent`** — Omitted. Leptos `Callback` is already stable (`Copy + 'static`), so the `_setVisible` wrapper from the React source is unnecessary.

3. **`useIsHydrated`** — Omitted. The Leptos port targets CSR-only (`wasm32-unknown-unknown`), so the component is always "hydrated". The pre-hydration `aria-hidden`/`tabIndex=-1` logic and the hydration gate for `aria-controls` are not needed. Per `notes/leptos-use-is-hydrated.md`, the hook always returns `true` in CSR.

4. **`createContextScope` / `ScopedProps`** — Omitted. Leptos uses `provide_context`/`expect_context` directly; the Radix scoping pattern (for composition of multiple instances) is not needed.

5. **`composeEventHandlers` for `onClick`, `onPointerCancel`, `onPointerUp`** — The React source intentionally does NOT use `composeEventHandlers` for these three handlers on the Toggle button (the comments explain why: the reset logic must always run regardless of `preventDefault`). The Leptos port follows the same pattern: user callbacks are called first, then internal logic always executes.

### Key Decisions

1. **`InternalFocusState` storage** — Uses `StoredValue<InternalFocusState>` for non-reactive mutable state, equivalent to React's `useRef`. This avoids triggering reactive updates for internal bookkeeping (click tracking, selection position).

2. **Form event listener cleanup** — Uses the `SendWrapper<Closure>` + `Owner::on_cleanup` pattern established in `one_time_password_field.rs:576-598`. Reset listener checks `event.default_prevented()` before hiding; submit listener always hides.

3. **`requestIdleCallback` polyfill** — Uses `setTimeout(..., 1)` as a fallback, matching React's polyfill. `requestIdleCallback` is not available in `web-sys`.

4. **MutationObserver for aria-label** — Uses `web_sys::MutationObserver` with `SendWrapper` for the callback closure. Observer watches `characterData` + `subtree` mutations on the toggle button element. Cleanup disconnects the observer and drops the closure via `Owner::on_cleanup`.

5. **Global pointerup listener** — Registered via `Effect` with `Owner::on_cleanup` for removal. Uses `setTimeout(reset, 1)` to defer the `click_triggered` reset (matching React's `requestIdleCallback` usage), with cleanup tracking of the timeout handle.

6. **Context type** — `input_id` uses `Signal<String>` (not `ReadSignal<String>`) since it's derived from `Signal::derive`.

7. **PasswordToggleFieldIcon** — Always renders with `as_child=true` on the SVG `Primitive`, matching the React `asChild` pattern. `ViewFn` props are stored via `StoredValue` since `ViewFn` is not `Copy`.

8. **PasswordToggleFieldSlot** — Adapted from React's union type pattern (`render` prop vs `visible`/`hidden` children) to Leptos's optional prop pattern. Uses `Option<Callback<bool, AnyView>>` for render prop and `Option<ChildrenFn>` for declarative content.

9. **React prop name typo** — The React source uses `onVisiblityChange` (missing 'i' in visibility). The Leptos port corrects this to `on_visibility_change`.
