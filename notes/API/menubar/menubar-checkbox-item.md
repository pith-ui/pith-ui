# MenubarCheckboxItem

## React Signature

```typescript
const MenubarCheckboxItem = React.forwardRef<MenubarCheckboxItemElement, MenubarCheckboxItemProps>(...)

type MenubarCheckboxItemElement = React.ComponentRef<typeof MenuPrimitive.CheckboxItem>;
type MenuCheckboxItemProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.CheckboxItem>;

interface MenubarCheckboxItemProps extends MenuCheckboxItemProps {}

// Where MenuCheckboxItemProps extends:
type CheckedState = boolean | 'indeterminate';

interface MenuCheckboxItemProps extends MenuItemProps {
  checked?: CheckedState;
  onCheckedChange?: (checked: boolean) => void;
}
```

## Leptos Signature

```rust
pub fn MenubarCheckboxItem(
    #[prop(into, optional)] checked: MaybeProp<CheckedState>,
    #[prop(into, optional)] on_checked_change: Option<Callback<bool>>,
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
| `checked` | `checked` | `boolean \| 'indeterminate'` | `MaybeProp<CheckedState>` | The controlled checked state. Can be `true`, `false`, or `indeterminate`. |
| `onCheckedChange` | `on_checked_change` | `(checked: boolean) => void` | `Option<Callback<bool>>` | Called when the checked state changes. Note: `onCheckedChange` can never receive `'indeterminate'` from internal toggling -- only `true` or `false`. |
| `disabled` | `disabled` | `boolean` (default `false`) | `MaybeProp<bool>` | Disables the item. |
| `onSelect` | `on_select` | `(event: Event) => void` | `Option<Callback<ev::Event>>` | Called when the item is selected. Call `event.preventDefault()` to prevent the menu from closing. |
| `textValue` | `text_value` | `string` | `MaybeProp<String>` | Text used for typeahead matching. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element. |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly, merging props and refs. |
| *(spread)* | -- | `...MenuItemProps` | -- | React allows spreading additional props. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"checked" \| "unchecked" \| "indeterminate"` | Reflects the current checked state. |
| `data-disabled` | `""` (present/absent) | Present when the item is disabled. |
| `data-highlighted` | `""` (present/absent) | Present when the item has visual focus. |

### Implicit behavior

- Renders with `role="menuitemcheckbox"` and `aria-checked` reflecting the checked state.
- Selecting the item toggles the checked state and calls `onCheckedChange`.
- Pure pass-through to the underlying `MenuCheckboxItem` component.
