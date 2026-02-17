---
react_location: "[[reference/react-radix-primitives/packages/react/navigation-menu/src/navigation-menu.tsx|navigation-menu]]"
rust_location: "[[packages/primitives/leptos/navigation-menu/src/navigation_menu.rs|navigation_menu]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/navigation-menu.stories.tsx|navigation-menu]]"
rust_story: "[[stories/leptos/src/primitives/navigation_menu.rs|navigation_menu]]"
dependencies:
  - "[[leptos-collection]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-direction]]"
  - "[[leptos-dismissable-layer]]"
  - "[[leptos-id]]"
  - "[[leptos-presence]]"
  - "[[leptos-primitive]]"
  - "[[leptos-use-controllable-state]]"
  - "[[leptos-use-previous]]"
  - "[[leptos-visually-hidden]]"
ported: true
tested: false
tested_story: false
unstable: false
---

## Intent

NavigationMenu is a collection of links for navigating websites. It provides accessible navigation with support for expandable trigger/content pairs, animated transitions between content panels, keyboard navigation within content via Tab, arrow-key navigation between focusable items within a FocusGroup, viewport-based rendering with animated motion attributes, an animated position indicator, and nested sub-menus. It closely mirrors the behavior of desktop application menu bars.

## React API

### Components

- **`NavigationMenu`** (root) — `<nav>` wrapper. Props: `value`, `defaultValue`, `onValueChange`, `dir`, `orientation` (default `"horizontal"`), `delayDuration` (default `200`), `skipDelayDuration` (default `300`).
- **`NavigationMenuSub`** — Nested sub-menu. Props: `value`, `defaultValue`, `onValueChange`, `orientation`.
- **`NavigationMenuList`** — `<ul>` wrapper for navigation items.
- **`NavigationMenuItem`** — `<li>` wrapper. Props: `value` (auto-generated if omitted).
- **`NavigationMenuTrigger`** — `<button>` that toggles associated content. Props: `disabled`. Sets `data-state`, `data-disabled`, `aria-expanded`, `aria-controls`.
- **`NavigationMenuLink`** — `<a>` link. Props: `active`, `onSelect`. Sets `data-active`, dispatches `LINK_SELECT` custom event.
- **`NavigationMenuContent`** — Content panel. Props: `forceMount`, `onPointerEnter`, `onPointerLeave`, `onEscapeKeyDown`, `onFocusOutside`, `onPointerDownOutside`, `onInteractOutside`. Sets `data-state`, `data-motion`, `data-orientation`.
- **`NavigationMenuIndicator`** — Animated position indicator. Props: `forceMount`.
- **`NavigationMenuViewport`** — Shared viewport for rendering content. Props: `forceMount`. Sets CSS custom properties `--radix-navigation-menu-viewport-width/height`.

### Internal components

- **`NavigationMenuProvider`** — Shared state provider used by both Root and Sub.
- **`NavigationMenuContentImpl`** — Wraps `DismissableLayer` with content-specific logic.
- **`NavigationMenuIndicatorImpl`** — Position calculation and resize observation.
- **`FocusGroup`** / **`FocusGroupItem`** — Arrow-key navigation within content panels.
- **`ViewportContentMounter`** — Registers content data into viewport context.

### Custom events

- `LINK_SELECT` (`"navigationMenu.linkSelect"`) — Dispatched when a link is clicked.
- `ROOT_CONTENT_DISMISS` (`"navigationMenu.rootContentDismiss"`) — Dispatched to dismiss root content.

## Leptos API

All public components follow the React API with snake_case prop names:

```rust
#[component]
pub fn NavigationMenu(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional, default = MaybeProp::from(Orientation::Horizontal))] orientation: MaybeProp<Orientation>,
    #[prop(into, optional, default = MaybeProp::from(200.0))] delay_duration: MaybeProp<f64>,
    #[prop(into, optional, default = MaybeProp::from(300.0))] skip_delay_duration: MaybeProp<f64>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView

#[component]
pub fn NavigationMenuSub(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional, default = MaybeProp::from(Orientation::Horizontal))] orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView

#[component]
pub fn NavigationMenuList(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView

#[component]
pub fn NavigationMenuItem(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView

#[component]
pub fn NavigationMenuTrigger(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView

#[component]
pub fn NavigationMenuLink(
    #[prop(into, optional)] active: MaybeProp<bool>,
    #[prop(into, optional)] on_select: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView

#[component]
pub fn NavigationMenuContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] on_pointer_enter: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_leave: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] on_interact_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView

#[component]
pub fn NavigationMenuIndicator(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView

#[component]
pub fn NavigationMenuViewport(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## React Implementation Notes

- ~1287 lines with 9 public sub-components and several internal components.
- Uses three timers (`openTimerRef`, `closeTimerRef`, `skipDelayTimerRef`) to manage hover delay behavior. Open and close are delayed; "skip delay" allows fast transitions between triggers after a content was already opened.
- `FocusGroup` / `FocusGroupItem` use a separate Collection instance (distinct from the item collection) to track focusable elements within content panels and handle arrow-key navigation.
- `NavigationMenuContentImpl` computes `data-motion` attribute by comparing current value/previous value indices in the collection to determine animation direction (`from-start`/`from-end`/`to-start`/`to-end`).
- `NavigationMenuIndicator` renders into the indicator track element (a child of `NavigationMenuList`) using a portal pattern. It measures the active trigger element and track to compute CSS transform/size.
- `NavigationMenuViewport` renders all registered content panels from the viewport content map, wrapping each in `Presence` with the active one visible. Sets CSS custom properties for width/height.
- Tab key in content redirects focus to a hidden proxy element (`VisuallyHidden`) positioned outside the content, which then triggers focus to move naturally to the trigger or next element.
- `useCallbackRef` is used extensively to wrap callback props for stable identity.
- `getTabbableCandidates` uses `TreeWalker` to find focusable elements in content, filtering out hidden inputs and hidden elements.
- Complex pointer-based interaction with mouse detection (`whenMouse` filter ensures only mouse pointer events trigger hover behavior, not touch).
- `DismissableLayer` for outside interaction handling with custom logic to prevent dismiss when interacting with triggers or the viewport itself.

## Leptos Implementation Notes

### Omissions

1. **`useCallbackRef`** — Omitted entirely. React's `useCallbackRef` ensures stable callback identity across renders to avoid stale closure issues. Leptos `Callback` values are inherently stable (they wrap `StoredValue` internally), making this unnecessary.

2. **`React.forwardRef`** — Not applicable. Leptos components accept `node_ref: AnyNodeRef` directly as a prop, and `use_composed_refs` is used to merge multiple refs when needed.

3. **`React.useLayoutEffect`** — Leptos has no layout effect equivalent. All side effects use `Effect::new()`. For cases where synchronous DOM measurement is critical (indicator positioning, viewport sizing), the resize observer pattern provides equivalent behavior.

4. **`useCallback` / `useMemo` wrapping** — React uses these to memoize event handlers and derived values. In Leptos, closures captured in signals and `Memo::new` provide equivalent reactivity without explicit memoization wrappers.

5. **`core-primitive` dependency** — The React source imports from `@radix-ui/primitive` for `composeEventHandlers`. In Leptos, `compose_callbacks` from `radix-leptos-primitive` serves the same purpose, so no separate `core-primitive` dependency is needed.

6. **`use-callback-ref` dependency** — Not needed per omission #1.

### Key decisions

1. **Timer management**: Follows the hover-card pattern exactly — `StoredValue<Option<i32>>` for timer IDs, `set_timeout` / `clear_timeout` helper functions using `Closure::once_into_js`.

2. **Two Collection instances**: `CollectionProvider<NavigationMenuItemData>` for tracking menu items (used by triggers, indicator, content motion calculation) and `CollectionProvider<FocusGroupItemData>` for tracking focusable elements within `FocusGroup`. The generic type parameter disambiguates the contexts.

3. **Viewport content registration**: `ContentData` struct holds all props needed to render `NavigationMenuContentImpl` inside the viewport. Children are stored as `StoredValue<Option<ChildrenFn>>`. A `ViewportContentMounter` component (renders no DOM) registers/unregisters content data in the viewport content map via effects.

4. **Indicator positioning**: Uses `Portal` component from `radix-leptos-portal` with `mount` prop targeting the indicator track `RwSignal<Option<SendWrapper<HtmlElement>>>`. The indicator impl uses `use_resize_observer` on both the active trigger and track to compute position.

5. **`SendWrapper` usage**: Web API types (`HtmlElement`, `ResizeObserver`, event handler `Closure`s) are not `Send`/`Sync` but need to be stored in Leptos signals. Wrapped in `SendWrapper` where needed (e.g., `RwSignal<Option<SendWrapper<HtmlElement>>>` for DOM element refs, `StoredValue<Option<SendWrapper<ResizeObserver>>>` for observer cleanup).

6. **`Orientation` enum**: Defined locally in the navigation menu module (not imported from roving-focus) per existing convention in the codebase. Implements `Display`, `Default`, and `IntoAttributeValue`.

7. **Content motion attribute**: Computed as a `Memo` that watches `context.value`, `context.previous_value`, and the collection items. Compares item indices to determine animation direction. Stored in `prev_motion_attribute: StoredValue<Option<&'static str>>` to persist across reactive updates.

8. **Event delegation for dismiss**: Uses `web_sys::CustomEvent` dispatch/listen for `LINK_SELECT` and `ROOT_CONTENT_DISMISS` events, following the React pattern of inter-component communication through the DOM event system rather than shared state.
