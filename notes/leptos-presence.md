---
react_location: "[[reference/react-radix-primitives/packages/react/presence/src/presence.tsx|presence]]"
rust_location: "[[packages/primitives/leptos/presence/src/presence.rs|presence]]"
dependencies:
  - "[[leptos-compose-refs]]"
ported: true
tested: false
---
## Intent

Manages mount/unmount with CSS animation support. Keeps an element in the DOM during exit animations before unmounting. Used by `CheckboxIndicator`, `Dialog`, and any component that needs animated enter/exit transitions.

## React API

```ts
interface PresenceProps {
  present: boolean;
  children: ReactElement | ((props: { present: boolean }) => ReactElement);
}
```

Supports render prop pattern for fine-grained control over presence state during animations.

## Leptos API

```rust
#[component]
fn Presence(
    present: MaybeSignal<bool>,
    node_ref: NodeRef<AnyElement>,
    children: ChildrenFn,
) -> impl IntoView
```

**Note:** Uses old Leptos API. Needs migration.

## React Implementation Notes

- State machine with three states: `mounted`, `unmountSuspended`, `unmounted`.
- Events: `MOUNT`, `UNMOUNT`, `ANIMATION_OUT`, `ANIMATION_END`.
- When `present` goes false: checks if `animation-name` changed from computed styles. If animating, enters `unmountSuspended`; otherwise unmounts immediately.
- Listens for `animationstart`, `animationcancel`, `animationend` on the node.
- After `animationend`, sets `animationFillMode: 'forwards'` temporarily to prevent flash (React 18 concurrency workaround).
- `useStateMachine`: simple reducer-based state machine.

## Leptos Implementation Notes

- State machine ported as `use_state_machine` with `HashMap<State, HashMap<Event, State>>`.
- Same three states and four events.
- Animation event listeners added via `web_sys` closures in effects, cleaned up via `on_cleanup`.
- `map_children` helper recursively traverses the view tree to attach `node_ref` — a workaround for Leptos's view model differences from React's `cloneElement`.
- Does not implement the `animationFillMode: 'forwards'` flash fix.
- Does not support the render prop pattern (React allows `children` as a function).
- Asserts exactly one child (`children().as_children().len() == 1`).
- Uses old Leptos API — needs migration.
- Dependencies: `leptos`, `radix-leptos-compose-refs`, `web-sys` (with `CssStyleDeclaration`).
