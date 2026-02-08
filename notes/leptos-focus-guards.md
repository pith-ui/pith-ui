---
react_location: "[[reference/react-radix-primitives/packages/react/focus-guards/src/focus-guards.tsx|focus-guards]]"
rust_location: "[[packages/primitives/leptos/focus-guards/src/focus_guards.rs|focus_guards]]"
react_story: ""
rust_story: ""
dependencies: []
ported: true
tested: false
---
## Intent

Injects a pair of invisible, focusable `<span>` elements at the start and end of `<body>` to ensure `focusin`/`focusout` events can be caught consistently. Used by focus-trapping primitives (e.g., `FocusScope`, `Dialog`). Reference-counted so multiple consumers share the same guards.

## React API

```ts
function FocusGuards(props: { children?: ReactNode }): ReactNode
function useFocusGuards(): void
```

`FocusGuards` is a component wrapper; `useFocusGuards` is the hook that does the work.

## Leptos API

```rust
#[component]
fn FocusGuards(children: ChildrenFn) -> impl IntoView

pub fn use_focus_guards()
```

## React Implementation Notes

- Module-level `count` tracks how many consumers are active.
- On mount: reuses existing `[data-radix-focus-guard]` elements or creates new ones, inserts at `afterbegin`/`beforeend` of `body`.
- On unmount: if last consumer (`count === 1`), removes all guard elements.
- Guard elements: `<span>` with `tabIndex=0`, `outline: none`, `opacity: 0`, `position: fixed`, `pointer-events: none`.

## Leptos Implementation Notes

- Uses `AtomicU64` for the global count (thread-safe, though WASM is single-threaded).
- `Effect::new` handles mount logic; `on_cleanup` handles unmount.
- Direct DOM manipulation via `web_sys`: `query_selector_all`, `insert_adjacent_element`, `create_element`.
- Guard element styles applied via `set_css_text` (single string) rather than individual properties.
- Dependencies: `leptos`, `web-sys` (with `Document`, `NodeList` features).
