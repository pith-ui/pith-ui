# SelectScrollDownButton

## React Signature

```typescript
const SelectScrollDownButton = React.forwardRef<
  SelectScrollDownButtonElement,
  SelectScrollDownButtonProps
>(...)

type SelectScrollDownButtonElement = SelectScrollButtonImplElement;

interface SelectScrollDownButtonProps
  extends Omit<SelectScrollButtonImplProps, 'onAutoScroll'> {}

// Underlying impl:
type SelectScrollButtonImplElement = React.ComponentRef<typeof Primitive.div>;
interface SelectScrollButtonImplProps extends PrimitiveDivProps {
  onAutoScroll(): void;
}
```

## Leptos Signature

```rust
pub fn SelectScrollDownButton(
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
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | The button content (e.g., a down arrow icon). |
| *(spread)* | -- | `...PrimitiveDivProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Only rendered when the viewport can be scrolled downward (`ceil(scrollTop) < scrollHeight - clientHeight`). Automatically hides when already at the bottom.
- Renders with `aria-hidden="true"` since it is a visual scroll affordance.
- Renders with `flex-shrink: 0` to prevent compression in the flex layout.
- On pointer down or pointer move: starts a 50ms interval timer that scrolls the viewport down by one item height per tick.
- On pointer leave: clears the auto-scroll timer.
- When mounted, scrolls the active (focused) item into view to ensure it remains visible after the button shrinks the viewport.
