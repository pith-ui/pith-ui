# ContextMenuRadioGroup

## React Signature

```typescript
const ContextMenuRadioGroup = React.forwardRef<
  ContextMenuRadioGroupElement,
  ContextMenuRadioGroupProps
>(...)

type ContextMenuRadioGroupElement = React.ComponentRef<typeof MenuPrimitive.RadioGroup>;
type MenuRadioGroupProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.RadioGroup>;

interface ContextMenuRadioGroupProps extends MenuRadioGroupProps {}

// MenuRadioGroupProps:
interface MenuRadioGroupProps extends MenuGroupProps {
  value?: string;
  onValueChange?: (value: string) => void;
}
```

## Leptos Signature

```rust
pub fn ContextMenuRadioGroup(
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
| `value` | `value` | `string` | `MaybeProp<String>` | The value of the currently selected radio item in this group. |
| `onValueChange` | `on_value_change` | `(value: string) => void` | `Option<Callback<String>>` | Called when a radio item in the group is selected. Receives the value of the newly selected item. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| *(spread)* | — | `...MenuGroupProps` | — | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Renders with `role="group"`, same as `ContextMenuGroup`.
- Provides context to child `ContextMenuRadioItem` components so they can determine their checked state by comparing their `value` against the group's `value`.
