---
react_location: "[[reference/react-radix-primitives/packages/react/popper/src/popper.tsx|popper]]"
rust_location: "[[packages/primitives/leptos/popper/src/popper.rs|popper]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/popper.stories.tsx|popper]]"
rust_story: "[[stories/leptos/src/primitives/popper.rs|popper]]"
dependencies:
  - "[[core-rect]]"
  - "[[leptos-arrow]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-primitive]]"
  - "[[leptos-use-callback-ref]]"
  - "[[leptos-use-size]]"
ported: true
tested: false
tested_story: false
---
## Intent

Positions floating content relative to an anchor element using Floating UI. The core positioning engine for tooltips, popovers, dropdowns, and similar overlay patterns. Provides four components: `Popper` (context), `PopperAnchor` (reference element), `PopperContent` (floating element), and `PopperArrow` (visual arrow).

## React API

```ts
const Popper: React.FC<PopperProps>
const PopperAnchor: React.ForwardRefExoticComponent<PopperAnchorProps>
const PopperContent: React.ForwardRefExoticComponent<PopperContentProps>
const PopperArrow: React.ForwardRefExoticComponent<PopperArrowProps>
```

`PopperContent` props: `side`, `sideOffset`, `align`, `alignOffset`, `arrowPadding`, `avoidCollisions`, `collisionBoundary`, `collisionPadding`, `sticky`, `hideWhenDetached`, `updatePositionStrategy`, `onPlaced`.

`PopperAnchor` supports a `virtualRef` for non-DOM anchors (e.g., pointer position).

## Leptos API

```rust
#[component] fn Popper(children: ChildrenFn) -> impl IntoView
#[component] fn PopperAnchor(...) -> impl IntoView
#[component] fn PopperContent(...) -> impl IntoView
#[component] fn PopperArrow(...) -> impl IntoView
```

Also defines `Align`, `Sticky`, `UpdatePositionStrategy` enums and re-exports `Padding`, `Side` from `floating-ui-leptos`.

## React Implementation Notes

- Uses `@floating-ui/react-dom` with `useFloating` hook.
- Middleware stack: `offset` → `shift` (with optional `limitShift`) → `flip` → `size` → `arrow` → custom `transformOrigin` → `hide`.
- `size` middleware sets CSS custom properties: `--radix-popper-available-width/height`, `--radix-popper-anchor-width/height`.
- Custom `transformOrigin` middleware computes CSS transform-origin based on placement and arrow position.
- `PopperAnchor` supports `virtualRef` for non-element anchors (e.g., pointer position).
- Uses `createContextScope` for scoped context.
- z-index read from computed style and applied to wrapper.

## Leptos Implementation Notes

- Uses `floating-ui-leptos` (Rust port of Floating UI) with the same middleware pattern.
- Custom `TransformOrigin` middleware implements `Middleware<web_sys::Element, web_sys::Window>` trait with `serde` for data serialization.
- Middleware data (arrow position, transform origin, hide) read via `middleware_data.get().get_as::<T>(NAME)`.
- `PopperAnchor` does not support `virtualRef` — only DOM-based anchors.
- Uses Leptos `Provider`/`expect_context` instead of React's scoped context.
- `PopperContent` uses `AttributeInterceptor` to forward consumer attributes to the inner `Primitive`.
- z-index is read in an effect and stored in a signal, same as React.
- All positioning props are reactive signals.
- Dependencies: `floating-ui-leptos`, `leptos`, `leptos-maybe-callback`, `leptos-node-ref`, `radix-leptos-arrow`, `radix-leptos-compose-refs`, `radix-leptos-primitive`, `radix-leptos-use-size`, `send_wrapper`, `serde`, `serde_json`, `web-sys`.
