---
react_location: "[[reference/react-radix-primitives/packages/react/avatar/src/avatar.tsx|avatar]]"
rust_location: "[[packages/primitives/leptos/avatar/src/avatar.rs|avatar]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/avatar.stories.tsx|avatar]]"
rust_story: "[[stories/leptos/src/primitives/avatar.rs|avatar]]"
dependencies:
  - "[[leptos-primitive]]"
  - "[[leptos-use-callback-ref]]"
  - "[[leptos-use-is-hydrated]]"
ported: true
tested: false
---
## Intent

An image element with a fallback for representing the user. Three sub-components: `Avatar` (wrapper), `AvatarImage` (loads and shows the image), `AvatarFallback` (shows when image is unavailable, with optional delay).

## React API

```ts
const Avatar: React.ForwardRefExoticComponent<AvatarProps>       // span wrapper
const AvatarImage: React.ForwardRefExoticComponent<AvatarImageProps>  // img, renders only when loaded
const AvatarFallback: React.ForwardRefExoticComponent<AvatarFallbackProps>  // span, shows when not loaded
```

`AvatarImage` tracks loading status (`idle` → `loading` → `loaded`/`error`) via a hidden `Image()` element. `AvatarFallback` accepts `delayMs` to defer rendering.

## Leptos API

```rust
#[component] fn Avatar(...) -> impl IntoView
#[component] fn AvatarImage(...) -> impl IntoView
#[component] fn AvatarFallback(delay_ms: MaybeProp<i32>, ...) -> impl IntoView
```

**Note:** Uses old Leptos API (`NodeRef<AnyElement>`, `#[prop(attrs)]`, `create_signal`, `Callback::call`). Needs migration to Leptos 0.7+.

## React Implementation Notes

- `useImageLoadingStatus` creates a hidden `Image()` to preload `src`, fires `load`/`error` events.
- Handles `referrerPolicy` and `crossOrigin` on the hidden image.
- `AvatarFallback` uses `setTimeout` with cleanup for delayed rendering.
- Hydration-aware: uses `useIsHydrated` to avoid creating `Image()` during SSR.

## Leptos Implementation Notes

- `use_image_loading_status` creates a hidden `<img>` via `document().create_element("img")` with `web_sys` closures for load/error.
- Uses `is_mounted` `StoredValue` to guard against state updates after cleanup.
- `AvatarFallback` uses `window().set_timeout_with_callback_and_timeout_and_arguments_0` for delay — cleanup clears the timer.
- No hydration/SSR handling.
- Uses old Leptos API — needs migration.
- Dependencies: `leptos`, `web-sys`.
