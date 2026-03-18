# PasswordToggleField (Root)

## Anatomy

The expected component nesting structure:

```
PasswordToggleField
├── PasswordToggleFieldInput
└── PasswordToggleFieldToggle
    └── PasswordToggleFieldSlot or PasswordToggleFieldIcon (optional)
```

### React

```tsx
<PasswordToggleField.Root>
  <PasswordToggleField.Input />
  <PasswordToggleField.Toggle>
    <PasswordToggleField.Slot visible="Hide" hidden="Show" />
  </PasswordToggleField.Toggle>
</PasswordToggleField.Root>
```

### Leptos

```rust
<PasswordToggleField>
  <PasswordToggleFieldInput />
  <PasswordToggleFieldToggle>
    <PasswordToggleFieldSlot
      visible_content=|| "Hide".into_any()
      hidden_content=|| "Show".into_any()
    />
  </PasswordToggleFieldToggle>
</PasswordToggleField>
```

## React Signature

`PasswordToggleField` is a plain `React.FC` (not `forwardRef`) since it renders no DOM element itself -- it is a context provider only.

```typescript
interface PasswordToggleFieldProps {
  id?: string;
  visible?: boolean;
  defaultVisible?: boolean;
  onVisiblityChange?: (visible: boolean) => void;  // NOTE: typo in React source ("Visiblity")
  children?: React.ReactNode;
}

const PasswordToggleField: React.FC<PasswordToggleFieldProps>
```

## Leptos Signature

```rust
pub fn PasswordToggleField(
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] visible: MaybeProp<bool>,
    #[prop(into, optional)] default_visible: MaybeProp<bool>,
    #[prop(into, optional)] on_visibility_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `id` | `id` | `string \| undefined` | `MaybeProp<String>` | Base ID used to derive the input element's ID (as `"{id}-input"`). If omitted, an auto-generated ID is used. |
| `visible` | `visible` | `boolean \| undefined` | `MaybeProp<bool>` | The controlled visibility state. When set, the component becomes controlled. `true` shows the password as plain text; `false` masks it. |
| `defaultVisible` | `default_visible` | `boolean \| undefined` (default `false`) | `MaybeProp<bool>` (default `false`) | The initial visibility state for the uncontrolled component. |
| `onVisiblityChange` | `on_visibility_change` | `(visible: boolean) => void` | `Option<Callback<bool>>` | Callback fired when visibility changes. Note: the React prop name has a typo (`onVisiblityChange`, missing "i" in "Visibility"); the Leptos prop corrects the spelling to `on_visibility_change`. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | The field contents (typically an `Input` and a `Toggle`). |

### Implicit behavior

- This component renders no DOM element. It is a pure context provider that manages visibility state and coordinates the `Input`, `Toggle`, `Slot`, and `Icon` sub-components.
- Generates a default input ID of the form `"{baseId}-input"` which is shared with child components via context for ARIA linking.

## Usage Examples

### Basic uncontrolled

#### React

```tsx
<label htmlFor="password">Password</label>
<PasswordToggleField.Root>
  <PasswordToggleField.Input id="password" />
  <PasswordToggleField.Toggle>
    <PasswordToggleField.Slot visible="Hide" hidden="Show" />
  </PasswordToggleField.Toggle>
</PasswordToggleField.Root>
```

#### Leptos

```rust
<label for="password">"Password"</label>
<PasswordToggleField>
  <PasswordToggleFieldInput id="password" />
  <PasswordToggleFieldToggle>
    <PasswordToggleFieldSlot
      visible_content=|| "Hide".into_any()
      hidden_content=|| "Show".into_any()
    />
  </PasswordToggleFieldToggle>
</PasswordToggleField>
```

### Controlled

#### React

```tsx
const [visible, setVisible] = React.useState(false);

<PasswordToggleField.Root
  visible={visible}
  onVisiblityChange={setVisible}
>
  <PasswordToggleField.Input />
  <PasswordToggleField.Toggle>
    <PasswordToggleField.Slot visible="Hide" hidden="Show" />
  </PasswordToggleField.Toggle>
</PasswordToggleField.Root>
```

#### Leptos

```rust
let (visible, set_visible) = signal(false);

<PasswordToggleField
  visible=visible
  on_visibility_change=Callback::new(move |v: bool| set_visible.set(v))
>
  <PasswordToggleFieldInput />
  <PasswordToggleFieldToggle>
    <PasswordToggleFieldSlot
      visible_content=|| "Hide".into_any()
      hidden_content=|| "Show".into_any()
    />
  </PasswordToggleFieldToggle>
</PasswordToggleField>
```

### With icon

#### React

```tsx
<PasswordToggleField.Root>
  <PasswordToggleField.Input />
  <PasswordToggleField.Toggle>
    <PasswordToggleField.Icon
      visible={<EyeOpenIcon />}
      hidden={<EyeClosedIcon />}
    />
  </PasswordToggleField.Toggle>
</PasswordToggleField.Root>
```

#### Leptos

```rust
<PasswordToggleField>
  <PasswordToggleFieldInput />
  <PasswordToggleFieldToggle>
    <PasswordToggleFieldIcon
      visible_icon=|| view! { <EyeOpenIcon /> }.into_any()
      hidden_icon=|| view! { <EyeClosedIcon /> }.into_any()
    />
  </PasswordToggleFieldToggle>
</PasswordToggleField>
```

### Inside a form

#### React

```tsx
<form onSubmit={(e) => e.preventDefault()}>
  <label htmlFor="password">Password</label>
  <PasswordToggleField.Root>
    <PasswordToggleField.Input id="password" />
    <PasswordToggleField.Toggle>
      <PasswordToggleField.Slot visible="Hide" hidden="Show" />
    </PasswordToggleField.Toggle>
  </PasswordToggleField.Root>
  <button>Submit</button>
</form>
```

#### Leptos

```rust
<form on:submit=|e: ev::SubmitEvent| e.prevent_default()>
  <label for="password">"Password"</label>
  <PasswordToggleField>
    <PasswordToggleFieldInput id="password" />
    <PasswordToggleFieldToggle>
      <PasswordToggleFieldSlot
        visible_content=|| "Hide".into_any()
        hidden_content=|| "Show".into_any()
      />
    </PasswordToggleFieldToggle>
  </PasswordToggleField>
  <button>"Submit"</button>
</form>
```

## Accessibility

The component follows accessible password field best practices. It does not implement a single named WAI-ARIA pattern but combines several accessible techniques:

- The toggle button is linked to the input via `aria-controls`.
- An auto-generated `aria-label` (e.g., "Show password" / "Hide password") is applied to the toggle if it has no visible text content.
- The input's `type` attribute toggles between `"password"` and `"text"` based on visibility state.

### Keyboard Interactions

| Key | Description |
|---|---|
| `Enter` / `Space` | When the toggle button is focused, toggles password visibility. Focus returns to the input with cursor position preserved. |
| `Tab` | Standard tab navigation between the input and the toggle button. |

### ARIA Attributes

| Element | Attribute | Value | Notes |
|---|---|---|---|
| `Toggle` (`<button>`) | `aria-controls` | `string` | Points to the input element's `id`, linking the button to the field it controls. |
| `Toggle` (`<button>`) | `aria-label` | `"Show password"` / `"Hide password"` | Auto-generated label when the button has no visible text content. If the button contains text (e.g., via `Slot`), this is omitted. If a user provides `aria-label`, that takes precedence. |
| `Toggle` (`<button>`) | `aria-pressed` | `"true"` / `"false"` | **Leptos only.** Indicates the toggle's pressed state. React does not set this attribute. |
| `Icon` (`<svg>`) | `aria-hidden` | `"true"` | The icon is decorative; the toggle button provides the accessible label. |

### Behavioral Notes

- When toggled via pointer click, focus is moved to the input and the previous cursor/selection position is restored.
- When the associated form is submitted, visibility is always reset to `false` (password masked) to prevent the browser from remembering the visible value.
- When the associated form is reset, visibility is reset to `false` unless the reset event's default action is prevented.
- Before hydration (SSR), React hides the toggle from assistive tech (`aria-hidden="true"`, `tabIndex={-1}`) and only enables it post-hydration. Leptos is CSR-only, so the toggle is always interactive immediately.
