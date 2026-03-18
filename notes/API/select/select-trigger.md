# SelectTrigger

## React Signature

```typescript
const SelectTrigger = React.forwardRef<SelectTriggerElement, SelectTriggerProps>(...)

type SelectTriggerElement = React.ComponentRef<typeof Primitive.button>;
type PrimitiveButtonProps = React.ComponentPropsWithoutRef<typeof Primitive.button>;

interface SelectTriggerProps extends PrimitiveButtonProps {}
```

`SelectTriggerProps` inherits from `PrimitiveButtonProps` -- all standard `<button>` attributes are accepted via spread.

## Leptos Signature

```rust
pub fn SelectTrigger(
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] on_pointer_down: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `disabled` | `disabled` | `boolean` (default `false`) | `MaybeProp<bool>` | Disables the trigger. Effective disabled state is `select.disabled \|\| trigger.disabled`. When disabled, the trigger ignores clicks and keyboard interaction. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<button>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<button>`, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | Typically contains `SelectValue` and `SelectIcon`. |
| `onClick` | `on_click` | `(event) => void` | `Option<Callback<ev::MouseEvent>>` | Click event handler, composed with internal click handling (Safari label focus, touch/pen open). |
| `onPointerDown` | `on_pointer_down` | `(event) => void` | `Option<Callback<ev::PointerEvent>>` | Pointer down event handler, composed with internal mouse-button open logic. |
| `onKeyDown` | `on_key_down` | `(event) => void` | `Option<Callback<ev::KeyboardEvent>>` | Key down event handler, composed with internal typeahead and open-key handling. |
| *(spread)* | -- | `...PrimitiveButtonProps` | -- | React allows spreading any `<button>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Renders as a `<button type="button">` with `role="combobox"`.
- Sets `aria-controls` pointing to the content's auto-generated `id`.
- Sets `aria-expanded` reflecting the open state.
- Sets `aria-required` when the parent `Select` has `required=true`.
- Sets `aria-autocomplete="none"`.
- Sets `dir` reflecting the parent `Select`'s `dir`.
- Typeahead search on the trigger changes the selected value immediately (without opening).
- Opens on left mouse button pointer down, on touch/pen click, or on Enter/Space/ArrowUp/ArrowDown key press.
- Releases implicit pointer capture on pointer down (per W3C Pointer Events spec).

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"open" \| "closed"` | Reflects whether the select content is open. |
| `data-disabled` | `""` (present/absent) | Present when the trigger is effectively disabled. |
| `data-placeholder` | `""` (present/absent) | Present when no value is selected (the placeholder is showing). |
