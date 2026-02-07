---
react_location: "[[reference/react-radix-primitives/packages/core/primitive/src/primitive.tsx|primitive]]"
rust_location: "[[packages/primitives/core/primitive/src/primitive.rs|primitive]]"
dependencies: []
ported: false
tested: false
---
## Intent

Internal DOM utilities shared across all primitives. The React package provides event handler composition, DOM environment detection, and element/window/document access helpers.

## React API

```ts
const canUseDOM: boolean
function composeEventHandlers<E>(original?, ours?, opts?): (event: E) => void
function getOwnerWindow(element: Node | null | undefined): Window
function getOwnerDocument(element: Node | null | undefined): Document
function getActiveElement(node: Node | null | undefined, activeDescendant?: boolean): HTMLElement | null
function isFrame(element: Element): element is HTMLIFrameElement
```

Also exports timer type aliases: `Timeout`, `Interval`, `Immediate` (from `types.ts`).

`composeEventHandlers` chains two event handlers, with an option (`checkForDefaultPrevented`, default `true`) to skip the second handler if `event.defaultPrevented` is true.

## Rust API

```rust
pub fn compose_event_handlers<E: Clone + Into<Event>>(
    original_event_handler: Option<fn(E)>,
    our_event_handler: Option<fn(E)>,
    check_for_default_prevented: Option<bool>,
) -> impl Fn(E)
```

Only `compose_event_handlers` is ported. Uses `web_sys::Event` for `default_prevented()` check.

## React Implementation Notes

- `canUseDOM` guards against SSR by checking `window`, `window.document`, and `createElement`.
- `composeEventHandlers` uses optional chaining to safely call either handler. Checks `event.defaultPrevented` between the two calls.
- `getOwnerDocument`/`getOwnerWindow` fall back to global `document`/`window` when the element has no `ownerDocument`.
- `getActiveElement` recurses into iframes and optionally follows `aria-activedescendant`. Adapted from Ariakit.

## Rust Implementation Notes

Only `compose_event_handlers` is implemented. The event parameter requires `Clone + Into<Event>` — it clones the event to call the original handler, then checks `default_prevented()` via conversion to `web_sys::Event`.

**Not yet ported:**
- `canUseDOM` — less relevant in WASM (always in a browser), but may be needed for SSR scenarios
- `getOwnerWindow`, `getOwnerDocument`, `getActiveElement`, `isFrame` — DOM traversal helpers
- `Timeout`, `Interval`, `Immediate` type aliases — not directly applicable in Rust

Dependency: `web-sys` with `Event` feature.
