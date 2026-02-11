---
react_location: "[[reference/react-radix-primitives/packages/react/collapsible/src/collapsible.tsx|collapsible]]"
rust_location: "[[packages/primitives/leptos/collapsible/src/collapsible.rs|collapsible]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/collapsible.stories.tsx|collapsible]]"
rust_story: "[[stories/leptos/src/primitives/collapsible.rs|collapsible]]"
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-primitive]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-presence]]"
  - "[[leptos-id]]"
  - "[[leptos-use-controllable-state]]"
ported: true
tested: false
tested_story: false
---
## Intent

A controlled, composable component for managing expandable/collapsible content sections. Supports toggle states with optional animation integration and smart dimension tracking for height/width animations.

## React API

```ts
const Collapsible: React.ForwardRefExoticComponent<CollapsibleProps>
const CollapsibleTrigger: React.ForwardRefExoticComponent<CollapsibleTriggerProps>
const CollapsibleContent: React.ForwardRefExoticComponent<CollapsibleContentProps>
```

Props: `open`, `defaultOpen`, `disabled`, `onOpenChange`. Content supports `forceMount` for animation libraries.

## Leptos API

```rust
#[component]
pub fn Collapsible(
    open: MaybeProp<bool>,
    default_open: MaybeProp<bool>,
    disabled: MaybeProp<bool>,
    on_open_change: Option<Callback<bool>>,
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView

#[component]
pub fn CollapsibleTrigger(
    on_click: Option<Callback<ev::MouseEvent>>,
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView

#[component]
pub fn CollapsibleContent(
    force_mount: MaybeProp<bool>,
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    children: Option<TypedChildrenFn<impl IntoView + 'static>>,
) -> impl IntoView
```

## React Implementation Notes

- ~247 lines.
- Uses `Presence` component for conditional rendering (animation-friendly).
- Tracks content dimensions via `getBoundingClientRect()` and exposes as CSS variables: `--radix-collapsible-content-height`, `--radix-collapsible-content-width`.
- Blocks animations temporarily while measuring to get full dimensions.
- Manages animation state with `isMountAnimationPreventedRef` via `requestAnimationFrame`.
- `data-state`: `open` | `closed`. `data-disabled` when disabled.
- Accessibility: Sets proper `aria-controls` and `aria-expanded` on trigger.

## Leptos Implementation Notes

1. **No `createContextScope`/`__scopeCollapsible`**: React's scoped context pattern replaced with standard `provide_context`/`expect_context`. Leptos does not need the scope indirection that React uses for nested context composition.

2. **No render-prop split for Presence**: React's `Presence` passes `present` via a render callback `{({ present }) => ...}`. In Leptos, `Presence` mounts/unmounts children directly. We still split `CollapsibleContentImpl` as a separate internal component for structural similarity with the React source, tracking `is_present` internally.

3. **`useLayoutEffect` replaced with `Effect::new`**: In Leptos CSR, effects run after render. Animation blocking during measurement (setting `transition-duration: 0s` and `animation-name: none` before reading `getBoundingClientRect`) prevents visual flash, matching the React behavior.

4. **CSS variables via inline style manipulation**: React merges CSS variables into the style object; Leptos sets the `style` attribute with the CSS variable declarations (`--radix-collapsible-content-height`, `--radix-collapsible-content-width`).

5. **`AttributeInterceptor` pattern**: Used on `Collapsible`, `CollapsibleTrigger`, and `CollapsibleContentImpl` to forward user-supplied attributes (e.g., `attr:class`) through to the underlying `Primitive`. For `CollapsibleContent`, the `.add_any_attr(attrs)` pattern (from scroll-area) forwards attributes across the `Presence` boundary.

6. **`requestAnimationFrame` for mount animation prevention**: Uses `SendWrapper<Closure<dyn Fn()>>` with `StoredValue` to hold the rAF closure, matching the React pattern of `isMountAnimationPreventedRef` that gets set to `false` after the first frame.

7. **`originalStylesRef` pattern**: React uses a ref that is lazily initialized. Leptos uses `RwSignal<Option<String>>` for `original_transition_duration` and `original_animation_name`, initialized on first effect run when the node is available.
