# ToolbarButton

## React Signature

```typescript
const ToolbarButton = React.forwardRef<ToolbarButtonElement, ToolbarButtonProps>(...)

type ToolbarButtonElement = React.ComponentRef<typeof Primitive.button>;
type PrimitiveButtonProps = React.ComponentPropsWithoutRef<typeof Primitive.button>;

interface ToolbarButtonProps extends PrimitiveButtonProps {}
```

`ToolbarButtonProps` extends `PrimitiveButtonProps` — all standard `<button>` attributes are accepted, including `disabled`.

## Leptos Signature

```rust
pub fn ToolbarButton(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `disabled` | `disabled` | `boolean` | `MaybeProp<bool>` (default `false`) | When `true`, the button is disabled and skipped during keyboard navigation. The underlying `RovingFocusGroupItem` sets `focusable=false` when disabled. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<button>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<button>`, merging props and refs. Useful for composing with other components (e.g., wrapping a `DropdownMenu.Trigger` or `Toggle`). |
| *(spread)* | — | `...PrimitiveButtonProps` | — | React allows spreading any `<button>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Wraps the button in a `RovingFocusGroupItem` to participate in the toolbar's keyboard navigation.
- Sets `type="button"` on the rendered `<button>` element to prevent form submission.

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-disabled` | `""` (present/absent) | Present when the button is disabled. |
