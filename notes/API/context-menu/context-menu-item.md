# ContextMenuItem

## React Signature

```typescript
const ContextMenuItem = React.forwardRef<ContextMenuItemElement, ContextMenuItemProps>(...)

type ContextMenuItemElement = React.ComponentRef<typeof MenuPrimitive.Item>;
type MenuItemProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.Item>;

interface ContextMenuItemProps extends MenuItemProps {}

// MenuItemProps:
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
pub fn ContextMenuItem(
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
| `disabled` | `disabled` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | When `true`, the item cannot be selected and is skipped during keyboard navigation. Renders with `aria-disabled` and `data-disabled`. |
| `onSelect` | `on_select` | `(event: Event) => void` | `Option<Callback<ev::Event>>` | Called when the user selects the item (via click or keyboard). If `event.preventDefault()` is called, the menu will not close after selection. |
| `textValue` | `text_value` | `string` | `MaybeProp<String>` | An optional text value used for typeahead matching. By default, the typeahead system uses the item's text content. Set this when the content is not plain text (e.g., contains icons) or when you want different typeahead behavior. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| *(spread)* | — | `...PrimitiveDivProps` | — | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-highlighted` | `""` (present/absent) | Present when the item is focused/highlighted via keyboard navigation or pointer hover. |
| `data-disabled` | `""` (present/absent) | Present when the item is disabled. |

### Implicit behavior

- Renders with `role="menuitem"`.
- When the item is focused, it is scrolled into view within the content viewport.
- The `textValue` (or the element's text content) is collected for typeahead: typing characters while the menu is open moves focus to the first matching item.
