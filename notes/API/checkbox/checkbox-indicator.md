# CheckboxIndicator

## React Signature

```typescript
const CheckboxIndicator = React.forwardRef<CheckboxIndicatorElement, CheckboxIndicatorProps>(...)

type CheckboxIndicatorElement = React.ComponentRef<typeof Primitive.span>;
type PrimitiveSpanProps = React.ComponentPropsWithoutRef<typeof Primitive.span>;

interface CheckboxIndicatorProps extends PrimitiveSpanProps {
  /**
   * Used to force mounting when more control is needed. Useful when
   * controlling animation with React animation libraries.
   */
  forceMount?: true;
}
```

## Leptos Signature

```rust
pub fn CheckboxIndicator(
    /// Used to force mounting when more control is needed.
    /// Useful when controlling animation with animation libraries.
    #[prop(into, optional)]
    force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop   | Leptos Prop   | Type (React)       | Type (Leptos)       | Description                                                                                                                                                                                                          |
| ------------ | ------------- | ------------------ | ------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>`   | When `true`, the indicator is always mounted in the DOM regardless of checked state. Useful for controlling enter/exit animations with external animation libraries. Without this, the indicator is only present when the checkbox is checked or indeterminate. |
| `ref`        | `node_ref`    | `React.Ref`        | `AnyNodeRef`        | Ref to the rendered DOM element (`<span>`).                                                                                                                                                                          |
| `asChild`    | `as_child`    | `boolean`          | `MaybeProp<bool>`   | When `true`, renders the child directly instead of wrapping in a `<span>`, merging props and refs.                                                                                                                   |
| `children`   | `children`    | `React.ReactNode`  | `Option<ChildrenFn>` | The content to render inside the indicator (e.g., a check icon, dash icon). In Leptos, children are optional — the indicator can be rendered empty for CSS-only styling.                                             |
| *(spread)*   | —             | `...PrimitiveSpanProps` | —              | React allows spreading any `<span>` HTML attribute. Leptos uses `attr:` directives on the call site instead.                                                                                                         |

### Data attributes (rendered on DOM)

| Attribute       | Value                                        | Description                                                          |
| --------------- | -------------------------------------------- | -------------------------------------------------------------------- |
| `data-state`    | `"checked" \| "unchecked" \| "indeterminate"` | Reflects the parent checkbox's current checked state.                |
| `data-disabled` | `""` (present/absent)                        | Present when the parent checkbox is disabled.                        |

### Implicit behavior

- **Presence control:** The indicator is wrapped in a `Presence` component. It is only mounted when `force_mount` is `true`, or when the parent checkbox's state is `CheckedState::True` or `CheckedState::Indeterminate`. When the state is `CheckedState::False` (unchecked), the indicator is removed from the DOM (unless force-mounted).
- **Context dependency:** `CheckboxIndicator` reads the `CheckboxContextValue` from context (provided by the parent `Checkbox` component). It uses `state` for presence logic and data attributes, and `disabled` for the `data-disabled` attribute.
- **Pointer events disabled:** The indicator has `pointer-events: none` set via inline style, ensuring click events pass through to the underlying `<button>` element.
