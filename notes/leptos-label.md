---
react_location: "[[reference/react-radix-primitives/packages/react/label/src/label.tsx|label]]"
rust_location: "[[packages/primitives/leptos/label/src/label.rs|label]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/label.stories.tsx|label]]"
rust_story: "[[stories/leptos/src/primitives/label.rs|label]]"
dependencies:
  - "[[leptos-primitive]]"
ported: true
tested: false
tested_story: false
---
## Intent

Renders an accessible `<label>` element with built-in UX polish: prevents text selection on double-click while preserving normal interaction with nested form controls.

## React API

```ts
interface LabelProps extends PrimitiveLabelProps {}

const Label: React.ForwardRefExoticComponent<LabelProps>
```

Wraps `Primitive.label`. Intercepts `onMouseDown` to prevent text selection on double-click (`event.detail > 1`) unless the click target is inside a `button`, `input`, `select`, or `textarea`.

## Leptos API

```rust
#[component]
fn Label(
    #[prop(into, optional)] on_mouse_down: MaybeCallback<MouseEvent>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView
```

## React Implementation Notes

- `onMouseDown` handler: checks if `event.target.closest('button, input, select, textarea')` matches — if so, returns early (allows normal behavior for interactive elements).
- Calls `props.onMouseDown?.(event)` to forward the consumer's handler.
- If `event.detail > 1` (double-click) and not `defaultPrevented`, calls `event.preventDefault()` to block text selection.

## Leptos Implementation Notes

- Faithfully mirrors the React logic. Uses `event_target::<web_sys::Element>(&event)` to get the target, then `.closest(...)` with the same selector string.
- Consumer's `on_mouse_down` is a `MaybeCallback<MouseEvent>` invoked via `.run()`.
- Clones the event before passing to the callback so the `default_prevented()` check afterward sees the original state.
- Dependencies: `leptos`, `leptos-maybe-callback`, `leptos-node-ref`, `radix-leptos-primitive`, `web-sys`.
