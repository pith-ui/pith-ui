# SelectScrollUpButton

## React Signature

```typescript
const SelectScrollUpButton = React.forwardRef<
  SelectScrollUpButtonElement,
  SelectScrollUpButtonProps
>(...)

type SelectScrollUpButtonElement = SelectScrollButtonImplElement;

interface SelectScrollUpButtonProps
  extends Omit<SelectScrollButtonImplProps, 'onAutoScroll'> {}

// Underlying impl:
type SelectScrollButtonImplElement = React.ComponentRef<typeof Primitive.div>;
interface SelectScrollButtonImplProps extends PrimitiveDivProps {
  onAutoScroll(): void;
}
```

## Leptos Signature

```rust
pub fn SelectScrollUpButton(
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
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | The button content (e.g., an up arrow icon). |
| *(spread)* | -- | `...PrimitiveDivProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Only rendered when the viewport can be scrolled upward (`scrollTop > 0`). Automatically hides when already at the top.
- Renders with `aria-hidden="true"` since it is a visual scroll affordance.
- Renders with `flex-shrink: 0` to prevent compression in the flex layout.
- On pointer down or pointer move: starts a 50ms interval timer that scrolls the viewport up by one item height per tick.
- On pointer leave: clears the auto-scroll timer.
- When mounted, scrolls the active (focused) item into view to ensure it remains visible after the button pushes the viewport down.
