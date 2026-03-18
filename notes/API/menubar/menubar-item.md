# MenubarItem

## React Signature

```typescript
const MenubarItem = React.forwardRef<MenubarItemElement, MenubarItemProps>(...)

type MenubarItemElement = React.ComponentRef<typeof MenuPrimitive.Item>;
type MenuItemProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.Item>;

interface MenubarItemProps extends MenuItemProps {}

// Where MenuItemProps extends:
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
pub fn MenubarItem(
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
| `disabled` | `disabled` | `boolean` (default `false`) | `MaybeProp<bool>` | Disables the item. When disabled, it cannot be selected, is skipped during keyboard navigation, and receives `data-disabled`. |
| `onSelect` | `on_select` | `(event: Event) => void` | `Option<Callback<ev::Event>>` | Called when the item is selected (via click or `Enter`/`Space`). Call `event.preventDefault()` to prevent the menu from closing after selection. |
| `textValue` | `text_value` | `string` | `MaybeProp<String>` | Text used for typeahead matching. By default, the text content of the item is used. Override this when the item contains non-text content or when you want typeahead to match differently. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>` with `role="menuitem"`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| *(spread)* | -- | `...PrimitiveDivProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-disabled` | `""` (present/absent) | Present when the item is disabled. |
| `data-highlighted` | `""` (present/absent) | Present when the item has visual focus (highlighted via keyboard or pointer). |

### Implicit behavior

- Renders as a `<div>` with `role="menuitem"`.
- Participates in the menu's roving focus and typeahead systems.
- Selecting an item dispatches a custom `menu.itemSelect` event and closes the menu (unless `event.preventDefault()` is called in `onSelect`).
- Pure pass-through to the underlying `MenuItem` component.
