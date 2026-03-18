# Tabs (Root)

## Anatomy

The expected component nesting structure:

```
Tabs
├── TabsList
│   ├── TabsTrigger (one per tab, each with a unique `value`)
│   ├── TabsTrigger
│   └── ...
├── TabsContent (one per tab, `value` matches its trigger)
├── TabsContent
└── ...
```

### React

```tsx
<Tabs.Root defaultValue="tab1">
  <Tabs.List aria-label="tabs example">
    <Tabs.Trigger value="tab1">Tab 1</Tabs.Trigger>
    <Tabs.Trigger value="tab2">Tab 2</Tabs.Trigger>
  </Tabs.List>
  <Tabs.Content value="tab1">Content one.</Tabs.Content>
  <Tabs.Content value="tab2">Content two.</Tabs.Content>
</Tabs.Root>
```

### Leptos

```rust
<Tabs default_value="tab1".to_string()>
  <TabsList attr:aria-label="tabs example">
    <TabsTrigger value="tab1".to_string()>"Tab 1"</TabsTrigger>
    <TabsTrigger value="tab2".to_string()>"Tab 2"</TabsTrigger>
  </TabsList>
  <TabsContent value="tab1".to_string()>"Content one."</TabsContent>
  <TabsContent value="tab2".to_string()>"Content two."</TabsContent>
</Tabs>
```

## React Signature

```typescript
const Tabs = React.forwardRef<TabsElement, TabsProps>(...)

type TabsElement = React.ComponentRef<typeof Primitive.div>;
type PrimitiveDivProps = React.ComponentPropsWithoutRef<typeof Primitive.div>;
type RovingFocusGroupProps = React.ComponentPropsWithoutRef<typeof RovingFocusGroup.Root>;

interface TabsProps extends PrimitiveDivProps {
  /** The value for the selected tab, if controlled */
  value?: string;
  /** The value of the tab to select by default, if uncontrolled */
  defaultValue?: string;
  /** A function called when a new tab is selected */
  onValueChange?: (value: string) => void;
  /**
   * The orientation the tabs are layed out.
   * Mainly so arrow navigation is done accordingly (left & right vs. up & down)
   * @defaultValue horizontal
   */
  orientation?: RovingFocusGroupProps['orientation'];
  /**
   * The direction of navigation between toolbar items.
   */
  dir?: RovingFocusGroupProps['dir'];
  /**
   * Whether a tab is activated automatically or manually.
   * @defaultValue automatic
   */
  activationMode?: 'automatic' | 'manual';
}
```

## Leptos Signature

```rust
pub fn Tabs(
    /// The controlled value of the active tab.
    #[prop(into, optional)]
    value: MaybeProp<String>,
    /// The default active tab value (uncontrolled).
    #[prop(into, optional)]
    default_value: MaybeProp<String>,
    /// Callback when the active tab changes.
    #[prop(into, optional)]
    on_value_change: Option<Callback<String>>,
    /// The orientation of the tabs. Determines arrow key navigation direction.
    #[prop(into, optional)]
    orientation: MaybeProp<Orientation>,
    /// The reading direction.
    #[prop(into, optional)]
    dir: MaybeProp<Direction>,
    /// Whether tabs activate automatically on focus or manually on click/Enter.
    #[prop(into, optional)]
    activation_mode: MaybeProp<ActivationMode>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `value` | `value` | `string \| undefined` | `MaybeProp<String>` | The controlled value of the currently active tab. When set, the component becomes controlled and only changes via `onValueChange`. |
| `defaultValue` | `default_value` | `string \| undefined` | `MaybeProp<String>` | The value of the tab to select on initial render. Use when you do not need to control tab state externally. React defaults to `''` internally if omitted. |
| `onValueChange` | `on_value_change` | `(value: string) => void` | `Option<Callback<String>>` | Callback fired when the active tab changes. Receives the new tab value string. |
| `orientation` | `orientation` | `'horizontal' \| 'vertical'` (default `'horizontal'`) | `MaybeProp<Orientation>` (default `Horizontal`) | The layout axis the tabs operate along. Controls which arrow keys navigate between triggers (`ArrowLeft`/`ArrowRight` for horizontal, `ArrowUp`/`ArrowDown` for vertical). Also sets `aria-orientation` on the tab list. |
| `dir` | `dir` | `'ltr' \| 'rtl'` | `MaybeProp<Direction>` | The reading direction. Affects horizontal arrow-key navigation: in RTL, `ArrowRight` moves to the previous trigger and `ArrowLeft` to the next. |
| `activationMode` | `activation_mode` | `'automatic' \| 'manual'` (default `'automatic'`) | `MaybeProp<ActivationMode>` (default `Automatic`) | When `automatic`, tabs activate immediately when they receive focus via keyboard navigation. When `manual`, tabs only activate on explicit click or `Enter`/`Space` keypress. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the root DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, the component renders its child directly instead of wrapping in a `<div>`, merging props and refs onto the child. |
| *(spread)* | -- | `...PrimitiveDivProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives on the call site instead. |

### Leptos-only: `ActivationMode` enum

```rust
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ActivationMode {
    #[default]
    Automatic,
    Manual,
}
```

### Leptos-only: `Orientation` enum (re-exported from `roving_focus`)

```rust
pub enum Orientation {
    Horizontal,
    Vertical,
}
```

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-orientation` | `"horizontal" \| "vertical"` | Reflects the `orientation` prop. |

### Implicit behavior

- A `dir` HTML attribute is set on the rendered `<div>`, reflecting the resolved reading direction.
- A `base_id` is auto-generated via `use_id()` and provided through context. It is used by `TabsTrigger` and `TabsContent` to derive deterministic IDs for ARIA wiring.

## Usage Examples

### Uncontrolled (automatic activation)

#### React

```tsx
<Tabs.Root defaultValue="tab1">
  <Tabs.List aria-label="tabs example">
    <Tabs.Trigger value="tab1">Tab 1</Tabs.Trigger>
    <Tabs.Trigger value="tab2" disabled>Tab 2</Tabs.Trigger>
    <Tabs.Trigger value="tab3">Tab 3</Tabs.Trigger>
  </Tabs.List>
  <Tabs.Content value="tab1">Content for tab one.</Tabs.Content>
  <Tabs.Content value="tab2">Content for tab two.</Tabs.Content>
  <Tabs.Content value="tab3">Content for tab three.</Tabs.Content>
</Tabs.Root>
```

#### Leptos

```rust
<Tabs default_value="tab1".to_string()>
  <TabsList attr:aria-label="tabs example">
    <TabsTrigger value="tab1".to_string()>"Tab 1"</TabsTrigger>
    <TabsTrigger value="tab2".to_string() disabled=true>"Tab 2"</TabsTrigger>
    <TabsTrigger value="tab3".to_string()>"Tab 3"</TabsTrigger>
  </TabsList>
  <TabsContent value="tab1".to_string()>"Content for tab one."</TabsContent>
  <TabsContent value="tab2".to_string()>"Content for tab two."</TabsContent>
  <TabsContent value="tab3".to_string()>"Content for tab three."</TabsContent>
</Tabs>
```

### Controlled

#### React

```tsx
const [value, setValue] = React.useState('tab1');

<Tabs.Root value={value} onValueChange={setValue}>
  <Tabs.List aria-label="tabs example">
    <Tabs.Trigger value="tab1">Tab 1</Tabs.Trigger>
    <Tabs.Trigger value="tab2">Tab 2</Tabs.Trigger>
  </Tabs.List>
  <Tabs.Content value="tab1">Content one.</Tabs.Content>
  <Tabs.Content value="tab2">Content two.</Tabs.Content>
</Tabs.Root>
```

#### Leptos

```rust
let (value, set_value) = signal("tab1".to_string());

<Tabs
  value=value
  on_value_change=Callback::new(move |v: String| set_value.set(v))
>
  <TabsList attr:aria-label="tabs example">
    <TabsTrigger value="tab1".to_string()>"Tab 1"</TabsTrigger>
    <TabsTrigger value="tab2".to_string()>"Tab 2"</TabsTrigger>
  </TabsList>
  <TabsContent value="tab1".to_string()>"Content one."</TabsContent>
  <TabsContent value="tab2".to_string()>"Content two."</TabsContent>
</Tabs>
```

### Vertical with manual activation

#### React

```tsx
<Tabs.Root defaultValue="tab1" orientation="vertical" activationMode="manual">
  <Tabs.List aria-label="tabs example">
    <Tabs.Trigger value="tab1">Tab 1</Tabs.Trigger>
    <Tabs.Trigger value="tab2">Tab 2</Tabs.Trigger>
  </Tabs.List>
  <Tabs.Content value="tab1">Content one.</Tabs.Content>
  <Tabs.Content value="tab2">Content two.</Tabs.Content>
</Tabs.Root>
```

#### Leptos

```rust
<Tabs
  default_value="tab1".to_string()
  orientation=Orientation::Vertical
  activation_mode=ActivationMode::Manual
>
  <TabsList attr:aria-label="tabs example">
    <TabsTrigger value="tab1".to_string()>"Tab 1"</TabsTrigger>
    <TabsTrigger value="tab2".to_string()>"Tab 2"</TabsTrigger>
  </TabsList>
  <TabsContent value="tab1".to_string()>"Content one."</TabsContent>
  <TabsContent value="tab2".to_string()>"Content two."</TabsContent>
</Tabs>
```

### RTL direction

#### React

```tsx
<Tabs.Root defaultValue="tab1" dir="rtl">
  {/* ...list, triggers, content */}
</Tabs.Root>
```

#### Leptos

```rust
<Tabs default_value="tab1".to_string() dir=Direction::Rtl>
  // ...list, triggers, content
</Tabs>
```

### Force-mounted content (for animation control)

#### React

```tsx
<Tabs.Root defaultValue="tab1">
  <Tabs.List aria-label="tabs example">
    <Tabs.Trigger value="tab1">Tab 1</Tabs.Trigger>
    <Tabs.Trigger value="tab2">Tab 2</Tabs.Trigger>
  </Tabs.List>
  <Tabs.Content value="tab1" forceMount>Content one.</Tabs.Content>
  <Tabs.Content value="tab2" forceMount>Content two.</Tabs.Content>
</Tabs.Root>
```

#### Leptos

```rust
<Tabs default_value="tab1".to_string()>
  <TabsList attr:aria-label="tabs example">
    <TabsTrigger value="tab1".to_string()>"Tab 1"</TabsTrigger>
    <TabsTrigger value="tab2".to_string()>"Tab 2"</TabsTrigger>
  </TabsList>
  <TabsContent value="tab1".to_string() force_mount=true>"Content one."</TabsContent>
  <TabsContent value="tab2".to_string() force_mount=true>"Content two."</TabsContent>
</Tabs>
```

## Accessibility

Implements the [WAI-ARIA Tabs pattern](https://www.w3.org/WAI/ARIA/apd/patterns/tabs/).

### Keyboard Interactions

| Key | Description |
|---|---|
| `Enter` / `Space` | When focus is on a trigger, activates the associated tab panel. In `automatic` mode this is redundant (the tab activates on focus), but in `manual` mode this is the primary activation mechanism. |
| `ArrowRight` | When `orientation="horizontal"` (default): moves focus to the next trigger. Wraps from last to first. In RTL, moves to the previous trigger. In `automatic` mode, also activates the newly focused tab. |
| `ArrowLeft` | When `orientation="horizontal"`: moves focus to the previous trigger. Wraps from first to last. In RTL, moves to the next trigger. In `automatic` mode, also activates the newly focused tab. |
| `ArrowDown` | When `orientation="vertical"`: moves focus to the next trigger. Wraps from last to first. In `automatic` mode, also activates the newly focused tab. |
| `ArrowUp` | When `orientation="vertical"`: moves focus to the previous trigger. Wraps from first to last. In `automatic` mode, also activates the newly focused tab. |
| `Home` | Moves focus to the first trigger. |
| `End` | Moves focus to the last trigger. |
| `Tab` | When focus enters the tab list, focuses the active trigger. When focus is on a trigger, `Tab` moves focus to the active tab panel (`tabindex="0"`). |

### ARIA Attributes

| Element | Attribute | Value | Notes |
|---|---|---|---|
| `TabsList` | `role` | `"tablist"` | Identifies the element as a tab list container. |
| `TabsList` | `aria-orientation` | `"horizontal" \| "vertical"` | Reflects the parent `Tabs` orientation. |
| `TabsTrigger` | `role` | `"tab"` | Identifies the element as a tab selector. |
| `TabsTrigger` | `aria-selected` | `"true" \| "false"` | `"true"` when this trigger's tab panel is active. |
| `TabsTrigger` | `aria-controls` | `string` | Points to the corresponding `TabsContent` element's auto-generated `id`. |
| `TabsContent` | `role` | `"tabpanel"` | Identifies the element as a tab panel. |
| `TabsContent` | `aria-labelledby` | `string` | Points to the corresponding `TabsTrigger`'s auto-generated `id`. |
| `TabsContent` | `tabindex` | `"0"` | Makes the panel focusable so keyboard users can `Tab` into the panel content. |

### Behavioral Notes

- Disabled triggers are skipped during roving focus keyboard navigation.
- In `automatic` mode (the default), navigating to a trigger with arrow keys immediately activates that tab. In `manual` mode, arrow keys only move focus; activation requires `Enter`, `Space`, or a click.
- The Leptos implementation defers automatic activation to a macrotask (`setTimeout(0)`) to avoid re-entrant signal updates during the focus handler. This is a WASM-specific adaptation -- React batches `setState` calls, but Leptos signal updates are synchronous.
- On initial mount, `TabsContent` suppresses entry animations by setting `animation-duration: 0s` for one animation frame (via `requestAnimationFrame`). This prevents a visual flash when the initially selected content appears.
- The `hidden` attribute: React sets `hidden={!present}` on `TabsContent` so force-mounted inactive panels are hidden. The Leptos implementation relies on the `Presence` component for mount/unmount behavior but does not set `hidden` on force-mounted inactive content.
- Arrow key navigation wraps around (loops) by default. The `loop` prop on `TabsList` controls this behavior.
