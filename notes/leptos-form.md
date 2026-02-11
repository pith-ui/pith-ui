---
react_location: "[[reference/react-radix-primitives/packages/react/form/src/form.tsx|form]]"
rust_location: "[[packages/primitives/leptos/form/src/form.rs|form]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/form.stories.tsx|form]]"
rust_story: "[[stories/leptos/src/primitives/form.rs|form]]"
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-id]]"
  - "[[leptos-label]]"
  - "[[leptos-primitive]]"
ported: true
tested: false
tested_story: false
---
## Intent

A form validation component with support for both native HTML validation and custom validation matchers (sync and async). Provides per-field validity state tracking and accessible error messaging.

## React API

```ts
// 7 sub-components:
Form, FormField, FormLabel, FormControl, FormMessage, FormValidityState, FormSubmit
```

Props: `onClearServerErrors`. FormMessage: `match` (native ValidityState key or custom function), `forceMatch`, `name`.

## Leptos API

```rust
// 7 public components + re-exports:
Form           // Root <form> with validation + aria-description contexts
FormField      // Field wrapper <div> with name, server_invalid, data-valid/data-invalid
FormLabel      // Wraps radix_leptos_label::Label, auto-links via field id
FormControl    // <input> with validation, aria-describedby, change/invalid handlers
FormMessage    // Error message <span>, supports built-in, custom sync, custom async matchers
FormValidityState  // Render callback exposing Option<Validity> for custom UI
FormSubmit     // <button type="submit"> wrapper

// Re-exports:
Root = Form, Field = FormField, Label = FormLabel, Control = FormControl,
Message = FormMessage, ValidityState = FormValidityState, Submit = FormSubmit

// Key types:
Validity           // Clone/Debug struct mirroring web_sys::ValidityState fields
ValidityMatcher    // Enum: BadInput, PatternMismatch, RangeOverflow, etc.
Match              // Enum: BuiltIn(ValidityMatcher), Custom(SyncFn), CustomAsync(AsyncFn)
CustomMatcher      // Enum: Sync(SyncCustomMatcherFn), Async(AsyncCustomMatcherFn)
SyncCustomMatcherFn   // Rc<dyn Fn(String, FormData) -> bool>
AsyncCustomMatcherFn  // Rc<dyn Fn(String, FormData) -> Pin<Box<dyn Future<Output = bool>>>>
```

## React Implementation Notes

- ~730 lines.
- Native HTML form validation integration.
- Custom matcher system: Sync and async validation functions.
- Per-field validity state tracking via context.
- Custom error messages with generated IDs for `aria-describedby`.
- First invalid control auto-focused on form submission.
- Server-side error tracking with `data-invalid`/`data-valid` attributes.
- `FormValidityState` render prop exposes `ValidityState` for custom UI.
- Message ID tracking for `aria-describedby` generation on `FormControl`.
- Cross-field validation possible via `FormData` in async matchers.

## Leptos Implementation Notes

### Context Architecture

Three contexts matching React, using Leptos `provide_context`/`expect_context`:

- **`ValidationContextValue`**: Stores per-field validity (`RwSignal<HashMap>`), custom matcher entries (`StoredValue<HashMap>` — non-reactive because entries contain `Rc<dyn Fn>` closures which are not `Send + Sync`), and custom errors (`RwSignal<HashMap>`).
- **`AriaDescriptionContextValue`**: Tracks message IDs per field for `aria-describedby` (`RwSignal<HashMap<String, HashSet<String>>>`).
- **`FormFieldContextValue`**: Per-field identity: generated `id`, `name`, `server_invalid: Signal<bool>`.

### Key Decisions

- **`StoredValue` for custom_matcher_entries_map**: Custom matcher entries contain `Rc<dyn Fn>` closures which are not `Send + Sync`. `StoredValue` doesn't require `Send + Sync`. This map doesn't need reactivity — it's only read during validation event handlers.
- **`RwSignal` for validity_map, custom_errors_map, message_ids_map**: These drive reactive rendering (data-valid/data-invalid attributes, message visibility, aria-describedby).
- **`SendWrapper` around `CustomMatcherEntry`**: Required because entries stored in `StoredValue` may be accessed across cleanup boundaries.
- **`VoidPrimitive` for FormControl**: `html::input` is a void element and doesn't implement `ElementWithChildren`, so `VoidPrimitive` is used instead of `Primitive`.
- **`use_composed_refs`**: FormControl composes the user-provided `node_ref` with an internal `AnyNodeRef` for DOM access in Effects.
- **Native event listeners everywhere**: All non-trivial event handlers (`invalid`, `change`, `reset`, `input`) use `web_sys::EventTarget::add_event_listener_with_callback` rather than Leptos `on:` directives. This is necessary because: (1) the `invalid` event does not bubble, so `on:invalid` on a `<form>` won't catch child input events — we use capture-phase (`addEventListener(..., true)`) on the form instead; (2) `on:` directives on component wrappers like `VoidPrimitive` may not reliably forward to the underlying DOM element through `TypedFallbackShow` layers. Native listeners via `addEventListener` in Effects are the proven pattern. Closures are wrapped in `SendWrapper` for cleanup.
- **`FormControl.children` is `Option<ChildrenFn>`**: Made optional since `<input>` is a void element and children are only relevant in `as_child` mode.
- **`Match` enum**: React auto-detects sync vs async matchers at runtime. Rust uses explicit enum variants (`BuiltIn`, `Custom`, `CustomAsync`) for type safety.
- **`FormValidityState` children**: Uses `Callback<Option<Validity>, AnyView>` instead of React's render prop pattern.
- **Async validation**: Uses `leptos::task::spawn_local` for async custom matchers.

### Omissions

- **`createFormScope`**: React's `createContextScope` pattern is not needed in Leptos. Components use standard `provide_context`/`expect_context`. Nested forms use Leptos's natural context scoping (inner `provide_context` shadows outer).
- **`validityStateToObject` utility**: Replaced by `impl From<web_sys::ValidityState> for Validity` which serves the same purpose (converting live DOM ValidityState to a clonable struct).
- **`DEFAULT_BUILT_IN_MESSAGES` map**: Replaced by `ValidityMatcher::default_message()` method, which is more idiomatic Rust.

### Notes

- doesn't rely on label like react impl, but maybe should
- uses `DEFAULT_INVALID_MESSAGE` for `ValidityMatcher::Valid`, should be a valid message