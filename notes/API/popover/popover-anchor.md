# PopoverAnchor

## React Signature

```typescript
const PopoverAnchor = React.forwardRef<PopoverAnchorElement, PopoverAnchorProps>(...)

type PopoverAnchorElement = React.ComponentRef<typeof PopperPrimitive.Anchor>;
type PopperAnchorProps = React.ComponentPropsWithoutRef<typeof PopperPrimitive.Anchor>;

interface PopoverAnchorProps extends PopperAnchorProps {}
```

`PopperAnchorProps` extends `PrimitiveDivProps` -- all standard `<div>` attributes are accepted.

## Leptos Signature

```rust
pub fn PopoverAnchor(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | The anchor content. Typically wraps the trigger and/or other elements that should define the positioning reference point. |
| *(spread)* | -- | `...PopperAnchorProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- On mount, signals to the `Popover` context that a custom anchor is present (`has_custom_anchor = true`). On cleanup/unmount, resets this to `false`.
- When a custom anchor is present, `PopoverTrigger` no longer wraps itself in a `PopperAnchor` -- the popover is positioned relative to this `PopoverAnchor` element instead.
- Delegates to `PopperAnchor` which measures the anchor element for floating-ui positioning calculations.
