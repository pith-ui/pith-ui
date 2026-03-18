# FormMessage

## React Signature

```typescript
const FormMessage = React.forwardRef<FormMessageElement, FormMessageProps>(...)

type FormMessageElement = React.ComponentRef<typeof Primitive.span>;

interface FormMessageProps extends Omit<FormMessageImplProps, 'name'> {
  match?: ValidityMatcher | CustomMatcher;
  forceMatch?: boolean;
  name?: string;
}

// ValidityMatcher is a string union:
type ValidityMatcher =
  | 'badInput'
  | 'patternMismatch'
  | 'rangeOverflow'
  | 'rangeUnderflow'
  | 'stepMismatch'
  | 'tooLong'
  | 'tooShort'
  | 'typeMismatch'
  | 'valid'
  | 'valueMissing';

// CustomMatcher is a function:
type SyncCustomMatcher = (value: string, formData: FormData) => boolean;
type AsyncCustomMatcher = (value: string, formData: FormData) => Promise<boolean>;
type CustomMatcher = SyncCustomMatcher | AsyncCustomMatcher;

interface FormMessageImplProps extends PrimitiveSpanProps {
  name: string;
}
```

## Leptos Signature

```rust
pub fn FormMessage(
    #[prop(into, optional)] r#match: Option<Match>,
    #[prop(into, optional)] force_match: MaybeProp<bool>,
    #[prop(into, optional)] name: Option<String>,
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

### Leptos-only: `Match` enum

React's `match` prop accepts either a `ValidityMatcher` string or a `CustomMatcher` function. Leptos uses a typed enum instead:

```rust
pub enum Match {
    BuiltIn(ValidityMatcher),
    Custom(SyncCustomMatcherFn),
    CustomAsync(AsyncCustomMatcherFn),
}

pub type SyncCustomMatcherFn = Rc<dyn Fn(String, web_sys::FormData) -> bool>;
pub type AsyncCustomMatcherFn =
    Rc<dyn Fn(String, web_sys::FormData) -> Pin<Box<dyn Future<Output = bool>>>>;
```

### Leptos-only: `ValidityMatcher` enum

```rust
pub enum ValidityMatcher {
    BadInput,
    PatternMismatch,
    RangeOverflow,
    RangeUnderflow,
    StepMismatch,
    TooLong,
    TooShort,
    TypeMismatch,
    Valid,
    ValueMissing,
}
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `match` | `r#match` | `ValidityMatcher \| CustomMatcher \| undefined` | `Option<Match>` | Determines when the message is shown. When `undefined` / `None`, the message is always shown (unconditional error). When a `ValidityMatcher` / `Match::BuiltIn`, shown when the corresponding HTML5 validity flag is `true`. When a custom matcher function / `Match::Custom` or `Match::CustomAsync`, shown when the matcher returns `true`. |
| `forceMatch` | `force_match` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | When `true`, the message is shown regardless of the field's client-side validity state. Use for server-side validation errors that should display even when the client considers the field valid. |
| `name` | `name` | `string \| undefined` | `Option<String>` | The field name to check validity against. Defaults to the parent `FormField`'s `name`. Override when a message needs to reference a different field. |
| -- | `id` | -- | `Option<String>` | The `id` attribute for the message element. Auto-generated if not provided. Used for `aria-describedby` wiring. Note: React passes this through the spread props on the underlying `<span>`. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<span>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<span>`, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | The message content. When omitted, a default message is used based on the `match` type (e.g., "This value is missing" for `valueMissing`, "This value is not valid" for custom matchers). |
| *(spread)* | -- | `...PrimitiveSpanProps` | -- | React allows spreading any `<span>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Three rendering modes based on the `match` prop:
  1. **No match** (unconditional): Always renders the message. Used for server-side error messages.
  2. **Built-in match**: Renders only when the field's `ValidityState` has the corresponding flag set to `true`, or when `forceMatch` is `true`.
  3. **Custom match**: Registers the matcher function with the `ValidationContext`. Renders only when the matcher returns `true` (and no built-in errors exist), or when `forceMatch` is `true`.
- Each rendered message's `id` is registered with the `AriaDescriptionContext` so `FormControl` can include it in `aria-describedby`. The ID is unregistered on unmount.
- Default messages per built-in matcher:
  - `badInput` / `BadInput`: "This value is not valid"
  - `patternMismatch` / `PatternMismatch`: "This value does not match the required pattern"
  - `rangeOverflow` / `RangeOverflow`: "This value is too large"
  - `rangeUnderflow` / `RangeUnderflow`: "This value is too small"
  - `stepMismatch` / `StepMismatch`: "This value does not match the required step"
  - `tooLong` / `TooLong`: "This value is too long"
  - `tooShort` / `TooShort`: "This value is too short"
  - `typeMismatch` / `TypeMismatch`: "This value does not match the required type"
  - `valid` / `Valid`: *(empty string)*
  - `valueMissing` / `ValueMissing`: "This value is missing"
