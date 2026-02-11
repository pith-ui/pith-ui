---
react_location: "[[reference/react-radix-primitives/packages/react/focus-scope/src/focus-scope.tsx|focus-scope]]"
rust_location: "[[packages/primitives/leptos/focus-scope/src/focus_scope.rs|focus_scope]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/focus-scope.stories.tsx|focus-scope]]"
rust_story: "[[stories/leptos/src/primitives/focus_scope.rs|focus_scope]]"
dependencies:
  - "[[leptos-compose-refs]]"
  - "[[leptos-primitive]]"
  - "[[leptos-use-callback-ref]]"
ported: true
tested: true
tested_story: false
---
## Intent

Traps focus within a DOM subtree and optionally loops tab navigation. Used by dialogs, popovers, and other modal components to prevent focus from escaping.

## React API

```ts
interface FocusScopeProps extends PrimitiveDivProps {
  loop?: boolean;       // wrap tab at edges
  trapped?: boolean;    // prevent focus escape
  onMountAutoFocus?: (event: Event) => void;
  onUnmountAutoFocus?: (event: Event) => void;
}
```

## Leptos API

```rust
#[component]
fn FocusScope(
    r#loop: MaybeProp<bool>,
    trapped: MaybeProp<bool>,
    on_mount_auto_focus: Option<Callback<Event>>,
    on_unmount_auto_focus: Option<Option<Callback<Event>>>,  // double Option
    ...
) -> impl IntoView
```

**Note:** Uses old Leptos API. Needs migration.

## React Implementation Notes

- Focus trapping: listens for `focusin`/`focusout` on the document. If focus moves outside, restores to last focused element inside.
- `MutationObserver` watches for removed DOM nodes — if the focused element is removed, refocuses the container.
- Auto-focus on mount: dispatches a custom `focusScope.autoFocusOnMount` event, focuses first tabbable element if not prevented.
- Auto-focus on unmount: dispatches `focusScope.autoFocusOnUnmount`, restores focus to previously focused element.
- Tab looping: intercepts `keydown` for Tab/Shift+Tab at tabbable edges.
- `focusScopesStack`: module-level stack that pauses/resumes scopes when nested (newest scope is active).
- `getTabbableCandidates`: uses `TreeWalker` to find elements with `tabIndex >= 0`.

## Leptos Implementation Notes

- Faithfully ports the React logic with raw `web_sys` APIs.
- `FocusScopeAPI` uses `AtomicBool` for `paused` state (shared via `Arc`), `AtomicU64` for unique IDs.
- `FOCUS_SCOPE_STACK` is a `Lazy<Mutex<FocusScopeStack>>` global.
- `get_tabbable_candidates` uses `web_sys::TreeWalker` with a `Closure` filter — mirrors the React implementation.
- Has unit tests for `FocusScopeAPI` and `FocusScopeStack`.
- `on_unmount_auto_focus` has a double `Option` (`Option<Option<Callback<Event>>>`) — likely a workaround.
- `focus()` does not support `preventScroll` option (noted as TODO — `web_sys` limitation).
- Uses old Leptos API — needs migration.
- Dependencies: `leptos`, `once_cell`, `radix-leptos-compose-refs`, `web-sys` (with MutationObserver, NodeFilter features).
