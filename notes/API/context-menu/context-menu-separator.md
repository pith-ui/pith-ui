# ContextMenuSeparator

## React Signature

```typescript
const ContextMenuSeparator = React.forwardRef<ContextMenuSeparatorElement, ContextMenuSeparatorProps>(...)

type ContextMenuSeparatorElement = React.ComponentRef<typeof MenuPrimitive.Separator>;
type MenuSeparatorProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.Separator>;

interface ContextMenuSeparatorProps extends MenuSeparatorProps {}

// MenuSeparatorProps extends PrimitiveDivProps {}
```

## Leptos Signature

```rust
pub fn ContextMenuSeparator(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | Optional content inside the separator. Leptos wraps in `Option` since separators are typically self-closing. |
| *(spread)* | — | `...PrimitiveDivProps` | — | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Renders with `role="separator"` and `aria-orientation="horizontal"`.
- Visually separates groups of menu items. Not focusable and not included in keyboard navigation.
