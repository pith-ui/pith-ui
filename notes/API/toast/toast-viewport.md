# ToastViewport

## React Signature

```typescript
const ToastViewport = React.forwardRef<ToastViewportElement, ToastViewportProps>(...)

type ToastViewportElement = React.ComponentRef<typeof Primitive.ol>;
type PrimitiveOrderedListProps = React.ComponentPropsWithoutRef<typeof Primitive.ol>;

interface ToastViewportProps extends PrimitiveOrderedListProps {
  /**
   * The keys to use as the keyboard shortcut that will move focus to the toast viewport.
   * @defaultValue ['F8']
   */
  hotkey?: string[];
  /**
   * An author-localized label for the toast viewport to provide context for screen reader users
   * when navigating page landmarks. The available `{hotkey}` placeholder will be replaced for you.
   * @defaultValue 'Notifications ({hotkey})'
   */
  label?: string;
}
```

## Leptos Signature

```rust
pub fn ToastViewport(
    /// The keys to use as the keyboard shortcut that will move focus to the toast viewport.
    #[prop(into, optional)]
    hotkey: Option<Vec<String>>,
    /// An author-localized label for the toast viewport to provide context for screen reader users
    /// when navigating page landmarks. The available `{hotkey}` placeholder will be replaced for you.
    #[prop(into, optional)]
    label: Option<String>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `hotkey` | `hotkey` | `string[]` (default `['F8']`) | `Option<Vec<String>>` (default `["F8"]`) | Keyboard shortcut key codes that focus the viewport. Uses `event.code` values (e.g., `"F8"`, `"KeyT"`). Multiple keys require all to be pressed simultaneously. Modifier keys can be specified as `"altKey"`, `"ctrlKey"`, `"metaKey"`, `"shiftKey"`. |
| `label` | `label` | `string` (default `'Notifications ({hotkey})'`) | `Option<String>` (default `"Notifications ({hotkey})"`) | The `aria-label` for the viewport wrapper region. The `{hotkey}` placeholder is replaced with a human-readable hotkey label (e.g., `"F8"`). |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the inner `<ol>` list element. |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in an `<ol>`, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | Optional children rendered inside the ordered list. Typically empty -- toasts are portaled into this element. |
| *(spread)* | -- | `...PrimitiveOrderedListProps` | -- | React allows spreading any `<ol>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- The viewport is wrapped in a `DismissableLayerBranch` with `role="region"` so that interactions within the toast viewport do not trigger dismissable-layer close handlers on parent layers (e.g., dialogs).
- The wrapper has `tabIndex={-1}` to allow programmatic focus and to participate in landmark navigation.
- When no toasts are present, `pointer-events: none` is set on the wrapper so it does not block interaction with underlying page elements.
- Head and tail `FocusProxy` elements are rendered when toasts exist. These visually hidden, focusable elements manage tab navigation in and out of the viewport, enabling reverse tab order (most recent toast first).
- The viewport element registers itself with the provider context on mount, allowing `Toast` components to portal their content into it.
- Custom tab key handling reverses the default tab order so focus moves from the most recently added toast to the least recent.

### Data attributes (rendered on DOM)

The `ToastViewport` itself does not render custom `data-*` attributes. The outer wrapper renders standard `role` and `aria-label` attributes.
