# AlertDialogDescription

## React Signature

```typescript
type AlertDialogDescriptionElement = React.ComponentRef<typeof DialogPrimitive.Description>;
type DialogDescriptionProps = React.ComponentPropsWithoutRef<typeof DialogPrimitive.Description>;
interface AlertDialogDescriptionProps extends DialogDescriptionProps {}

const AlertDialogDescription = React.forwardRef<
  AlertDialogDescriptionElement,
  AlertDialogDescriptionProps
>((props, forwardedRef) => {
  return <DialogPrimitive.Description {...descriptionProps} ref={forwardedRef} />;
});
```

`DialogDescriptionProps` extends `PrimitiveParagraphProps` -- all standard `<p>` attributes are accepted.

## Leptos Signature

```rust
pub fn AlertDialogDescription(
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
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | The description text content. |
| *(spread)* | -- | `...PrimitiveParagraphProps` | -- | React allows spreading any `<p>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- An auto-generated `id` is set on the rendered element. This ID is referenced by `AlertDialogContent`'s `aria-describedby` attribute.
- The description is recommended for accessibility. In development mode, a console warning is emitted if the description element is missing when the dialog opens. Unlike the title (which produces an error), the description warning is non-fatal.
