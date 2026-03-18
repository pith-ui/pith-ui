# ScrollAreaViewport

## React Signature

```typescript
const ScrollAreaViewport = React.forwardRef<ScrollAreaViewportElement, ScrollAreaViewportProps>(...)

type ScrollAreaViewportElement = React.ComponentRef<typeof Primitive.div>;

interface ScrollAreaViewportProps extends PrimitiveDivProps {
  nonce?: string;
}
```

## Leptos Signature

```rust
pub fn ScrollAreaViewport(
    #[prop(into, optional)] nonce: Option<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `nonce` | `nonce` | `string \| undefined` | `Option<String>` | A nonce value passed to the injected `<style>` element that hides native scrollbars. Required when using a Content Security Policy (CSP) that restricts inline styles. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the viewport DOM element (`<div>`). This is the scrollable container. |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| *(spread)* | — | `...PrimitiveDivProps` | — | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-radix-scroll-area-viewport` | `""` | Marker attribute used by the injected `<style>` element to target the viewport and hide native scrollbars. |

### Implicit behavior

- Injects a `<style>` element into the DOM that hides native scrollbars on any element with `[data-radix-scroll-area-viewport]`. The styles apply `scrollbar-width: none`, `-ms-overflow-style: none`, `-webkit-overflow-scrolling: touch`, and `::-webkit-scrollbar { display: none }`.
- Sets `overflow-x` and `overflow-y` based on whether horizontal and vertical scrollbars are registered in the scroll area context. When a `ScrollAreaScrollbar` with a given orientation is mounted, the viewport switches that axis from `hidden` to `scroll`.
- Wraps children in an internal `<div>` with `style="min-width: 100%; display: table"`. This wrapper ensures the content div matches the size of its children on both axes so that `scrollWidth`/`scrollHeight` changes can be detected for thumb sizing. This wrapper is not controllable by the consumer.
- Registers the viewport element with the parent `ScrollArea` context so other parts (scrollbar, thumb, corner) can read scroll position and dimensions.
