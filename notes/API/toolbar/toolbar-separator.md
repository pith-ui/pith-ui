# ToolbarSeparator

## React Signature

```typescript
const ToolbarSeparator = React.forwardRef<ToolbarSeparatorElement, ToolbarSeparatorProps>(...)

type ToolbarSeparatorElement = React.ComponentRef<typeof SeparatorPrimitive.Root>;
type SeparatorProps = React.ComponentPropsWithoutRef<typeof SeparatorPrimitive.Root>;

interface ToolbarSeparatorProps extends SeparatorProps {}
```

`ToolbarSeparatorProps` extends `SeparatorProps`, which extends `PrimitiveDivProps` and includes `orientation` and `decorative`.

## Leptos Signature

```rust
pub fn ToolbarSeparator(
    #[prop(into, optional)] decorative: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `orientation` | — | `'horizontal' \| 'vertical'` | — | In React this can be overridden, but both React and Leptos auto-set it to the perpendicular of the toolbar's orientation (e.g., vertical separator in a horizontal toolbar). The Leptos version does not expose this prop; it always reads from the toolbar context. |
| `decorative` | `decorative` | `boolean` | `MaybeProp<bool>` (default `false`) | When `true`, the separator is purely visual: `role` becomes `"none"` instead of `"separator"` and `aria-orientation` is omitted. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | Optional children. Separators typically have no children. |
| *(spread)* | — | `...SeparatorProps` | — | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Automatically sets its orientation to the perpendicular of the parent toolbar's orientation. In a horizontal toolbar, the separator is vertical; in a vertical toolbar, the separator is horizontal.
- Delegates to the `Separator` primitive, which handles `role` and `aria-orientation` based on orientation and `decorative`.

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-orientation` | `"horizontal" \| "vertical"` | Reflects the separator's effective orientation (perpendicular to the toolbar). |
