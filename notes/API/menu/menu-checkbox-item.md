# MenuCheckboxItem

## React Signature

```typescript
const MenuCheckboxItem = React.forwardRef<MenuCheckboxItemElement, MenuCheckboxItemProps>(...)

type MenuCheckboxItemElement = MenuItemElement;

type CheckedState = boolean | 'indeterminate';

interface MenuCheckboxItemProps extends MenuItemProps {
  checked?: CheckedState;
  // `onCheckedChange` can never be called with `"indeterminate"` from the inside
  onCheckedChange?: (checked: boolean) => void;
}
```

## Leptos Signature

```rust
pub fn MenuCheckboxItem(
    #[prop(into, optional)] checked: MaybeProp<CheckedState>,
    #[prop(into, optional)] on_checked_change: Option<Callback<bool>>,
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
| `checked` | `checked` | `boolean \| 'indeterminate'` (default `false`) | `MaybeProp<CheckedState>` (default `CheckedState::False`) | The controlled checked state. Supports three states: `true`/`CheckedState::True`, `false`/`CheckedState::False`, and `'indeterminate'`/`CheckedState::Indeterminate`. |
| `onCheckedChange` | `on_checked_change` | `(checked: boolean) => void` | `Option<Callback<bool>>` | Called when the checked state changes. Always receives a `boolean` — when toggling from indeterminate, it resolves to `true`. |
| `disabled` | `disabled` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | When `true`, the item cannot be selected and is skipped during keyboard navigation. |
| `onSelect` | `on_select` | `(event: Event) => void` | `Option<Callback<ev::Event>>` | Called when the item is selected. The checked toggle always fires regardless of `preventDefault()` (React uses `checkForDefaultPrevented: false`). |
| `textValue` | `text_value` | `string` | `MaybeProp<String>` | Text for typeahead matching. Defaults to `.textContent`. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping. |
| *(spread)* | -- | `...MenuItemProps` | -- | React allows spreading MenuItem props. Leptos uses `attr:` directives. |

### Leptos-only: `CheckedState` enum

```rust
pub enum CheckedState {
    False,
    True,
    Indeterminate,
}

impl From<bool> for CheckedState { ... }
```

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"checked" \| "unchecked" \| "indeterminate"` | Reflects the current checked state. |
| `data-highlighted` | `""` (present/absent) | Present when the item is focused. Inherited from `MenuItem`. |
| `data-disabled` | `""` (present/absent) | Present when the item is disabled. Inherited from `MenuItem`. |

### Implicit behavior

- Renders with `role="menuitemcheckbox"`.
- Sets `aria-checked` to `"true"`, `"false"`, or `"mixed"` (for indeterminate).
- Provides an `ItemIndicatorContextValue` to children so `MenuItemIndicator` can read the checked state.
- The toggle always fires on select, even if `onSelect` calls `preventDefault()`. This matches the React behavior where `checkForDefaultPrevented: false` is used.
