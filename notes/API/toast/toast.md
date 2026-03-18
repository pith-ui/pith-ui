# Toast (Root)

## Anatomy

The expected component nesting structure:

```
ToastProvider
├── Toast (one or more)
│   ├── ToastTitle
│   ├── ToastDescription
│   ├── ToastAction (optional)
│   └── ToastClose (optional)
└── ToastViewport
```

### React

```tsx
<Toast.Provider>
  <Toast.Root>
    <Toast.Title>...</Toast.Title>
    <Toast.Description>...</Toast.Description>
    <Toast.Action altText="...">...</Toast.Action>
    <Toast.Close>...</Toast.Close>
  </Toast.Root>
  <Toast.Viewport />
</Toast.Provider>
```

### Leptos

```rust
<ToastProvider>
  <Toast>
    <ToastTitle>"..."</ToastTitle>
    <ToastDescription>"..."</ToastDescription>
    <ToastAction alt_text="...">"..."</ToastAction>
    <ToastClose>"..."</ToastClose>
  </Toast>
  <ToastViewport />
</ToastProvider>
```

## React Signature

```typescript
const Toast = React.forwardRef<ToastElement, ToastProps>(...)

type ToastElement = ToastImplElement;
type ToastImplElement = React.ComponentRef<typeof Primitive.li>;

interface ToastProps extends Omit<ToastImplProps, keyof ToastImplPrivateProps> {
  open?: boolean;
  defaultOpen?: boolean;
  onOpenChange?(open: boolean): void;
  /**
   * Used to force mounting when more control is needed. Useful when
   * controlling animation with React animation libraries.
   */
  forceMount?: true;
}

type SwipeEvent = { currentTarget: EventTarget & ToastElement } & Omit<
  CustomEvent<{ originalEvent: React.PointerEvent; delta: { x: number; y: number } }>,
  'currentTarget'
>;

// Private props managed internally (not part of public API):
type ToastImplPrivateProps = { open: boolean; onClose(): void };

interface ToastImplProps extends ToastImplPrivateProps, PrimitiveListItemProps {
  type?: 'foreground' | 'background';
  duration?: number;
  onEscapeKeyDown?: DismissableLayerProps['onEscapeKeyDown'];
  onPause?(): void;
  onResume?(): void;
  onSwipeStart?(event: SwipeEvent): void;
  onSwipeMove?(event: SwipeEvent): void;
  onSwipeCancel?(event: SwipeEvent): void;
  onSwipeEnd?(event: SwipeEvent): void;
}
```

## Leptos Signature

```rust
pub fn Toast(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    /// Used to force mounting when more control is needed. Useful when
    /// controlling animation with animation libraries.
    #[prop(into, optional)]
    force_mount: Option<bool>,
    /// The type of toast. `Foreground` toasts are announced as `assertive`,
    /// `Background` toasts as `polite`.
    #[prop(into, optional)]
    r#type: MaybeProp<ToastType>,
    /// Time in milliseconds that toast should remain visible for. Overrides value
    /// given to `ToastProvider`.
    #[prop(into, optional)]
    duration: MaybeProp<i32>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_pause: Option<Callback<()>>,
    #[prop(into, optional)] on_resume: Option<Callback<()>>,
    #[prop(into, optional)] on_swipe_start: Option<Callback<SwipeEvent>>,
    #[prop(into, optional)] on_swipe_move: Option<Callback<SwipeEvent>>,
    #[prop(into, optional)] on_swipe_cancel: Option<Callback<SwipeEvent>>,
    #[prop(into, optional)] on_swipe_end: Option<Callback<SwipeEvent>>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `open` | `open` | `boolean \| undefined` | `MaybeProp<bool>` | The controlled open state. When set, the toast becomes controlled and `onOpenChange` must be used to respond to state changes. Defaults to `true` when uncontrolled. |
| `defaultOpen` | `default_open` | `boolean` (default `true`) | `MaybeProp<bool>` (default `true`) | The initial open state when uncontrolled. Defaults to `true` (toast appears immediately). |
| `onOpenChange` | `on_open_change` | `(open: boolean) => void` | `Option<Callback<bool>>` | Callback fired when the open state changes. Called with `false` when the toast is dismissed (by duration expiry, escape key, close button, or swipe). |
| `forceMount` | `force_mount` | `true \| undefined` | `Option<bool>` | Forces the toast to remain mounted in the DOM even when closed. Useful for controlling exit animations with CSS or animation libraries. |
| `type` | `r#type` | `'foreground' \| 'background'` (default `'foreground'`) | `MaybeProp<ToastType>` (default `Foreground`) | Controls the urgency of the screen reader announcement. `Foreground` uses `aria-live="assertive"` and `Background` uses `aria-live="polite"`. |
| `duration` | `duration` | `number \| undefined` | `MaybeProp<i32>` | Time in milliseconds the toast remains visible. Overrides the provider-level `duration`. Omit to inherit from `ToastProvider`. |
| `onEscapeKeyDown` | `on_escape_key_down` | `(event: KeyboardEvent) => void` | `Option<Callback<ev::KeyboardEvent>>` | Callback fired when the escape key is pressed while the toast has focus. Call `event.preventDefault()` to prevent the toast from closing. |
| `onPause` | `on_pause` | `() => void` | `Option<Callback<()>>` | Callback fired when the auto-close timer is paused (e.g., when the viewport receives focus or pointer hover). |
| `onResume` | `on_resume` | `() => void` | `Option<Callback<()>>` | Callback fired when the auto-close timer resumes after being paused. |
| `onSwipeStart` | `on_swipe_start` | `(event: SwipeEvent) => void` | `Option<Callback<SwipeEvent>>` | Callback fired when a swipe gesture begins (pointer moves past the start buffer in the configured swipe direction). |
| `onSwipeMove` | `on_swipe_move` | `(event: SwipeEvent) => void` | `Option<Callback<SwipeEvent>>` | Callback fired continuously as the pointer moves during a swipe. The `delta` field contains the clamped displacement. |
| `onSwipeCancel` | `on_swipe_cancel` | `(event: SwipeEvent) => void` | `Option<Callback<SwipeEvent>>` | Callback fired when a swipe gesture ends without exceeding the swipe threshold. |
| `onSwipeEnd` | `on_swipe_end` | `(event: SwipeEvent) => void` | `Option<Callback<SwipeEvent>>` | Callback fired when a swipe gesture ends after exceeding the swipe threshold. The toast will close after this fires. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<li>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in an `<li>`, merging props and refs. |
| *(spread)* | -- | `...PrimitiveListItemProps` | -- | React allows spreading any `<li>` HTML attribute. Leptos uses `attr:` directives instead. |

### Leptos-only: `ToastType` enum

```rust
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ToastType {
    #[default]
    Foreground,
    Background,
}
```

### Leptos-only: `SwipeEvent` struct

```rust
#[derive(Clone, Debug)]
pub struct SwipeEvent {
    pub current_target: Option<SwipeEventTarget>,
    pub delta: (f64, f64),
}
```

In React, `SwipeEvent` is a `CustomEvent` with `detail.delta` as `{ x: number; y: number }`. In Leptos, `delta` is a tuple `(f64, f64)` where `.0` is x and `.1` is y.

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"open" \| "closed"` | Reflects the toast's current open state. |
| `data-swipe-direction` | `"up" \| "down" \| "left" \| "right"` | Reflects the provider's configured swipe direction. Useful for CSS animations. |
| `data-swipe` | `"start" \| "move" \| "cancel" \| "end"` | Set dynamically during swipe gestures. Use in CSS to style the toast during swipe interactions. |

### Implicit behavior

- The toast is portaled into the `ToastViewport` element. It does not render in the normal DOM tree.
- A separate visually-hidden live region (`ToastAnnounce`) is rendered outside the viewport to announce the toast content to screen readers. This uses `role="status"` and the appropriate `aria-live` value based on the `type` prop. The announcement auto-removes after 1 second.
- Auto-close timer starts when the toast opens. The timer pauses when the viewport receives focus or pointer hover, and resumes when focus/pointer leaves.
- When the toast closes while focus is inside it, focus is automatically moved to the viewport element so screen readers can announce the remaining toast count.
- The toast element has `tabIndex={0}`, `user-select: none`, and `touch-action: none` set to support keyboard focus and swipe gestures.
- During swipe gestures, CSS custom properties are set on the element for animation (see CSS Custom Properties section).
- A `DismissableLayer` wraps the toast to handle escape key dismissal. The escape key is handled both on the toast element itself (for when the toast has focus) and on the `DismissableLayer` (for when focus is elsewhere). A coordination flag prevents double-dismissal.
- After a swipe gesture, a one-time click event listener is added to prevent accidental clicks from triggering on elements within the toast.

## Usage Examples

### Basic (uncontrolled, defaults to open)

#### React

```tsx
<Toast.Provider>
  <Toast.Root>
    <Toast.Title>Notification</Toast.Title>
    <Toast.Description>Your file has been saved.</Toast.Description>
    <Toast.Close>Dismiss</Toast.Close>
  </Toast.Root>
  <Toast.Viewport />
</Toast.Provider>
```

#### Leptos

```rust
<ToastProvider>
  <Toast>
    <ToastTitle>"Notification"</ToastTitle>
    <ToastDescription>"Your file has been saved."</ToastDescription>
    <ToastClose>"Dismiss"</ToastClose>
  </Toast>
  <ToastViewport />
</ToastProvider>
```

### Controlled

#### React

```tsx
const [open, setOpen] = React.useState(false);

<Toast.Provider>
  <button onClick={() => setOpen(true)}>Show toast</button>
  <Toast.Root open={open} onOpenChange={setOpen}>
    <Toast.Description>Something happened.</Toast.Description>
    <Toast.Action altText="Undo the action" onClick={handleUndo}>
      Undo
    </Toast.Action>
  </Toast.Root>
  <Toast.Viewport />
</Toast.Provider>
```

#### Leptos

```rust
let (open, set_open) = signal(false);

<ToastProvider>
  <button on:click=move |_| set_open.set(true)>"Show toast"</button>
  <Toast
    open=open
    on_open_change=Callback::new(move |o: bool| set_open.set(o))
  >
    <ToastDescription>"Something happened."</ToastDescription>
    <ToastAction alt_text="Undo the action">"Undo"</ToastAction>
  </Toast>
  <ToastViewport />
</ToastProvider>
```

### With action button

#### React

```tsx
<Toast.Root>
  <Toast.Title>Upgrade available</Toast.Title>
  <Toast.Description>We've just released version 3.0</Toast.Description>
  <Toast.Action altText="Goto account settings to upgrade">
    Upgrade
  </Toast.Action>
  <Toast.Close aria-label="Close">
    <span aria-hidden>&times;</span>
  </Toast.Close>
</Toast.Root>
```

#### Leptos

```rust
<Toast>
  <ToastTitle>"Upgrade available"</ToastTitle>
  <ToastDescription>"We've just released version 3.0"</ToastDescription>
  <ToastAction alt_text="Goto account settings to upgrade">
    "Upgrade"
  </ToastAction>
  <ToastClose attr:aria-label="Close">
    <span aria-hidden="true">"\u{00d7}"</span>
  </ToastClose>
</Toast>
```

### Custom duration and swipe direction

#### React

```tsx
<Toast.Provider swipeDirection="up" swipeThreshold={25}>
  <Toast.Root duration={10000}>
    <Toast.Description>This toast lasts 10 seconds.</Toast.Description>
  </Toast.Root>
  <Toast.Viewport />
</Toast.Provider>
```

#### Leptos

```rust
<ToastProvider
  swipe_direction=Signal::derive(|| SwipeDirection::Up)
  swipe_threshold=Signal::derive(|| 25.0)
>
  <Toast duration=MaybeProp::from(Some(10000))>
    <ToastDescription>"This toast lasts 10 seconds."</ToastDescription>
  </Toast>
  <ToastViewport />
</ToastProvider>
```

### Pause/resume with progress bar

#### React

```tsx
const [paused, setPaused] = React.useState(false);

<Toast.Root
  duration={3000}
  onPause={() => setPaused(true)}
  onResume={() => setPaused(false)}
>
  <Toast.Description>Saving...</Toast.Description>
  <div className="progress-bar">
    <div style={{ animationPlayState: paused ? 'paused' : 'running' }} />
  </div>
</Toast.Root>
```

#### Leptos

```rust
let (paused, set_paused) = signal(false);

<Toast
  duration=MaybeProp::from(Some(3000))
  on_pause=Callback::new(move |_: ()| set_paused.set(true))
  on_resume=Callback::new(move |_: ()| set_paused.set(false))
>
  <ToastDescription>"Saving..."</ToastDescription>
  <div class="progress-bar">
    <div style:animation-play-state=move || {
      if paused.get() { "paused" } else { "running" }
    } />
  </div>
</Toast>
```

## Accessibility

Implements the [WAI-ARIA status role pattern](https://www.w3.org/WAI/ARIA/apd/patterns/status/). Toasts are non-modal, time-limited notifications.

### Keyboard Interactions

| Key | Description |
|---|---|
| `F8` | Moves focus to the toast viewport (configurable via the `hotkey` prop on `ToastViewport`). |
| `Escape` | When a toast has focus, dismisses that toast. When the viewport has focus (but not a specific toast), the `DismissableLayer` escape handler fires. |
| `Tab` | Moves focus through tabbable elements within the viewport. Tab order is reversed so the most recently added toast is focused first. `Shift+Tab` moves backwards through the reversed order. |

### ARIA Attributes

| Element | Attribute | Value | Notes |
|---|---|---|---|
| `ToastViewport` wrapper | `role` | `"region"` | Identifies the viewport as a landmark region for screen reader navigation. |
| `ToastViewport` wrapper | `aria-label` | `string` | Localized label with hotkey info, e.g., `"Notifications (F8)"`. |
| `ToastAnnounce` (internal) | `role` | `"status"` | Used for the visually-hidden announce element. Always `"status"` to avoid stuttering in screen readers. |
| `ToastAnnounce` (internal) | `aria-live` | `"assertive" \| "polite"` | `"assertive"` for foreground toasts, `"polite"` for background. |
| `ToastAnnounce` (internal) | `aria-atomic` | `"true"` | Ensures the full announcement is read, not just the changed portion. |
| `ToastAction` | `data-radix-toast-announce-alt` | `string` | The `altText` value, used by the announce system to provide an alternative description for the action button. |

### Behavioral Notes

- Toast announcements are separated from the interactive toast element. A visually-hidden live region announces the toast content with the provider's label prefix (e.g., "Notification Your file has been saved."). This region auto-removes after 1 second.
- A two-frame delay (two `requestAnimationFrame` calls) is used before rendering the announce text, which prevents stuttering in NVDA.
- The `ToastAction` component wraps `ToastClose` in a `ToastAnnounceExclude` wrapper. The `altText` is used as the alternative announcement text instead of the button's visible label, since screen reader users may not be able to easily navigate to the action button.
- When a toast is dismissed while focus is inside it, focus automatically moves to the viewport `<ol>` element. This ensures focus is not lost and screen readers can announce the remaining toast count.
- The swipe start buffer differs by pointer type: 10px for touch, 2px for mouse/pen. This prevents accidental swipe detection on touch devices.

## CSS Custom Properties

These properties are set dynamically on the `Toast` element during swipe gestures. Use them in CSS to animate the toast position during and after a swipe.

| Property | Source | Description |
|---|---|---|
| `--radix-toast-swipe-move-x` | Set during `data-swipe="move"` | The horizontal displacement (in px) of an in-progress swipe. Removed when the swipe ends or is cancelled. |
| `--radix-toast-swipe-move-y` | Set during `data-swipe="move"` | The vertical displacement (in px) of an in-progress swipe. Removed when the swipe ends or is cancelled. |
| `--radix-toast-swipe-end-x` | Set during `data-swipe="end"` | The final horizontal displacement (in px) when a swipe exceeds the threshold and the toast is dismissed. |
| `--radix-toast-swipe-end-y` | Set during `data-swipe="end"` | The final vertical displacement (in px) when a swipe exceeds the threshold and the toast is dismissed. |

### Example: swipe-to-dismiss animation

```css
.toast[data-swipe="move"] {
  transform: translateX(var(--radix-toast-swipe-move-x));
}

.toast[data-swipe="cancel"] {
  transition: transform 200ms ease;
  transform: translateX(0);
}

.toast[data-swipe="end"] {
  animation: slideOut 100ms ease-out;
}

@keyframes slideOut {
  from { transform: translateX(var(--radix-toast-swipe-end-x)); }
  to   { transform: translateX(calc(100% + 16px)); }
}
```
