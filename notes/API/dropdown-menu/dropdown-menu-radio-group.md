# DropdownMenuRadioGroup

## React Signature

```typescript
const DropdownMenuRadioGroup = React.forwardRef<
  DropdownMenuRadioGroupElement,
  DropdownMenuRadioGroupProps
>(...)

type DropdownMenuRadioGroupElement = React.ComponentRef<typeof MenuPrimitive.RadioGroup>;
type MenuRadioGroupProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.RadioGroup>;

interface DropdownMenuRadioGroupProps extends MenuRadioGroupProps {}
```

`MenuRadioGroupProps` extends `MenuGroupProps` and adds:

```typescript
interface MenuRadioGroupProps extends MenuGroupProps {
  value?: string;
  onValueChange?: (value: string) => void;
}
```

## Leptos Signature

```rust
pub fn DropdownMenuRadioGroup(
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
| `value` | `value` | `string \| undefined` | `MaybeProp<String>` | The value of the currently selected radio item within the group. |
| `onValueChange` | `on_value_change` | `(value: string) => void` | `Option<Callback<String>>` | Called when a radio item in the group is selected, receiving the selected item's value. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| *(spread)* | -- | `...MenuGroupProps` | -- | React allows spreading group props (`<div>` attributes). Leptos uses `attr:` directives instead. |

### Implicit behavior

- Renders with `role="group"` (inherited from `MenuGroup`).
- Provides `RadioGroupContext` to child `DropdownMenuRadioItem` components so they can determine their checked state and fire value changes.
