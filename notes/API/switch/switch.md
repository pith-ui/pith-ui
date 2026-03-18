# Switch (Root)

## Anatomy

The expected component nesting structure:

```
Switch
└── SwitchThumb
```

### React

```tsx
<Switch.Root>
  <Switch.Thumb />
</Switch.Root>
```

### Leptos

```rust
<Switch>
    <SwitchThumb />
</Switch>
```

## React Signature

```typescript
const Switch = React.forwardRef<SwitchElement, SwitchProps>(...)

type SwitchElement = React.ComponentRef<typeof Primitive.button>;
type PrimitiveButtonProps = React.ComponentPropsWithoutRef<typeof Primitive.button>;

interface SwitchProps extends PrimitiveButtonProps {
  checked?: boolean;
  defaultChecked?: boolean;
  required?: boolean;
  onCheckedChange?(checked: boolean): void;
}
```

The `name`, `value`, `disabled`, and `form` props are accepted via `PrimitiveButtonProps` (spread). The component destructures them explicitly for internal use (`name` for the hidden input, `value` defaults to `"on"`, `disabled` for ARIA and interaction gating, `form` for form association).

## Leptos Signature

```rust
pub fn Switch(
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] checked: MaybeProp<bool>,
    #[prop(into, optional)] default_checked: MaybeProp<bool>,
    #[prop(into, optional)] on_checked_change: Option<Callback<bool>>,
    #[prop(into, optional)] required: MaybeProp<bool>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] form: MaybeProp<String>,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `checked` | `checked` | `boolean \| undefined` | `MaybeProp<bool>` | The controlled checked state. When set, the component becomes controlled and ignores internal state. |
| `defaultChecked` | `default_checked` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | The checked state on initial render. Use when you do not need to control the switch state externally. |
| `onCheckedChange` | `on_checked_change` | `(checked: boolean) => void` | `Option<Callback<bool>>` | Callback fired when the checked state changes. Receives the new boolean value. |
| `required` | `required` | `boolean \| undefined` | `MaybeProp<bool>` (default `false`) | When `true`, sets `aria-required` on the button and `required` on the hidden input for native form validation. |
| `disabled` | `disabled` | `boolean \| undefined` | `MaybeProp<bool>` (default `false`) | When `true`, prevents user interaction. Sets `aria-disabled` and the native `disabled` attribute on the button, and `disabled` on the hidden input. |
| `name` | `name` | `string \| undefined` | `MaybeProp<String>` | The name of the hidden input submitted with the form. Only relevant when the switch is inside (or associated with) a form. |
| `value` | `value` | `string` (default `"on"`) | `MaybeProp<String>` (default `"on"`) | The value submitted with the form when checked. Defaults to `"on"` in both React and Leptos. |
| `form` | `form` | `string \| undefined` | `MaybeProp<String>` | The `id` of a `<form>` element to associate the switch with. Allows the switch to participate in a form even when it is not a descendant of that form. |
| *(via spread)* | `on_click` | `onClick` via `...PrimitiveButtonProps` | `Option<Callback<ev::MouseEvent>>` | Click handler composed with the internal toggle logic. In React this comes through the spread; Leptos exposes it as an explicit prop. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the root DOM element (`<button>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<button>`, merging props and refs onto the child. |
| *(spread)* | -- | `...PrimitiveButtonProps` | -- | React allows spreading any `<button>` HTML attribute. Leptos uses `attr:` directives on the call site instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"checked" \| "unchecked"` | Reflects the current checked state of the switch. |
| `data-disabled` | `""` (present/absent) | Present when the switch is disabled. |

### Implicit behavior

- The component renders `role="switch"` and `type="button"` on the underlying `<button>` element.
- `aria-checked` is set to `"true"` or `"false"` based on the checked state.
- `aria-required` is set to `"true"` when the `required` prop is `true`. When `required` is `false` (or not set), `aria-required` is omitted entirely rather than being set to `"false"`.
- When the switch is inside a form (detected by `button.closest("form")`) or has a `form` prop, a hidden `<input type="checkbox">` (the "bubble input") is rendered alongside the button. This input mirrors the switch state and enables native form submission and validation. It is visually hidden (`opacity: 0`, `position: absolute`, `pointer-events: none`) and `aria-hidden`.
- Clicking the button toggles the checked state. If inside a form, the click event on the button is stopped to prevent double-propagation -- the hidden input dispatches its own click event for form change tracking.
- The hidden input's dimensions are synchronized to the button's size via `useSize`, so it occupies the same space for hit-testing purposes.

## Usage Examples

### Uncontrolled

#### React

```tsx
<Switch.Root>
  <Switch.Thumb />
</Switch.Root>
```

#### Leptos

```rust
<Switch>
    <SwitchThumb />
</Switch>
```

### Uncontrolled (default checked)

#### React

```tsx
<Switch.Root defaultChecked>
  <Switch.Thumb />
</Switch.Root>
```

#### Leptos

```rust
<Switch default_checked=true>
    <SwitchThumb />
</Switch>
```

### Controlled

#### React

```tsx
const [checked, setChecked] = React.useState(true);

<Switch.Root checked={checked} onCheckedChange={setChecked}>
  <Switch.Thumb />
</Switch.Root>
```

#### Leptos

```rust
let (checked, set_checked) = signal(true);

<Switch
    checked=checked
    on_checked_change=move |checked| set_checked.set(checked)
>
    <SwitchThumb />
</Switch>
```

### Disabled

#### React

```tsx
<Switch.Root disabled>
  <Switch.Thumb />
</Switch.Root>
```

#### Leptos

```rust
<Switch disabled=true>
    <SwitchThumb />
</Switch>
```

### Within a form

#### React

```tsx
<form onSubmit={(e) => e.preventDefault()}>
  <Switch.Root name="airplane-mode" required>
    <Switch.Thumb />
  </Switch.Root>
  <button>Submit</button>
</form>
```

#### Leptos

```rust
<form on:submit=move |event| event.prevent_default()>
    <Switch name="airplane-mode" required=true>
        <SwitchThumb />
    </Switch>
    <button>"Submit"</button>
</form>
```

### With a label

#### React

```tsx
<label>
  Enable notifications{' '}
  <Switch.Root>
    <Switch.Thumb />
  </Switch.Root>
</label>
```

#### Leptos

```rust
<label>
    "Enable notifications "
    <Switch>
        <SwitchThumb />
    </Switch>
</label>
```

## Accessibility

Implements the [WAI-ARIA Switch pattern](https://www.w3.org/WAI/ARIA/apd/patterns/switch/).

### Keyboard Interactions

| Key | Description |
|---|---|
| `Space` | Toggles the switch between checked and unchecked. |
| `Enter` | Toggles the switch between checked and unchecked. |

### ARIA Attributes

| Element | Attribute | Value | Notes |
|---|---|---|---|
| `Switch` (button) | `role` | `"switch"` | Identifies the element as a switch. |
| `Switch` (button) | `aria-checked` | `"true" \| "false"` | Reflects the current checked state. |
| `Switch` (button) | `aria-required` | `"true"` or omitted | Set to `"true"` when the `required` prop is `true`; omitted entirely when `false`. |
| `Switch` (button) | `type` | `"button"` | Prevents form submission when pressed. |
| Hidden input | `aria-hidden` | `"true"` | Hides the input from the accessibility tree. |

### Behavioral Notes

- The switch is a native `<button>` element, so it is focusable by default and responds to `Space` and `Enter` key presses natively (via the browser's click event on buttons).
- When `disabled` is `true`, the button is natively disabled and cannot receive focus or be activated.
- The hidden checkbox input enables native form participation. It is invisible and non-interactive to users but carries the `name`, `value`, `required`, and `disabled` attributes needed for form submission.
- The `value` prop defaults to `"on"`, matching the default value of an HTML checkbox.
