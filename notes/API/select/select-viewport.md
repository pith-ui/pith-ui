# SelectViewport

## React Signature

```typescript
const SelectViewport = React.forwardRef<SelectViewportElement, SelectViewportProps>(...)

type SelectViewportElement = React.ComponentRef<typeof Primitive.div>;

interface SelectViewportProps extends PrimitiveDivProps {
  nonce?: string;
}
```

## Leptos Signature

```rust
pub fn SelectViewport(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `nonce` | -- | `string` | -- | CSP nonce for the injected `<style>` tag that hides scrollbars. Not implemented in Leptos. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | The viewport children (items, groups, separators, etc.). |
| *(spread)* | -- | `...PrimitiveDivProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Renders with `role="presentation"` (it is a presentational scroll container, not semantically meaningful).
- Sets `data-radix-select-viewport=""` for targeting by the injected scrollbar-hiding CSS.
- Injects a `<style>` tag that hides scrollbars cross-browser:
  - `scrollbar-width: none` (Firefox)
  - `-ms-overflow-style: none` (IE/Edge)
  - `-webkit-overflow-scrolling: touch` (iOS momentum scrolling)
  - `::-webkit-scrollbar { display: none }` (Webkit/Blink)
- Renders with inline styles: `position: relative`, `flex: 1`, `overflow: hidden auto`.
- Wraps children in a `CollectionSlot` to register items with the collection system.
- In `item-aligned` mode (React only), the viewport handles expand-on-scroll behavior: when the user scrolls, the content wrapper expands up to the available viewport height.
