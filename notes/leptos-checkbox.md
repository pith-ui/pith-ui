---
react_location: "[[reference/react-radix-primitives/packages/react/checkbox/src/checkbox.tsx|checkbox]]"
rust_location: "[[packages/primitives/leptos/checkbox/src/checkbox.rs|checkbox]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/checkbox.stories.tsx|checkbox]]"
rust_story: "[[stories/leptos/src/primitives/checkbox.rs|checkbox]]"
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-primitive]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-presence]]"
  - "[[leptos-use-controllable-state]]"
  - "[[leptos-use-previous]]"
  - "[[leptos-use-size]]"
ported: true
tested: false
---
## Intent

A tri-state checkbox (`true`/`false`/`indeterminate`) with form integration. Renders a `<button>` with `role="checkbox"` plus a hidden `<input type="checkbox">` that bubbles change events to native forms.

## React API

```ts
const Checkbox: React.ForwardRefExoticComponent<CheckboxProps>
const CheckboxIndicator: React.ForwardRefExoticComponent<CheckboxIndicatorProps>
// Also: CheckboxProvider, CheckboxTrigger, CheckboxBubbleInput (unstable)
```

`CheckedState = boolean | 'indeterminate'`. Supports controlled/uncontrolled via `useControllableState`. `CheckboxIndicator` uses `Presence` for mount/unmount animations.

## Leptos API

```rust
pub enum CheckedState { False, True, Indeterminate }

#[component] fn Checkbox(...) -> impl IntoView
#[component] fn CheckboxIndicator(force_mount: MaybeProp<bool>, ...) -> impl IntoView
```

**Note:** Uses old Leptos API. Needs migration.

## React Implementation Notes

- `Checkbox` = `CheckboxProvider` + `CheckboxTrigger` + `BubbleInput` (recently split for composability).
- Enter key is intercepted (`preventDefault`) per WAI-ARIA spec.
- `BubbleInput` uses property descriptor hack to set `checked` programmatically and dispatch a `click` event to bubble changes to forms.
- Form reset is handled by listening for `reset` events on the parent form.
- `hasConsumerStoppedPropagationRef` tracks whether the consumer stopped propagation on the button click.

## Leptos Implementation Notes

- Combined component (not split into Provider/Trigger). Old Leptos `Callback::call` style.
- `CheckedState` is an enum with `Display` and `IntoAttribute` impls (old API).
- Form reset listener uses `Rc<Closure<dyn Fn(Event)>>` with manual add/remove.
- `BubbleInput` dispatches a `click` event with `web_sys::EventInit` — simpler than React's property descriptor approach.
- `CheckboxIndicator` uses `Presence` for animation support with `force_mount` option.
- Uses old Leptos API — needs migration.
- Dependencies: `leptos`, `radix-leptos-compose-refs`, `radix-leptos-presence`, `radix-leptos-use-controllable-state`, `radix-leptos-use-previous`, `radix-leptos-use-size`, `web-sys`.
