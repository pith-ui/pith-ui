# MenubarRadioItem

## React Signature

```typescript
const MenubarRadioItem = React.forwardRef<MenubarRadioItemElement, MenubarRadioItemProps>(...)

type MenubarRadioItemElement = React.ComponentRef<typeof MenuPrimitive.RadioItem>;
type MenuRadioItemProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.RadioItem>;

interface MenubarRadioItemProps extends MenuRadioItemProps {}

// Where MenuRadioItemProps extends:
interface MenuRadioItemProps extends MenuItemProps {
  value: string;
}
```

## Leptos Signature

```rust
pub fn MenubarRadioItem(
    #[prop(into)] value: String,
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
| `value` | `value` | `string` (required) | `String` (required, `#[prop(into)]`) | The unique value for this radio item. Compared against the parent `MenubarRadioGroup`'s value to determine if this item is checked. |
| `disabled` | `disabled` | `boolean` (default `false`) | `MaybeProp<bool>` | Disables the item. |
| `onSelect` | `on_select` | `(event: Event) => void` | `Option<Callback<ev::Event>>` | Called when the item is selected. Call `event.preventDefault()` to prevent the menu from closing. |
| `textValue` | `text_value` | `string` | `MaybeProp<String>` | Text used for typeahead matching. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element. |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly, merging props and refs. |
| *(spread)* | -- | `...MenuItemProps` | -- | React allows spreading additional props. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"checked" \| "unchecked"` | Reflects whether this item is the currently selected radio item. |
| `data-disabled` | `""` (present/absent) | Present when the item is disabled. |
| `data-highlighted` | `""` (present/absent) | Present when the item has visual focus. |

### Implicit behavior

- Renders with `role="menuitemradio"` and `aria-checked` reflecting whether this item's `value` matches the parent `MenubarRadioGroup`'s value.
- Selecting the item sets the parent radio group's value and calls `onValueChange`.
- Pure pass-through to the underlying `MenuRadioItem` component.
