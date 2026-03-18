# RadioGroupItem

## React Signature

```typescript
const RadioGroupItem = React.forwardRef<RadioGroupItemElement, RadioGroupItemProps>(...)

type RadioGroupItemElement = React.ComponentRef<typeof Radio>;
type RadioProps = React.ComponentPropsWithoutRef<typeof Radio>;

interface RadioGroupItemProps extends Omit<RadioProps, 'onCheck' | 'name'> {
  value: string;
}

// Where Radio extends:
interface RadioProps extends PrimitiveButtonProps {
  checked?: boolean;
  required?: boolean;
  onCheck?(): void;
}
```

`RadioGroupItemProps` inherits from `PrimitiveButtonProps` (via `Radio`) with `onCheck` and `name` omitted — those are managed internally by the radio group context.

## Leptos Signature

```rust
pub fn RadioGroupItem(
    #[prop(into)] value: String,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `value` | `value` | `string` (required) | `String` (required, `into`) | A unique string identifying this radio item within the group. This value is passed to `onValueChange` when the item is selected and compared against the group's `value` to determine checked state. Accepts `&str` via `#[prop(into)]`. |
| `disabled` | `disabled` | `boolean \| undefined` | `MaybeProp<bool>` | Disables this specific item. The effective disabled state is `group.disabled \|\| item.disabled`. When disabled, the item cannot be clicked, is skipped during keyboard navigation, and the native `disabled` attribute is set on the `<button>`. |
| — | `on_click` | *(via spread)* | `Option<Callback<ev::MouseEvent>>` | Optional click handler composed with the internal click logic. In React, this is passed via prop spread on the underlying `<button>`. Leptos exposes it as an explicit prop. If the handler calls `stopPropagation()`, the hidden form input's change event will also not bubble. |
| `required` | — | `boolean \| undefined` | — | In React, `required` can be set per-item (inherited from `RadioProps`). In Leptos, `required` is only set on the `RadioGroup` root and propagated to all items via context. |
| `checked` | — | `boolean` | — | In React, `checked` is available on the underlying `Radio` component. In both React and Leptos, checked state is managed internally by the radio group context (comparing `item.value` against `group.value`). Not exposed as a user-facing prop on `RadioGroupItem`. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<button>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<button>`, merging props and refs. The child must be a focusable, clickable element for accessibility. |
| *(spread)* | — | `...Omit<RadioProps, 'onCheck' \| 'name'>` | — | React allows spreading any `<button>` HTML attribute (except `onCheck` and `name`, which are managed internally). Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"checked" \| "unchecked"` | Whether this item is the currently selected value in the group. |
| `data-disabled` | `""` (present/absent) | Present when the item is disabled (either directly or via the group's `disabled` prop). |

### Implicit behavior

- Renders as a `<button>` with `type="button"` and `role="radio"`.
- Sets `aria-checked` to `"true"` or `"false"` based on whether `item.value === group.value`.
- Sets the native `disabled` attribute on the `<button>` when disabled.
- Wrapped in a `RovingFocusGroupItem` that manages tabindex and keyboard navigation. The item is `focusable` when not disabled and `active` when checked.
- When an arrow key is pressed and focus moves to this item, the item is automatically clicked (programmatic `element.click()`) to check it and fire the group's `onValueChange`. This implements the WAI-ARIA pattern where arrow keys both move focus and select.
- The `Enter` key is explicitly prevented (`event.preventDefault()`) per WAI-ARIA — radio groups do not activate on Enter.
- When the radio group is inside a `<form>` (or has a `form` prop), a hidden `<input type="radio">` is rendered alongside the button. This input mirrors the button's state (`checked`, `disabled`, `required`, `name`, `value`, `form`) for native form participation. The input is visually hidden, absolutely positioned, and has `aria-hidden`, `tabindex="-1"`, and `pointer-events: none`.
- Radios cannot be unchecked by clicking — `onCheck` only fires when the item is not already checked.
