# ContextMenuRadioItem

## React Signature

```typescript
const ContextMenuRadioItem = React.forwardRef<
  ContextMenuRadioItemElement,
  ContextMenuRadioItemProps
>(...)

type ContextMenuRadioItemElement = React.ComponentRef<typeof MenuPrimitive.RadioItem>;
type MenuRadioItemProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.RadioItem>;

interface ContextMenuRadioItemProps extends MenuRadioItemProps {}

// MenuRadioItemProps:
interface MenuRadioItemProps extends MenuItemProps {
  value: string;
}
```

## Leptos Signature

```rust
pub fn ContextMenuRadioItem(
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
| `value` | `value` | `string` (required) | `String` (required, `#[prop(into)]`) | The unique value for this radio item. Compared against the parent `ContextMenuRadioGroup`'s `value` to determine checked state. |
| `disabled` | `disabled` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | When `true`, the item cannot be selected and is skipped during keyboard navigation. |
| `onSelect` | `on_select` | `(event: Event) => void` | `Option<Callback<ev::Event>>` | Called when the user selects the item. If `event.preventDefault()` is called, the menu will not close and the radio value will not change. |
| `textValue` | `text_value` | `string` | `MaybeProp<String>` | Text value for typeahead matching. Defaults to the element's text content. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| *(spread)* | — | `...MenuItemProps` | — | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"checked" \| "unchecked"` | Reflects whether this radio item is currently selected. |
| `data-highlighted` | `""` (present/absent) | Present when the item is focused/highlighted. |
| `data-disabled` | `""` (present/absent) | Present when the item is disabled. |

### Implicit behavior

- Renders with `role="menuitemradio"` and `aria-checked` reflecting whether this item's `value` matches the parent `ContextMenuRadioGroup`'s `value`.
- On select, calls the parent group's `onValueChange` with this item's `value`.
