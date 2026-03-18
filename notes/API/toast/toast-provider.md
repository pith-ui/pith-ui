# ToastProvider

## React Signature

```typescript
const ToastProvider: React.FC<ToastProviderProps>

type SwipeDirection = 'up' | 'down' | 'left' | 'right';

interface ToastProviderProps {
  children?: React.ReactNode;
  /**
   * An author-localized label for each toast. Used to help screen reader users
   * associate the interruption with a toast.
   * @defaultValue 'Notification'
   */
  label?: string;
  /**
   * Time in milliseconds that each toast should remain visible for.
   * @defaultValue 5000
   */
  duration?: number;
  /**
   * Direction of pointer swipe that should close the toast.
   * @defaultValue 'right'
   */
  swipeDirection?: SwipeDirection;
  /**
   * Distance in pixels that the swipe must pass before a close is triggered.
   * @defaultValue 50
   */
  swipeThreshold?: number;
}
```

`ToastProvider` is a plain `React.FC` -- it does not use `forwardRef` and does not accept `ref` or `asChild`.

## Leptos Signature

```rust
pub fn ToastProvider(
    /// An author-localized label for each toast. Used to help screen reader users
    /// associate the interruption with a toast.
    #[prop(into, optional, default = "Notification".to_string())]
    label: String,
    /// Time in milliseconds that each toast should remain visible for.
    #[prop(into, optional, default = 5000.into())]
    duration: Signal<i32>,
    /// Direction of pointer swipe that should close the toast.
    #[prop(into, optional, default = Signal::derive(|| SwipeDirection::Right))]
    swipe_direction: Signal<SwipeDirection>,
    /// Distance in pixels that the swipe must pass before a close is triggered.
    #[prop(into, optional, default = 50.0.into())]
    swipe_threshold: Signal<f64>,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `label` | `label` | `string` (default `'Notification'`) | `String` (default `"Notification"`) | An author-localized label prepended to each toast's announce text for screen readers. Must be a non-empty string. |
| `duration` | `duration` | `number` (default `5000`) | `Signal<i32>` (default `5000`) | Default time in milliseconds that each toast remains visible. Individual toasts can override this via their own `duration` prop. |
| `swipeDirection` | `swipe_direction` | `SwipeDirection` (default `'right'`) | `Signal<SwipeDirection>` (default `Right`) | The pointer swipe direction that dismisses toasts. |
| `swipeThreshold` | `swipe_threshold` | `number` (default `50`) | `Signal<f64>` (default `50.0`) | Distance in pixels that a swipe must travel before triggering a dismiss. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | The application content. Must contain a `ToastViewport` and any `Toast` instances. |

### Implicit behavior

- Provides context to all descendant toast components (`ToastViewport`, `Toast`, `ToastAction`, `ToastClose`).
- Tracks the number of currently mounted toasts and the viewport element reference.
- Manages the `isClosePausedRef` and `isFocusedToastEscapeKeyDownRef` flags used for coordinating pause/resume and escape key behavior across toasts.
- In Leptos, `duration`, `swipe_direction`, and `swipe_threshold` are `Signal` types rather than plain values, making them reactive. React re-renders on prop changes to achieve the same effect.
