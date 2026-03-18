# HoverCard (Root)

## Anatomy

The expected component nesting structure:

```
HoverCard
├── HoverCardTrigger
└── HoverCardPortal
    └── HoverCardContent
        └── HoverCardArrow (optional)
```

### React

```tsx
<HoverCard.Root>
  <HoverCard.Trigger href="/">trigger</HoverCard.Trigger>
  <HoverCard.Portal>
    <HoverCard.Content>
      <HoverCard.Arrow />
      Content here.
    </HoverCard.Content>
  </HoverCard.Portal>
</HoverCard.Root>
```

### Leptos

```rust
<HoverCard>
  <HoverCardTrigger attr:href="/">"trigger"</HoverCardTrigger>
  <HoverCardPortal>
    <HoverCardContent>
      <HoverCardArrow />
      "Content here."
    </HoverCardContent>
  </HoverCardPortal>
</HoverCard>
```

## React Signature

```typescript
const HoverCard: React.FC<HoverCardProps>

interface HoverCardProps {
  children?: React.ReactNode;
  open?: boolean;
  defaultOpen?: boolean;
  onOpenChange?: (open: boolean) => void;
  openDelay?: number;   // default: 700
  closeDelay?: number;  // default: 300
}
```

`HoverCard` is not a `forwardRef` component — it renders no DOM element of its own.

## Leptos Signature

```rust
pub fn HoverCard(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional, default = MaybeProp::from(700.0))] open_delay: MaybeProp<f64>,
    #[prop(into, optional, default = MaybeProp::from(300.0))] close_delay: MaybeProp<f64>,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `open` | `open` | `boolean \| undefined` | `MaybeProp<bool>` | The controlled open state. When set, the component becomes controlled and `onOpenChange` should be used to respond to state changes. |
| `defaultOpen` | `default_open` | `boolean \| undefined` | `MaybeProp<bool>` | The open state on initial render. Use when you do not need to control the open state externally. Defaults to `false`. |
| `onOpenChange` | `on_open_change` | `(open: boolean) => void` | `Option<Callback<bool>>` | Callback fired when the open state changes. Receives the new boolean value. |
| `openDelay` | `open_delay` | `number` (default `700`) | `MaybeProp<f64>` (default `700.0`) | Delay in milliseconds before the hover card opens after the pointer enters the trigger. |
| `closeDelay` | `close_delay` | `number` (default `300`) | `MaybeProp<f64>` (default `300.0`) | Delay in milliseconds before the hover card closes after the pointer leaves the trigger or content. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | The hover card parts (trigger, portal, content). |

### Implicit behavior

- Wraps children in a `Popper` context provider for positioning.
- Provides `HoverCardContextValue` via context so child parts can access open state and open/close handlers.
- Timers for open and close are cleaned up on unmount.
- In the Leptos implementation, nested hover cards automatically chain open/close signals to parent hover cards. When a child hover card opens, it cancels all ancestor close timers. When a child closes, it restarts ancestor close timers. This compensates for native DOM `pointerleave` events firing across portal boundaries (React's synthetic events do not fire in this case).

## Usage Examples

### Basic (uncontrolled)

#### React

```tsx
<HoverCard.Root>
  <HoverCard.Trigger href="/">trigger</HoverCard.Trigger>
  <HoverCard.Portal>
    <HoverCard.Content sideOffset={5}>
      <HoverCard.Arrow width={20} height={10} />
      Card content here.
    </HoverCard.Content>
  </HoverCard.Portal>
</HoverCard.Root>
```

#### Leptos

```rust
<HoverCard>
  <HoverCardTrigger attr:href="/">"trigger"</HoverCardTrigger>
  <HoverCardPortal>
    <HoverCardContent side_offset=5.0>
      <HoverCardArrow width=20.0 height=10.0 />
      "Card content here."
    </HoverCardContent>
  </HoverCardPortal>
</HoverCard>
```

### Controlled

#### React

```tsx
const [open, setOpen] = React.useState(false);

<HoverCard.Root open={open} onOpenChange={setOpen}>
  <HoverCard.Trigger href="/">trigger</HoverCard.Trigger>
  <HoverCard.Portal>
    <HoverCard.Content>
      <HoverCard.Arrow width={20} height={10} />
      Card content here.
    </HoverCard.Content>
  </HoverCard.Portal>
</HoverCard.Root>
```

#### Leptos

```rust
let (open, set_open) = signal(false);

<HoverCard open=open on_open_change=Callback::new(move |value: bool| set_open.set(value))>
  <HoverCardTrigger attr:href="/">"trigger"</HoverCardTrigger>
  <HoverCardPortal>
    <HoverCardContent>
      <HoverCardArrow width=20.0 height=10.0 />
      "Card content here."
    </HoverCardContent>
  </HoverCardPortal>
</HoverCard>
```

### Custom delays

#### React

```tsx
<HoverCard.Root openDelay={0} closeDelay={0}>
  <HoverCard.Trigger href="/">instant</HoverCard.Trigger>
  <HoverCard.Portal>
    <HoverCard.Content>Card content.</HoverCard.Content>
  </HoverCard.Portal>
</HoverCard.Root>
```

#### Leptos

```rust
<HoverCard open_delay=0.0 close_delay=0.0>
  <HoverCardTrigger attr:href="/">"instant"</HoverCardTrigger>
  <HoverCardPortal>
    <HoverCardContent>"Card content."</HoverCardContent>
  </HoverCardPortal>
</HoverCard>
```

### Nested hover cards

#### React

```tsx
<HoverCard.Root>
  <HoverCard.Trigger href="/">level 1</HoverCard.Trigger>
  <HoverCard.Portal>
    <HoverCard.Content sideOffset={5}>
      <HoverCard.Root>
        <HoverCard.Trigger href="/">level 2</HoverCard.Trigger>
        <HoverCard.Portal>
          <HoverCard.Content side="top" sideOffset={5}>
            Nested content.
          </HoverCard.Content>
        </HoverCard.Portal>
      </HoverCard.Root>
      <HoverCard.Arrow width={20} height={10} />
    </HoverCard.Content>
  </HoverCard.Portal>
</HoverCard.Root>
```

#### Leptos

```rust
<HoverCard>
  <HoverCardTrigger attr:href="/">"level 1"</HoverCardTrigger>
  <HoverCardPortal>
    <HoverCardContent side_offset=5.0>
      <HoverCard>
        <HoverCardTrigger attr:href="/">"level 2"</HoverCardTrigger>
        <HoverCardPortal>
          <HoverCardContent side=Side::Top side_offset=5.0>
            "Nested content."
          </HoverCardContent>
        </HoverCardPortal>
      </HoverCard>
      <HoverCardArrow width=20.0 height=10.0 />
    </HoverCardContent>
  </HoverCardPortal>
</HoverCard>
```

### Animated content (CSS keyframes)

Use the `--radix-hover-card-content-transform-origin` CSS custom property to animate from the content's transform origin, and `data-state` / `data-side` attributes for directional animations:

```css
.content[data-state='open'] {
  animation: fadeIn 200ms ease-out;
}

.content[data-state='closed'] {
  animation: fadeOut 200ms ease-in;
}

@keyframes fadeIn {
  from { opacity: 0; transform: scale(0.95); }
  to   { opacity: 1; transform: scale(1); }
}

@keyframes fadeOut {
  from { opacity: 1; transform: scale(1); }
  to   { opacity: 0; transform: scale(0.95); }
}
```

### Force-mounted content

To keep the content in the DOM even when closed (useful for CSS animations), use `forceMount` on the Portal:

#### React

```tsx
<HoverCard.Portal forceMount>
  <HoverCard.Content>...</HoverCard.Content>
</HoverCard.Portal>
```

#### Leptos

```rust
<HoverCardPortal force_mount=true>
  <HoverCardContent>"..."</HoverCardContent>
</HoverCardPortal>
```

### Non-portalled usage

The content can be rendered inline (without a portal) for cases where you need it in the local DOM tree:

#### React

```tsx
<HoverCard.Root>
  <HoverCard.Trigger href="/">trigger</HoverCard.Trigger>
  <HoverCard.Content sideOffset={5}>
    <HoverCard.Arrow width={20} height={10} />
    Content without portal.
  </HoverCard.Content>
</HoverCard.Root>
```

#### Leptos

```rust
<HoverCard>
  <HoverCardTrigger attr:href="/">"trigger"</HoverCardTrigger>
  <HoverCardContent side_offset=5.0>
    <HoverCardArrow width=20.0 height=10.0 />
    "Content without portal."
  </HoverCardContent>
</HoverCard>
```

## Accessibility

The HoverCard is not covered by a specific WAI-ARIA pattern. It is a non-modal, non-focusable overlay that appears on hover/focus of a trigger element (typically a link). It is designed for sighted mouse/pointer users to preview content behind a link.

### Keyboard Interactions

| Key | Description |
|---|---|
| `Escape` | Dismisses the hover card immediately (bypassing the close delay). |

### Behavioral Notes

- The trigger renders as an `<a>` element by default, making it naturally keyboard-focusable and link-navigable.
- Opening also occurs on `focus` of the trigger (without delay differentiation from pointer enter). Closing occurs on `blur`.
- Touch devices are excluded from pointer-based open/close logic. The `touchstart` event on the trigger calls `preventDefault()` to suppress the subsequent `focus` event on touch.
- All tabbable elements inside the hover card content have their `tabindex` set to `-1` at render time, preventing keyboard focus from entering the content. The content is meant to be read-only preview information.
- Text selection within the content is preserved: while a text selection is active, the hover card will not close on pointer leave. The content also manages `user-select` on the body during pointer-down to contain text selection within the content area.
- When the content is dismissed (via Escape, outside pointer down, or outside focus), it closes immediately without the close delay.

## CSS Custom Properties

These properties are set on `HoverCardContent` and alias the underlying `Popper` values. Use them in CSS for positioning-aware animations and layouts.

| Property | Source | Description |
|---|---|---|
| `--radix-hover-card-content-transform-origin` | `var(--radix-popper-transform-origin)` | The CSS transform origin computed from the content's placement relative to the trigger. Useful for scale/rotate animations that originate from the arrow direction. |
| `--radix-hover-card-content-available-width` | `var(--radix-popper-available-width)` | The available width between the trigger and the viewport edge on the content's side. |
| `--radix-hover-card-content-available-height` | `var(--radix-popper-available-height)` | The available height between the trigger and the viewport edge on the content's side. |
| `--radix-hover-card-trigger-width` | `var(--radix-popper-anchor-width)` | The width of the trigger element. Useful for matching content width to trigger width. |
| `--radix-hover-card-trigger-height` | `var(--radix-popper-anchor-height)` | The height of the trigger element. |
