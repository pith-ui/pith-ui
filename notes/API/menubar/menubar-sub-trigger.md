# MenubarSubTrigger

## React Signature

```typescript
const MenubarSubTrigger = React.forwardRef<MenubarSubTriggerElement, MenubarSubTriggerProps>(...)

type MenubarSubTriggerElement = React.ComponentRef<typeof MenuPrimitive.SubTrigger>;
type MenuSubTriggerProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.SubTrigger>;

interface MenubarSubTriggerProps extends MenuSubTriggerProps {}

// Where MenuSubTriggerProps extends:
interface MenuSubTriggerProps extends MenuItemImplProps {}

interface MenuItemImplProps extends PrimitiveDivProps {
  disabled?: boolean;
  textValue?: string;
}
```

## Leptos Signature

```rust
pub fn MenubarSubTrigger(
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
| `disabled` | `disabled` | `boolean` (default `false`) | `MaybeProp<bool>` | Disables the sub-trigger. When disabled, the submenu cannot be opened. |
| `textValue` | `text_value` | `string` | `MaybeProp<String>` | Text used for typeahead matching. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| *(spread)* | -- | `...MenuItemImplProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"open" \| "closed"` | Whether the associated submenu is currently open. |
| `data-disabled` | `""` (present/absent) | Present when the sub-trigger is disabled. |
| `data-highlighted` | `""` (present/absent) | Present when the sub-trigger has visual focus. |
| `data-radix-menubar-subtrigger` | `""` | Marker attribute used internally by `MenubarContent` to detect sub-triggers during arrow-key navigation. |

### Implicit behavior

- Renders as a `<div>` with `role="menuitem"`, `aria-haspopup="menu"`, `aria-expanded`, and `aria-controls`.
- Opens the submenu on pointer hover (after a delay) and via the "open" arrow key (`ArrowRight` in LTR, `ArrowLeft` in RTL) or `Enter`/`Space`.
- The `data-radix-menubar-subtrigger` marker is set by `MenubarSubTrigger` to allow `MenubarContent`'s key-down handler to distinguish sub-triggers from regular items when deciding whether to navigate between menus.
