# Toolbar (Root)

## Anatomy

The expected component nesting structure:

```
Toolbar
├── ToolbarButton
├── ToolbarLink
├── ToolbarSeparator
└── ToolbarToggleGroup
    └── ToolbarToggleItem
```

### React

```tsx
<Toolbar.Root>
  <Toolbar.Button>...</Toolbar.Button>
  <Toolbar.Link href="...">...</Toolbar.Link>
  <Toolbar.Separator />
  <Toolbar.ToggleGroup type="single">
    <Toolbar.ToggleItem value="left">...</Toolbar.ToggleItem>
    <Toolbar.ToggleItem value="center">...</Toolbar.ToggleItem>
  </Toolbar.ToggleGroup>
</Toolbar.Root>
```

### Leptos

```rust
<Toolbar>
  <ToolbarButton>"..."</ToolbarButton>
  <ToolbarLink attr:href="...">"..."</ToolbarLink>
  <ToolbarSeparator />
  <ToolbarToggleGroup r#type=ToggleGroupType::Single>
    <ToolbarToggleItem value="left">"..."</ToolbarToggleItem>
    <ToolbarToggleItem value="center">"..."</ToolbarToggleItem>
  </ToolbarToggleGroup>
</Toolbar>
```

## React Signature

```typescript
const Toolbar = React.forwardRef<ToolbarElement, ToolbarProps>(...)

type ToolbarElement = React.ComponentRef<typeof Primitive.div>;
type PrimitiveDivProps = React.ComponentPropsWithoutRef<typeof Primitive.div>;
type RovingFocusGroupProps = React.ComponentPropsWithoutRef<typeof RovingFocusGroup.Root>;

interface ToolbarProps extends PrimitiveDivProps {
  orientation?: RovingFocusGroupProps['orientation']; // 'horizontal' | 'vertical'
  loop?: RovingFocusGroupProps['loop'];
  dir?: RovingFocusGroupProps['dir']; // 'ltr' | 'rtl'
}
```

## Leptos Signature

```rust
pub fn Toolbar(
    #[prop(into, optional)] orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] r#loop: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `orientation` | `orientation` | `'horizontal' \| 'vertical'` (default `'horizontal'`) | `Option<Orientation>` (default `Horizontal`) | The layout axis. Controls which arrow keys navigate between focusable items (`ArrowLeft`/`ArrowRight` for horizontal, `ArrowUp`/`ArrowDown` for vertical). Also controls the perpendicular orientation of `ToolbarSeparator`. |
| `loop` | `r#loop` | `boolean` (default `true`) | `MaybeProp<bool>` (default `true`) | Whether keyboard navigation wraps from the last focusable item back to the first (and vice versa). |
| `dir` | `dir` | `'ltr' \| 'rtl'` | `MaybeProp<Direction>` | The reading direction. Affects horizontal arrow-key navigation: in RTL, `ArrowRight` moves to the previous item and `ArrowLeft` to the next. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the root DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| *(spread)* | — | `...PrimitiveDivProps` | — | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-orientation` | `"horizontal" \| "vertical"` | Reflects the `orientation` prop. |

### Implicit behavior

- Renders with `role="toolbar"` on the root `<div>`.
- Sets `aria-orientation` to the orientation value.
- Sets `dir` attribute on the DOM element to the resolved direction.
- Provides a `ToolbarContext` (orientation + direction) consumed by child parts (`ToolbarSeparator`, `ToolbarToggleGroup`).
- Wraps children in a `RovingFocusGroup` to manage arrow-key navigation among focusable toolbar items.

## Usage Examples

### Basic toolbar (horizontal)

#### React

```tsx
<Toolbar.Root aria-label="Formatting">
  <Toolbar.Button>Bold</Toolbar.Button>
  <Toolbar.Button>Italic</Toolbar.Button>
  <Toolbar.Separator />
  <Toolbar.Link href="https://example.com" target="_blank">
    Help
  </Toolbar.Link>
</Toolbar.Root>
```

#### Leptos

```rust
<Toolbar attr:aria-label="Formatting">
  <ToolbarButton>"Bold"</ToolbarButton>
  <ToolbarButton>"Italic"</ToolbarButton>
  <ToolbarSeparator />
  <ToolbarLink attr:href="https://example.com" attr:target="_blank">
    "Help"
  </ToolbarLink>
</Toolbar>
```

### With toggle group

#### React

```tsx
<Toolbar.Root aria-label="Text alignment">
  <Toolbar.ToggleGroup type="single" defaultValue="left">
    <Toolbar.ToggleItem value="left">Left</Toolbar.ToggleItem>
    <Toolbar.ToggleItem value="center">Center</Toolbar.ToggleItem>
    <Toolbar.ToggleItem value="right">Right</Toolbar.ToggleItem>
  </Toolbar.ToggleGroup>
</Toolbar.Root>
```

#### Leptos

```rust
<Toolbar attr:aria-label="Text alignment">
  <ToolbarToggleGroup
    r#type=ToggleGroupType::Single
    default_value=vec!["left".into()]
  >
    <ToolbarToggleItem value="left">"Left"</ToolbarToggleItem>
    <ToolbarToggleItem value="center">"Center"</ToolbarToggleItem>
    <ToolbarToggleItem value="right">"Right"</ToolbarToggleItem>
  </ToolbarToggleGroup>
</Toolbar>
```

### Vertical orientation

#### React

```tsx
<Toolbar.Root orientation="vertical" aria-label="Drawing tools">
  <Toolbar.Button>Pen</Toolbar.Button>
  <Toolbar.Button>Eraser</Toolbar.Button>
</Toolbar.Root>
```

#### Leptos

```rust
<Toolbar orientation=Orientation::Vertical attr:aria-label="Drawing tools">
  <ToolbarButton>"Pen"</ToolbarButton>
  <ToolbarButton>"Eraser"</ToolbarButton>
</Toolbar>
```

### Disabled button

#### React

```tsx
<Toolbar.Root>
  <Toolbar.Button>Undo</Toolbar.Button>
  <Toolbar.Button disabled>Redo</Toolbar.Button>
</Toolbar.Root>
```

#### Leptos

```rust
<Toolbar>
  <ToolbarButton>"Undo"</ToolbarButton>
  <ToolbarButton disabled=true>"Redo"</ToolbarButton>
</Toolbar>
```

## Accessibility

Implements the [WAI-ARIA Toolbar pattern](https://www.w3.org/WAI/ARIA/apd/patterns/toolbar/).

### Keyboard Interactions

| Key | Description |
|---|---|
| `Tab` | Moves focus into and out of the toolbar. When focus enters, it lands on the first focusable item (or the last focused item if returning). |
| `ArrowRight` | When `orientation="horizontal"` (default): moves focus to the next focusable item. In RTL, moves to the previous item. Wraps if `loop` is `true`. |
| `ArrowLeft` | When `orientation="horizontal"`: moves focus to the previous focusable item. In RTL, moves to the next item. Wraps if `loop` is `true`. |
| `ArrowDown` | When `orientation="vertical"`: moves focus to the next focusable item. Wraps if `loop` is `true`. |
| `ArrowUp` | When `orientation="vertical"`: moves focus to the previous focusable item. Wraps if `loop` is `true`. |
| `Home` | Moves focus to the first focusable item. |
| `End` | Moves focus to the last focusable item. |
| `Enter` / `Space` | Activates the focused button, toggle item, or link. For `ToolbarLink`, Space also triggers a click (since links do not natively respond to Space). |

### ARIA Attributes

| Element | Attribute | Value | Notes |
|---|---|---|---|
| `Toolbar` (root) | `role` | `"toolbar"` | Identifies the container as a toolbar widget. |
| `Toolbar` (root) | `aria-orientation` | `"horizontal" \| "vertical"` | Reflects the `orientation` prop. |
| `ToolbarSeparator` | `role` | `"separator"` or `"none"` | `"separator"` by default; `"none"` when `decorative` is `true`. |
| `ToolbarSeparator` | `aria-orientation` | `"vertical"` or absent | Only set when the separator's effective orientation is vertical (i.e., when the toolbar is horizontal). |
| `ToolbarToggleItem` (single mode) | `role` | `"radio"` | In single-selection mode, toggle items behave like radio buttons. |
| `ToolbarToggleItem` (single mode) | `aria-checked` | `true` / `false` | Reflects whether the item is pressed, used instead of `aria-pressed` in single mode. |
| `ToolbarToggleItem` (multiple mode) | `aria-pressed` | `true` / `false` | Reflects whether the item is pressed. Inherited from `Toggle`. |

### Behavioral Notes

- Disabled buttons (`ToolbarButton` with `disabled=true`) are skipped during keyboard navigation.
- `ToolbarLink` converts Space keypress to a click, since `<a>` elements do not natively respond to Space.
- `ToolbarToggleGroup` disables its own roving focus (`rovingFocus={false}`) because the parent `Toolbar`'s `RovingFocusGroup` already manages navigation. Each `ToolbarToggleItem` is individually wrapped in a `RovingFocusGroupItem` via `ToolbarButton`.
- Arrow key navigation prevents page scroll (`event.preventDefault()`) within the toolbar.
