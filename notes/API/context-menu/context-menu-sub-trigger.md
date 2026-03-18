# ContextMenuSubTrigger

## React Signature

```typescript
const ContextMenuSubTrigger = React.forwardRef<
  ContextMenuSubTriggerElement,
  ContextMenuSubTriggerProps
>(...)

type ContextMenuSubTriggerElement = React.ComponentRef<typeof MenuPrimitive.SubTrigger>;
type MenuSubTriggerProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.SubTrigger>;

interface ContextMenuSubTriggerProps extends MenuSubTriggerProps {}

// MenuSubTriggerProps extends MenuItemImplProps:
interface MenuItemImplProps extends PrimitiveDivProps {
  disabled?: boolean;
  textValue?: string;
}
```

## Leptos Signature

```rust
pub fn ContextMenuSubTrigger(
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
| `disabled` | `disabled` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | When `true`, the sub-trigger cannot be activated and is skipped during keyboard navigation. The submenu cannot be opened. |
| `textValue` | `text_value` | `string` | `MaybeProp<String>` | Text value for typeahead matching. Defaults to the element's text content. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| *(spread)* | — | `...PrimitiveDivProps` | — | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"open" \| "closed"` | Reflects whether the associated submenu is currently open. |
| `data-highlighted` | `""` (present/absent) | Present when the trigger is focused/highlighted. |
| `data-disabled` | `""` (present/absent) | Present when the trigger is disabled. |

### Implicit behavior

- Renders with `role="menuitem"`, `aria-haspopup="menu"`, `aria-expanded`, and `aria-controls` pointing to the submenu content.
- Hovering over the trigger opens the submenu after a short delay. Moving the pointer away closes it (with a grace area to allow diagonal pointer movement toward the submenu).
- Pressing `ArrowRight` (LTR) or `ArrowLeft` (RTL) opens the submenu and focuses the first item.
- Clicking the trigger also opens the submenu.
