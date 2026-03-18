# Tooltip (Root)

## Anatomy

The expected component nesting structure:

```
TooltipProvider
└── Tooltip
    ├── TooltipTrigger
    └── TooltipPortal
        └── TooltipContent
            └── TooltipArrow (optional)
```

### React

```tsx
<Tooltip.Provider>
  <Tooltip.Root>
    <Tooltip.Trigger>...</Tooltip.Trigger>
    <Tooltip.Portal>
      <Tooltip.Content>
        ...
        <Tooltip.Arrow />
      </Tooltip.Content>
    </Tooltip.Portal>
  </Tooltip.Root>
</Tooltip.Provider>
```

### Leptos

```rust
<TooltipProvider>
  <Tooltip>
    <TooltipTrigger>"..."</TooltipTrigger>
    <TooltipPortal>
      <TooltipContent>
        "..."
        <TooltipArrow />
      </TooltipContent>
    </TooltipPortal>
  </Tooltip>
</TooltipProvider>
```

## React Signature

```typescript
const Tooltip: React.FC<TooltipProps> = (props) => { ... }

interface TooltipProps {
  children?: React.ReactNode;
  open?: boolean;
  defaultOpen?: boolean;
  onOpenChange?: (open: boolean) => void;
  /**
   * The duration from when the pointer enters the trigger until the tooltip gets opened. This will
   * override the prop with the same name passed to Provider.
   * @defaultValue 700
   */
  delayDuration?: number;
  /**
   * When `true`, trying to hover the content will result in the tooltip closing as the pointer leaves the trigger.
   * @defaultValue false
   */
  disableHoverableContent?: boolean;
}
```

Note: `Tooltip` is an `FC` (not `forwardRef`) because it does not render a DOM element itself — it is a state/context container wrapping `PopperPrimitive.Root`.

## Leptos Signature

```rust
pub fn Tooltip(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] delay_duration: MaybeProp<f64>,
    #[prop(into, optional)] disable_hoverable_content: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `open` | `open` | `boolean \| undefined` | `MaybeProp<bool>` | The controlled open state of the tooltip. When set, the component becomes controlled. |
| `defaultOpen` | `default_open` | `boolean \| undefined` (default `false`) | `MaybeProp<bool>` (default `false`) | The open state on initial render. Use when you do not need to control tooltip state. |
| `onOpenChange` | `on_open_change` | `(open: boolean) => void` | `Option<Callback<bool>>` | Callback fired when the open state changes. Receives the new boolean state. |
| `delayDuration` | `delay_duration` | `number` (default `700`) | `MaybeProp<f64>` (default inherits from provider) | The duration in milliseconds from when the pointer enters the trigger until the tooltip opens. Overrides the same prop on `TooltipProvider` for this tooltip instance. |
| `disableHoverableContent` | `disable_hoverable_content` | `boolean` (default `false`) | `MaybeProp<bool>` (default inherits from provider) | When `true`, the tooltip closes immediately when the pointer leaves the trigger. When `false` (default), the user can move the pointer to the content without the tooltip closing. Overrides the same prop on `TooltipProvider` for this tooltip instance. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | The trigger and portal/content parts. |

### Implicit behavior

- Wraps children in a `PopperPrimitive.Root` (Leptos: `Popper`) for positioning context.
- Provides a `TooltipContextValue` to descendants, containing the content ID, open state, state attribute, trigger element ref, and open/close handlers.
- Dispatches a `tooltip.open` custom event on the `document` when opening, causing any other open tooltip to close.
- Manages delay timers: when the provider's `isOpenDelayed` flag is true, opening is delayed by `delayDuration`; otherwise, the tooltip opens instantly (skip-delay behavior after a recently closed tooltip).

## Usage Examples

### Basic (uncontrolled)

#### React

```tsx
<Tooltip.Provider>
  <Tooltip.Root>
    <Tooltip.Trigger>Hover me</Tooltip.Trigger>
    <Tooltip.Portal>
      <Tooltip.Content sideOffset={5}>
        Tooltip text
        <Tooltip.Arrow />
      </Tooltip.Content>
    </Tooltip.Portal>
  </Tooltip.Root>
</Tooltip.Provider>
```

#### Leptos

```rust
<TooltipProvider>
  <Tooltip>
    <TooltipTrigger>"Hover me"</TooltipTrigger>
    <TooltipPortal>
      <TooltipContent side_offset=5.0>
        "Tooltip text"
        <TooltipArrow />
      </TooltipContent>
    </TooltipPortal>
  </Tooltip>
</TooltipProvider>
```

### Controlled

#### React

```tsx
const [open, setOpen] = React.useState(false);

<Tooltip.Provider>
  <Tooltip.Root open={open} onOpenChange={setOpen}>
    <Tooltip.Trigger>Hover me</Tooltip.Trigger>
    <Tooltip.Portal>
      <Tooltip.Content sideOffset={5}>
        Tooltip text
        <Tooltip.Arrow />
      </Tooltip.Content>
    </Tooltip.Portal>
  </Tooltip.Root>
</Tooltip.Provider>
```

#### Leptos

```rust
let (open, set_open) = signal(false);

<TooltipProvider>
  <Tooltip
    open=open
    on_open_change=Callback::new(move |val: bool| set_open.set(val))
  >
    <TooltipTrigger>"Hover me"</TooltipTrigger>
    <TooltipPortal>
      <TooltipContent side_offset=5.0>
        "Tooltip text"
        <TooltipArrow />
      </TooltipContent>
    </TooltipPortal>
  </Tooltip>
</TooltipProvider>
```

### Custom delay duration

#### React

```tsx
<Tooltip.Provider delayDuration={0}>
  <Tooltip.Root>
    <Tooltip.Trigger>Instant tooltip</Tooltip.Trigger>
    <Tooltip.Portal>
      <Tooltip.Content>Opens immediately</Tooltip.Content>
    </Tooltip.Portal>
  </Tooltip.Root>
</Tooltip.Provider>
```

#### Leptos

```rust
<TooltipProvider delay_duration=0.0>
  <Tooltip>
    <TooltipTrigger>"Instant tooltip"</TooltipTrigger>
    <TooltipPortal>
      <TooltipContent>"Opens immediately"</TooltipContent>
    </TooltipPortal>
  </Tooltip>
</TooltipProvider>
```

### Disable hoverable content

#### React

```tsx
<Tooltip.Provider disableHoverableContent>
  <Tooltip.Root>
    <Tooltip.Trigger>Hover me</Tooltip.Trigger>
    <Tooltip.Portal>
      <Tooltip.Content>Closes when pointer leaves trigger</Tooltip.Content>
    </Tooltip.Portal>
  </Tooltip.Root>
</Tooltip.Provider>
```

#### Leptos

```rust
<TooltipProvider disable_hoverable_content=true>
  <Tooltip>
    <TooltipTrigger>"Hover me"</TooltipTrigger>
    <TooltipPortal>
      <TooltipContent>"Closes when pointer leaves trigger"</TooltipContent>
    </TooltipPortal>
  </Tooltip>
</TooltipProvider>
```

### Tooltip with aria-label

#### React

```tsx
<Tooltip.Provider>
  <Tooltip.Root>
    <Tooltip.Trigger>
      <span aria-hidden>bell icon (3)</span>
    </Tooltip.Trigger>
    <Tooltip.Portal>
      <Tooltip.Content aria-label="3 notifications">
        Notifications
        <Tooltip.Arrow />
      </Tooltip.Content>
    </Tooltip.Portal>
  </Tooltip.Root>
</Tooltip.Provider>
```

#### Leptos

```rust
<TooltipProvider>
  <Tooltip>
    <TooltipTrigger>
      <span aria-hidden="true">"bell icon (3)"</span>
    </TooltipTrigger>
    <TooltipPortal>
      <TooltipContent aria_label="3 notifications">
        "Notifications"
        <TooltipArrow />
      </TooltipContent>
    </TooltipPortal>
  </Tooltip>
</TooltipProvider>
```

## Accessibility

Implements the [WAI-ARIA Tooltip pattern](https://www.w3.org/WAI/ARIA/apd/patterns/tooltip/).

### Keyboard Interactions

| Key | Description |
|---|---|
| `Tab` | Opens/closes the tooltip by focusing/blurring the trigger. When the trigger receives focus (not from a pointer down), the tooltip opens instantly (no delay). |
| `Escape` | Closes the tooltip if open (handled by `DismissableLayer`). |

### ARIA Attributes

| Element | Attribute | Value | Notes |
|---|---|---|---|
| `TooltipTrigger` | `aria-describedby` | `string \| undefined` | Points to the visually hidden tooltip content element's `id` when open. Removed when closed. |
| Visually hidden element inside `TooltipContent` | `role` | `"tooltip"` | The actual `role="tooltip"` is placed on a `VisuallyHidden` element inside the content, not on the visible content element. This allows rich content in the visible tooltip while the accessible name is either `aria-label` or the text content. |
| Visually hidden element inside `TooltipContent` | `id` | auto-generated | The ID referenced by the trigger's `aria-describedby`. |

### Behavioral Notes

- The tooltip opens on `pointerMove` (not `pointerEnter`) to avoid opening when the cursor is passing over the trigger quickly. Touch pointer types are ignored.
- A `pointerDown` on the trigger closes an open tooltip immediately.
- Keyboard focus opens the tooltip instantly (no delay), but only if the focus was not caused by a pointer down event.
- Clicking the trigger closes the tooltip.
- When the trigger is scrolled out of view, the tooltip closes.
- Opening a tooltip dispatches a `tooltip.open` document event, causing all other open tooltips to close. Only one tooltip can be open at a time.
- When `disableHoverableContent` is `false` (default), a convex-hull "grace area" is computed between the trigger and content elements. The pointer can move through this area without closing the tooltip, enabling the user to hover the content itself.

## CSS Custom Properties

These properties are set on `TooltipContent` and alias the underlying Popper values. Use them in CSS to size or position the tooltip relative to the trigger and available space.

| Property | Source | Description |
|---|---|---|
| `--radix-tooltip-content-transform-origin` | `var(--radix-popper-transform-origin)` | The CSS transform origin computed by the positioning engine. Useful for scale animations that originate from the anchor point. |
| `--radix-tooltip-content-available-width` | `var(--radix-popper-available-width)` | The available width between the trigger and the viewport edge. |
| `--radix-tooltip-content-available-height` | `var(--radix-popper-available-height)` | The available height between the trigger and the viewport edge. |
| `--radix-tooltip-trigger-width` | `var(--radix-popper-anchor-width)` | The width of the trigger element. |
| `--radix-tooltip-trigger-height` | `var(--radix-popper-anchor-height)` | The height of the trigger element. |
