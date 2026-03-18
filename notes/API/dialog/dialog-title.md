# DialogTitle

## React Signature

```typescript
const DialogTitle = React.forwardRef<DialogTitleElement, DialogTitleProps>(...)

type DialogTitleElement = React.ComponentRef<typeof Primitive.h2>;
type PrimitiveHeading2Props = React.ComponentPropsWithoutRef<typeof Primitive.h2>;

interface DialogTitleProps extends PrimitiveHeading2Props {}
```

## Leptos Signature

```rust
pub fn DialogTitle(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<h2>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in an `<h2>`, merging props and refs. Useful when you want a different heading level or element type. |
| *(spread)* | -- | `...PrimitiveHeading2Props` | -- | React allows spreading any `<h2>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Sets `id` to the auto-generated `title_id` from context. This `id` is referenced by `DialogContent`'s `aria-labelledby` attribute, providing the accessible name for the dialog.
- Required for accessibility. In debug builds, a console error is emitted by `DialogContent` if no element with the `title_id` is found in the DOM. If you want to visually hide the title, wrap it with `VisuallyHidden`.
