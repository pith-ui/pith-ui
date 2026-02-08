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
pub fn Presence(
    #[prop(into)] present: Signal<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView
```

## React Implementation Notes

- State machine with three states: `mounted`, `unmountSuspended`, `unmounted`.
- Events: `MOUNT`, `UNMOUNT`, `ANIMATION_OUT`, `ANIMATION_END`.
- When `present` goes false: checks if `animation-name` changed from computed styles. If animating, enters `unmountSuspended`; otherwise unmounts immediately.
- Listens for `animationstart`, `animationcancel`, `animationend` on the node.
- After `animationend`, sets `animationFillMode: 'forwards'` temporarily to prevent flash (React 18 concurrency workaround).
- `useStateMachine`: simple reducer-based state machine.

## Leptos Implementation Notes

- **Migrated to Leptos 0.8** from 0.7. Key changes:
  - `create_signal` → `signal`, `MaybeSignal` → `Signal`, `ChildrenFn` → `TypedChildrenFn`
  - `NodeRef<AnyElement>` → `AnyNodeRef` from `leptos-node-ref`
  - `on_cleanup` → `Owner::on_cleanup`
  - `window()` → `web_sys::window().unwrap()`
- State machine ported as `use_state_machine` with `HashMap<S, HashMap<E, S>>`, generic with `Send + Sync` bounds on state/event types.
- Same three states (`Mounted`, `UnmountSuspended`, `Unmounted`) and four events (`Mount`, `Unmount`, `AnimationOut`, `AnimationEnd`).
- **Removed `map_children`**: The Leptos 0.7 version recursively walked the `View` enum tree to attach a `NodeRef` to children. This doesn't work in Leptos 0.8 where the view representation changed. Instead, `Presence` now accepts a `node_ref: AnyNodeRef` prop. Callers must apply this ref to their child element and compose it with any other refs using `use_composed_refs`.
- Animation event listeners (`animationstart`, `animationcancel`, `animationend`) use `web_sys::Closure` wrapped in `SendWrapper` and stored in `StoredValue` for `Send + Sync` compliance required by `Owner::on_cleanup`.
- Computed styles stored as `RwSignal<Option<SendWrapper<web_sys::CssStyleDeclaration>>>`.
- **Omitted**: `animationFillMode: 'forwards'` flash fix (React 18 concurrency workaround, not applicable to Leptos).
- **Omitted**: Render prop pattern (`children` as function receiving `{ present }`). Leptos callers can read the `present` signal directly.
- **Omitted**: `useCallbackRef` — not needed in Leptos.
- Dependencies: `leptos`, `leptos-node-ref`, `send_wrapper`, `web-sys` (with `CssStyleDeclaration`).
