# TabsList

## React Signature

```typescript
const TabsList = React.forwardRef<TabsListElement, TabsListProps>(...)

type TabsListElement = React.ComponentRef<typeof Primitive.div>;
type PrimitiveDivProps = React.ComponentPropsWithoutRef<typeof Primitive.div>;
type RovingFocusGroupProps = React.ComponentPropsWithoutRef<typeof RovingFocusGroup.Root>;

interface TabsListProps extends PrimitiveDivProps {
  loop?: RovingFocusGroupProps['loop'];
}
```

## Leptos Signature

```rust
pub fn TabsList(
    /// Whether keyboard navigation loops around. Default true.
    #[prop(into, optional)]
    r#loop: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `loop` | `r#loop` | `boolean` (default `true`) | `MaybeProp<bool>` (default `true`) | Whether keyboard navigation wraps from the last trigger back to the first (and vice versa). Leptos uses `r#loop` because `loop` is a Rust reserved keyword. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| *(spread)* | -- | `...PrimitiveDivProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Renders with `role="tablist"` and `aria-orientation` set to the parent `Tabs` component's orientation.
- Wraps its children in a `RovingFocusGroup`, inheriting `orientation`, `dir`, and `loop` from context/props. This enables arrow-key navigation between triggers.

### Data attributes (rendered on DOM)

The `TabsList` element itself does not render `data-*` attributes. The `role` and `aria-orientation` attributes are set directly.
