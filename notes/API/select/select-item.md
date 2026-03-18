# SelectItem

## React Signature

```typescript
const SelectItem = React.forwardRef<SelectItemElement, SelectItemProps>(...)

type SelectItemElement = React.ComponentRef<typeof Primitive.div>;

interface SelectItemProps extends PrimitiveDivProps {
  value: string;
  disabled?: boolean;
  textValue?: string;
}
```

## Leptos Signature

```rust
pub fn SelectItem(
    #[prop(into)] value: String,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] text_value: MaybeProp<String>,
    #[prop(into, optional)] on_pointer_up: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_down: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_move: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_leave: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_focus: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] on_blur: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `value` | `value` | `string` (required) | `String` (required, `#[prop(into)]`) | The value of this item. Must not be an empty string. This value is passed to `onValueChange` when the item is selected. |
| `disabled` | `disabled` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | Disables the item. Disabled items cannot be selected and are skipped during keyboard navigation. |
| `textValue` | `text_value` | `string` | `MaybeProp<String>` | Optional text used for typeahead matching. When omitted, the text content of `SelectItemText` is used automatically. Useful when the item content is not plain text (e.g., contains icons). |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | The item children (typically `SelectItemText` + optional `SelectItemIndicator`). |
| `onFocus` | `on_focus` | `(event) => void` | `Option<Callback<ev::FocusEvent>>` | Focus event handler, composed with internal focus tracking. |
| `onBlur` | `on_blur` | `(event) => void` | `Option<Callback<ev::FocusEvent>>` | Blur event handler, composed with internal focus tracking. |
| `onClick` | `on_click` | `(event) => void` | `Option<Callback<ev::MouseEvent>>` | Click event handler, composed with internal touch/pen selection logic. |
| `onPointerUp` | `on_pointer_up` | `(event) => void` | `Option<Callback<ev::PointerEvent>>` | Pointer up event handler, composed with internal mouse selection logic. |
| `onPointerDown` | `on_pointer_down` | `(event) => void` | `Option<Callback<ev::PointerEvent>>` | Pointer down event handler, composed with internal pointer type tracking. |
| `onPointerMove` | `on_pointer_move` | `(event) => void` | `Option<Callback<ev::PointerEvent>>` | Pointer move event handler, composed with internal mouse hover-focus logic. |
| `onPointerLeave` | `on_pointer_leave` | `(event) => void` | `Option<Callback<ev::PointerEvent>>` | Pointer leave event handler, composed with internal item-leave logic. |
| `onKeyDown` | `on_key_down` | `(event) => void` | `Option<Callback<ev::KeyboardEvent>>` | Key down event handler, composed with internal selection-key logic. |
| *(spread)* | -- | `...PrimitiveDivProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Registers itself with the collection system for typeahead and keyboard navigation.
- Auto-generates a `text_id` used by `SelectItemText` for the `aria-labelledby` relationship.
- Selection on mouse: item is selected on `pointerup` (allows click-drag-release pattern). On touch/pen: item is selected on `click`.
- When the pointer moves over an enabled item with `pointerType === "mouse"`, the item receives focus (hover-to-focus).
- When the pointer leaves a focused item, focus moves back to the content element.
- Enter or Space key selects the item and closes the select.
- Space key press is always prevented from scrolling the page.
- React throws if `value` is an empty string; Leptos does not enforce this at runtime.

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"checked" \| "unchecked"` | Whether this item is the currently selected item. |
| `data-highlighted` | `""` (present/absent) | Present when the item is focused (highlighted). Used for styling the focused item. |
| `data-disabled` | `""` (present/absent) | Present when the item is disabled. |
