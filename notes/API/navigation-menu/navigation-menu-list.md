# NavigationMenuList

## React Signature

```typescript
type NavigationMenuListElement = React.ComponentRef<typeof Primitive.ul>;
type PrimitiveUnorderedListProps = React.ComponentPropsWithoutRef<typeof Primitive.ul>;

interface NavigationMenuListProps extends PrimitiveUnorderedListProps {}

const NavigationMenuList = React.forwardRef<NavigationMenuListElement, NavigationMenuListProps>(...)
```

## Leptos Signature

```rust
pub fn NavigationMenuList(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the `<ul>` DOM element. Note: the ref points to the `<ul>`, not the outer wrapper `<div>`. |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<ul>`, merging props and refs. |
| *(spread)* | -- | `...PrimitiveUnorderedListProps` | -- | React allows spreading any `<ul>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-orientation` | `"horizontal" \| "vertical"` | Reflects the parent navigation menu's `orientation`. Set on the `<ul>` element. |

### Implicit behavior

- Wraps the `<ul>` in a `<div style="position: relative">` that acts as the **indicator track**. The `NavigationMenuIndicator` is portaled into this wrapper `<div>`, keeping it positioned relative to the list.
- When inside a root menu (`isRootMenu=true`), the `<ul>` is additionally wrapped in a `FocusGroup` component that enables arrow-key navigation between triggers and links. In a sub-menu (`isRootMenu=false`), the `FocusGroup` wrapper is omitted.
- User-supplied attributes (e.g., `attr:class`) are forwarded to the `<ul>` element, not the outer wrapper `<div>`.
