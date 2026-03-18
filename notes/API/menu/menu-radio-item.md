# MenuRadioItem

## React Signature

```typescript
const MenuRadioItem = React.forwardRef<MenuRadioItemElement, MenuRadioItemProps>(...)

type MenuRadioItemElement = React.ComponentRef<typeof MenuItem>;

interface MenuRadioItemProps extends MenuItemProps {
  value: string;
}
```

## Leptos Signature

```rust
pub fn MenuRadioItem(
    #[prop(into)] value: MaybeProp<String>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_select: Option<Callback<ev::Event>>,
    #[prop(into, optional)] text_value: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `value` | `value` | `string` (required) | `MaybeProp<String>` (required, `#[prop(into)]`) | The unique value of this radio item. Compared against the parent `MenuRadioGroup`'s `value` to determine checked state. |
| `disabled` | `disabled` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | When `true`, the item cannot be selected and is skipped during keyboard navigation. |
| `onSelect` | `on_select` | `(event: Event) => void` | `Option<Callback<ev::Event>>` | Called when the item is selected. The value change always fires regardless of `preventDefault()` (uses `checkForDefaultPrevented: false`). |
| `textValue` | `text_value` | `string` | `MaybeProp<String>` | Text for typeahead matching. Defaults to `.textContent`. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping. |
| *(spread)* | -- | `...MenuItemProps` | -- | React allows spreading MenuItem props. Leptos uses `attr:` directives. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"checked" \| "unchecked"` | Whether this radio item is the currently selected value in its group. |
| `data-highlighted` | `""` (present/absent) | Present when the item is focused. Inherited from `MenuItem`. |
| `data-disabled` | `""` (present/absent) | Present when the item is disabled. Inherited from `MenuItem`. |

### Implicit behavior

- Renders with `role="menuitemradio"`.
- Sets `aria-checked` to `"true"` or `"false"` based on whether this item's `value` matches the parent `MenuRadioGroup`'s `value`.
- Provides an `ItemIndicatorContextValue` to children so `MenuItemIndicator` can read the checked state.
- On select, calls the parent `RadioGroupContextValue.on_value_change` with this item's value, regardless of whether `onSelect` called `preventDefault()`.
