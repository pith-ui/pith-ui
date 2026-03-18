# DropdownMenuSubTrigger

## React Signature

```typescript
const DropdownMenuSubTrigger = React.forwardRef<
  DropdownMenuSubTriggerElement,
  DropdownMenuSubTriggerProps
>(...)

type DropdownMenuSubTriggerElement = React.ComponentRef<typeof MenuPrimitive.SubTrigger>;
type MenuSubTriggerProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.SubTrigger>;

interface DropdownMenuSubTriggerProps extends MenuSubTriggerProps {}
```

`MenuSubTriggerProps` extends `MenuItemImplProps`:

```typescript
interface MenuItemImplProps extends PrimitiveDivProps {
  disabled?: boolean;
  textValue?: string;
}
```

## Leptos Signature

```rust
pub fn DropdownMenuSubTrigger(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] text_value: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `disabled` | `disabled` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | When `true`, the sub-trigger cannot be interacted with and the submenu cannot be opened. |
| `textValue` | `text_value` | `string \| undefined` | `MaybeProp<String>` | Optional text for typeahead search. Defaults to the item's text content. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| *(spread)* | -- | `...MenuItemImplProps` | -- | React allows spreading menu item impl props. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"open" \| "closed"` | Reflects the submenu's open state. |
| `data-highlighted` | `""` (present/absent) | Present when the item has focus. |
| `data-disabled` | `""` (present/absent) | Present when the item is disabled. |

### Implicit behavior

- Renders as a `MenuItemImpl` wrapped in a `MenuAnchor`, so the Popper positions the submenu content relative to it.
- Sets `aria-haspopup="menu"`, `aria-expanded`, and `aria-controls` pointing to the sub-content's auto-generated `id`.
- Opens the submenu on pointer hover (with a 100ms delay) or on `ArrowRight` (LTR) / `ArrowLeft` (RTL) / `Enter` / `Space`.
- Clicking the sub-trigger also opens the submenu and focuses it.
- Uses a pointer "grace area" to prevent accidental closure when moving the pointer diagonally from the sub-trigger to the sub-content.
