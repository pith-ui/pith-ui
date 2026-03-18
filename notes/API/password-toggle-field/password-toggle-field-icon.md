# PasswordToggleFieldIcon

## React Signature

```typescript
const PasswordToggleFieldIcon = React.forwardRef<
  SVGSVGElement,
  PasswordToggleFieldIconProps
>(...)

interface PasswordToggleFieldIconProps extends Omit<PrimitiveSvgProps, 'children'> {
  visible: React.ReactElement;
  hidden: React.ReactElement;
}
```

## Leptos Signature

```rust
pub fn PasswordToggleFieldIcon(
    #[prop(into)] visible_icon: ViewFn,
    #[prop(into)] hidden_icon: ViewFn,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `visible` | `visible_icon` | `React.ReactElement` | `ViewFn` | The SVG icon element to render when the password is visible (shown as plain text). Note: Leptos renames from `visible` to `visible_icon`. |
| `hidden` | `hidden_icon` | `React.ReactElement` | `ViewFn` | The SVG icon element to render when the password is hidden (masked). Note: Leptos renames from `hidden` to `hidden_icon`. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered `<svg>` DOM element. |
| `asChild` | -- | `boolean` | -- | React has `asChild` (always `true` internally). Leptos does not expose this -- the component always renders with `as_child=true` internally, delegating to the provided icon element as the `<svg>`. |
| *(spread)* | -- | `...Omit<PrimitiveSvgProps, 'children'>` | -- | React allows spreading any `<svg>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Renders a `<svg>` wrapper with `aria-hidden="true"` and `asChild` set to `true`. The wrapper delegates to whichever icon (`visible_icon` or `hidden_icon`) matches the current visibility state.
- The icon is always hidden from assistive technology (`aria-hidden="true"`) because the parent `Toggle` button provides the accessible label.
- In React, `children` is explicitly omitted from the prop type to prevent misuse (the icon children come from the `visible`/`hidden` props). Leptos achieves the same by not having a `children` prop.
