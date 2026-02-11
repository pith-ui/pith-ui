---
react_location: "[[reference/react-radix-primitives/packages/react/portal/src/portal.tsx|portal]]"
rust_location: "[[packages/primitives/leptos/portal/src/portal.rs|portal]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/portal.stories.tsx|portal]]"
rust_story: "[[stories/leptos/src/primitives/portal.rs|portal]]"
dependencies:
  - "[[leptos-primitive]]"
ported: true
tested: false
tested_story: true
---
## Intent

Renders a subtree into a different part of the DOM (default: `document.body`). Used by overlays, tooltips, and dialogs to escape parent clipping/stacking contexts.

## React API

```ts
interface PortalProps extends PrimitiveDivProps {
  container?: Element | DocumentFragment | null;
}
```

Uses `ReactDOM.createPortal`. Waits for mount (`useState` + `useLayoutEffect`) before portaling, falls back to `document.body`.

## Leptos API

```rust
#[component]
fn Portal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    #[prop(optional)] container_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

Accepts either a direct element (`container`) or a ref (`container_ref`). Wraps content in `Primitive` inside a custom `LeptosPortal`.

## React Implementation Notes

- Simple: `createPortal` into `container || document.body`.
- Defers portal creation until after mount to avoid SSR issues.

## Leptos Implementation Notes

- Contains a custom `LeptosPortal` module (based on Leptos's built-in `Portal`) that supports reactive mount targets.
- Uses `mount_to` from `leptos_dom` to render children into the target element.
- Mount target resolution: `mount_ref` (AnyNodeRef) > `mount` (Element) > `document.body`.
- Tracks `current_mount` in an `RwSignal` — if the mount target changes, the portal re-mounts.
- `SendWrapper` is needed because `web_sys::Element` is not `Send`, but Leptos signals require `Send`.
- Has a TODO comment about passing attributes to the Primitive (`AttributeInterceptor` code is commented out).
- Dependencies: `leptos`, `leptos-node-ref`, `leptos_dom`, `radix-leptos-primitive`, `send_wrapper`, `web-sys`.

### Review Notes

- hand-rolled storybook makes the portals look weird & overlapping. The real storybook renders stories inside an iframe so that everything is contained within those bounds. could consider doing the same in the future.
