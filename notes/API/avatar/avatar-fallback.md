# AvatarFallback

## React Signature

```typescript
const AvatarFallback = React.forwardRef<AvatarFallbackElement, AvatarFallbackProps>(...)

type AvatarFallbackElement = React.ComponentRef<typeof Primitive.span>;

interface AvatarFallbackProps extends PrimitiveSpanProps {
  delayMs?: number;
}
```

## Leptos Signature

```rust
pub fn AvatarFallback(
    #[prop(into, optional)] delay_ms: MaybeProp<i32>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `delayMs` | `delay_ms` | `number \| undefined` | `MaybeProp<i32>` | Delay in milliseconds before the fallback renders. Useful when the image is expected to load quickly — avoids a brief flash of fallback content. When omitted, the fallback renders immediately (if the image has not loaded). |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<span>`). Only available when the fallback is actually visible. |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<span>`, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | The fallback content — typically initials (e.g., `"JS"`) or an icon component. |
| *(spread)* | — | `...PrimitiveSpanProps` | — | React allows spreading any `<span>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Reads `imageLoadingStatus` from the parent `Avatar` context. Renders only when `canRender` is `true` AND the image loading status is not `Loaded`.
- When `delayMs` is omitted (`None`), `canRender` starts as `true` — the fallback is shown immediately if the image hasn't loaded.
- When `delayMs` is provided, `canRender` starts as `false` and a `setTimeout` / `set_timeout` is scheduled. After the delay elapses, `canRender` becomes `true` and the fallback appears (if the image still hasn't loaded).
- The timer is cleaned up on unmount to prevent memory leaks and stale state updates.
