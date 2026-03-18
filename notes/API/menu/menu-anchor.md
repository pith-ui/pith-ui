# MenuAnchor

## React Signature

```typescript
const MenuAnchor = React.forwardRef<MenuAnchorElement, MenuAnchorProps>(...)

type MenuAnchorElement = React.ComponentRef<typeof PopperPrimitive.Anchor>;
type PopperAnchorProps = React.ComponentPropsWithoutRef<typeof PopperPrimitive.Anchor>;
interface MenuAnchorProps extends PopperAnchorProps {}
```

## Leptos Signature

```rust
pub fn MenuAnchor(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the anchor DOM element. |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a default element, merging props and refs. |
| *(spread)* | -- | `...PopperAnchorProps` | -- | React allows spreading any HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Delegates to `PopperAnchor`, which registers the anchor element for the Popper positioning engine.
- Composes the user-provided `node_ref` with the Menu's internal popper anchor ref, ensuring the Menu's Popper context receives the anchor element even when a closer Popper context (e.g., from a Tooltip wrapping the trigger) would shadow it.
