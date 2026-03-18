# RadioGroupIndicator

## React Signature

```typescript
const RadioGroupIndicator = React.forwardRef<RadioGroupIndicatorElement, RadioGroupIndicatorProps>(...)

type RadioGroupIndicatorElement = React.ComponentRef<typeof RadioIndicator>;
type RadioIndicatorProps = React.ComponentPropsWithoutRef<typeof RadioIndicator>;

interface RadioGroupIndicatorProps extends RadioIndicatorProps {}

// Where RadioIndicator extends:
interface RadioIndicatorProps extends PrimitiveSpanProps {
  /**
   * Used to force mounting when more control is needed. Useful when
   * controlling animation with React animation libraries.
   */
  forceMount?: true;
}
```

## Leptos Signature

```rust
pub fn RadioGroupIndicator(
    /// Used to force mounting when more control is needed. Useful when
    /// controlling animation with animation libraries.
    #[prop(into, optional)]
    force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` | Forces the indicator to stay mounted in the DOM even when the parent item is unchecked. Useful when controlling show/hide animations with CSS or animation libraries — without this, the element is removed from the DOM when unchecked and animations cannot run. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<span>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<span>`, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | The indicator content (typically a dot or icon). Leptos wraps this in `Option` to allow an empty indicator. |
| *(spread)* | — | `...PrimitiveSpanProps` | — | React allows spreading any `<span>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"checked" \| "unchecked"` | Reflects the parent `RadioGroupItem`'s checked state. |
| `data-disabled` | `""` (present/absent) | Present when the parent `RadioGroupItem` is disabled. |

### Implicit behavior

- Renders as a `<span>` inside a `Presence` wrapper that controls mount/unmount based on the parent item's checked state (or `forceMount`).
- Reads its checked and disabled state from the `RadioContextValue` provided by the parent `RadioGroupItem`, not from its own props.
- The indicator is a thin wrapper: `RadioGroupIndicator` delegates to the internal `RadioIndicator` component, passing through all props. This layer exists to scope the radio context correctly — each `RadioGroupItem` provides its own `RadioContextValue`, ensuring each indicator reads from its parent item rather than a sibling.
