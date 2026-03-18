# DropdownMenuItem

## React Signature

```typescript
const DropdownMenuItem = React.forwardRef<DropdownMenuItemElement, DropdownMenuItemProps>(...)

type DropdownMenuItemElement = React.ComponentRef<typeof MenuPrimitive.Item>;
type MenuItemProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.Item>;

interface DropdownMenuItemProps extends MenuItemProps {}
```

`MenuItemProps` extends `MenuItemImplProps` (minus `onSelect`) and adds:

```typescript
interface MenuItemProps extends Omit<MenuItemImplProps, 'onSelect'> {
  onSelect?: (event: Event) => void;
}

interface MenuItemImplProps extends PrimitiveDivProps {
  disabled?: boolean;
  textValue?: string;
}
```

## Leptos Signature

```rust
pub fn DropdownMenuItem(
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
| `disabled` | `disabled` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | When `true`, the item cannot be selected and is skipped during keyboard navigation. |
| `onSelect` | `on_select` | `(event: Event) => void` | `Option<Callback<ev::Event>>` | Called when the item is selected (via click, `Enter`, or `Space`). Call `event.preventDefault()` to prevent the menu from closing after selection. |
| `textValue` | `text_value` | `string \| undefined` | `MaybeProp<String>` | An optional text value used for typeahead search. By default, the item's text content is used. Set this explicitly when the item contains non-text content (icons, images) or when you want typeahead to match a different string. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| *(spread)* | -- | `...PrimitiveDivProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-highlighted` | `""` (present/absent) | Present when the item has focus (keyboard or pointer). |
| `data-disabled` | `""` (present/absent) | Present when the item is disabled. |

### Implicit behavior

- Renders with `role="menuitem"`.
- Sets `aria-disabled` when disabled.
- Participates in roving focus group for keyboard navigation.
- On pointer move (mouse), the item receives focus. On pointer leave, focus returns to the content container.
- On `Enter` or `Space`, triggers a click which fires `onSelect`. If `onSelect` does not call `preventDefault()`, the menu closes.
- The text content is automatically extracted for typeahead unless `textValue` is explicitly provided.
