# Checkbox (Root)

## Anatomy

The expected component nesting structure:

```
Checkbox
└── CheckboxIndicator
```

### React

React exposes two API styles. The **legacy** API uses a monolithic `Checkbox.Root`:

```tsx
<Checkbox.Root>
  <Checkbox.Indicator>...</Checkbox.Indicator>
</Checkbox.Root>
```

The **newer (unstable)** API splits state management from the trigger:

```tsx
<Checkbox.unstable_Provider>
  <Checkbox.unstable_Trigger>
    <Checkbox.Indicator>...</Checkbox.Indicator>
  </Checkbox.unstable_Trigger>
  <Checkbox.unstable_BubbleInput />
</Checkbox.unstable_Provider>
```

### Leptos

Leptos implements the legacy/monolithic pattern. The `Checkbox` component combines the roles of `CheckboxProvider`, `CheckboxTrigger`, and `CheckboxBubbleInput`:

```rust
<Checkbox>
  <CheckboxIndicator>"..."</CheckboxIndicator>
</Checkbox>
```

## React Signature

### Legacy API (Checkbox.Root)

```typescript
const Checkbox = React.forwardRef<CheckboxElement, CheckboxProps>(...)

type CheckboxElement = React.ComponentRef<typeof Primitive.button>;
type PrimitiveButtonProps = React.ComponentPropsWithoutRef<typeof Primitive.button>;

type CheckedState = boolean | 'indeterminate';

interface CheckboxProps extends Omit<PrimitiveButtonProps, 'checked' | 'defaultChecked'> {
  checked?: CheckedState;
  defaultChecked?: CheckedState;
  required?: boolean;
  onCheckedChange?(checked: CheckedState): void;
  // Inherited from PrimitiveButtonProps (via spread):
  // name?: string;
  // disabled?: boolean;
  // value?: string | number | readonly string[];
  // form?: string;
}
```

### Newer (unstable) API

```typescript
// CheckboxProvider — manages state, renders context
interface CheckboxProviderProps<State extends CheckedState = CheckedState> {
  checked?: State | boolean;
  defaultChecked?: State | boolean;
  required?: boolean;
  onCheckedChange?(checked: State | boolean): void;
  name?: string;
  form?: string;
  disabled?: boolean;
  value?: string | number | readonly string[];
  children?: React.ReactNode;
}

// CheckboxTrigger — the button element
interface CheckboxTriggerProps
  extends Omit<
    React.ComponentPropsWithoutRef<typeof Primitive.button>,
    keyof CheckboxProviderProps
  > {
  children?: React.ReactNode;
}

// CheckboxBubbleInput — hidden input for form participation
interface CheckboxBubbleInputProps extends Omit<InputProps, 'checked'> {}
```

## Leptos Signature

```rust
pub fn Checkbox(
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] checked: MaybeProp<CheckedState>,
    #[prop(into, optional)] default_checked: MaybeProp<CheckedState>,
    #[prop(into, optional)] on_checked_change: Option<Callback<CheckedState>>,
    #[prop(into, optional)] required: MaybeProp<bool>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] value: MaybeProp<String>,
    /// The `id` of a `<form>` element to associate the checkbox with.
    #[prop(into, optional)] form: MaybeProp<String>,
    #[prop(into, optional)] on_keydown: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

### Leptos-only: `CheckedState` enum

```rust
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CheckedState {
    False,
    True,
    Indeterminate,
}
```

React uses `boolean | 'indeterminate'` as the `CheckedState` type. Leptos replaces this with a three-variant enum, mapping `false` to `CheckedState::False`, `true` to `CheckedState::True`, and `'indeterminate'` to `CheckedState::Indeterminate`.

## Prop Mapping

| React Prop       | Leptos Prop         | Type (React)                                       | Type (Leptos)                    | Description                                                                                                                                                                                                                                     |
| ---------------- | ------------------- | -------------------------------------------------- | -------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `checked`        | `checked`           | `CheckedState` (`boolean \| 'indeterminate'`)      | `MaybeProp<CheckedState>`        | The controlled checked state. Can be `true`, `false`, or `'indeterminate'` (React) / `CheckedState::True`, `CheckedState::False`, or `CheckedState::Indeterminate` (Leptos). When set, the component becomes controlled.                       |
| `defaultChecked` | `default_checked`   | `CheckedState`                                     | `MaybeProp<CheckedState>`        | The checked state on initial render. Use when you do not need to control state externally. Defaults to unchecked if not provided.                                                                                                               |
| `onCheckedChange`| `on_checked_change`  | `(checked: CheckedState) => void`                  | `Option<Callback<CheckedState>>` | Callback fired when the checked state changes. Receives the new `CheckedState`.                                                                                                                                                                 |
| `required`       | `required`          | `boolean`                                          | `MaybeProp<bool>`                | When `true`, sets `aria-required="true"` on the button element, indicating the checkbox must be checked for form submission.                                                                                                                    |
| `disabled`       | `disabled`          | `boolean`                                          | `MaybeProp<bool>`                | When `true`, disables the checkbox. The `disabled` HTML attribute and `data-disabled` data attribute are set on the button, and click interactions are suppressed.                                                                               |
| `name`           | `name`              | `string`                                           | `MaybeProp<String>`              | The name attribute for the hidden input element used for form submission.                                                                                                                                                                       |
| `value`          | `value`             | `string \| number \| readonly string[]` (default `'on'`) | `MaybeProp<String>` (default `"on"`) | The value submitted with the form when the checkbox is checked. Defaults to `"on"`. Note: React accepts `number` and `string[]` types in addition to `string`; Leptos only accepts `String`.                                             |
| `form`           | `form`              | `string`                                           | `MaybeProp<String>`              | The `id` of a `<form>` element to associate the checkbox with. Allows the checkbox to participate in a form even when not a descendant.                                                                                                         |
| —                | `on_keydown`        | *(via spread)*                                     | `Option<Callback<ev::KeyboardEvent>>` | Leptos surfaces this as an explicit prop. React accepts it via spread. The component internally composes this with its own keydown handler (which prevents `Enter` from activating the checkbox per WAI-ARIA).                              |
| —                | `on_click`          | *(via spread)*                                     | `Option<Callback<ev::MouseEvent>>` | Leptos surfaces this as an explicit prop. React accepts it via spread. The component composes this with its own click handler that toggles the checked state.                                                                                 |
| `ref`            | `node_ref`          | `React.Ref`                                        | `AnyNodeRef`                     | Ref to the root DOM element (`<button>`).                                                                                                                                                                                                       |
| `asChild`        | `as_child`          | `boolean`                                          | `MaybeProp<bool>`                | When `true`, renders the child directly instead of wrapping in a `<button>`, merging props and refs. The child must be a focusable, clickable element for accessibility.                                                                        |
| *(spread)*       | —                   | `...Omit<PrimitiveButtonProps, 'checked' \| 'defaultChecked'>` | —                        | React allows spreading any `<button>` HTML attribute. Leptos uses `attr:` directives on the call site instead.                                                                                                                                  |

### Data attributes (rendered on DOM)

| Attribute       | Value                                        | Description                                                                                                   |
| --------------- | -------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| `data-state`    | `"checked" \| "unchecked" \| "indeterminate"` | Reflects the current checked state. Useful for styling based on state.                                        |
| `data-disabled` | `""` (present/absent)                        | Present when the checkbox is disabled.                                                                        |

### Implicit behavior

- **Hidden input for form participation:** When the checkbox is inside a `<form>` (or has a `form` prop set), the component renders a hidden `<input type="checkbox">` that mirrors the checked state. This allows the checkbox to participate in native form submission, validation, and reset behavior. The hidden input is absolutely positioned, invisible, and has `aria-hidden="true"`.
- **Form reset support:** The component listens for the `reset` event on its parent form and restores the initial checked state when the form is reset.
- **Click propagation management:** When a hidden bubble input is present, the button's click event propagation is stopped to avoid duplicate events. The click is re-dispatched from the hidden input so that native form `change` events fire correctly.
- **Enter key prevention:** Per WAI-ARIA, checkboxes should not activate on `Enter` keypress (only `Space`). The component calls `event.preventDefault()` on `Enter` keydown events.

## Usage Examples

### Basic uncontrolled

#### React

```tsx
<Checkbox.Root>
  <Checkbox.Indicator>
    <CheckIcon />
  </Checkbox.Indicator>
</Checkbox.Root>
```

#### Leptos

```rust
<Checkbox>
  <CheckboxIndicator>
    <CheckIcon />
  </CheckboxIndicator>
</Checkbox>
```

### Default checked (uncontrolled)

#### React

```tsx
<Checkbox.Root defaultChecked>
  <Checkbox.Indicator>
    <CheckIcon />
  </Checkbox.Indicator>
</Checkbox.Root>
```

#### Leptos

```rust
<Checkbox default_checked=CheckedState::True>
  <CheckboxIndicator>
    <CheckIcon />
  </CheckboxIndicator>
</Checkbox>
```

### Controlled

#### React

```tsx
const [checked, setChecked] = React.useState<CheckedState>(true);

<Checkbox.Root checked={checked} onCheckedChange={setChecked}>
  <Checkbox.Indicator>
    <CheckIcon />
  </Checkbox.Indicator>
</Checkbox.Root>
```

#### Leptos

```rust
let (checked, set_checked) = signal(CheckedState::True);

<Checkbox
  checked=checked
  on_checked_change=move |state| set_checked.set(state)
>
  <CheckboxIndicator>
    <CheckIcon />
  </CheckboxIndicator>
</Checkbox>
```

### Indeterminate state

#### React

```tsx
const [checked, setChecked] = React.useState<CheckedState>('indeterminate');

<Checkbox.Root checked={checked} onCheckedChange={setChecked}>
  <Checkbox.Indicator>
    {checked === 'indeterminate' ? <DashIcon /> : <CheckIcon />}
  </Checkbox.Indicator>
</Checkbox.Root>

<button onClick={() => setChecked(prev =>
  prev === 'indeterminate' ? false : 'indeterminate'
)}>
  Toggle indeterminate
</button>
```

#### Leptos

```rust
let (checked, set_checked) = signal(CheckedState::Indeterminate);

<Checkbox
  checked=checked
  on_checked_change=move |state| set_checked.set(state)
>
  <CheckboxIndicator>
    {move || match checked.get() {
      CheckedState::Indeterminate => view! { <DashIcon /> }.into_any(),
      _ => view! { <CheckIcon /> }.into_any(),
    }}
  </CheckboxIndicator>
</Checkbox>

<button on:click=move |_| {
  set_checked.update(|c| {
    *c = match c {
      CheckedState::Indeterminate => CheckedState::False,
      _ => CheckedState::Indeterminate,
    };
  });
}>
  "Toggle indeterminate"
</button>
```

### Disabled

#### React

```tsx
<Checkbox.Root disabled>
  <Checkbox.Indicator>
    <CheckIcon />
  </Checkbox.Indicator>
</Checkbox.Root>
```

#### Leptos

```rust
<Checkbox disabled=true>
  <CheckboxIndicator>
    <CheckIcon />
  </CheckboxIndicator>
</Checkbox>
```

### Within a form

#### React

```tsx
<form onSubmit={handleSubmit}>
  <Checkbox.Root name="agree" required>
    <Checkbox.Indicator>
      <CheckIcon />
    </Checkbox.Indicator>
  </Checkbox.Root>
  <button type="submit">Submit</button>
</form>
```

#### Leptos

```rust
<form on:submit=handle_submit>
  <Checkbox name="agree" required=true>
    <CheckboxIndicator>
      <CheckIcon />
    </CheckboxIndicator>
  </Checkbox>
  <button type="submit">"Submit"</button>
</form>
```

### With label

#### React

```tsx
<label>
  <Checkbox.Root>
    <Checkbox.Indicator>
      <CheckIcon />
    </Checkbox.Indicator>
  </Checkbox.Root>{' '}
  Accept terms and conditions
</label>
```

#### Leptos

```rust
<label>
  <Checkbox>
    <CheckboxIndicator>
      <CheckIcon />
    </CheckboxIndicator>
  </Checkbox>
  " Accept terms and conditions"
</label>
```

## Accessibility

Implements the [WAI-ARIA Checkbox pattern](https://www.w3.org/WAI/ARIA/apd/patterns/checkbox/).

### Keyboard Interactions

| Key     | Description                                                                                                       |
| ------- | ----------------------------------------------------------------------------------------------------------------- |
| `Space` | Toggles the checkbox between checked and unchecked. When indeterminate, transitions to checked.                   |
| `Enter` | **Prevented.** Per WAI-ARIA, checkboxes do not activate on `Enter`. The component calls `event.preventDefault()`. |

### ARIA Attributes

| Element    | Attribute       | Value                              | Notes                                                                                                                                                    |
| ---------- | --------------- | ---------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Checkbox` | `role`          | `"checkbox"`                       | Identifies the element as a checkbox.                                                                                                                    |
| `Checkbox` | `aria-checked`  | `"true" \| "false" \| "mixed"`    | Reflects the current checked state. `"mixed"` corresponds to the indeterminate state.                                                                    |
| `Checkbox` | `aria-required` | `"true" \| "false"`                | Set based on the `required` prop.                                                                                                                        |
| `Checkbox` | `disabled`      | *(HTML attribute, not ARIA)*       | Set when the checkbox is disabled. Native `<button>` disabled behavior prevents interaction.                                                             |

### Behavioral Notes

- The underlying element is a `<button type="button">` with `role="checkbox"`. This avoids form submission on click while providing native focusability and keyboard support.
- Clicking the checkbox cycles through states: unchecked toggles to checked, checked toggles to unchecked, indeterminate transitions to checked (not unchecked).
- When `disabled` is `true`, the Leptos implementation explicitly guards against click handling (checking `disabled.get()` before toggling state), in addition to setting the HTML `disabled` attribute.
- A hidden `<input type="checkbox">` is rendered when inside a form to ensure native form submission, validation, and reset work correctly.
- The `Checkbox` component works with both native `<label>` elements and the Radix `Label` component. Wrapping the checkbox inside a `<label>` or associating via `htmlFor`/`for` and `id` both work.
