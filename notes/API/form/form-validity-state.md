# FormValidityState

## React Signature

```typescript
interface FormValidityStateProps {
  children(validity: ValidityState | undefined): React.ReactNode;
  name?: string;
}

const FormValidityState = (props: ScopedProps<FormValidityStateProps>) => {
  // ...
  return <>{children(validity)}</>;
};
```

Note: `FormValidityState` is **not** a `forwardRef` component -- it is a render-prop component that does not render a DOM element of its own.

## Leptos Signature

```rust
pub fn FormValidityState(
    #[prop(into, optional)] name: Option<String>,
    children: Callback<Option<Validity>, AnyView>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `name` | `name` | `string \| undefined` | `Option<String>` | The field name to look up validity for. Defaults to the parent `FormField`'s `name`. |
| `children` | `children` | `(validity: ValidityState \| undefined) => React.ReactNode` | `Callback<Option<Validity>, AnyView>` | A render function that receives the field's current validity state. In React, this is a render-prop pattern via `children(validity)`. In Leptos, this is a `Callback` that takes `Option<Validity>` and returns an `AnyView`. |

### Leptos-only: `Validity` struct

The Leptos port defines its own `Validity` struct mirroring the browser's `ValidityState` interface:

```rust
pub struct Validity {
    pub bad_input: bool,
    pub custom_error: bool,
    pub pattern_mismatch: bool,
    pub range_overflow: bool,
    pub range_underflow: bool,
    pub step_mismatch: bool,
    pub too_long: bool,
    pub too_short: bool,
    pub type_mismatch: bool,
    pub valid: bool,
    pub value_missing: bool,
}
```

### Implicit behavior

- Does not render any DOM element of its own. Acts purely as a render-prop (React) or callback-based (Leptos) passthrough.
- Reads the `ValidationContext` to get the current validity for the specified field.
- Re-renders whenever the field's validity changes.
- Returns `undefined` (React) / `None` (Leptos) if the field has not yet been validated.
