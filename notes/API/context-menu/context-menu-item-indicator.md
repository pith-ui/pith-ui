# ContextMenuItemIndicator

## React Signature

```typescript
const ContextMenuItemIndicator = React.forwardRef<
  ContextMenuItemIndicatorElement,
  ContextMenuItemIndicatorProps
>(...)

type ContextMenuItemIndicatorElement = React.ComponentRef<typeof MenuPrimitive.ItemIndicator>;
type MenuItemIndicatorProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.ItemIndicator>;

interface ContextMenuItemIndicatorProps extends MenuItemIndicatorProps {}

// MenuItemIndicatorProps:
interface MenuItemIndicatorProps extends PrimitiveSpanProps {
  /** Used to force mounting when more control is needed. */
  forceMount?: true;
}
```

## Leptos Signature

```rust
pub fn ContextMenuItemIndicator(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` | Forces the indicator to stay mounted even when the parent item is unchecked. Useful for animating in/out with CSS or animation libraries. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<span>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<span>`, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | The indicator content (typically a checkmark icon). Leptos wraps in `Option` to allow an empty indicator. |
| *(spread)* | — | `...PrimitiveSpanProps` | — | React allows spreading any `<span>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"checked" \| "unchecked" \| "indeterminate"` | Reflects the checked state of the parent `ContextMenuCheckboxItem` or `ContextMenuRadioItem`. |

### Implicit behavior

- Only renders (is present in the DOM) when the parent checkbox or radio item is in a checked or indeterminate state, unless `forceMount` is set.
- Must be placed inside a `ContextMenuCheckboxItem` or `ContextMenuRadioItem` to access the checked state context.
