# DropdownMenuCheckboxItem

## React Signature

```typescript
const DropdownMenuCheckboxItem = React.forwardRef<
  DropdownMenuCheckboxItemElement,
  DropdownMenuCheckboxItemProps
>(...)

type DropdownMenuCheckboxItemElement = React.ComponentRef<typeof MenuPrimitive.CheckboxItem>;
type MenuCheckboxItemProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.CheckboxItem>;

interface DropdownMenuCheckboxItemProps extends MenuCheckboxItemProps {}
```

`MenuCheckboxItemProps` extends `MenuItemProps` and adds:

```typescript
type CheckedState = boolean | 'indeterminate';

interface MenuCheckboxItemProps extends MenuItemProps {
  checked?: CheckedState; // default false
  onCheckedChange?: (checked: boolean) => void;
}
```

## Leptos Signature

```rust
pub fn DropdownMenuCheckboxItem(
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
| `checked` | `checked` | `CheckedState` (default `false`) | `MaybeProp<CheckedState>` (default `False`) | The controlled checked state. Can be `true`, `false`, or `'indeterminate'` (React) / `CheckedState::True`, `CheckedState::False`, `CheckedState::Indeterminate` (Leptos). |
| `onCheckedChange` | `on_checked_change` | `(checked: boolean) => void` | `Option<Callback<bool>>` | Called when the checked state changes. Note: `onCheckedChange` is never called with `'indeterminate'` -- selecting an indeterminate item toggles it to `true`. |
| `disabled` | `disabled` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | When `true`, the item cannot be selected and is skipped during keyboard navigation. |
| `onSelect` | `on_select` | `(event: Event) => void` | `Option<Callback<ev::Event>>` | Called when the item is selected. The toggle fires regardless of `preventDefault()` (`checkForDefaultPrevented: false` in React). |
| `textValue` | `text_value` | `string \| undefined` | `MaybeProp<String>` | Optional text for typeahead. Defaults to the item's text content. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| *(spread)* | -- | `...MenuItemProps` | -- | React allows spreading menu item props. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"checked" \| "unchecked" \| "indeterminate"` | Reflects the current checked state. |
| `data-highlighted` | `""` (present/absent) | Present when the item has focus. |
| `data-disabled` | `""` (present/absent) | Present when the item is disabled. |

### Implicit behavior

- Renders with `role="menuitemcheckbox"`.
- Sets `aria-checked` to `true`, `false`, or `"mixed"` (for indeterminate).
- Provides `ItemIndicatorContext` to child `DropdownMenuItemIndicator` components so they can respond to the checked state.
- On select, toggles the checked state: `indeterminate` becomes `true`, `true` becomes `false`, `false` becomes `true`.
