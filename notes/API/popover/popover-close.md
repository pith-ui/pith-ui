# PopoverClose

## React Signature

```typescript
const PopoverClose = React.forwardRef<PopoverCloseElement, PopoverCloseProps>(...)

type PopoverCloseElement = React.ComponentRef<typeof Primitive.button>;
type PrimitiveButtonProps = React.ComponentPropsWithoutRef<typeof Primitive.button>;

interface PopoverCloseProps extends PrimitiveButtonProps {}
```

## Leptos Signature

```rust
pub fn PopoverClose(
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<button>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<button>`, merging props and refs. |
| `onClick` | `on_click` | `MouseEventHandler` | `Option<Callback<ev::MouseEvent>>` | Optional click handler composed with the internal close handler. Your handler runs first; the close action always runs after. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | The close button content. Note: Leptos uses required `ChildrenFn` while React uses optional `React.ReactNode`. |
| *(spread)* | -- | `...PrimitiveButtonProps` | -- | React allows spreading any `<button>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Renders `type="button"` to prevent form submission.
- Clicking the button closes the popover by calling `onOpenChange(false)` on the context.
