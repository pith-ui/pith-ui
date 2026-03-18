# AlertDialogTitle

## React Signature

```typescript
type AlertDialogTitleElement = React.ComponentRef<typeof DialogPrimitive.Title>;
type DialogTitleProps = React.ComponentPropsWithoutRef<typeof DialogPrimitive.Title>;
interface AlertDialogTitleProps extends DialogTitleProps {}

const AlertDialogTitle = React.forwardRef<AlertDialogTitleElement, AlertDialogTitleProps>(
  (props, forwardedRef) => {
    return <DialogPrimitive.Title {...titleProps} ref={forwardedRef} />;
  },
);
```

`DialogTitleProps` extends `PrimitiveHeading2Props` -- all standard `<h2>` attributes are accepted.

## Leptos Signature

```rust
pub fn AlertDialogTitle(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<h2>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in an `<h2>`, merging props and refs. Useful when you want a different heading level. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | The title text content. |
| *(spread)* | -- | `...PrimitiveHeading2Props` | -- | React allows spreading any `<h2>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- An auto-generated `id` is set on the rendered element. This ID is referenced by `AlertDialogContent`'s `aria-labelledby` attribute.
- The title is required for accessibility. In development mode, a console error is emitted if the title element is missing when the dialog opens.
