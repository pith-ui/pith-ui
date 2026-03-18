# MenuItem

## React Signature

```typescript
const MenuItem = React.forwardRef<MenuItemElement, MenuItemProps>(...)

type MenuItemElement = MenuItemImplElement; // = Primitive.div element

interface MenuItemProps extends Omit<MenuItemImplProps, 'onSelect'> {
  onSelect?: (event: Event) => void;
}

// Internal base:
interface MenuItemImplProps extends PrimitiveDivProps {
  disabled?: boolean;
  textValue?: string;
}
```

## Leptos Signature

```rust
pub fn MenuItem(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_select: Option<Callback<ev::Event>>,
    #[prop(into, optional)] text_value: MaybeProp<String>,
    #[prop(into, optional)] role: MaybeProp<String>,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] on_pointer_down: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_up: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `disabled` | `disabled` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | When `true`, the item cannot be selected and is skipped during keyboard navigation. Renders `aria-disabled="true"` and `data-disabled`. |
| `onSelect` | `on_select` | `(event: Event) => void` | `Option<Callback<ev::Event>>` | Called when the item is selected (via click or `Enter`/`Space`). Receives a cancelable custom event — call `event.preventDefault()` to prevent the menu from closing after selection. |
| `textValue` | `text_value` | `string` | `MaybeProp<String>` | The text used for typeahead matching. Defaults to the element's `.textContent`. Provide explicitly when the item contains complex children (icons, badges) that would produce an unhelpful text content string. |
| -- | `role` | (always `"menuitem"`) | `MaybeProp<String>` | Leptos exposes the role as a prop so `MenuCheckboxItem` and `MenuRadioItem` can override it. Defaults to `"menuitem"`. Not typically set by end users. |
| `onClick` | `on_click` | `(event: MouseEvent) => void` | `Option<Callback<ev::MouseEvent>>` | Click handler, composed with the internal selection handler. |
| `onPointerDown` | `on_pointer_down` | `(event: PointerEvent) => void` | `Option<Callback<ev::PointerEvent>>` | Pointer down handler. |
| `onPointerUp` | `on_pointer_up` | `(event: PointerEvent) => void` | `Option<Callback<ev::PointerEvent>>` | Pointer up handler, composed with internal click dispatch for cross-item pointer drags. |
| `onKeyDown` | `on_key_down` | `(event: KeyboardEvent) => void` | `Option<Callback<ev::KeyboardEvent>>` | Keydown handler, composed with internal `Enter`/`Space` selection logic. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`. |
| *(spread)* | -- | `...PrimitiveDivProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-highlighted` | `""` (present/absent) | Present when the item is focused (via keyboard or pointer). |
| `data-disabled` | `""` (present/absent) | Present when the item is disabled. |

### Implicit behavior

- Renders with `role="menuitem"` by default (overridden to `"menuitemcheckbox"` or `"menuitemradio"` by checkbox/radio item variants).
- Participates in the roving focus group — focusable when not disabled.
- On `pointermove` (mouse only), the item gains focus to match native menu highlighting behavior. Disabled items trigger `onItemLeave` instead.
- On `pointerleave`, focus returns to the content container.
- `Enter` and `Space` trigger a click (and thus `onSelect`). During typeahead (search buffer non-empty), `Space` is suppressed to avoid accidental selection.
- If `onSelect` does not call `preventDefault()`, the entire menu tree closes after selection.
- Pointer down followed by pointer up on a *different* item dispatches a click on the new item, enabling drag-to-select behavior.
