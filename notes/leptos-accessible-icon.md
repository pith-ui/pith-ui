---
react_location: "[[reference/react-radix-primitives/packages/react/accessible-icon/src/accessible-icon.tsx|accessible-icon]]"
rust_location: "[[packages/primitives/leptos/accessible-icon/src/accessible_icon.rs|accessible_icon]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/accessible-icon.stories.tsx|accessible-icon]]"
rust_story: "[[stories/leptos/src/primitives/accessible_icon.rs|accessible_icon]]"
dependencies:
  - "[[leptos-visually-hidden]]"
ported: true
tested: false
tested_story: false
---
## Intent

Makes icons accessible by pairing them with a visually hidden label. The icon is hidden from assistive technology (`aria-hidden="true"`) while the label is announced by screen readers, similar to `alt` text on images.

## React API

```ts
interface AccessibleIconProps {
  children?: React.ReactNode;  // expects exactly one child (the icon)
  label: string;
}

const AccessibleIcon: React.FC<AccessibleIconProps>
```

Uses `React.Children.only` to enforce a single child. Clones the child element with `aria-hidden="true"` and `focusable="false"`, then renders a `VisuallyHidden` sibling containing the label text.

## Leptos API

```rust
#[component]
fn AccessibleIcon(
    #[prop(into)] label: Signal<String>,
    children: TypedChildren<impl IntoView + 'static>,
) -> impl IntoView
```

`label` is reactive (`Signal<String>`). Children receive `aria-hidden` and `focusable` attributes via `.add_any_attr()`.

## React Implementation Notes

- Enforces exactly one child with `React.Children.only` — throws if zero or multiple children.
- Clones the child SVG element to inject `aria-hidden: "true"` and `focusable: "false"` (the `focusable` attribute prevents SVGs from being focusable in IE/Edge).
- Delegates to `@radix-ui/react-visually-hidden` for the label.

## Leptos Implementation Notes

- Does not enforce a single child — `TypedChildren` accepts any `IntoView`. Attributes are applied to the children view as a whole via `.add_any_attr()` rather than cloning a single element.
- `label` is a reactive signal, allowing dynamic label updates (React version uses a static string).
- Wraps `label` in an extra `Signal::derive` — this appears redundant since `label` is already a `Signal<String>`.
- Dependencies: `leptos`, `radix-leptos-visually-hidden`.
