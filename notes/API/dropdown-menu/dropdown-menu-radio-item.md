# DropdownMenuRadioItem

## React Signature

```typescript
const DropdownMenuRadioItem = React.forwardRef<
  DropdownMenuRadioItemElement,
  DropdownMenuRadioItemProps
>(...)

type DropdownMenuRadioItemElement = React.ComponentRef<typeof MenuPrimitive.RadioItem>;
type MenuRadioItemProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.RadioItem>;

interface DropdownMenuRadioItemProps extends MenuRadioItemProps {}
```

`MenuRadioItemProps` extends `MenuItemProps` and adds:

```typescript
interface MenuRadioItemProps extends MenuItemProps {
  value: string;
}
```

## Leptos Signature

```rust
pub fn DropdownMenuRadioItem(
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
| `value` | `value` | `string` (required) | `MaybeProp<String>` (required, `#[prop(into)]`) | The unique value identifying this radio item within the group. Compared against the parent `DropdownMenuRadioGroup`'s `value` to determine checked state. |
| `disabled` | `disabled` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | When `true`, the item cannot be selected and is skipped during keyboard navigation. |
| `onSelect` | `on_select` | `(event: Event) => void` | `Option<Callback<ev::Event>>` | Called when the item is selected. The value change fires regardless of `preventDefault()` (`checkForDefaultPrevented: false` in React). |
| `textValue` | `text_value` | `string \| undefined` | `MaybeProp<String>` | Optional text for typeahead. Defaults to the item's text content. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| *(spread)* | -- | `...MenuItemProps` | -- | React allows spreading menu item props. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"checked" \| "unchecked"` | Reflects whether this radio item is currently selected. |
| `data-highlighted` | `""` (present/absent) | Present when the item has focus. |
| `data-disabled` | `""` (present/absent) | Present when the item is disabled. |

### Implicit behavior

- Renders with `role="menuitemradio"`.
- Sets `aria-checked` based on whether this item's `value` matches the parent `RadioGroup`'s current `value`.
- Provides `ItemIndicatorContext` to child `DropdownMenuItemIndicator` components.
- On select, fires the parent `RadioGroup`'s `onValueChange` with this item's `value`.
