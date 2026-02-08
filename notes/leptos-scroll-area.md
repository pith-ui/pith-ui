---
react_location: "[[reference/react-radix-primitives/packages/react/scroll-area/src/scroll-area.tsx|scroll-area]]"
rust_location: "[[packages/primitives/leptos/scroll-area/src/scroll_area.rs|scroll-area]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/scroll-area.stories.tsx|scroll-area]]"
rust_story: "[[stories/leptos/src/primitives/scroll_area.rs|scroll_area]]"
dependencies:
  - "[[core-number]]"
  - "[[core-primitive]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-direction]]"
  - "[[leptos-presence]]"
  - "[[leptos-primitive]]"
  - "[[leptos-use-callback-ref]]"
ported: true
tested: false
---
## Intent

A custom scrollbar component that hides native scrollbars while providing styled scroll thumb and track elements. Supports multiple visibility modes (hover, scroll, auto, always), RTL direction, and animated show/hide of scrollbars via CSS animations.

## React API

```ts
// 5 public sub-components:
ScrollArea       // Root container, provides context, CSS vars for corner size
ScrollAreaViewport  // Scrollable area, hides native scrollbars via injected <style>
ScrollAreaScrollbar // Scrollbar track; dispatches to mode-specific impl
ScrollAreaThumb     // Draggable thumb with pointer capture
ScrollAreaCorner    // Corner fill when both scrollbars visible

// Re-exports:
Root, Viewport, Scrollbar, Thumb, Corner

// Props:
ScrollArea: type ('hover'|'scroll'|'auto'|'always'), dir, scrollHideDelay (default 600ms)
ScrollAreaViewport: nonce
ScrollAreaScrollbar: orientation ('vertical'|'horizontal'), forceMount
ScrollAreaThumb: forceMount
ScrollAreaCorner: (no special props)
```

## Leptos API

```rust
pub enum ScrollAreaType { Auto, Always, Scroll, Hover } // default: Hover
pub enum Orientation { Horizontal, Vertical }            // default: Vertical

#[component]
pub fn ScrollArea(
    #[prop(optional)] r#type: ScrollAreaType,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(optional)] scroll_hide_delay: u32,  // default: 600
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView

#[component]
pub fn ScrollAreaViewport(
    #[prop(into, optional)] nonce: Option<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView

#[component]
pub fn ScrollAreaScrollbar(
    #[prop(optional)] orientation: Orientation,
    #[prop(into, optional)] force_mount: Option<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView

#[component]
pub fn ScrollAreaThumb(
    #[prop(into, optional)] force_mount: Option<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView

#[component]
pub fn ScrollAreaCorner(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView

// Re-exports:
pub use ScrollArea as Root;
pub use ScrollAreaViewport as Viewport;
pub use ScrollAreaScrollbar as Scrollbar;
pub use ScrollAreaThumb as Thumb;
pub use ScrollAreaCorner as Corner;
```

## React Implementation Notes

- ~1040 lines.
- Multiple scrollbar visibility modes: `hover` (show on hover), `scroll` (show while scrolling), `auto` (native-like), `always` (always visible).
- State machine for scroll state: hidden → idle → scrolling → interacting.
- CSS to hide native scrollbars while keeping scroll functionality.
- `ResizeObserver` for dynamic viewport size changes.
- Pointer capture for drag interactions on thumb.
- Wheel event handling with `passive: false`.
- Direction-aware positioning for RTL.
- Debounced resize handling.
- `ScrollAreaCorner` fills the gap between horizontal and vertical scrollbars.

## Leptos Implementation Notes

- ~1690 lines of Rust, ported from ~1040 lines of React.
- **Internal component hierarchy**: `ScrollAreaScrollbar` dispatches to mode-specific wrappers (`ScrollAreaScrollbarHover`, `ScrollAreaScrollbarScroll`, `ScrollAreaScrollbarAuto`, `ScrollAreaScrollbarVisible`) which eventually render `ScrollAreaScrollbarImpl`. Similarly, `ScrollAreaThumb` wraps `ScrollAreaThumbImpl`, and `ScrollAreaCorner` wraps `ScrollAreaCornerImpl`.
- **Context architecture**: Two context types — `ScrollAreaContextValue` (provided by `ScrollArea`, contains type, direction, delay, element refs, enabled flags, corner dimensions) and `ScrollbarContextValue` (provided by `ScrollAreaScrollbarImpl`, contains thumb state and callbacks).
- **DOM element refs**: All `web_sys::HtmlElement` refs stored as `RwSignal<Option<SendWrapper<web_sys::HtmlElement>>>` because `RwSignal` requires `Send + Sync`.
- **State machine**: `use_scrollbar_state_machine()` with states `Hidden`/`Scrolling`/`Interacting`/`Idle` and events `Scroll`/`ScrollEnd`/`PointerEnter`/`PointerLeave`/`Hide`. Reuses the pattern from `presence/src/use_state_machine.rs`.
- **ResizeObserver**: Helper `use_resize_observer` using `web_sys::ResizeObserver` with closure wrapped in `SendWrapper` and stored in `StoredValue` for cleanup.
- **Debouncing**: Helper `use_debounce_callback` using `window.set_timeout` / `clear_timeout` with `StoredValue<i32>` for the timeout ID.
- **Thumb positioning**: Pure functions `get_thumb_ratio`, `get_thumb_size` (min 18px), `get_scroll_position_from_pointer`, `get_thumb_offset_from_scroll`, `linear_scale`. Uses `radix_number::clamp`.
- **Pointer capture**: `set_pointer_capture` / `release_pointer_capture` on scrollbar for drag-to-scroll. Saves/restores `document.body.style.webkitUserSelect` during drag.
- **Wheel events**: Document-level `wheel` listener with `AddEventListenerOptions { passive: false }` to allow `preventDefault()` when wheeling over scrollbar.
- **Thumb scroll tracking**: Uses `StoredValue<bool>` flag + `StoredValue<i32>` cancel ID with inline rAF loop for polling scroll position changes, rather than storing a cleanup function (avoids lifetime issues in `Fn` closures).
- **Omitted**: `useCallbackRef` — Leptos closures don't cause reactive re-execution, so stable callback identity is unnecessary.
- **Omitted**: `createContextScope` — Standard `provide_context`/`expect_context` with `Provider` used instead.
- **Omitted**: `useLayoutEffect` distinction — Leptos `Effect::new` runs synchronously after render, which is sufficient.
- **Omitted**: `add_unlinked_scroll_listener` as a standalone reusable function — the logic is inlined directly in `ScrollAreaThumbImpl` using `StoredValue`-based control flow for Send+Sync compliance.
- **Key decision**: `RafClosureHolder` type alias (`Rc<RefCell<Option<Closure<dyn Fn()>>>>`) introduced to satisfy clippy's `type_complexity` lint for the rAF closure pattern used in resize observer and debouncing.
- **Key decision**: Every component in the chain uses `AttributeInterceptor` to capture caller-applied attributes (e.g. `attr:class="scrollbar"`) and forward them to the next component via two patterns: (1) `{..attrs}` spread on the child element when it's the only element in the `AttributeInterceptor` scope and followed only by `attr:`/`style:`/`on:` attributes (the Leptos `view!` macro misparsing E0574 occurs if `{..attrs}` immediately follows a regular prop like `node_ref`); (2) `.add_any_attr(attrs)` method chain on the inner `view!` result for cases with nested `view!` macro invocations (match dispatchers, components wrapping `Presence`/`Show`). Non-Copy values like `ScrollAreaContextValue` and `ScrollbarContextValue` use `provide_context()` instead of `<Provider>` to avoid move-in-`Fn`-closure errors from `AttributeInterceptor`. An internal `style: Signal<String>` prop is threaded from `ScrollAreaScrollbarX/Y` to `ScrollAreaScrollbarImpl` to merge axis-specific positioning CSS (e.g. `bottom: 0; right: var(...)`) with the base `position: absolute;` style on the scrollbar's `Primitive`.
- **Key decision**: `class` prop forwarding via context — `AnyAttribute` is not `Send+Sync` and gets consumed by `add_any_attr()`, making it impossible to pass through `Fn` closures required by `Presence`/`Show` children. To forward CSS classes from the public `ScrollAreaScrollbar`, `ScrollAreaThumb`, and `ScrollAreaCorner` components to their inner `*Impl` components (where the `Primitive` div lives), explicit `class: MaybeProp<String>` props are accepted on the public components, stored in forwarding context types (`ForwardedScrollbarClass`, `ForwardedThumbClass`, `ForwardedCornerClass`), and read in the `*Impl` components via `use_context`. The class values are wrapped in `StoredValue<String>` for `Copy`-ability in `Fn` closures.
- **Bug fix**: `to_int` must mimic JS `parseInt(value, 10)` behavior by stripping non-numeric suffixes (e.g. `"2px"` → `2`). Rust's `str::parse::<f64>()` fails on such inputs, returning 0, which broke scrollbar padding calculations when CSS padding was set (the story CSS uses `padding: 2px` on scrollbars).
