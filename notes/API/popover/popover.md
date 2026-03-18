# Popover (Root)

## Anatomy

The expected component nesting structure:

```
Popover
├── PopoverAnchor (optional, custom anchor point)
├── PopoverTrigger
└── PopoverPortal
    └── PopoverContent
        ├── PopoverClose
        └── PopoverArrow
```

### React

```tsx
<Popover.Root>
  <Popover.Trigger>Open</Popover.Trigger>
  <Popover.Portal>
    <Popover.Content>
      <Popover.Close>Close</Popover.Close>
      <Popover.Arrow />
    </Popover.Content>
  </Popover.Portal>
</Popover.Root>
```

### Leptos

```rust
<Popover>
    <PopoverTrigger>"Open"</PopoverTrigger>
    <PopoverPortal>
        <PopoverContent>
            <PopoverClose>"Close"</PopoverClose>
            <PopoverArrow />
        </PopoverContent>
    </PopoverPortal>
</Popover>
```

## React Signature

```typescript
const Popover: React.FC<PopoverProps>

interface PopoverProps {
  children?: React.ReactNode;
  open?: boolean;
  defaultOpen?: boolean;
  onOpenChange?: (open: boolean) => void;
  modal?: boolean;
}
```

`Popover` is not a `forwardRef` component -- it is a plain `React.FC` that provides context and does not render a DOM element.

## Leptos Signature

```rust
pub fn Popover(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] modal: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `open` | `open` | `boolean \| undefined` | `MaybeProp<bool>` | The controlled open state. When set, the component becomes controlled and `onOpenChange` should be used to respond to state changes. |
| `defaultOpen` | `default_open` | `boolean \| undefined` | `MaybeProp<bool>` | The open state on initial render. Use when you do not need to control the open state externally. Defaults to `false`. |
| `onOpenChange` | `on_open_change` | `(open: boolean) => void` | `Option<Callback<bool>>` | Callback fired when the open state changes. Receives the new boolean value. |
| `modal` | `modal` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | When `true`, interaction with outside elements is disabled, scroll is locked, and other elements are hidden from assistive technology via `aria-hidden`. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | The popover parts to render. |

### Implicit behavior

- Provides a `PopoverContextValue` via context containing the open state, trigger ref, content ID, modal flag, and custom anchor tracking.
- Wraps children in an internal `Popper` component that provides the positioning infrastructure.
- Auto-generates a unique `id` for the content element, used for `aria-controls` on the trigger.

## Usage Examples

### Basic uncontrolled

#### React

```tsx
<Popover.Root>
  <Popover.Trigger>Open popover</Popover.Trigger>
  <Popover.Portal>
    <Popover.Content sideOffset={5}>
      <p>Popover content</p>
      <Popover.Close>Close</Popover.Close>
      <Popover.Arrow width={20} height={10} />
    </Popover.Content>
  </Popover.Portal>
</Popover.Root>
```

#### Leptos

```rust
<Popover>
    <PopoverTrigger>"Open popover"</PopoverTrigger>
    <PopoverPortal>
        <PopoverContent side_offset=5.0>
            <p>"Popover content"</p>
            <PopoverClose>"Close"</PopoverClose>
            <PopoverArrow width=20.0 height=10.0 />
        </PopoverContent>
    </PopoverPortal>
</Popover>
```

### Controlled

#### React

```tsx
const [open, setOpen] = React.useState(false);

<Popover.Root open={open} onOpenChange={setOpen}>
  <Popover.Trigger>{open ? 'close' : 'open'}</Popover.Trigger>
  <Popover.Portal>
    <Popover.Content>
      <Popover.Close>close</Popover.Close>
      <Popover.Arrow />
    </Popover.Content>
  </Popover.Portal>
</Popover.Root>
```

#### Leptos

```rust
let (open, set_open) = signal(false);

<Popover
    open=MaybeProp::derive(move || Some(open.get()))
    on_open_change=Callback::new(move |value: bool| set_open.set(value))
>
    <PopoverTrigger>
        {move || if open.get() { "close" } else { "open" }}
    </PopoverTrigger>
    <PopoverPortal>
        <PopoverContent>
            <PopoverClose>"close"</PopoverClose>
            <PopoverArrow />
        </PopoverContent>
    </PopoverPortal>
</Popover>
```

### Modal mode

#### React

```tsx
<Popover.Root modal>
  <Popover.Trigger>Open modal popover</Popover.Trigger>
  <Popover.Portal>
    <Popover.Content sideOffset={5}>
      <Popover.Close>Close</Popover.Close>
      <Popover.Arrow />
    </Popover.Content>
  </Popover.Portal>
</Popover.Root>
```

#### Leptos

```rust
<Popover modal=true>
    <PopoverTrigger>"Open modal popover"</PopoverTrigger>
    <PopoverPortal>
        <PopoverContent side_offset=5.0>
            <PopoverClose>"Close"</PopoverClose>
            <PopoverArrow />
        </PopoverContent>
    </PopoverPortal>
</Popover>
```

### Custom anchor

Use `PopoverAnchor` to position the popover relative to a different element than the trigger:

#### React

```tsx
<Popover.Root>
  <Popover.Anchor style={{ display: 'flex', alignItems: 'center', width: 250 }}>
    Item <Popover.Trigger>open</Popover.Trigger>
  </Popover.Anchor>
  <Popover.Portal>
    <Popover.Content side="right" sideOffset={1} align="start">
      <Popover.Close>close</Popover.Close>
    </Popover.Content>
  </Popover.Portal>
</Popover.Root>
```

#### Leptos

```rust
<Popover>
    <PopoverAnchor attr:style="display: flex; align-items: center; width: 250px;">
        "Item "
        <PopoverTrigger>"open"</PopoverTrigger>
    </PopoverAnchor>
    <PopoverPortal>
        <PopoverContent side=Side::Right side_offset=1.0 align=Align::Start>
            <PopoverClose>"close"</PopoverClose>
        </PopoverContent>
    </PopoverPortal>
</Popover>
```

### Animated content

Use CSS keyframes targeting `data-state` to animate open/close:

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

## Accessibility

Implements a dialog-style popover following the [WAI-ARIA Dialog pattern](https://www.w3.org/WAI/ARIA/apd/patterns/dialog-modal/), adapted for non-modal use by default.

### Keyboard Interactions

| Key | Description |
|---|---|
| `Enter` / `Space` | When focus is on the trigger, toggles the popover open/closed. |
| `Escape` | Closes the popover and returns focus to the trigger (non-modal) or container (modal, with two-escape pattern for text inputs). |
| `Tab` | In modal mode, focus is trapped within the content. In non-modal mode, Tab moves focus normally and the popover closes when focus leaves. |

### ARIA Attributes

| Element | Attribute | Value | Notes |
|---|---|---|---|
| `PopoverTrigger` | `aria-haspopup` | `"dialog"` | Indicates the trigger opens a dialog-type popup. |
| `PopoverTrigger` | `aria-expanded` | `"true" \| "false"` | Reflects the popover's open state. |
| `PopoverTrigger` | `aria-controls` | `string` | Points to the content element's auto-generated `id`. |
| `PopoverContent` | `role` | `"dialog"` | Identifies the content as a dialog. |
| `PopoverContent` | `id` | `string` | Auto-generated, linked via trigger's `aria-controls`. |

### Behavioral Notes

- In **non-modal** mode (default), clicking outside the popover closes it. Clicking the trigger while open closes it. Focus is returned to the trigger on close unless the user interacted with an outside element.
- In **modal** mode, outside pointer events are disabled, scroll is locked, and `aria-hidden` is applied to all elements outside the content. Focus is trapped inside the content and returned to the trigger on close.
- Clicking the trigger while the popover is open correctly toggles it closed in non-modal mode, preventing a close-then-immediately-reopen race.
- Right-clicking outside a modal popover does not return focus to the trigger on close (to avoid interfering with context menus).

## CSS Custom Properties

These properties are set on `PopoverContent` and alias the underlying Popper values. Use them in CSS to size the popover relative to its trigger or available viewport space.

| Property | Source | Description |
|---|---|---|
| `--radix-popover-content-transform-origin` | `var(--radix-popper-transform-origin)` | The CSS transform origin computed from the content's position relative to the trigger. Useful for scale animations. |
| `--radix-popover-content-available-width` | `var(--radix-popper-available-width)` | The remaining width between the trigger and the viewport edge. |
| `--radix-popover-content-available-height` | `var(--radix-popper-available-height)` | The remaining height between the trigger and the viewport edge. |
| `--radix-popover-trigger-width` | `var(--radix-popper-anchor-width)` | The width of the trigger (or custom anchor). |
| `--radix-popover-trigger-height` | `var(--radix-popper-anchor-height)` | The height of the trigger (or custom anchor). |
