# PasswordToggleFieldToggle

## React Signature

```typescript
const PasswordToggleFieldToggle = React.forwardRef<
  HTMLButtonElement,
  PasswordToggleFieldToggleProps
>(...)

interface PasswordToggleFieldToggleProps extends Omit<PrimitiveButtonProps, 'type'> {}
```

The `type` prop is omitted because it is always set to `"button"` internally.

## Leptos Signature

```rust
pub fn PasswordToggleFieldToggle(
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] on_pointer_down: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_cancel: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_up: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `onClick` | `on_click` | `React.MouseEventHandler` | `Option<Callback<ev::MouseEvent>>` | Callback for click events. Runs before the internal toggle logic. If the event's default is prevented, visibility is not toggled. |
| `onPointerDown` | `on_pointer_down` | `React.PointerEventHandler` | `Option<Callback<ev::PointerEvent>>` | Callback for pointer down events. The component tracks pointer state internally for focus management. |
| `onPointerCancel` | `on_pointer_cancel` | `React.PointerEventHandler` | `Option<Callback<ev::PointerEvent>>` | Callback for pointer cancel events. Internal state is always reset on cancel regardless of `preventDefault`. |
| `onPointerUp` | `on_pointer_up` | `React.PointerEventHandler` | `Option<Callback<ev::PointerEvent>>` | Callback for pointer up events. Internal state is cleaned up after a short delay. |
| `aria-label` | `aria_label` | `string \| undefined` | `MaybeProp<String>` | Explicit ARIA label for the toggle button. If provided, it overrides the auto-generated label. Note: In React, this is passed as the `aria-label` HTML attribute; in Leptos, it is a named prop. |
| `aria-controls` | -- | `string \| undefined` | -- | In React, this can be explicitly set (defaults to the input ID post-hydration). In Leptos, it is always auto-set from context. |
| `aria-hidden` | -- | `boolean \| undefined` | -- | React uses this for pre-hydration SSR (`aria-hidden="true"` before hydration). Not applicable in Leptos (CSR only). |
| `tabIndex` | -- | `number \| undefined` | -- | React defaults to `-1` before hydration. Not applicable in Leptos (CSR only). |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered `<button>` DOM element. |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<button>`, merging props and refs. |
| *(spread)* | -- | `...Omit<PrimitiveButtonProps, 'type'>` | -- | React allows spreading any `<button>` HTML attribute (except `type`). Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

No custom data attributes are rendered.

### Implicit behavior

- Always renders with `type="button"` to prevent accidental form submission.
- On click, toggles the visibility state. If the click was triggered by a pointer (not keyboard), focus is moved to the input and the previous cursor/selection position is restored.
- `aria-controls` is set to the input's `id` (auto-derived from context).
- `aria-pressed` is set to reflect the current visibility state. **Note:** This attribute is set in the Leptos implementation but not in the React implementation.
- Auto-generates an `aria-label` of `"Show password"` (when hidden) or `"Hide password"` (when visible) if the button has no visible text content and no explicit `aria-label` is provided. A `MutationObserver` monitors text changes to update this dynamically.
- A global `pointerup` listener resets the internal click-tracking state after the pointer is released, ensuring consistent behavior even if the pointer moves outside the button.
