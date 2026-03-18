# MenuItemIndicator

## React Signature

```typescript
const MenuItemIndicator = React.forwardRef<MenuItemIndicatorElement, MenuItemIndicatorProps>(...)

type MenuItemIndicatorElement = React.ComponentRef<typeof Primitive.span>;
type PrimitiveSpanProps = React.ComponentPropsWithoutRef<typeof Primitive.span>;

interface MenuItemIndicatorProps extends PrimitiveSpanProps {
  /** Used to force mounting when more control is needed. Useful when controlling animation with React animation libraries. */
  forceMount?: true;
}
```

## Leptos Signature

```rust
pub fn MenuItemIndicator(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` | Forces the indicator to stay mounted in the DOM even when unchecked. Useful for animating check/uncheck transitions. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<span>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<span>`. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | The indicator content (e.g., a checkmark icon). Leptos wraps in `Option` to allow an empty indicator. |
| *(spread)* | -- | `...PrimitiveSpanProps` | -- | React allows spreading any `<span>` HTML attribute. Leptos uses `attr:` directives. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"checked" \| "unchecked" \| "indeterminate"` | Reflects the checked state of the parent `MenuCheckboxItem` or `MenuRadioItem`. |

### Implicit behavior

- Must be placed as a descendant of `MenuCheckboxItem` or `MenuRadioItem`. Reads the checked state from `ItemIndicatorContextValue` provided by the parent.
- Wraps content in a `Presence` component — the indicator is only rendered (or stays mounted with `forceMount`) when the parent item is checked or indeterminate.
- Renders as a `<span>` element.
