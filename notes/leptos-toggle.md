---
react_location: "[[reference/react-radix-primitives/packages/react/toggle/src/toggle.tsx|toggle]]"
rust_location: "[[packages/primitives/leptos/toggle/src/toggle.rs|toggle]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/toggle.stories.tsx|toggle]]"
rust_story: "[[stories/leptos/src/primitives/toggle.rs|toggle]]"
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-primitive]]"
  - "[[leptos-use-controllable-state]]"
ported: true
tested: false
tested_story: true
---
## Intent

A two-state button that can be either on or off. Simpler than Switch — no form integration, no hidden input, just a `<button>` with `aria-pressed`.

## React API

```ts
interface ToggleProps extends PrimitiveButtonProps {
  pressed?: boolean;
  defaultPressed?: boolean;
  onPressedChange?(pressed: boolean): void;
}
```

## Leptos API

```rust
#[component]
fn Toggle(
    pressed: MaybeProp<bool>,
    default_pressed: MaybeProp<bool>,
    on_pressed_change: Option<Callback<bool>>,
    disabled: MaybeProp<bool>,
    on_click: Option<Callback<ev::MouseEvent>>,
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView
```

## React Implementation Notes

- Simple: `useControllableState` for pressed, `composeEventHandlers` for click.
- `data-state`: `on` | `off`.
- Respects `disabled` — won't toggle when disabled.
- Sets `type="button"` on the underlying `<button>`.

## Leptos Implementation Notes

- Faithful port. Uses `compose_callbacks` and `use_controllable_state`.
- Disabled check is in the click handler (same as React).
- Uses `AttributeInterceptor` + `{..attrs}` pattern for attribute forwarding (Leptos 0.8).
- Uses `TypedChildrenFn` + `StoredValue` for children (Leptos 0.8).
- Uses `AnyNodeRef` from `leptos-node-ref` instead of `NodeRef<AnyElement>`.
- Dependencies: `leptos`, `leptos-node-ref`, `radix-leptos-primitive`, `radix-leptos-use-controllable-state`.
