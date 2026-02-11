---
react_location: "[[reference/react-radix-primitives/packages/react/use-escape-keydown/src/use-escape-keydown.tsx|use-escape-keydown]]"
rust_location: "[[packages/primitives/leptos/use-escape-keydown/src/use_escape_keydown.rs|use_escape_keydown]]"
react_story: ""
rust_story: ""
dependencies:
  - "[[leptos-use-callback-ref]]"
ported: true
tested: false
tested_story: false
---
## Intent

Listens for Escape key presses on a document and fires a callback. Used by dismissable layers (dialogs, popovers) to close on Escape. Listens in capture phase to intercept before other handlers.

## React API

```ts
function useEscapeKeydown(
    onEscapeKeyDown?: (event: KeyboardEvent) => void,
    ownerDocument?: Document,
): void
```

## Leptos API

```rust
pub fn use_escape_keydown(
    on_escape_key_down: Option<Callback<KeyboardEvent>>,
    owner_document: Option<Document>,
)
```

## React Implementation Notes

- Adds a `keydown` listener with `{ capture: true }` on `ownerDocument` (defaults to `globalThis.document`).
- Uses `useCallbackRef` to keep the callback reference stable without re-subscribing.
- Cleanup removes the listener on unmount.

## Leptos Implementation Notes

- Creates a `Closure<dyn Fn(KeyboardEvent)>` wrapped in `Arc<SendWrapper<...>>` for the event handler.
- `Effect::new` adds the listener; `on_cleanup` removes it. Both use `capture: true`.
- `owner_document` stored in `StoredValue<SendWrapper<Document>>` for `Send` compatibility.
- Uses raw `web_sys` listener APIs (`add_event_listener_with_callback_and_add_event_listener_options`) rather than Leptos event directives, since this is a document-level listener.
- Dependencies: `leptos`, `send_wrapper`, `web-sys` (with `EventListenerOptions` feature).
