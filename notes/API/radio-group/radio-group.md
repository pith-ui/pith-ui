# RadioGroup (Root)

## Anatomy

The expected component nesting structure:

```
RadioGroup
└── RadioGroupItem (one per option, each with a unique `value`)
    └── RadioGroupIndicator
```

### React

```tsx
<RadioGroup.Root defaultValue="1">
  <RadioGroup.Item value="1">
    <RadioGroup.Indicator />
  </RadioGroup.Item>
  <RadioGroup.Item value="2">
    <RadioGroup.Indicator />
  </RadioGroup.Item>
</RadioGroup.Root>
```

### Leptos

```rust
<RadioGroup default_value="1">
  <RadioGroupItem value="1">
    <RadioGroupIndicator />
  </RadioGroupItem>
  <RadioGroupItem value="2">
    <RadioGroupIndicator />
  </RadioGroupItem>
</RadioGroup>
```

## React Signature

```typescript
const RadioGroup = React.forwardRef<RadioGroupElement, RadioGroupProps>(...)

type RadioGroupElement = React.ComponentRef<typeof Primitive.div>;
type RovingFocusGroupProps = React.ComponentPropsWithoutRef<typeof RovingFocusGroup.Root>;
type PrimitiveDivProps = React.ComponentPropsWithoutRef<typeof Primitive.div>;

interface RadioGroupProps extends PrimitiveDivProps {
  name?: string;
  required?: boolean;
  disabled?: boolean;
  dir?: RovingFocusGroupProps['dir'];        // 'ltr' | 'rtl'
  orientation?: RovingFocusGroupProps['orientation']; // 'horizontal' | 'vertical'
  loop?: RovingFocusGroupProps['loop'];      // boolean
  defaultValue?: string;
  value?: string | null;
  onValueChange?(value: string): void;
}
```

## Leptos Signature

```rust
pub fn RadioGroup(
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] required: MaybeProp<bool>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    /// The `id` of a `<form>` element to associate the radio group with.
    #[prop(into, optional)]
    form: MaybeProp<String>,
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
| `name` | `name` | `string \| undefined` | `MaybeProp<String>` | The name attribute for the hidden `<input>` elements rendered by each `RadioGroupItem`. Groups items for native form submission. |
| `value` | `value` | `string \| null \| undefined` | `MaybeProp<String>` | The controlled value of the selected radio item. When set, the component becomes controlled. Pass `null` (React) or omit (Leptos) to indicate no selection. |
| `defaultValue` | `default_value` | `string \| undefined` | `MaybeProp<String>` | The value of the radio item selected on initial render. Use when you do not need to control radio group state. |
| `onValueChange` | `on_value_change` | `(value: string) => void` | `Option<Callback<String>>` | Callback fired when the selected value changes. Receives the newly selected item's `value` string. |
| `required` | `required` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | Whether the radio group is required for form submission. Sets `aria-required` on the group and `required` on the hidden `<input>` elements. |
| `disabled` | `disabled` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | Disables the entire radio group. All items ignore user interaction and are skipped during keyboard navigation. |
| `dir` | `dir` | `'ltr' \| 'rtl'` | `MaybeProp<Direction>` | The reading direction. Affects horizontal arrow-key navigation: in RTL, `ArrowRight` moves to the previous item and `ArrowLeft` to the next. |
| `orientation` | `orientation` | `'horizontal' \| 'vertical' \| undefined` | `MaybeProp<Orientation>` | The layout axis. Controls which arrow keys navigate between items (`ArrowUp`/`ArrowDown` for vertical, `ArrowLeft`/`ArrowRight` for horizontal). When unset, both vertical and horizontal arrow keys work. |
| `loop` | `r#loop` | `boolean` (default `true`) | `MaybeProp<bool>` (default `true`) | Whether keyboard navigation wraps from the last item to the first (and vice versa). |
| — | `form` | — | `MaybeProp<String>` | *Leptos-only.* The `id` of a `<form>` element to associate with, allowing form participation when the radio group is not a DOM descendant of that form. React achieves this via the `form` attribute in prop spread. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the root DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, the component renders its child directly instead of wrapping in a `<div>`, merging props and refs onto the child. |
| *(spread)* | — | `...PrimitiveDivProps` | — | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives on the call site instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-disabled` | `""` (present/absent) | Present when the `disabled` prop is `true`. |

### Implicit behavior

- Renders with `role="radiogroup"` on the root `<div>`.
- Sets `aria-required` to `"true"` when `required` is `true`.
- Sets `aria-orientation` to the `orientation` value when provided.
- Sets `dir` attribute on the DOM element reflecting the resolved direction.
- Wraps children in a `RovingFocusGroup` to manage keyboard focus among items.
- Provides context to descendant `RadioGroupItem` and `RadioGroupIndicator` components (`name`, `required`, `disabled`, `value`, `onValueChange`, `form`).

## Usage Examples

### Basic uncontrolled

#### React

```tsx
<RadioGroup.Root defaultValue="cat">
  <RadioGroup.Item value="cat">
    <RadioGroup.Indicator />
  </RadioGroup.Item>
  <RadioGroup.Item value="dog">
    <RadioGroup.Indicator />
  </RadioGroup.Item>
  <RadioGroup.Item value="rabbit">
    <RadioGroup.Indicator />
  </RadioGroup.Item>
</RadioGroup.Root>
```

#### Leptos

```rust
<RadioGroup default_value="cat">
  <RadioGroupItem value="cat">
    <RadioGroupIndicator />
  </RadioGroupItem>
  <RadioGroupItem value="dog">
    <RadioGroupIndicator />
  </RadioGroupItem>
  <RadioGroupItem value="rabbit">
    <RadioGroupIndicator />
  </RadioGroupItem>
</RadioGroup>
```

### Controlled

#### React

```tsx
const [value, setValue] = React.useState('cat');

<RadioGroup.Root value={value} onValueChange={setValue}>
  <RadioGroup.Item value="cat">
    <RadioGroup.Indicator />
  </RadioGroup.Item>
  <RadioGroup.Item value="dog">
    <RadioGroup.Indicator />
  </RadioGroup.Item>
</RadioGroup.Root>
```

#### Leptos

```rust
let (value, set_value) = signal("cat".to_string());

<RadioGroup
  value=Signal::derive(move || value.get())
  on_value_change=Callback::new(move |v: String| set_value.set(v))
>
  <RadioGroupItem value="cat">
    <RadioGroupIndicator />
  </RadioGroupItem>
  <RadioGroupItem value="dog">
    <RadioGroupIndicator />
  </RadioGroupItem>
</RadioGroup>
```

### Within a form

#### React

```tsx
<form onChange={(e) => console.log(e.target.value)}>
  <RadioGroup.Root name="pet" required>
    <RadioGroup.Item value="cat">
      <RadioGroup.Indicator />
    </RadioGroup.Item>
    <RadioGroup.Item value="dog">
      <RadioGroup.Indicator />
    </RadioGroup.Item>
  </RadioGroup.Root>
  <button>Submit</button>
</form>
```

#### Leptos

```rust
<form on:change=move |event: ev::Event| {
    // handle change
}>
  <RadioGroup name="pet" required=true>
    <RadioGroupItem value="cat">
      <RadioGroupIndicator />
    </RadioGroupItem>
    <RadioGroupItem value="dog">
      <RadioGroupIndicator />
    </RadioGroupItem>
  </RadioGroup>
  <button>"Submit"</button>
</form>
```

### Disabled item

#### React

```tsx
<RadioGroup.Root defaultValue="cat">
  <RadioGroup.Item value="cat">
    <RadioGroup.Indicator />
  </RadioGroup.Item>
  <RadioGroup.Item value="dog" disabled>
    <RadioGroup.Indicator />
  </RadioGroup.Item>
</RadioGroup.Root>
```

#### Leptos

```rust
<RadioGroup default_value="cat">
  <RadioGroupItem value="cat">
    <RadioGroupIndicator />
  </RadioGroupItem>
  <RadioGroupItem value="dog" disabled=true>
    <RadioGroupIndicator />
  </RadioGroupItem>
</RadioGroup>
```

## Accessibility

Implements the [WAI-ARIA Radio Group pattern](https://www.w3.org/WAI/ARIA/apd/patterns/radiobutton/).

### Keyboard Interactions

| Key | Description |
|---|---|
| `ArrowDown` | When `orientation` is unset or `"vertical"`: moves focus to the next item and checks it. Wraps from last to first when `loop` is `true` (default). |
| `ArrowUp` | When `orientation` is unset or `"vertical"`: moves focus to the previous item and checks it. Wraps from first to last when `loop` is `true`. |
| `ArrowRight` | When `orientation` is unset or `"horizontal"`: moves focus to the next item (LTR) or previous item (RTL) and checks it. Wraps around when `loop` is `true`. |
| `ArrowLeft` | When `orientation` is unset or `"horizontal"`: moves focus to the previous item (LTR) or next item (RTL) and checks it. Wraps around when `loop` is `true`. |
| `Space` | When focus is on an unchecked item, checks it. |
| `Enter` | Explicitly prevented — radio groups do not activate items on Enter per the WAI-ARIA spec. |
| `Tab` | Moves focus into the radio group (to the checked item, or the first item if none is checked). A second `Tab` moves focus out of the group. |

### ARIA Attributes

| Element | Attribute | Value | Notes |
|---|---|---|---|
| `RadioGroup` | `role` | `"radiogroup"` | Identifies the container as a radio group. |
| `RadioGroup` | `aria-required` | `"true"` / absent | Set when the `required` prop is `true`. |
| `RadioGroup` | `aria-orientation` | `"horizontal" \| "vertical"` / absent | Reflects the `orientation` prop. Absent when `orientation` is not set. |
| `RadioGroupItem` | `role` | `"radio"` | Identifies each item as a radio button. |
| `RadioGroupItem` | `aria-checked` | `"true" \| "false"` | Reflects whether this item is the currently selected value. |

### Behavioral Notes

- Arrow key navigation automatically checks the focused item. This is implemented by programmatically clicking the radio button when focus arrives via an arrow key press, so the `onValueChange` callback fires and native form change events are emitted.
- Disabled items are skipped during keyboard navigation (roving focus treats them as non-focusable).
- Arrow key navigation prevents page scroll (`event.preventDefault()` is handled by the underlying `RovingFocusGroup`).
- When the radio group is inside a `<form>`, each `RadioGroupItem` renders a hidden native `<input type="radio">` to participate in native form validation and submission. The hidden input mirrors the button's checked/disabled/required state.
- The `Enter` key is explicitly prevented from activating a radio item, matching the WAI-ARIA specification for radio groups.
- When `disabled` is set on the root, all items are disabled regardless of their individual `disabled` prop.

## CSS Custom Properties

This component does not expose any CSS custom properties.
