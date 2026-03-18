# DropdownMenuItemIndicator

## React Signature

```typescript
const DropdownMenuItemIndicator = React.forwardRef<
  DropdownMenuItemIndicatorElement,
  DropdownMenuItemIndicatorProps
>(...)

type DropdownMenuItemIndicatorElement = React.ComponentRef<typeof MenuPrimitive.ItemIndicator>;
type MenuItemIndicatorProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.ItemIndicator>;

interface DropdownMenuItemIndicatorProps extends MenuItemIndicatorProps {}
```

`MenuItemIndicatorProps` extends `PrimitiveSpanProps` and adds:

```typescript
interface MenuItemIndicatorProps extends PrimitiveSpanProps {
  /** Used to force mounting when more control is needed. Useful when controlling animation with React animation libraries. */
  forceMount?: true;
}
```

## Leptos Signature

```rust
pub fn DropdownMenuItemIndicator(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` | Forces the indicator to stay mounted in the DOM even when the parent item is unchecked. Useful for animating the indicator in/out. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<span>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<span>`, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | The indicator content (e.g. a checkmark icon). Wrapped in `Option` in Leptos. |
| *(spread)* | -- | `...PrimitiveSpanProps` | -- | React allows spreading any `<span>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"checked" \| "unchecked" \| "indeterminate"` | Reflects the checked state of the parent `CheckboxItem` or `RadioItem`. |

### Implicit behavior

- Only renders when the parent `CheckboxItem` or `RadioItem` is checked or indeterminate (unless `forceMount` is set).
- Reads the checked state from `ItemIndicatorContext` provided by the parent `CheckboxItem` or `RadioItem`.
- Must be placed as a child of `DropdownMenuCheckboxItem` or `DropdownMenuRadioItem`.
