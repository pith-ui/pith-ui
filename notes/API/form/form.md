# Form (Root)

## Anatomy

The expected component nesting structure:

```
Form
├── FormField (one per input, identified by `name`)
│   ├── FormLabel
│   ├── FormControl
│   ├── FormMessage (zero or more, for validation messages)
│   └── FormValidityState (optional, render prop for validity)
└── FormSubmit
```

### React

```tsx
<Form.Root onClearServerErrors={() => setServerErrors({})}>
  <Form.Field name="email" serverInvalid={serverErrors.email}>
    <Form.Label>Email</Form.Label>
    <Form.Control type="email" required />
    <Form.Message match="valueMissing" />
    <Form.Message match="typeMismatch">Email is invalid</Form.Message>
  </Form.Field>
  <Form.Submit>Submit</Form.Submit>
</Form.Root>
```

### Leptos

```rust
<Form
    on_clear_server_errors=Callback::new(move |_| set_server_errors.set(false))
>
    <FormField name="email" server_invalid=Signal::derive(move || server_errors.get())>
        <FormLabel>"Email"</FormLabel>
        <FormControl attr:r#type="email" attr:required="" />
        <FormMessage r#match=Match::BuiltIn(ValidityMatcher::ValueMissing) />
        <FormMessage r#match=Match::BuiltIn(ValidityMatcher::TypeMismatch)>
            "Email is invalid"
        </FormMessage>
    </FormField>
    <FormSubmit>"Submit"</FormSubmit>
</Form>
```

## React Signature

```typescript
const Form = React.forwardRef<FormElement, FormProps>(...)

type FormElement = React.ComponentRef<typeof Primitive.form>;
type PrimitiveFormProps = React.ComponentPropsWithoutRef<typeof Primitive.form>;

interface FormProps extends PrimitiveFormProps {
  onClearServerErrors?(): void;
}
```

## Leptos Signature

```rust
pub fn Form(
    #[prop(into, optional)] on_clear_server_errors: Option<Callback<()>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `onClearServerErrors` | `on_clear_server_errors` | `() => void` | `Option<Callback<()>>` | Callback fired when the form is submitted or reset, to allow clearing server-side validation errors. Called on both `submit` and `reset` events. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the root DOM element (`<form>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<form>`, merging props and refs. |
| *(spread)* | -- | `...PrimitiveFormProps` | -- | React allows spreading any `<form>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Provides two contexts consumed by all descendant parts:
  - **ValidationContext** -- tracks per-field validity state, custom matcher entries, and custom errors.
  - **AriaDescriptionContext** -- tracks per-field message IDs so `FormControl` can set `aria-describedby`.
- Registers a **capture-phase** `invalid` event listener on the `<form>` to:
  1. Focus the first invalid control when the form is submitted with invalid fields.
  2. Suppress the browser's default validation UI (tooltips) via `event.preventDefault()`.
- On `submit`, calls `onClearServerErrors` (without preventing default -- the user's `onSubmit` handler is responsible for `event.preventDefault()` if needed).
- On `reset`, calls `onClearServerErrors`.

### Data attributes (rendered on DOM)

No data attributes are rendered on the root `<form>` element.

## Usage Examples

### Basic form with built-in validation

#### React

```tsx
<Form.Root>
  <Form.Field name="name">
    <Form.Label>Name</Form.Label>
    <Form.Control type="text" required />
    <Form.Message match="valueMissing">Name is required</Form.Message>
  </Form.Field>
  <Form.Submit>Submit</Form.Submit>
</Form.Root>
```

#### Leptos

```rust
<Form>
    <FormField name="name">
        <FormLabel>"Name"</FormLabel>
        <FormControl attr:r#type="text" attr:required="" />
        <FormMessage r#match=Match::BuiltIn(ValidityMatcher::ValueMissing)>
            "Name is required"
        </FormMessage>
    </FormField>
    <FormSubmit>"Submit"</FormSubmit>
</Form>
```

### Server-side validation

#### React

```tsx
const [serverErrors, setServerErrors] = React.useState({});

<Form.Root onClearServerErrors={() => setServerErrors({})}>
  <Form.Field name="email" serverInvalid={serverErrors.email}>
    <Form.Label>Email</Form.Label>
    <Form.Control type="email" />
    <Form.Message match="typeMismatch" forceMatch={serverErrors.email}>
      Email is invalid
    </Form.Message>
  </Form.Field>
  <Form.Submit>Submit</Form.Submit>
</Form.Root>
```

#### Leptos

```rust
let (server_errors, set_server_errors) = signal(false);

<Form
    on_clear_server_errors=Callback::new(move |_| set_server_errors.set(false))
>
    <FormField name="email" server_invalid=Signal::derive(move || server_errors.get())>
        <FormLabel>"Email"</FormLabel>
        <FormControl attr:r#type="email" />
        <FormMessage
            r#match=Match::BuiltIn(ValidityMatcher::TypeMismatch)
            force_match=Signal::derive(move || server_errors.get())
        >
            "Email is invalid"
        </FormMessage>
    </FormField>
    <FormSubmit>"Submit"</FormSubmit>
</Form>
```

### Custom sync validation

#### React

```tsx
<Form.Field name="secret">
  <Form.Label>Secret</Form.Label>
  <Form.Control type="text" />
  <Form.Message match={(value) => value !== 'shush'}>
    Wrong secret
  </Form.Message>
</Form.Field>
```

#### Leptos

```rust
<FormField name="secret">
    <FormLabel>"Secret"</FormLabel>
    <FormControl attr:r#type="text" />
    <FormMessage
        r#match=Match::Custom(Rc::new(|value: String, _form_data: web_sys::FormData| {
            value != "shush"
        }))
    >
        "Wrong secret"
    </FormMessage>
</FormField>
```

### Custom async validation

#### React

```tsx
<Form.Field name="username">
  <Form.Label>Username</Form.Label>
  <Form.Control type="text" />
  <Form.Message match={async (value) => {
    await checkUsernameAvailability(value);
    return false;
  }}>
    Username is taken
  </Form.Message>
</Form.Field>
```

#### Leptos

```rust
<FormField name="username">
    <FormLabel>"Username"</FormLabel>
    <FormControl attr:r#type="text" />
    <FormMessage
        r#match=Match::CustomAsync(Rc::new(|value: String, _form_data: web_sys::FormData| {
            Box::pin(async move {
                // check availability...
                value == "taken"
            })
        }))
    >
        "Username is taken"
    </FormMessage>
</FormField>
```

## Accessibility

Implements custom form validation that enhances the native [HTML form validation API](https://developer.mozilla.org/en-US/docs/Web/HTML/Constraint_validation) with accessible error messaging.

### Keyboard Interactions

| Key | Description |
|---|---|
| `Enter` | When focus is on a form control or the submit button, triggers form submission (standard HTML behavior). |
| `Tab` / `Shift+Tab` | Moves focus between form controls (standard HTML behavior). |

### ARIA Attributes

| Element | Attribute | Value | Notes |
|---|---|---|---|
| `FormControl` | `aria-invalid` | `"true"` / absent | Set to `"true"` when the field's `serverInvalid` prop is `true`. |
| `FormControl` | `aria-describedby` | space-separated IDs | Points to all rendered `FormMessage` elements for this field. Auto-managed: IDs are added when messages mount and removed when they unmount. |

### Behavioral Notes

- The form suppresses the browser's default constraint validation UI (built-in tooltips) by calling `preventDefault()` on the `invalid` event in the capture phase.
- When the form is submitted and has invalid controls, focus is automatically moved to the first invalid control.
- When `serverInvalid` is set to `true` on a field, focus is automatically moved to the first server-invalid control.
- Validation runs on the native `change` event (not on every keystroke). When the user modifies a value, existing validation is cleared (via the native `input` event).
- Validation is also cleared when the form is reset.
- Built-in validation messages (`match="valueMissing"`, etc.) only display when the corresponding `ValidityState` flag is `true`.
- Custom matchers run only after all built-in validations pass.
- Async custom matchers run only after all sync custom matchers pass.
