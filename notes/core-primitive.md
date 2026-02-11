---
react_location: "[[reference/react-radix-primitives/packages/core/primitive/src/primitive.tsx|primitive]]"
rust_location: "[[packages/primitives/core/primitive/src/primitive.rs|primitive]]"
react_story: ""
rust_story: ""
dependencies: []
ported: true
tested: false
tested_story: false
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
pub fn can_use_dom() -> bool
pub fn compose_event_handlers<E: Clone + Into<Event>>(
    original_event_handler: Option<fn(E)>,
    our_event_handler: Option<fn(E)>,
    check_for_default_prevented: Option<bool>,
) -> impl Fn(E)
pub fn get_owner_window(element: Option<&Node>) -> Result<Window, &'static str>
pub fn get_owner_document(element: Option<&Node>) -> Result<Document, &'static str>
pub fn get_active_element(node: Option<&Node>, active_descendant: bool) -> Option<HtmlElement>
pub fn is_frame(element: &Element) -> bool
```

## React Implementation Notes

- `canUseDOM` guards against SSR by checking `window`, `window.document`, and `createElement`.
- `composeEventHandlers` uses optional chaining to safely call either handler. Checks `event.defaultPrevented` between the two calls.
- `getOwnerDocument`/`getOwnerWindow` fall back to global `document`/`window` when the element has no `ownerDocument`.
- `getActiveElement` recurses into iframes and optionally follows `aria-activedescendant`. Adapted from Ariakit.

## Rust Implementation Notes

**`can_use_dom`** is a function rather than a constant because `web_sys::window()` is a runtime call.

**`compose_event_handlers`** requires `E: Clone + Into<Event>` — it clones the event to call the original handler, then checks `default_prevented()` via conversion to `web_sys::Event`.

**`get_owner_window` / `get_owner_document`** return `Result<_, &'static str>` instead of panicking (idiomatic Rust). React throws an `Error`; Rust callers can propagate with `?` or `.ok()`.

**`get_active_element`** takes `active_descendant: bool` (not `Option<bool>`) since React defaults it to `false` — a plain `bool` is simpler and equivalent. Returns `Option<HtmlElement>` rather than `HTMLElement | null`. When `dyn_into::<HtmlElement>()` fails (e.g., active element is an SVG element), `None` is returned. This is slightly safer than React's unchecked `as HTMLElement | null` cast.

**`is_frame`** compares `element.tag_name()` against `"IFRAME"`. `tag_name()` returns uppercase for HTML elements, matching the React behavior.

### Omissions

- **`Timeout`, `Interval`, `Immediate` type aliases** — These are `ReturnType<typeof setTimeout>` etc. in TypeScript, which map to `i32` in WASM. They provide no value as Rust type aliases.
