---
react_location: "[[reference/react-radix-primitives/packages/react/dialog/src/dialog.tsx|dialog]]"
rust_location: "[[packages/primitives/leptos/dialog/src/dialog.rs|dialog]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/dialog.stories.tsx|dialog]]"
rust_story: "[[stories/leptos/src/primitives/dialog.rs|dialog]]"
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-dismissable-layer]]"
  - "[[leptos-focus-guards]]"
  - "[[leptos-focus-scope]]"
  - "[[leptos-id]]"
  - "[[leptos-portal]]"
  - "[[leptos-presence]]"
  - "[[leptos-primitive]]"
  - "[[leptos-slot]]"
  - "[[leptos-use-controllable-state]]"
ported: true
tested: false
tested_story: true
---
## Intent

A fully-featured modal dialog with dual modes (modal and non-modal), focus management, scroll prevention, keyboard handling, and accessibility. Supports animated transitions and flexible portal rendering.

## React API

```ts
// 8 sub-components:
Dialog, DialogTrigger, DialogPortal, DialogOverlay, DialogContent,
DialogTitle, DialogDescription, DialogClose
```

Props: `open`, `defaultOpen`, `onOpenChange`, `modal` (default true).

## Leptos API

```rust
// 8 public components:
Dialog(open, default_open, on_open_change, modal, children)
DialogTrigger(on_click, as_child, node_ref, children)
DialogPortal(container, container_ref, force_mount, children)
DialogOverlay(force_mount, as_child, node_ref, children?)
DialogContent(force_mount, on_open_auto_focus, on_close_auto_focus,
              on_escape_key_down, on_pointer_down_outside,
              on_focus_outside, on_interact_outside, as_child, node_ref, children?)
DialogTitle(as_child, node_ref, children)
DialogDescription(as_child, node_ref, children)
DialogClose(on_click, as_child, node_ref, children)
```

## React Implementation Notes

- ~592 lines.
- Two implementations: `DialogContentModal` and `DialogContentNonModal`.
- Modal mode: Uses `hideOthers()` from `aria-hidden` library, `FocusScope` with trapping, disables outside pointer events, prevents right-click from closing, auto-focuses trigger on close.
- Non-modal mode: No focus trap, tracks outside interactions to decide whether to focus trigger, special Safari pointerdown/focusin edge case handling.
- Uses `DismissableLayer` for escape key and outside-click dismissal.
- `RemoveScroll` from `react-remove-scroll` to prevent body scroll in modal mode.
- Warning system for missing title (accessibility compliance).
- External dependencies: `react-remove-scroll`, `aria-hidden`.

## Leptos Implementation Notes

### Structure
The implementation mirrors the React source with the same component hierarchy: `Dialog` → `DialogTrigger`/`DialogPortal` → `DialogOverlay`/`DialogContent` → `DialogTitle`/`DialogDescription`/`DialogClose`. Internally, `DialogContent` branches to `DialogContentModal` or `DialogContentNonModal`, both wrapping a shared `DialogContentImpl`.

### Callback prop forwarding
`Option<Callback<T>>` cannot be directly passed through Leptos's `#[prop(into, optional)]` macro between component layers. To work around this, a `ContentCallbacks` struct with `StoredValue<Option<Callback<...>>>` fields is used to thread callbacks from `DialogContent` through to the modal/non-modal variants. The inner `DialogContentImpl` takes non-optional `Callback<T>` props with `#[prop(into)]`, and callers always provide concrete callbacks (defaulting to no-ops where needed).

### Omissions

1. **`react-remove-scroll` shard support** — React uses `react-remove-scroll` with `shards` prop pointing to `contentRef`, allowing the dialog content to remain scrollable while the body is locked. The Leptos port uses a simplified `use_body_scroll_lock()` that sets `body.style.overflow = "hidden"` on mount and restores on cleanup. This covers the primary use case (preventing background scroll) but does not support the shard mechanism, pinch-zoom allowance, or iOS scroll bounce prevention.

2. **`aria-hidden` tree walking** — React uses the `aria-hidden` library's `hideOthers()` which walks the full DOM tree to aria-hide all elements except the dialog content and its ancestors. The Leptos port uses a simplified `hide_others()` that only sets `aria-hidden="true"` on body's direct children that don't contain the dialog. This is sufficient for portal-based dialogs (where content is a direct child of body) but less thorough for non-portal scenarios.

3. **`TitleWarning` / `DescriptionWarning`** — Dev-only accessibility warning components that check for the presence of title/description elements. Omitted as they are development-time-only utilities with no runtime behavior.

4. **`WarningProvider`** — Context for configuring warning messages. Omitted along with the warning components.

5. **`createDialogScope`** — React's scoping mechanism for nested contexts. Leptos uses standard `Provider`/`expect_context` which doesn't require explicit scoping.

6. **`Slot` wrapper for `RemoveScroll`** — React creates a `Slot` component for `RemoveScroll` in the overlay. Not needed since we use direct body scroll locking.

### Key decisions

- **`ContentCallbacks` pattern**: Chose to use a `StoredValue`-based struct to pass optional callbacks through component layers, avoiding Leptos's trait bound issues with `Option<Callback<T>>` on `#[prop(into, optional)]` builder methods.
- **`ChildrenFn` for `DialogOverlay` and `DialogContent`**: Used `Option<ChildrenFn>` instead of `Option<TypedChildrenFn<impl IntoView>>` to avoid type inference issues when these components are used without children (e.g., self-closing `<DialogOverlay />`).
- **`js_sys::Reflect` for CustomEvent detail access**: Used `js_sys::Reflect::get` to extract `originalEvent` from `CustomEvent.detail()`, matching the DismissableLayer's event structure.
