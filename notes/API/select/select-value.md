# SelectValue

## React Signature

```typescript
const SelectValue = React.forwardRef<SelectValueElement, SelectValueProps>(...)

type SelectValueElement = React.ComponentRef<typeof Primitive.span>;
type PrimitiveSpanProps = React.ComponentPropsWithoutRef<typeof Primitive.span>;

interface SelectValueProps extends Omit<PrimitiveSpanProps, 'placeholder'> {
  placeholder?: React.ReactNode;
}
```

React ignores `className` and `style` on this component since it should not be styled directly.

## Leptos Signature

```rust
pub fn SelectValue(
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `placeholder` | `placeholder` | `React.ReactNode` | `MaybeProp<String>` | Content shown when no value is selected. React accepts any React node; Leptos accepts a string. Defaults to empty string in React. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<span>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<span>`, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | Custom render for the selected value. When omitted, the selected `SelectItemText`'s content is automatically copied into this span. |
| *(spread)* | -- | `...PrimitiveSpanProps` | -- | React allows spreading any `<span>` HTML attribute (except `placeholder`). Leptos uses `attr:` directives instead. |

### Implicit behavior

- Renders with `style="pointer-events: none"` to prevent events from portalled children bubbling through the wrong element.
- When no value is selected (value is `undefined` or empty string), the placeholder content is displayed.
- When a value is selected and no `children` are provided, the selected `SelectItemText`'s text content is automatically copied into this span via an Effect.
- When `children` are provided, the component renders those children instead of the auto-copied text. It notifies the parent context that it has static children so the auto-copy mechanism is disabled.
