# ToolbarLink

## React Signature

```typescript
const ToolbarLink = React.forwardRef<ToolbarLinkElement, ToolbarLinkProps>(...)

type ToolbarLinkElement = React.ComponentRef<typeof Primitive.a>;
type PrimitiveLinkProps = React.ComponentPropsWithoutRef<typeof Primitive.a>;

interface ToolbarLinkProps extends PrimitiveLinkProps {}
```

`ToolbarLinkProps` extends `PrimitiveLinkProps` — all standard `<a>` attributes are accepted, including `href`, `target`, etc.

## Leptos Signature

```rust
pub fn ToolbarLink(
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `onKeyDown` | `on_key_down` | `(event: KeyboardEvent) => void` | `Option<Callback<ev::KeyboardEvent>>` | Optional callback for keydown events. Composed with the internal handler that converts Space to a click. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<a>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in an `<a>`, merging props and refs. |
| *(spread)* | — | `...PrimitiveLinkProps` | — | React allows spreading any `<a>` HTML attribute (e.g., `href`, `target`). Leptos uses `attr:` directives instead (e.g., `attr:href="..."`, `attr:target="_blank"`). |

### Implicit behavior

- Wraps the link in a `RovingFocusGroupItem` (always focusable) to participate in the toolbar's keyboard navigation.
- Adds a keydown handler that converts a Space keypress into a click on the element, since `<a>` elements do not natively respond to Space.

### Data attributes (rendered on DOM)

No component-specific data attributes. Standard HTML anchor attributes apply.
