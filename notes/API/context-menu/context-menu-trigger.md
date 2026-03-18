# ContextMenuTrigger

## React Signature

```typescript
const ContextMenuTrigger = React.forwardRef<ContextMenuTriggerElement, ContextMenuTriggerProps>(...)

type ContextMenuTriggerElement = React.ComponentRef<typeof Primitive.span>;
type PrimitiveSpanProps = React.ComponentPropsWithoutRef<typeof Primitive.span>;

interface ContextMenuTriggerProps extends PrimitiveSpanProps {
  disabled?: boolean;
}
```

## Leptos Signature

```rust
pub fn ContextMenuTrigger(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `disabled` | `disabled` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | When `true`, the trigger does not intercept the context menu event, allowing the native browser context menu to appear. Long press timers are also cleared. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<span>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<span>`, merging props and refs. |
| *(spread)* | — | `...PrimitiveSpanProps` | — | React allows spreading any `<span>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"open" \| "closed"` | Reflects whether the context menu is currently open. |
| `data-disabled` | `""` (present/absent) | Present when the trigger is disabled. |

### Implicit behavior

- Renders a `MenuAnchor` with a virtual ref alongside the `<span>`. The virtual anchor is a zero-size rect positioned at the right-click (or long-press) coordinates, which the popper uses for positioning the content.
- On `contextmenu` event (right-click): records the click coordinates, opens the menu, and calls `event.preventDefault()` to suppress the native context menu.
- On touch/pen `pointerdown`: starts a 700ms long-press timer. If the timer fires, the menu opens at the pointer coordinates.
- On touch/pen `pointermove`, `pointercancel`, `pointerup`: clears the long-press timer (cancels the pending open).
- Mouse pointer events are ignored for the long-press logic (only touch and pen are handled).
- Applies `style: -webkit-touch-callout: none` to prevent the iOS native context menu overlay.
- When `disabled` changes to `true`, any active long-press timer is cleared.
- On unmount, the long-press timer is cleared.
