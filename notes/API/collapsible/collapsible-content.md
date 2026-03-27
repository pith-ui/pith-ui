# CollapsibleContent

## React Signature

```typescript
const CollapsibleContent = React.forwardRef<CollapsibleContentElement, CollapsibleContentProps>(...)

type CollapsibleContentElement = CollapsibleContentImplElement;

interface CollapsibleContentProps extends Omit<CollapsibleContentImplProps, 'present'> {
  /**
   * Used to force mounting when more control is needed. Useful when
   * controlling animation with React animation libraries.
   */
  forceMount?: true;
}

// Internal implementation type (not directly exposed):
type CollapsibleContentImplElement = React.ComponentRef<typeof Primitive.div>;
interface CollapsibleContentImplProps extends PrimitiveDivProps {
  present: boolean;
}
```

## Leptos Signature

```rust
pub fn CollapsibleContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` | Forces the content to stay mounted in the DOM even when collapsed. Useful when controlling open/close animations with CSS or animation libraries -- without this, the element is removed from the DOM when closed and animations cannot run. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | The collapsible content. Leptos wraps this in `Option` to allow an empty content region. |
| *(spread)* | — | `...PrimitiveDivProps` | — | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"open" \| "closed"` | Reflects the parent collapsible's open state. |
| `data-disabled` | `""` (present/absent) | Present when the parent collapsible's `disabled` prop is `true`. |

### Implicit behavior

- The element's `id` is automatically set to the `content_id` from the parent `Collapsible` context. This ID is referenced by `CollapsibleTrigger`'s `aria-controls`.
- The `hidden` attribute is set when the content is not open and not animating out (`!isOpen`). During exit animations, `hidden` is removed so the animation can complete visually, and children are unmounted only after the animation finishes.
- CSS custom properties `--collapsible-content-height` and `--collapsible-content-width` are set as inline styles, containing the measured pixel dimensions of the content when fully expanded. These update whenever `open` or `present` changes.
- On initial mount, animations and transitions are temporarily suppressed (transition-duration set to `0s`, animation-name set to `none`) to measure the full dimensions without visual artifacts. After the first animation frame, normal animation behavior is restored.
- The `isOpen` derived state (`context.open || isPresent`) ensures the content is visible during both opening (immediately) and closing (until the exit animation completes).
