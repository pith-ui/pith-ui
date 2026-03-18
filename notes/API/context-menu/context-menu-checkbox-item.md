# ContextMenuCheckboxItem

## React Signature

```typescript
const ContextMenuCheckboxItem = React.forwardRef<
  ContextMenuCheckboxItemElement,
  ContextMenuCheckboxItemProps
>(...)

type ContextMenuCheckboxItemElement = React.ComponentRef<typeof MenuPrimitive.CheckboxItem>;
type MenuCheckboxItemProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.CheckboxItem>;

interface ContextMenuCheckboxItemProps extends MenuCheckboxItemProps {}

// MenuCheckboxItemProps:
interface MenuCheckboxItemProps extends MenuItemProps {
  checked?: CheckedState; // boolean | 'indeterminate'
  onCheckedChange?: (checked: boolean) => void;
}
```

## Leptos Signature

```rust
pub fn ContextMenuCheckboxItem(
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
| `checked` | `checked` | `CheckedState` (default `false`) | `MaybeProp<CheckedState>` | The controlled checked state. Can be `true`, `false`, or `'indeterminate'` (React) / `CheckedState::True`, `CheckedState::False`, `CheckedState::Indeterminate` (Leptos). |
| `onCheckedChange` | `on_checked_change` | `(checked: boolean) => void` | `Option<Callback<bool>>` | Called when the checked state changes. Receives the new boolean value. Note: the callback never receives `'indeterminate'` — selecting an indeterminate item toggles it to `true`. |
| `disabled` | `disabled` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | When `true`, the item cannot be selected and is skipped during keyboard navigation. |
| `onSelect` | `on_select` | `(event: Event) => void` | `Option<Callback<ev::Event>>` | Called when the user selects the item. If `event.preventDefault()` is called, the menu will not close and the checked state will not toggle. |
| `textValue` | `text_value` | `string` | `MaybeProp<String>` | Text value for typeahead matching. Defaults to the element's text content. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| *(spread)* | — | `...MenuItemProps` | — | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"checked" \| "unchecked" \| "indeterminate"` | Reflects the current checked state. |
| `data-highlighted` | `""` (present/absent) | Present when the item is focused/highlighted. |
| `data-disabled` | `""` (present/absent) | Present when the item is disabled. |

### Implicit behavior

- Renders with `role="menuitemcheckbox"` and `aria-checked` reflecting the checked state.
- On select, toggles the checked state and fires `onCheckedChange`. If `checked` is `'indeterminate'`, selecting it sets it to `true`.
