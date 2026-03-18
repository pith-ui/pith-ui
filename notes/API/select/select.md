# Select (Root)

## Anatomy

The expected component nesting structure:

```
Select
├── SelectTrigger
│   ├── SelectValue
│   └── SelectIcon
└── SelectPortal
    └── SelectContent
        ├── SelectScrollUpButton
        ├── SelectViewport
        │   ├── SelectItem
        │   │   ├── SelectItemText
        │   │   └── SelectItemIndicator
        │   ├── SelectGroup
        │   │   ├── SelectLabel
        │   │   └── SelectItem ...
        │   └── SelectSeparator
        ├── SelectScrollDownButton
        └── SelectArrow (popper mode only)
```

### React

```tsx
<Select.Root>
  <Select.Trigger>
    <Select.Value placeholder="Pick one..." />
    <Select.Icon />
  </Select.Trigger>
  <Select.Portal>
    <Select.Content>
      <Select.ScrollUpButton />
      <Select.Viewport>
        <Select.Item value="apple">
          <Select.ItemText>Apple</Select.ItemText>
          <Select.ItemIndicator />
        </Select.Item>
        <Select.Separator />
        <Select.Group>
          <Select.Label>Citrus</Select.Label>
          <Select.Item value="orange">
            <Select.ItemText>Orange</Select.ItemText>
            <Select.ItemIndicator />
          </Select.Item>
        </Select.Group>
      </Select.Viewport>
      <Select.ScrollDownButton />
    </Select.Content>
  </Select.Portal>
</Select.Root>
```

### Leptos

```rust
<Select>
  <SelectTrigger>
    <SelectValue placeholder="Pick one..." />
    <SelectIcon />
  </SelectTrigger>
  <SelectPortal>
    <SelectContent>
      <SelectScrollUpButton />
      <SelectViewport>
        <SelectItem value="apple">
          <SelectItemText>"Apple"</SelectItemText>
          <SelectItemIndicator />
        </SelectItem>
        <SelectSeparator />
        <SelectGroup>
          <SelectLabel>"Citrus"</SelectLabel>
          <SelectItem value="orange">
            <SelectItemText>"Orange"</SelectItemText>
            <SelectItemIndicator />
          </SelectItem>
        </SelectGroup>
      </SelectViewport>
      <SelectScrollDownButton />
    </SelectContent>
  </SelectPortal>
</Select>
```

## React Signature

```typescript
type SelectProps = {
  children?: React.ReactNode;
  value?: string;
  defaultValue?: string;
  onValueChange?(value: string): void;
  open?: boolean;
  defaultOpen?: boolean;
  onOpenChange?(open: boolean): void;
  dir?: Direction;          // 'ltr' | 'rtl'
  name?: string;
  autoComplete?: string;
  disabled?: boolean;
  required?: boolean;
  form?: string;
};

const Select: React.FC<SelectProps> = (props) => { ... };
```

`Select` is a functional component (not `forwardRef`) -- it does not accept a `ref` or render its own DOM element.

## Leptos Signature

```rust
pub fn Select(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] auto_complete: MaybeProp<String>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] required: MaybeProp<bool>,
    #[prop(into, optional)] form: MaybeProp<String>,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `value` | `value` | `string \| undefined` | `MaybeProp<String>` | The controlled value of the selected item. When set, the component is controlled and `onValueChange` is required to update the value. |
| `defaultValue` | `default_value` | `string \| undefined` | `MaybeProp<String>` | The value of the item selected on initial render. Use when you do not need to control the select state. |
| `onValueChange` | `on_value_change` | `(value: string) => void` | `Option<Callback<String>>` | Callback fired when the selected value changes. |
| `open` | `open` | `boolean \| undefined` | `MaybeProp<bool>` | The controlled open state. When set, `onOpenChange` is required to toggle open/close. |
| `defaultOpen` | `default_open` | `boolean \| undefined` | `MaybeProp<bool>` | Whether the select is open on initial render. Defaults to `false`. |
| `onOpenChange` | `on_open_change` | `(open: boolean) => void` | `Option<Callback<bool>>` | Callback fired when the open state changes. |
| `dir` | `dir` | `'ltr' \| 'rtl'` | `MaybeProp<Direction>` | The reading direction. Affects arrow-key navigation within the content. |
| `name` | `name` | `string` | `MaybeProp<String>` | The name of the hidden native `<select>` used for form submission. |
| `autoComplete` | `auto_complete` | `string` | `MaybeProp<String>` | The `autocomplete` attribute on the hidden native `<select>` for form autofill. |
| `disabled` | `disabled` | `boolean` | `MaybeProp<bool>` | Disables the entire select. When `true`, the trigger cannot be interacted with and the content cannot be opened. |
| `required` | `required` | `boolean` | `MaybeProp<bool>` | Marks the select as required for form validation. Sets `aria-required` on the trigger and `required` on the hidden native `<select>`. |
| `form` | `form` | `string` | `MaybeProp<String>` | The `form` attribute on the hidden native `<select>`, associating it with a specific `<form>` by ID. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | The select's child components (Trigger, Portal, etc.). |

### Implicit behavior

- Renders a hidden native `<select>` element for form integration. This element bubbles `change` events to parent forms when the value changes.
- Wraps children in a `Popper` and `CollectionProvider` context for positioning and item registration.
- Does not render its own DOM element -- it is a context provider only.

## Usage Examples

### Basic uncontrolled

#### React

```tsx
<Select.Root defaultValue="apple">
  <Select.Trigger>
    <Select.Value placeholder="Select a fruit..." />
    <Select.Icon />
  </Select.Trigger>
  <Select.Portal>
    <Select.Content>
      <Select.Viewport>
        <Select.Item value="apple">
          <Select.ItemText>Apple</Select.ItemText>
        </Select.Item>
        <Select.Item value="banana">
          <Select.ItemText>Banana</Select.ItemText>
        </Select.Item>
        <Select.Item value="cherry">
          <Select.ItemText>Cherry</Select.ItemText>
        </Select.Item>
      </Select.Viewport>
    </Select.Content>
  </Select.Portal>
</Select.Root>
```

#### Leptos

```rust
<Select default_value="apple">
  <SelectTrigger>
    <SelectValue placeholder="Select a fruit..." />
    <SelectIcon />
  </SelectTrigger>
  <SelectPortal>
    <SelectContent>
      <SelectViewport>
        <SelectItem value="apple">
          <SelectItemText>"Apple"</SelectItemText>
        </SelectItem>
        <SelectItem value="banana">
          <SelectItemText>"Banana"</SelectItemText>
        </SelectItem>
        <SelectItem value="cherry">
          <SelectItemText>"Cherry"</SelectItemText>
        </SelectItem>
      </SelectViewport>
    </SelectContent>
  </SelectPortal>
</Select>
```

### Controlled

#### React

```tsx
const [value, setValue] = React.useState('apple');

<Select.Root value={value} onValueChange={setValue}>
  <Select.Trigger>
    <Select.Value />
    <Select.Icon />
  </Select.Trigger>
  <Select.Portal>
    <Select.Content>
      <Select.Viewport>
        {/* ...items */}
      </Select.Viewport>
    </Select.Content>
  </Select.Portal>
</Select.Root>
```

#### Leptos

```rust
let (value, set_value) = signal("apple".to_string());

<Select
  value=value
  on_value_change=Callback::new(move |v: String| set_value.set(v))
>
  <SelectTrigger>
    <SelectValue />
    <SelectIcon />
  </SelectTrigger>
  <SelectPortal>
    <SelectContent>
      <SelectViewport>
        // ...items
      </SelectViewport>
    </SelectContent>
  </SelectPortal>
</Select>
```

### With groups and scroll buttons

#### React

```tsx
<Select.Root>
  <Select.Trigger>
    <Select.Value placeholder="Choose..." />
    <Select.Icon />
  </Select.Trigger>
  <Select.Portal>
    <Select.Content>
      <Select.ScrollUpButton>^</Select.ScrollUpButton>
      <Select.Viewport>
        <Select.Group>
          <Select.Label>Fruits</Select.Label>
          <Select.Item value="apple">
            <Select.ItemText>Apple</Select.ItemText>
            <Select.ItemIndicator>✓</Select.ItemIndicator>
          </Select.Item>
        </Select.Group>
        <Select.Separator />
        <Select.Group>
          <Select.Label>Vegetables</Select.Label>
          <Select.Item value="carrot">
            <Select.ItemText>Carrot</Select.ItemText>
            <Select.ItemIndicator>✓</Select.ItemIndicator>
          </Select.Item>
        </Select.Group>
      </Select.Viewport>
      <Select.ScrollDownButton>v</Select.ScrollDownButton>
    </Select.Content>
  </Select.Portal>
</Select.Root>
```

#### Leptos

```rust
<Select>
  <SelectTrigger>
    <SelectValue placeholder="Choose..." />
    <SelectIcon />
  </SelectTrigger>
  <SelectPortal>
    <SelectContent>
      <SelectScrollUpButton>"^"</SelectScrollUpButton>
      <SelectViewport>
        <SelectGroup>
          <SelectLabel>"Fruits"</SelectLabel>
          <SelectItem value="apple">
            <SelectItemText>"Apple"</SelectItemText>
            <SelectItemIndicator>"✓"</SelectItemIndicator>
          </SelectItem>
        </SelectGroup>
        <SelectSeparator />
        <SelectGroup>
          <SelectLabel>"Vegetables"</SelectLabel>
          <SelectItem value="carrot">
            <SelectItemText>"Carrot"</SelectItemText>
            <SelectItemIndicator>"✓"</SelectItemIndicator>
          </SelectItem>
        </SelectGroup>
      </SelectViewport>
      <SelectScrollDownButton>"v"</SelectScrollDownButton>
    </SelectContent>
  </SelectPortal>
</Select>
```

### Popper positioning mode

By default, `SelectContent` uses `position="item-aligned"` which aligns the selected item with the trigger. Set `position="popper"` to use floating-UI-based positioning instead.

#### React

```tsx
<Select.Content position="popper" sideOffset={5}>
  <Select.Viewport>
    {/* ...items */}
  </Select.Viewport>
  <Select.Arrow />
</Select.Content>
```

#### Leptos

```rust
<SelectContent position="popper" side_offset=5.0>
  <SelectViewport>
    // ...items
  </SelectViewport>
  <SelectArrow />
</SelectContent>
```

### Disabled item

#### React

```tsx
<Select.Item value="disabled-item" disabled>
  <Select.ItemText>Cannot select this</Select.ItemText>
</Select.Item>
```

#### Leptos

```rust
<SelectItem value="disabled-item" disabled=true>
  <SelectItemText>"Cannot select this"</SelectItemText>
</SelectItem>
```

### Form integration

#### React

```tsx
<form onSubmit={handleSubmit}>
  <Select.Root name="fruit" required>
    {/* ...trigger and content */}
  </Select.Root>
</form>
```

#### Leptos

```rust
<form on:submit=handle_submit>
  <Select name="fruit" required=true>
    // ...trigger and content
  </Select>
</form>
```

## Accessibility

Implements the [WAI-ARIA Listbox pattern](https://www.w3.org/WAI/ARIA/apd/patterns/listbox/).

### Keyboard Interactions

| Key | Description |
|---|---|
| `Enter` / `Space` | When focus is on the trigger, opens the select. When focus is on an item, selects it and closes the select. |
| `ArrowDown` | When focus is on the trigger, opens the select. When focus is inside the content, moves focus to the next item. |
| `ArrowUp` | When focus is on the trigger, opens the select. When focus is inside the content, moves focus to the previous item. |
| `Home` | When focus is inside the content, moves focus to the first item. |
| `End` | When focus is inside the content, moves focus to the last item. |
| `Tab` | Prevented inside the content to keep focus trapped. |
| Printable characters | Typeahead search: typing characters focuses the matching item (in content) or changes the value (on trigger). Resets after 1 second of inactivity. |

### ARIA Attributes

| Element | Attribute | Value | Notes |
|---|---|---|---|
| `SelectTrigger` | `role` | `"combobox"` | Identifies the trigger as a combobox control. |
| `SelectTrigger` | `aria-controls` | `string` | Points to the content element's auto-generated `id`. |
| `SelectTrigger` | `aria-expanded` | `"true" \| "false"` | Reflects whether the content is open. |
| `SelectTrigger` | `aria-required` | `"true"` / absent | Present when the select is required. |
| `SelectTrigger` | `aria-autocomplete` | `"none"` | Indicates no autocomplete behavior. |
| Content | `role` | `"listbox"` | Identifies the content as a listbox. |
| `SelectItem` | `role` | `"option"` | Identifies each item as an option within the listbox. |
| `SelectItem` | `aria-selected` | `"true"` / absent | Set to `"true"` only when the item is both selected and focused (VoiceOver fix). |
| `SelectItem` | `aria-disabled` | `"true"` / absent | Present when the item is disabled. |
| `SelectItem` | `aria-labelledby` | `string` | Points to the `SelectItemText`'s auto-generated `id`. |
| `SelectGroup` | `role` | `"group"` | Groups related items. |
| `SelectGroup` | `aria-labelledby` | `string` | Points to the `SelectLabel`'s auto-generated `id`. |
| `SelectScrollUpButton` | `aria-hidden` | `"true"` | Decorative scroll control, hidden from screen readers. |
| `SelectScrollDownButton` | `aria-hidden` | `"true"` | Decorative scroll control, hidden from screen readers. |
| `SelectSeparator` | `aria-hidden` | `"true"` | Decorative separator, hidden from screen readers. |
| `SelectIcon` | `aria-hidden` | `"true"` | Decorative icon, hidden from screen readers. |
| `SelectItemIndicator` | `aria-hidden` | `"true"` | Decorative check indicator, hidden from screen readers. |
| Hidden `<select>` | `aria-hidden` | `"true"` | The native select is hidden from the accessibility tree. |

### Behavioral Notes

- Disabled items are skipped during keyboard navigation.
- Arrow key navigation prevents page scroll (`event.preventDefault()`).
- The content closes on window blur and window resize.
- When the content opens, focus is moved to the currently selected item (or the first enabled item if none is selected).
- When the content closes, focus returns to the trigger.
- Outside pointer events are blocked while the content is open (`disableOutsidePointerEvents`).
- `aria-hidden` is applied to all sibling elements while the content is open (equivalent to `aria-modal`).
- Typeahead search on the trigger immediately changes the selected value without opening the content.
- Typeahead search inside the content moves focus to the matching item without selecting it.
- The select uses `FocusScope` with `trapped=true` to keep focus within the content while open.

## CSS Custom Properties

These properties are exposed when `position="popper"` is used. They alias the underlying Popper values.

| Property | Source | Description |
|---|---|---|
| `--radix-select-content-transform-origin` | `var(--radix-popper-transform-origin)` | The CSS transform origin for animations, based on the content's placement. |
| `--radix-select-content-available-width` | `var(--radix-popper-available-width)` | The available width between the content and the viewport edge. |
| `--radix-select-content-available-height` | `var(--radix-popper-available-height)` | The available height between the content and the viewport edge. |
| `--radix-select-trigger-width` | `var(--radix-popper-anchor-width)` | The width of the trigger element. Useful for matching content width to trigger width. |
| `--radix-select-trigger-height` | `var(--radix-popper-anchor-height)` | The height of the trigger element. |
