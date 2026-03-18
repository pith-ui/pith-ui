# TabsTrigger

## React Signature

```typescript
const TabsTrigger = React.forwardRef<TabsTriggerElement, TabsTriggerProps>(...)

type TabsTriggerElement = React.ComponentRef<typeof Primitive.button>;
type PrimitiveButtonProps = React.ComponentPropsWithoutRef<typeof Primitive.button>;

interface TabsTriggerProps extends PrimitiveButtonProps {
  value: string;
}
```

`TabsTriggerProps` inherits from `PrimitiveButtonProps` -- all standard `<button>` attributes (including `disabled`) are accepted via spread.

## Leptos Signature

```rust
pub fn TabsTrigger(
    /// A unique value identifying this tab.
    #[prop(into)] value: String,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_mouse_down: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_focus: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `value` | `value` | `string` (required) | `String` (required) | A unique string identifying this tab within the tabs component. This value is passed to `onValueChange` and used to match with the corresponding `TabsContent`. **Note:** `value` does not have `#[prop(into)]`, so it requires `.to_string()` at the call site for string literals. |
| `disabled` | `disabled` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | Disables this trigger. When disabled, the trigger cannot be clicked, is skipped during keyboard navigation, and renders with `data-disabled` and the HTML `disabled` attribute. In React, `disabled` comes via `PrimitiveButtonProps` spread; in Leptos it is an explicit prop. |
| `onMouseDown` | `on_mouse_down` | `(event: MouseEvent) => void` | `Option<Callback<ev::MouseEvent>>` | Optional mouse-down handler composed with the internal handler. The internal handler activates the tab on left-click (ignoring Ctrl+click on macOS). |
| `onKeyDown` | `on_key_down` | `(event: KeyboardEvent) => void` | `Option<Callback<ev::KeyboardEvent>>` | Optional key-down handler composed with the internal handler. The internal handler activates the tab on `Enter` or `Space`. |
| `onFocus` | `on_focus` | `(event: FocusEvent) => void` | `Option<Callback<ev::FocusEvent>>` | Optional focus handler composed with the internal handler. In `automatic` activation mode, the internal handler activates the tab when it receives focus. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<button>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<button>`, merging props and refs. The child must be a focusable, clickable element for accessibility. |
| *(spread)* | -- | `...PrimitiveButtonProps` | -- | React allows spreading any `<button>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- The trigger's `id` is auto-generated from the parent `Tabs` context's `base_id` in the format `{baseId}-trigger-{value}`. This ID is used by `TabsContent` for `aria-labelledby`.
- `aria-controls` is auto-set to point to the corresponding `TabsContent`'s `id` (`{baseId}-content-{value}`).
- `aria-selected` is auto-set based on whether this trigger's `value` matches the active tab value.
- The trigger is wrapped in a `RovingFocusGroupItem`, which manages keyboard focus within the tab list. The item is `focusable` when not disabled and `active` when selected.
- `type="button"` is set on the rendered `<button>` to prevent form submission.
- In automatic activation mode, the focus handler defers the value change to a macrotask (`setTimeout(0)`) in the Leptos implementation to avoid re-entrant signal updates.

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"active" \| "inactive"` | Whether this trigger's tab panel is currently active. |
| `data-disabled` | `""` (present/absent) | Present when the trigger is disabled. |
