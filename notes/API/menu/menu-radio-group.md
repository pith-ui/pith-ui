# MenuRadioGroup

## React Signature

```typescript
const MenuRadioGroup = React.forwardRef<MenuRadioGroupElement, MenuRadioGroupProps>(...)

type MenuRadioGroupElement = React.ComponentRef<typeof MenuGroup>;

interface MenuRadioGroupProps extends MenuGroupProps {
  value?: string;
  onValueChange?: (value: string) => void;
}
```

## Leptos Signature

```rust
pub fn MenuRadioGroup(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `value` | `value` | `string \| undefined` | `MaybeProp<String>` | The value of the currently selected radio item. |
| `onValueChange` | `on_value_change` | `(value: string) => void` | `Option<Callback<String>>` | Called when a radio item is selected. Receives the value of the newly selected item. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping. |
| *(spread)* | -- | `...MenuGroupProps` | -- | React allows spreading MenuGroup (div) props. Leptos uses `attr:` directives. |

### Implicit behavior

- Wraps `MenuGroup` (inheriting `role="group"`) and provides a `RadioGroupContextValue` to descendant `MenuRadioItem` components.
- The context carries the current `value` and the `on_value_change` callback, enabling radio items to determine their checked state and fire selection events.
