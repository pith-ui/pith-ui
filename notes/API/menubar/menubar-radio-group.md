# MenubarRadioGroup

## React Signature

```typescript
const MenubarRadioGroup = React.forwardRef<MenubarRadioGroupElement, MenubarRadioGroupProps>(...)

type MenubarRadioGroupElement = React.ComponentRef<typeof MenuPrimitive.RadioGroup>;
type MenuRadioGroupProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.RadioGroup>;

interface MenubarRadioGroupProps extends MenuRadioGroupProps {}

// Where MenuRadioGroupProps extends:
interface MenuRadioGroupProps extends MenuGroupProps {
  value?: string;
  onValueChange?: (value: string) => void;
}
```

## Leptos Signature

```rust
pub fn MenubarRadioGroup(
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
| `value` | `value` | `string \| undefined` | `MaybeProp<String>` | The value of the currently selected radio item in the group. |
| `onValueChange` | `on_value_change` | `(value: string) => void` | `Option<Callback<String>>` | Called when the selected radio item changes. Receives the new value. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>` with `role="group"`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| *(spread)* | -- | `...MenuGroupProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Renders as a `<div>` with `role="group"`.
- Provides context to child `MenubarRadioItem` components with the current value and change handler.
- Pure pass-through to the underlying `MenuRadioGroup` component.
