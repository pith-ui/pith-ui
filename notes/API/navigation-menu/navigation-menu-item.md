# NavigationMenuItem

## React Signature

```typescript
type NavigationMenuItemElement = React.ComponentRef<typeof Primitive.li>;
type PrimitiveListItemProps = React.ComponentPropsWithoutRef<typeof Primitive.li>;

interface NavigationMenuItemProps extends PrimitiveListItemProps {
  value?: string;
}

const NavigationMenuItem = React.forwardRef<NavigationMenuItemElement, NavigationMenuItemProps>(...)
```

## Leptos Signature

```rust
pub fn NavigationMenuItem(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `value` | `value` | `string \| undefined` | `MaybeProp<String>` | A unique string identifying this item within the navigation menu. Used to determine which item is active. If not provided, an auto-generated ID is used. When using controlled mode, this is the value passed to `onValueChange`. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<li>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<li>`, merging props and refs. |
| *(spread)* | -- | `...PrimitiveListItemProps` | -- | React allows spreading any `<li>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- If `value` is not provided, an auto-generated unique ID is used via `useId()`. This means items without explicit values will have non-deterministic IDs, so controlled mode requires explicit values.
- Provides an `NavigationMenuItemContext` to descendant `NavigationMenuTrigger` and `NavigationMenuContent` parts, containing:
  - The item's `value`
  - Refs for the trigger, content, and focus proxy elements
  - Callbacks for content entry/exit (used for tab-order management)
  - An escape-close tracking flag
- Manages tab-order restoration for content: when focus enters the content area, tabbable elements are restored; when focus exits, they are removed from tab order so the focus proxy handles tab navigation correctly.
