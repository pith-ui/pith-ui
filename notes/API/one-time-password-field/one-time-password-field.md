# OneTimePasswordField (Root)

## Anatomy

The expected component nesting structure:

```
OneTimePasswordField
├── OneTimePasswordFieldInput (one per character slot)
├── ... (optional separators or other elements between inputs)
└── OneTimePasswordFieldHiddenInput (submits the combined value with the form)
```

### React

```tsx
<OneTimePasswordField.Root>
  <OneTimePasswordField.Input />
  <OneTimePasswordField.Input />
  <OneTimePasswordField.Input />
  <OneTimePasswordField.Input />
  <OneTimePasswordField.Input />
  <OneTimePasswordField.Input />
  <OneTimePasswordField.HiddenInput name="code" />
</OneTimePasswordField.Root>
```

### Leptos

```rust
<OneTimePasswordField>
  <OneTimePasswordFieldInput />
  <OneTimePasswordFieldInput />
  <OneTimePasswordFieldInput />
  <OneTimePasswordFieldInput />
  <OneTimePasswordFieldInput />
  <OneTimePasswordFieldInput />
  <OneTimePasswordFieldHiddenInput name="code" />
</OneTimePasswordField>
```

## React Signature

```typescript
const OneTimePasswordField = React.forwardRef<HTMLDivElement, OneTimePasswordFieldProps>(...)

interface OneTimePasswordFieldOwnProps {
  autoComplete?: AutoComplete;       // 'off' | 'one-time-code', default 'one-time-code'
  autoFocus?: boolean;               // default false
  autoSubmit?: boolean;              // default false
  defaultValue?: string;
  dir?: RovingFocusGroupProps['dir'];
  disabled?: boolean;                // default false
  form?: string | undefined;
  name?: string | undefined;
  onAutoSubmit?: (value: string) => void;
  onValueChange?: (value: string) => void;
  orientation?: RovingFocusGroupProps['orientation']; // default 'horizontal'
  placeholder?: string | undefined;
  readOnly?: boolean;                // default false
  sanitizeValue?: (value: string) => string;
  type?: InputType;                  // 'password' | 'text', default 'text'
  validationType?: InputValidationType; // 'alpha' | 'numeric' | 'alphanumeric' | 'none', default 'numeric'
  value?: string;
}

interface OneTimePasswordFieldProps
  extends OneTimePasswordFieldOwnProps,
    Omit<Primitive.PrimitivePropsWithRef<'div'>, keyof OneTimePasswordFieldOwnProps> {}

type InputType = 'password' | 'text';
type AutoComplete = 'off' | 'one-time-code';
type InputValidationType = 'alpha' | 'numeric' | 'alphanumeric' | 'none';
```

## Leptos Signature

```rust
pub fn OneTimePasswordField(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] auto_submit: MaybeProp<bool>,
    #[prop(into, optional)] on_auto_submit: Option<Callback<String>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] read_only: MaybeProp<bool>,
    #[prop(into, optional)] auto_complete: MaybeProp<AutoComplete>,
    #[prop(into, optional)] auto_focus: MaybeProp<bool>,
    #[prop(into, optional)] form: MaybeProp<String>,
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    #[prop(into, optional)] r#type: MaybeProp<InputType>,
    #[prop(into, optional)] orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] validation_type: MaybeProp<InputValidationType>,
    #[prop(into, optional)] sanitize_value: Option<Callback<String, String>>,
    #[prop(into, optional)] on_paste: Option<Callback<ev::ClipboardEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `value` | `value` | `string \| undefined` | `MaybeProp<String>` | The controlled value of the field as a single string. Each character maps to one input slot. When set, the component becomes controlled. |
| `defaultValue` | `default_value` | `string \| undefined` | `MaybeProp<String>` | The initial value of the uncontrolled field. |
| `onValueChange` | `on_value_change` | `(value: string) => void` | `Option<Callback<String>>` | Callback fired when the field's value changes. Receives the full value as a joined string. |
| `autoSubmit` | `auto_submit` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | Whether the component should attempt to automatically submit the associated form when all fields are filled. |
| `onAutoSubmit` | `on_auto_submit` | `(value: string) => void` | `Option<Callback<String>>` | Callback fired before auto-submit is attempted. Called whether or not a form is located or submission is allowed. |
| `disabled` | `disabled` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | Whether the field's input elements are disabled. |
| `readOnly` | `read_only` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | Whether the input elements can be updated by the user. |
| `autoComplete` | `auto_complete` | `'off' \| 'one-time-code'` (default `'one-time-code'`) | `MaybeProp<AutoComplete>` (default `OneTimeCode`) | The autocomplete hint for the browser. Only `"one-time-code"` and `"off"` are supported. |
| `autoFocus` | `auto_focus` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | Whether the first fillable input should be focused on page load. |
| `form` | `form` | `string \| undefined` | `MaybeProp<String>` | The `id` of the `<form>` element to associate with. If omitted, uses the nearest ancestor form. |
| `name` | `name` | `string \| undefined` | `MaybeProp<String>` | The name submitted with form data. Passed to the hidden input. |
| `placeholder` | `placeholder` | `string \| undefined` | `MaybeProp<String>` | Placeholder text. Each character maps to one input slot's placeholder (e.g., `"123456"` shows `"1"` in the first input, `"2"` in the second, etc.). Only displayed when the value is empty. |
| `type` | `r#type` | `'password' \| 'text'` (default `'text'`) | `MaybeProp<InputType>` (default `Text`) | The HTML input type. Use `"password"` to mask characters. |
| `orientation` | `orientation` | `'horizontal' \| 'vertical'` (default `'horizontal'`) | `MaybeProp<Orientation>` (default `Horizontal`) | The layout axis for roving focus navigation. Controls which arrow keys move between inputs. |
| `dir` | `dir` | `'ltr' \| 'rtl'` | `MaybeProp<Direction>` | The reading direction. Affects horizontal arrow-key navigation. |
| `validationType` | `validation_type` | `'alpha' \| 'numeric' \| 'alphanumeric' \| 'none'` (default `'numeric'`) | `MaybeProp<InputValidationType>` (default `Numeric`) | The type of input validation. Characters not matching the type are rejected. Set to `"none"` to allow any input or use a custom `sanitizeValue`. |
| `sanitizeValue` | `sanitize_value` | `(value: string) => string` | `Option<Callback<String, String>>` | Custom sanitization function for when `validationType` is `"none"`. Called before updating values in response to user interactions. |
| `onPaste` | `on_paste` | `React.ClipboardEventHandler` | `Option<Callback<ev::ClipboardEvent>>` | Callback fired on paste events on the root container. The component handles paste internally (distributing pasted characters across inputs), but this allows additional behavior. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the root DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| *(spread)* | -- | `...PrimitiveDivProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Leptos-only enums

```rust
pub enum InputValidationType {
    Alpha,
    Numeric,      // default
    Alphanumeric,
    None,
}

pub enum InputType {
    Password,
    Text,         // default
}

pub enum AutoComplete {
    Off,
    OneTimeCode,  // default
}
```

### Implicit behavior

- The root element renders with `role="group"`.
- Paste events on the root are intercepted: pasted text is sanitized per the active `validationType` and distributed across the input slots.
- Listens for `reset` events on the associated form and clears the field value.
- When `autoSubmit` is `true` and all input slots are filled, calls `onAutoSubmit` then invokes `requestSubmit()` on the associated form.
- Re-validates the current value when `validationType` changes (e.g., switching from `"alphanumeric"` to `"numeric"` strips letters).

## Usage Examples

### Basic uncontrolled

#### React

```tsx
<OneTimePasswordField.Root>
  <OneTimePasswordField.Input />
  <OneTimePasswordField.Input />
  <OneTimePasswordField.Input />
  <OneTimePasswordField.Input />
  <OneTimePasswordField.Input />
  <OneTimePasswordField.Input />
  <OneTimePasswordField.HiddenInput name="code" />
</OneTimePasswordField.Root>
```

#### Leptos

```rust
<OneTimePasswordField>
  <OneTimePasswordFieldInput />
  <OneTimePasswordFieldInput />
  <OneTimePasswordFieldInput />
  <OneTimePasswordFieldInput />
  <OneTimePasswordFieldInput />
  <OneTimePasswordFieldInput />
  <OneTimePasswordFieldHiddenInput name="code" />
</OneTimePasswordField>
```

### Controlled

#### React

```tsx
const [code, setCode] = React.useState('');

<OneTimePasswordField.Root
  value={code}
  onValueChange={setCode}
>
  <OneTimePasswordField.Input />
  <OneTimePasswordField.Input />
  <OneTimePasswordField.Input />
  <OneTimePasswordField.Input />
  <OneTimePasswordField.HiddenInput name="code" />
</OneTimePasswordField.Root>
```

#### Leptos

```rust
let (code, set_code) = signal(String::new());

<OneTimePasswordField
  value=code
  on_value_change=Callback::new(move |v: String| set_code.set(v))
>
  <OneTimePasswordFieldInput />
  <OneTimePasswordFieldInput />
  <OneTimePasswordFieldInput />
  <OneTimePasswordFieldInput />
  <OneTimePasswordFieldHiddenInput name="code" />
</OneTimePasswordField>
```

### Auto-submit

#### React

```tsx
<OneTimePasswordField.Root
  autoSubmit
  onAutoSubmit={(value) => console.log('Submitting:', value)}
>
  {/* ...inputs */}
  <OneTimePasswordField.HiddenInput name="code" />
</OneTimePasswordField.Root>
```

#### Leptos

```rust
<OneTimePasswordField
  auto_submit=true
  on_auto_submit=Callback::new(move |value: String| {
    log::info!("Submitting: {}", value);
  })
>
  // ...inputs
  <OneTimePasswordFieldHiddenInput name="code" />
</OneTimePasswordField>
```

### Password-masked input

#### React

```tsx
<OneTimePasswordField.Root type="password">
  <OneTimePasswordField.Input />
  <OneTimePasswordField.Input />
  <OneTimePasswordField.Input />
  <OneTimePasswordField.Input />
  <OneTimePasswordField.HiddenInput name="code" />
</OneTimePasswordField.Root>
```

#### Leptos

```rust
<OneTimePasswordField r#type=InputType::Password>
  <OneTimePasswordFieldInput />
  <OneTimePasswordFieldInput />
  <OneTimePasswordFieldInput />
  <OneTimePasswordFieldInput />
  <OneTimePasswordFieldHiddenInput name="code" />
</OneTimePasswordField>
```

### Alpha validation

#### React

```tsx
<OneTimePasswordField.Root validationType="alpha">
  <OneTimePasswordField.Input />
  <OneTimePasswordField.Input />
  <OneTimePasswordField.Input />
  <OneTimePasswordField.Input />
  <OneTimePasswordField.HiddenInput name="code" />
</OneTimePasswordField.Root>
```

#### Leptos

```rust
<OneTimePasswordField validation_type=InputValidationType::Alpha>
  <OneTimePasswordFieldInput />
  <OneTimePasswordFieldInput />
  <OneTimePasswordFieldInput />
  <OneTimePasswordFieldInput />
  <OneTimePasswordFieldHiddenInput name="code" />
</OneTimePasswordField>
```

## Accessibility

The component does not implement a single named WAI-ARIA pattern but combines several accessible practices:

- The root renders with `role="group"` to semantically group the individual inputs.
- Each input has a computed `aria-label` of the form `"Character N of M"` (e.g., `"Character 1 of 6"`).
- Roving focus (via `RovingFocusGroup`) manages tab stops so that only one input is in the tab order at a time.

### Keyboard Interactions

| Key | Description |
|---|---|
| `0`-`9` / `a`-`z` | Enters the character in the focused input (if it passes validation) and advances focus to the next input. |
| `Backspace` | Clears the current input's value and moves focus to the previous input. If the current input is empty, moves focus to the previous input. With `Ctrl`/`Cmd`, clears all inputs. |
| `Delete` | Clears the current input's value. Focus stays on the same input. |
| `Enter` | Submits the associated form via `requestSubmit()`. |
| `ArrowRight` | Moves focus to the next input (in `horizontal` orientation). |
| `ArrowLeft` | Moves focus to the previous input (in `horizontal` orientation). |
| `ArrowDown` | Moves focus to the next input (in `vertical` orientation). In `horizontal` orientation, the event is prevented (no-op). |
| `ArrowUp` | Moves focus to the previous input (in `vertical` orientation). In `horizontal` orientation, the event is prevented (no-op). |
| `Ctrl+V` / `Cmd+V` | Pastes clipboard content. The pasted string is sanitized and distributed across all inputs starting from the first. |
| `Home` | Moves focus to the first input (via roving focus). |
| `End` | Moves focus to the last input (via roving focus). |

### ARIA Attributes

| Element | Attribute | Value | Notes |
|---|---|---|---|
| Root (`<div>`) | `role` | `"group"` | Groups the input elements semantically. |
| Each `Input` | `aria-label` | `"Character N of M"` | Auto-generated label indicating position in the sequence. |

### Behavioral Notes

- Only inputs up to and including the first empty slot are focusable. Clicking a later input redirects focus to the last selectable position.
- On focus, the input's value is selected so that typing replaces it.
- Password managers that auto-fill a single input with multiple characters are detected and handled as a paste operation.
- The `placeholder` prop distributes one character per input: `placeholder="123456"` shows `"1"`, `"2"`, ..., `"6"` across six inputs. Placeholders are only displayed when the entire value is empty.
- When the form is reset (via a reset button or `form.reset()`), the field value is cleared.
