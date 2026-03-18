# ToggleGroupItem

## React Signature

```typescript
const ToggleGroupItem = React.forwardRef<ToggleGroupItemElement, ToggleGroupItemProps>(...)

type ToggleGroupItemElement = ToggleGroupItemImplElement;
interface ToggleGroupItemProps extends Omit<ToggleGroupItemImplProps, 'pressed'> {}

// The underlying impl interface:
type ToggleGroupItemImplElement = React.ComponentRef<typeof Toggle>;
type ToggleProps = React.ComponentPropsWithoutRef<typeof Toggle>;
interface ToggleGroupItemImplProps extends Omit<ToggleProps, 'defaultPressed' | 'onPressedChange'> {
  /**
   * A string value for the toggle group item. All items within a toggle group
   * should use a unique value.
   */
  value: string;
}
```

`ToggleGroupItemProps` inherits all `Toggle` props except `pressed`, `defaultPressed`, and `onPressedChange` (which are managed internally by the group). Inherited Toggle props include `disabled`.

## Leptos Signature

```rust
pub fn ToggleGroupItem(
    /// A string value for the toggle group item. All items within a toggle group
    /// should use a unique value.
    #[prop(into)]
    value: Signal<String>,
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
| `value` | `value` | `string` (required) | `Signal<String>` (required, `#[prop(into)]`) | A unique string identifying this item within the toggle group. Used to determine pressed state and passed to `onValueChange` callbacks. Because of `#[prop(into)]`, `&str` can be passed directly. |
| `disabled` | `disabled` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | Disables this specific item. The effective disabled state is `group.disabled \|\| item.disabled`. When disabled, the item cannot be clicked and is skipped during roving focus navigation. |
| -- | `on_click` | *(via spread / `onClick`)* | `Option<Callback<ev::MouseEvent>>` | An optional click handler composed with the internal click handler. React supports this via prop spreading on the underlying `Toggle`; Leptos exposes it as an explicit prop. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the underlying DOM element (`<button>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<button>`, merging props and refs. |
| *(spread)* | -- | `...Omit<ToggleProps, 'defaultPressed' \| 'onPressedChange'>` | -- | React allows spreading Toggle props (except pressed-state props, which are managed by the group). Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"on" \| "off"` | Whether this item is currently pressed. |
| `data-disabled` | `""` / absent | Present when the item is disabled (from item prop or inherited from group). |

### Implicit behavior

- **Pressed state is derived from group context.** The item reads the group's `value` context and considers itself pressed if `value.contains(item_value)`. There is no user-facing `pressed` prop — it is computed internally.
- **ARIA attributes are mode-dependent.** In single mode (`ToggleGroupType::Single`), the item renders `role="radio"` and `aria-checked` instead of `aria-pressed`. In multiple mode, the item renders `aria-pressed` with no special role. This is handled internally by `ToggleGroupItemImpl`.
- **Click handling toggles group state.** Clicking an unpressed item calls `on_item_activate(value)` on the group context; clicking a pressed item calls `on_item_deactivate(value)`. The group context handles updating the value vec according to the mode (single: replace/clear, multiple: append/filter).
- **Roving focus integration.** When the group has `roving_focus=true` (default), each item is wrapped in a `RovingFocusGroupItem` with `focusable=!disabled` and `active=pressed`. This makes the item participate in arrow-key navigation and ensures only the active/first-focusable item is in the tab order.
- **Disabled inheritance.** The effective disabled state is the logical OR of the group's `disabled` prop and the item's `disabled` prop: `context.disabled || item.disabled`.
