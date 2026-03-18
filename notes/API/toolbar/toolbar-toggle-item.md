# ToolbarToggleItem

## React Signature

```typescript
const ToolbarToggleItem = React.forwardRef<ToolbarToggleItemElement, ToolbarToggleItemProps>(...)

type ToolbarToggleItemElement = React.ComponentRef<typeof ToggleGroupPrimitive.Item>;
type ToggleGroupItemProps = React.ComponentPropsWithoutRef<typeof ToggleGroupPrimitive.Item>;

interface ToolbarToggleItemProps extends ToggleGroupItemProps {}

// The underlying ToggleGroupItem props:
interface ToggleGroupItemProps extends Omit<ToggleProps, 'defaultPressed' | 'onPressedChange'> {
  value: string;
}
```

`ToggleGroupItemProps` extends `ToggleProps` (minus `defaultPressed` and `onPressedChange`, which are managed by the group). `ToggleProps` extends `PrimitiveButtonProps`, so all standard `<button>` attributes are accepted.

## Leptos Signature

```rust
pub fn ToolbarToggleItem(
    #[prop(into)] value: Signal<String>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `value` | `value` | `string` (required) | `Signal<String>` (required) | A unique string identifying this item within the toggle group. This value is passed to `onValueChange` callbacks. Note: Leptos uses `Signal<String>` (with `#[prop(into)]`), so pass a `&str` directly. |
| `disabled` | `disabled` | `boolean` | `MaybeProp<bool>` (default `false`) | Disables this individual toggle item. Inherits the group-level disabled state — the effective disabled is `group.disabled \|\| item.disabled`. When disabled, the item cannot be toggled and is skipped during keyboard navigation. |
| `onClick` | `on_click` | `(event: MouseEvent) => void` | `Option<Callback<ev::MouseEvent>>` | Optional click handler. Called when the toggle item is clicked, in addition to the internal toggle logic. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<button>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<button>`, merging props and refs. |
| *(spread)* | — | `...ToggleGroupItemProps` | — | React allows spreading any `<button>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Composed as `ToolbarButton > ToggleGroupItem`: wraps the `ToggleGroupItem` inside a `ToolbarButton` (with `as_child=true`), which in turn wraps it in a `RovingFocusGroupItem`. This ensures the toggle item participates in the toolbar's keyboard navigation.
- In single-selection mode (`type="single"`), the item renders with `role="radio"` and `aria-checked` instead of `aria-pressed`.
- In multiple-selection mode, the item uses the default `aria-pressed` attribute from `Toggle`.

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"on" \| "off"` | Whether the item is currently pressed. Inherited from `Toggle`. |
| `data-disabled` | `""` (present/absent) | Present when the item is disabled (either directly or via the group). |
