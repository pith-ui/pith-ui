# ToggleGroup (Root)

## Anatomy

The expected component nesting structure:

```
ToggleGroup
└── ToggleGroupItem (one per option, each with a unique `value`)
```

### React

```tsx
<ToggleGroup.Root type="single" defaultValue="a">
  <ToggleGroup.Item value="a">Option A</ToggleGroup.Item>
  <ToggleGroup.Item value="b">Option B</ToggleGroup.Item>
  <ToggleGroup.Item value="c">Option C</ToggleGroup.Item>
</ToggleGroup.Root>
```

### Leptos

```rust
<ToggleGroup r#type=ToggleGroupType::Single default_value=vec!["a".to_string()]>
  <ToggleGroupItem value="a">"Option A"</ToggleGroupItem>
  <ToggleGroupItem value="b">"Option B"</ToggleGroupItem>
  <ToggleGroupItem value="c">"Option C"</ToggleGroupItem>
</ToggleGroup>
```

## React Signature

React exposes a single `ToggleGroup` component that dispatches on a `type` discriminator:

```typescript
const ToggleGroup = React.forwardRef<
  ToggleGroupElement,
  ToggleGroupSingleProps | ToggleGroupMultipleProps
>(...)

// When type === 'single':
interface ToggleGroupSingleProps extends ToggleGroupImplSingleProps {
  type: 'single';
}
interface ToggleGroupImplSingleProps extends ToggleGroupImplProps {
  /** The controlled stateful value of the item that is pressed. */
  value?: string;
  /** The value of the item that is pressed when initially rendered. Use
   *  `defaultValue` if you do not need to control the state of a toggle group. */
  defaultValue?: string;
  /** The callback that fires when the value of the toggle group changes. */
  onValueChange?(value: string): void;
}

// When type === 'multiple':
interface ToggleGroupMultipleProps extends ToggleGroupImplMultipleProps {
  type: 'multiple';
}
interface ToggleGroupImplMultipleProps extends ToggleGroupImplProps {
  /** The controlled stateful value of the items that are pressed. */
  value?: string[];
  /** The value of the items that are pressed when initially rendered. Use
   *  `defaultValue` if you do not need to control the state of a toggle group. */
  defaultValue?: string[];
  /** The callback that fires when the state of the toggle group changes. */
  onValueChange?(value: string[]): void;
}

// Shared base:
type RovingFocusGroupProps = React.ComponentPropsWithoutRef<typeof RovingFocusGroup.Root>;
interface ToggleGroupImplProps extends PrimitiveDivProps {
  /**
   * Whether the group is disabled from user interaction.
   * @defaultValue false
   */
  disabled?: boolean;
  /**
   * Whether the group should maintain roving focus of its buttons.
   * @defaultValue true
   */
  rovingFocus?: boolean;
  loop?: RovingFocusGroupProps['loop'];
  orientation?: RovingFocusGroupProps['orientation'];
  dir?: RovingFocusGroupProps['dir'];
}
```

## Leptos Signature

Leptos uses a single unified `ToggleGroup` component rather than splitting into separate `ToggleGroupSingle` / `ToggleGroupMultiple` components. The `type` prop selects single vs. multiple mode. Unlike the Accordion port (which has `AccordionSingle` and `AccordionMultiple`), ToggleGroup always uses `Vec<String>` for value props regardless of mode.

```rust
pub fn ToggleGroup(
    /// Whether the group is single or multiple selection.
    r#type: ToggleGroupType,
    /// The controlled value of the pressed items.
    #[prop(into, optional)]
    value: MaybeProp<Vec<String>>,
    /// The default value of the pressed items when uncontrolled.
    #[prop(into, optional)]
    default_value: MaybeProp<Vec<String>>,
    /// Callback when the value changes.
    #[prop(into, optional)]
    on_value_change: Option<Callback<Vec<String>>>,
    /// Whether the group is disabled from user interaction.
    #[prop(into, optional)]
    disabled: MaybeProp<bool>,
    /// Whether the group should maintain roving focus of its buttons.
    #[prop(into, optional)]
    roving_focus: MaybeProp<bool>,
    #[prop(into, optional)] r#loop: MaybeProp<bool>,
    #[prop(into, optional)] orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

### Single-mode value props

In React, single mode uses `string` for the value; in Leptos, single mode still uses `Vec<String>` (the same type as multiple mode). The component internally treats a single-element vec as the selected value and an empty vec as no selection.

| React Prop      | Leptos Prop       | Type (React)              | Type (Leptos)                   | Description                                                                                                                                                                          |
| --------------- | ----------------- | ------------------------- | ------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `type`          | `r#type`          | `'single'`                | `ToggleGroupType::Single`       | Discriminator selecting single-item mode.                                                                                                                                            |
| `value`         | `value`           | `string \| undefined`     | `MaybeProp<Vec<String>>`        | The controlled value of the pressed item. React accepts a single string; Leptos uses `Vec<String>` (wrap in a one-element vec). See **API Divergence** note below.                   |
| `defaultValue`  | `default_value`   | `string \| undefined`     | `MaybeProp<Vec<String>>`        | The value of the pressed item on initial render. React accepts a string; Leptos accepts `Vec<String>`.                                                                               |
| `onValueChange` | `on_value_change` | `(value: string) => void` | `Option<Callback<Vec<String>>>` | Callback fired when the pressed item changes. React passes a single string (or empty string `""` for deselection); Leptos passes `Vec<String>` (empty vec for deselection). See **API Divergence** note below. |

### Multiple-mode value props

| React Prop      | Leptos Prop       | Type (React)                | Type (Leptos)                   | Description                                                                                          |
| --------------- | ----------------- | --------------------------- | ------------------------------- | ---------------------------------------------------------------------------------------------------- |
| `type`          | `r#type`          | `'multiple'`                | `ToggleGroupType::Multiple`     | Discriminator selecting multi-item mode.                                                             |
| `value`         | `value`           | `string[] \| undefined`     | `MaybeProp<Vec<String>>`        | The controlled list of pressed item values. Same type in both frameworks (array/vec).                |
| `defaultValue`  | `default_value`   | `string[] \| undefined`     | `MaybeProp<Vec<String>>`        | The values of the pressed items on initial render.                                                   |
| `onValueChange` | `on_value_change` | `(value: string[]) => void` | `Option<Callback<Vec<String>>>` | Callback fired when the set of pressed items changes. Receives the full updated list.                |

### Shared props (both modes)

| React Prop    | Leptos Prop    | Type (React)                                         | Type (Leptos)                                 | Description                                                                                                                                                                      |
| ------------- | -------------- | ---------------------------------------------------- | --------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `disabled`    | `disabled`     | `boolean` (default `false`)                          | `MaybeProp<bool>` (default `false`)           | Disables the entire group. When `true`, all items ignore user interaction and are skipped during keyboard navigation.                                                            |
| `rovingFocus` | `roving_focus` | `boolean` (default `true`)                           | `MaybeProp<bool>` (default `true`)            | Whether the group maintains roving focus of its buttons. When `false`, items receive regular tab-order focus instead of arrow-key navigation.                                    |
| `loop`        | `r#loop`       | `boolean` (default `true`)                           | `MaybeProp<bool>` (default `true`)            | Whether keyboard navigation should loop from last item to first and vice versa.                                                                                                  |
| `orientation` | `orientation`  | `'horizontal' \| 'vertical'` (default `'horizontal'`) | `MaybeProp<Orientation>`                      | The layout axis the group operates along. Controls which arrow keys navigate between items.                                                                                      |
| `dir`         | `dir`          | `'ltr' \| 'rtl'`                                     | `MaybeProp<Direction>`                        | The reading direction. Affects horizontal arrow-key navigation: in RTL, `ArrowRight` moves to the previous item and `ArrowLeft` to the next.                                     |
| `ref`         | `node_ref`     | `React.Ref`                                          | `AnyNodeRef`                                  | Ref to the root DOM element (`<div>`).                                                                                                                                           |
| `asChild`     | `as_child`     | `boolean`                                            | `MaybeProp<bool>`                             | When `true`, the component renders its child directly instead of wrapping in a `<div>`, merging props and refs onto the child.                                                   |
| *(spread)*    | --             | `...PrimitiveDivProps`                               | --                                            | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives on the call site instead.                                                                      |

### Leptos-only: `ToggleGroupType` enum

```rust
pub enum ToggleGroupType {
    Single,
    Multiple,
}
```

### Data attributes (rendered on DOM)

| Attribute          | Value                        | Description                      |
| ------------------ | ---------------------------- | -------------------------------- |
| `data-orientation` | `"horizontal" \| "vertical"` | Reflects the `orientation` prop. Only rendered when `orientation` is explicitly set. |

### API Divergence: Unified `Vec<String>` value type

React uses different value types for single mode (`string`) and multiple mode (`string[]`), enforced at the type level by the `type` discriminated union. The Leptos port uses a single `ToggleGroup` component with `Vec<String>` for all value-related props in both modes.

This means:
- **Single mode, React:** `value="a"`, `onValueChange` receives `"a"` or `""` (empty string for deselection)
- **Single mode, Leptos:** `value=vec!["a".to_string()]`, `on_value_change` receives `vec!["a".to_string()]` or `vec![]` (empty vec for deselection)

This differs from the Accordion pattern where the Leptos port split into `AccordionSingle` (with `String` value) and `AccordionMultiple` (with `Vec<String>` values). A task has been filed to track this inconsistency.

## Usage Examples

### Single mode (uncontrolled)

#### React

```tsx
<ToggleGroup.Root type="single" defaultValue="1" aria-label="Options">
  <ToggleGroup.Item value="1">Option 1</ToggleGroup.Item>
  <ToggleGroup.Item value="2">Option 2</ToggleGroup.Item>
  <ToggleGroup.Item value="3">Option 3</ToggleGroup.Item>
</ToggleGroup.Root>
```

#### Leptos

```rust
<ToggleGroup
  r#type=ToggleGroupType::Single
  default_value=vec!["1".to_string()]
  attr:aria-label="Options"
>
  <ToggleGroupItem value="1">"Option 1"</ToggleGroupItem>
  <ToggleGroupItem value="2">"Option 2"</ToggleGroupItem>
  <ToggleGroupItem value="3">"Option 3"</ToggleGroupItem>
</ToggleGroup>
```

### Single mode (controlled)

#### React

```tsx
const [value, setValue] = React.useState<string>();

<ToggleGroup.Root type="single" value={value} onValueChange={setValue}>
  {/* ...items */}
</ToggleGroup.Root>
```

#### Leptos

```rust
let (value, set_value) = signal::<Vec<String>>(vec![]);

<ToggleGroup
  r#type=ToggleGroupType::Single
  value=value
  on_value_change=Callback::new(move |v: Vec<String>| set_value.set(v))
>
  // ...items
</ToggleGroup>
```

### Multiple mode (uncontrolled)

#### React

```tsx
<ToggleGroup.Root type="multiple" defaultValue={['1']} aria-label="Options">
  <ToggleGroup.Item value="1">Option 1</ToggleGroup.Item>
  <ToggleGroup.Item value="2">Option 2</ToggleGroup.Item>
  <ToggleGroup.Item value="3">Option 3</ToggleGroup.Item>
</ToggleGroup.Root>
```

#### Leptos

```rust
<ToggleGroup
  r#type=ToggleGroupType::Multiple
  default_value=vec!["1".to_string()]
  attr:aria-label="Options"
>
  <ToggleGroupItem value="1">"Option 1"</ToggleGroupItem>
  <ToggleGroupItem value="2">"Option 2"</ToggleGroupItem>
  <ToggleGroupItem value="3">"Option 3"</ToggleGroupItem>
</ToggleGroup>
```

### Multiple mode (controlled)

#### React

```tsx
const [value, setValue] = React.useState<string[]>([]);

<ToggleGroup.Root type="multiple" value={value} onValueChange={setValue}>
  {/* ...items */}
</ToggleGroup.Root>
```

#### Leptos

```rust
let (value, set_value) = signal::<Vec<String>>(vec![]);

<ToggleGroup
  r#type=ToggleGroupType::Multiple
  value=value
  on_value_change=Callback::new(move |v: Vec<String>| set_value.set(v))
>
  // ...items
</ToggleGroup>
```

### Disabled item

#### React

```tsx
<ToggleGroup.Item value="3" disabled>
  Option 3
</ToggleGroup.Item>
```

#### Leptos

```rust
<ToggleGroupItem value="3" disabled=true>
  "Option 3"
</ToggleGroupItem>
```

### Vertical orientation

#### React

```tsx
<ToggleGroup.Root type="single" orientation="vertical" defaultValue="1">
  {/* ...items */}
</ToggleGroup.Root>
```

#### Leptos

```rust
<ToggleGroup
  r#type=ToggleGroupType::Single
  orientation=Orientation::Vertical
  default_value=vec!["1".to_string()]
>
  // ...items
</ToggleGroup>
```

### Without roving focus

#### React

```tsx
<ToggleGroup.Root type="single" rovingFocus={false}>
  {/* ...items — each receives regular tab focus */}
</ToggleGroup.Root>
```

#### Leptos

```rust
<ToggleGroup r#type=ToggleGroupType::Single roving_focus=false>
  // ...items — each receives regular tab focus
</ToggleGroup>
```

## Accessibility

Implements the [WAI-ARIA Toolbar pattern](https://www.w3.org/WAI/ARIA/apd/patterns/toolbar/) for grouping toggle buttons. In single mode, items use `role="radio"` with `aria-checked` to form a radio-group-like pattern. In multiple mode, items use standard `aria-pressed` toggle semantics.

### Keyboard Interactions

| Key               | Description                                                                                                           |
| ----------------- | --------------------------------------------------------------------------------------------------------------------- |
| `Enter` / `Space` | When focus is on an item, toggles its pressed state.                                                                  |
| `ArrowRight`      | When `orientation="horizontal"` (default): moves focus to the next item (LTR) or previous item (RTL). Wraps if `loop` is `true`. |
| `ArrowLeft`       | When `orientation="horizontal"`: moves focus to the previous item (LTR) or next item (RTL). Wraps if `loop` is `true`.           |
| `ArrowDown`       | When `orientation="vertical"`: moves focus to the next item. Wraps if `loop` is `true`.                               |
| `ArrowUp`         | When `orientation="vertical"`: moves focus to the previous item. Wraps if `loop` is `true`.                           |
| `Home`            | Moves focus to the first item.                                                                                        |
| `End`             | Moves focus to the last item.                                                                                         |
| `Tab`             | When `rovingFocus` is `true` (default): moves focus into/out of the group as a single tab stop. When `false`: each item is a separate tab stop. |

### ARIA Attributes

| Element           | Attribute        | Value            | Notes                                                                                                      |
| ----------------- | ---------------- | ---------------- | ---------------------------------------------------------------------------------------------------------- |
| `ToggleGroup`     | `role`           | `"group"`        | Identifies the container as a group.                                                                       |
| `ToggleGroup`     | `dir`            | `"ltr" \| "rtl"` | Reflects the resolved reading direction.                                                                   |
| `ToggleGroupItem` | `role`           | `"radio"` / absent | In single mode, each item has `role="radio"`. In multiple mode, the role is omitted (standard button).   |
| `ToggleGroupItem` | `aria-checked`   | `"true" \| "false"` / absent | In single mode, reflects whether the item is pressed. Absent in multiple mode.                       |
| `ToggleGroupItem` | `aria-pressed`   | `"true" \| "false"` / absent | In multiple mode, reflects whether the item is pressed. Absent in single mode.                       |
| `ToggleGroupItem` | `disabled`       | present / absent | The HTML `disabled` attribute, set when the item is disabled (from item prop or inherited from group).     |

### Behavioral Notes

- The root element renders as a `<div>` with `role="group"`.
- Items render as `<button type="button">` elements.
- Disabled items are skipped during roving focus keyboard navigation.
- In single mode, clicking an already-pressed item deselects it (value becomes empty string in React, empty vec in Leptos).
- In multiple mode, items toggle independently. Clicking a pressed item removes it from the value array; clicking an unpressed item adds it.
- When `rovingFocus` is `false`, the `RovingFocusGroup` wrapper is not rendered; items receive normal tab-order focus and arrow keys do not navigate between items.
- Arrow key navigation is provided by the `RovingFocusGroup` primitive and follows `orientation` and `dir` props.
