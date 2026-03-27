# Collapsible (Root)

## Anatomy

The expected component nesting structure:

```
Collapsible
├── CollapsibleTrigger
└── CollapsibleContent
```

### React

```tsx
<Collapsible.Root>
  <Collapsible.Trigger>...</Collapsible.Trigger>
  <Collapsible.Content>...</Collapsible.Content>
</Collapsible.Root>
```

### Leptos

```rust
<Collapsible>
  <CollapsibleTrigger>"..."</CollapsibleTrigger>
  <CollapsibleContent>"..."</CollapsibleContent>
</Collapsible>
```

## React Signature

```typescript
const Collapsible = React.forwardRef<CollapsibleElement, CollapsibleProps>(...)

type CollapsibleElement = React.ComponentRef<typeof Primitive.div>;
type PrimitiveDivProps = React.ComponentPropsWithoutRef<typeof Primitive.div>;

interface CollapsibleProps extends PrimitiveDivProps {
  defaultOpen?: boolean;
  open?: boolean;
  disabled?: boolean;
  onOpenChange?(open: boolean): void;
}
```

## Leptos Signature

```rust
pub fn Collapsible(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `open` | `open` | `boolean \| undefined` | `MaybeProp<bool>` | The controlled open state. When set, the component becomes controlled and `onOpenChange` must be used to update the state. |
| `defaultOpen` | `default_open` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | The open state on initial render. Use when you do not need to control the open state. |
| `disabled` | `disabled` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | Disables the collapsible. When `true`, the trigger cannot be clicked and `data-disabled` is set on all parts. |
| `onOpenChange` | `on_open_change` | `(open: boolean) => void` | `Option<Callback<bool>>` | Callback fired when the open state changes. Receives the new boolean value. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the root DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs onto the child. |
| *(spread)* | — | `...PrimitiveDivProps` | — | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives on the call site instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"open" \| "closed"` | Reflects the current open/closed state of the collapsible. |
| `data-disabled` | `""` (present/absent) | Present when the `disabled` prop is `true`. |

### Implicit behavior

- A unique `content_id` is auto-generated via `useId()` (React) / `use_id()` (Leptos) and provided to children through context. This ID is used by `CollapsibleTrigger` for `aria-controls` and by `CollapsibleContent` as its element `id`.
- The `open` state and `disabled` state are provided to children through context, so `CollapsibleTrigger` and `CollapsibleContent` automatically reflect the root's state.

## Usage Examples

### Uncontrolled (default closed)

#### React

```tsx
<Collapsible.Root>
  <Collapsible.Trigger>Toggle</Collapsible.Trigger>
  <Collapsible.Content>Collapsible content here.</Collapsible.Content>
</Collapsible.Root>
```

#### Leptos

```rust
<Collapsible>
  <CollapsibleTrigger>"Toggle"</CollapsibleTrigger>
  <CollapsibleContent>"Collapsible content here."</CollapsibleContent>
</Collapsible>
```

### Uncontrolled (default open)

#### React

```tsx
<Collapsible.Root defaultOpen>
  <Collapsible.Trigger>Toggle</Collapsible.Trigger>
  <Collapsible.Content>Collapsible content here.</Collapsible.Content>
</Collapsible.Root>
```

#### Leptos

```rust
<Collapsible default_open=true>
  <CollapsibleTrigger>"Toggle"</CollapsibleTrigger>
  <CollapsibleContent>"Collapsible content here."</CollapsibleContent>
</Collapsible>
```

### Controlled

#### React

```tsx
const [open, setOpen] = React.useState(false);

<Collapsible.Root open={open} onOpenChange={setOpen}>
  <Collapsible.Trigger>{open ? 'close' : 'open'}</Collapsible.Trigger>
  <Collapsible.Content>Collapsible content here.</Collapsible.Content>
</Collapsible.Root>
```

#### Leptos

```rust
let (open, set_open) = signal(false);

<Collapsible
  open=open
  on_open_change=Callback::new(move |value| set_open.set(value))
>
  <CollapsibleTrigger>
    {move || if open.get() { "close" } else { "open" }}
  </CollapsibleTrigger>
  <CollapsibleContent>"Collapsible content here."</CollapsibleContent>
</Collapsible>
```

### Disabled

#### React

```tsx
<Collapsible.Root disabled>
  <Collapsible.Trigger>Toggle</Collapsible.Trigger>
  <Collapsible.Content>Content</Collapsible.Content>
</Collapsible.Root>
```

#### Leptos

```rust
<Collapsible disabled=true>
  <CollapsibleTrigger>"Toggle"</CollapsibleTrigger>
  <CollapsibleContent>"Content"</CollapsibleContent>
</Collapsible>
```

### Animated content (CSS keyframes)

Use the `--collapsible-content-height` and `--collapsible-content-width` CSS custom properties to animate open/close transitions:

```css
.content {
  overflow: hidden;
}

.content[data-state='open'] {
  animation: slideDown 300ms ease-out;
}

.content[data-state='closed'] {
  animation: slideUp 300ms ease-in;
}

@keyframes slideDown {
  from { height: 0; }
  to   { height: var(--collapsible-content-height); }
}

@keyframes slideUp {
  from { height: var(--collapsible-content-height); }
  to   { height: 0; }
}
```

Horizontal animation works the same way with `--collapsible-content-width`:

```css
.content[data-state='open'] {
  animation: openRight 300ms ease-out;
}

.content[data-state='closed'] {
  animation: closeRight 300ms ease-in;
}

@keyframes openRight {
  from { width: 0; }
  to   { width: var(--collapsible-content-width); }
}

@keyframes closeRight {
  from { width: var(--collapsible-content-width); }
  to   { width: 0; }
}
```

## Accessibility

Implements the [WAI-ARIA Disclosure (Show/Hide) pattern](https://www.w3.org/WAI/ARIA/apd/patterns/disclosure/).

### Keyboard Interactions

| Key | Description |
|---|---|
| `Enter` / `Space` | When focus is on the trigger, toggles the content open/closed. |

### ARIA Attributes

| Element | Attribute | Value | Notes |
|---|---|---|---|
| `CollapsibleTrigger` | `aria-expanded` | `"true" \| "false"` | Reflects the collapsible's open state. |
| `CollapsibleTrigger` | `aria-controls` | `string` | Points to the `CollapsibleContent` element's auto-generated `id`. |
| `CollapsibleContent` | `id` | `string` | Auto-generated ID referenced by the trigger's `aria-controls`. |
| `CollapsibleContent` | `hidden` | `""` (present/absent) | Present when the content is not open (both `context.open` and `isPresent` are false). Removed during exit animations to allow them to complete. |

### Behavioral Notes

- The trigger renders as a `<button type="button">`, providing native keyboard activation via `Enter` and `Space`.
- When `disabled` is `true`, the trigger's native `disabled` attribute is set, which prevents focus and click events at the browser level.
- Collapsible has no concept of `collapsible=false` (unlike Accordion) -- it can always be toggled both open and closed.
- In controlled mode (`open` is set), clicking the trigger fires `onOpenChange` but does not change the visual state until the parent updates the `open` prop.

## CSS Custom Properties

These properties are set directly on the `CollapsibleContent` element's inline style. Use them in CSS animations or transitions to animate to/from the content's natural dimensions.

| Property | Source | Description |
|---|---|---|
| `--collapsible-content-height` | Measured via `getBoundingClientRect().height` | The measured height (in `px`) of the content when fully expanded. |
| `--collapsible-content-width` | Measured via `getBoundingClientRect().width` | The measured width (in `px`) of the content when fully expanded. |

These properties update dynamically when the content's dimensions change (e.g., from dynamic content being added). On initial mount, animations are suppressed to prevent a flash; subsequent open/close transitions run normally.
