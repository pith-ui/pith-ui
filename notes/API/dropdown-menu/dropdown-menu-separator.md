# DropdownMenuSeparator

## React Signature

```typescript
const DropdownMenuSeparator = React.forwardRef<DropdownMenuSeparatorElement, DropdownMenuSeparatorProps>(...)

type DropdownMenuSeparatorElement = React.ComponentRef<typeof MenuPrimitive.Separator>;
type MenuSeparatorProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.Separator>;

interface DropdownMenuSeparatorProps extends MenuSeparatorProps {}
```

`MenuSeparatorProps` extends `PrimitiveDivProps` -- all standard `<div>` attributes are accepted.

## Leptos Signature

```rust
pub fn DropdownMenuSeparator(
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
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | Optional children. Typically empty -- the separator is styled purely via CSS. Wrapped in `Option` in Leptos. |
| *(spread)* | -- | `...PrimitiveDivProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Renders with `role="separator"` and `aria-orientation="horizontal"`.
- Not a focusable item -- skipped during keyboard navigation.
