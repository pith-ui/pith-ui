# DialogDescription

## React Signature

```typescript
const DialogDescription = React.forwardRef<DialogDescriptionElement, DialogDescriptionProps>(...)

type DialogDescriptionElement = React.ComponentRef<typeof Primitive.p>;
type PrimitiveParagraphProps = React.ComponentPropsWithoutRef<typeof Primitive.p>;

interface DialogDescriptionProps extends PrimitiveParagraphProps {}
```

## Leptos Signature

```rust
pub fn DialogDescription(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<p>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<p>`, merging props and refs. |
| *(spread)* | -- | `...PrimitiveParagraphProps` | -- | React allows spreading any `<p>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Sets `id` to the auto-generated `description_id` from context. This `id` is referenced by `DialogContent`'s `aria-describedby` attribute, providing the accessible description for the dialog.
- Optional but recommended. In debug builds, a console warning is emitted by `DialogContent` if the `aria-describedby` attribute is set but no element with the `description_id` is found in the DOM.
