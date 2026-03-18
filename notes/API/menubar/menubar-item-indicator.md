# MenubarItemIndicator

## React Signature

```typescript
const MenubarItemIndicator = React.forwardRef<MenubarItemIndicatorElement, MenubarItemIndicatorProps>(...)

type MenubarItemIndicatorElement = React.ComponentRef<typeof MenuPrimitive.ItemIndicator>;
type MenuItemIndicatorProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.ItemIndicator>;

interface MenubarItemIndicatorProps extends MenuItemIndicatorProps {}

// Where MenuItemIndicatorProps extends:
type PrimitiveSpanProps = React.ComponentPropsWithoutRef<typeof Primitive.span>;
interface MenuItemIndicatorProps extends PrimitiveSpanProps {
  /** Used to force mounting when more control is needed. Useful when controlling animation with React animation libraries. */
  forceMount?: true;
}
```

## Leptos Signature

```rust
pub fn MenubarItemIndicator(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` | Forces the indicator to stay mounted in the DOM even when the item is unchecked. Useful for animating the indicator in/out. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<span>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<span>`, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | The indicator content (typically a check/dot icon). Leptos wraps in `Option` to allow an empty indicator. |
| *(spread)* | -- | `...PrimitiveSpanProps` | -- | React allows spreading any `<span>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"checked" \| "unchecked" \| "indeterminate"` | Reflects the checked state of the parent checkbox/radio item. |

### Implicit behavior

- Only renders (is present in the DOM) when the parent `MenubarCheckboxItem` or `MenubarRadioItem` is in a checked or indeterminate state, unless `forceMount` is set.
- Pure pass-through to the underlying `MenuItemIndicator` component.
