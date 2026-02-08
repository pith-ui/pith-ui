---
react_location: "[[reference/react-radix-primitives/packages/react/announce/src/announce.tsx|announce]]"
rust_location: "[[packages/primitives/leptos/announce/src/announce.rs|announce]]"
react_story: ""
rust_story: ""
dependencies:
  - "[[leptos-primitive]]"
ported: true
tested: false
---
## Intent

Accessible ARIA live region announcements. Renders visible content while also portaling it to hidden live regions for screen reader announcement with configurable politeness levels.

## React API

```ts
const Announce: React.ForwardRefExoticComponent<AnnounceProps>
```

Props: `type` (`'polite'` | `'assertive'` | `'off'`), `role`, `aria-atomic`, `aria-relevant`, `regionIdentifier`, `children`.

## Leptos API

```rust
#[component]
pub fn Announce(
    #[prop(into, optional)] r#type: Option<RegionType>,
    #[prop(into, optional)] role: Option<RegionRole>,
    #[prop(into, optional)] aria_atomic: Option<bool>,
    #[prop(into, optional)] aria_relevant: Option<String>,
    #[prop(into, optional)] region_identifier: Option<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView

pub use Announce as Root;

pub enum RegionType { Polite (default), Assertive, Off }
pub enum RegionRole { Status, Alert, Log, None }
```

## React Implementation Notes

- ~236 lines.
- Manages live region elements at document root.
- Deduplicates live regions by configuration (type + role + atomic + relevant).
- Uses `visibilitychange` listener to suppress announcements in background tabs.
- Caches live regions per configuration to reuse across multiple `Announce` instances.
- Handles owner document switching for dynamic contexts.
- Portals content into invisible live regions via `ReactDOM.createPortal`.

## Leptos Implementation Notes

- `ChildrenFn` used instead of `TypedChildren` because children must render in both the visible `Primitive` div and the hidden live region portal.
- `mount_to` used for portal rendering into the hidden live region, following the same pattern as `LeptosPortal` in the portal package. Mount handle is wrapped in `SendWrapper` and dropped on cleanup.
- `thread_local! { RefCell<Vec<(Element, u32)>> }` for listener ref-counting, matching the React `listenerMap` pattern. Linear scan is acceptable given the small expected number of unique regions.
- `Closure::forget()` is used for the `visibilitychange` listener closure. When the last `Announce` instance for a region unmounts, attributes are reset to their original values so the leaked listener becomes a no-op.
- Removed `leptos-compose-refs` dependency — React uses `useComposedRefs` to detect owner document from the ref callback, but Leptos uses `document()` from `leptos_dom::helpers` directly, which is sufficient for WASM targets.
- `RegionType` and `RegionRole` are enums with `as_str()` methods rather than string literals. `RegionType` defaults to `Polite` via `#[default]`.
- The React component uses a `useLayoutEffect` + `useState` to lazily find/create the region element. The Leptos version does this synchronously during component construction (inside the `cfg!(target_arch = "wasm32")` guard), which is equivalent since we only run in the browser.
- No stories exist — the React reference has no stories for this component.
