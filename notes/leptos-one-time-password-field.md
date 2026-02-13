---
react_location: "[[reference/react-radix-primitives/packages/react/one-time-password-field/src/one-time-password-field.tsx|one-time-password-field]]"
rust_location: "[[packages/primitives/leptos/one-time-password-field/src/one_time_password_field.rs|one-time-password-field]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/one-time-password-field.stories.tsx|one-time-password-field]]"
rust_story: "[[stories/leptos/src/primitives/one_time_password_field.rs|one-time-password-field]]"
dependencies:
  - "[[core-number]]"
  - "[[core-primitive]]"
  - "[[leptos-collection]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-direction]]"
  - "[[leptos-primitive]]"
  - "[[leptos-roving-focus]]"
  - "[[leptos-use-controllable-state]]"
ported: true
tested: false
tested_story: false
unstable: true
---
## Intent

A multi-input OTP (one-time password) field component that handles character-by-character input, paste distribution, auto-submit, and validation. Marked as `unstable` in React.

## React API

```ts
// 3 sub-components:
OneTimePasswordField, OneTimePasswordFieldInput, OneTimePasswordFieldHiddenInput
```

Props: `value`, `defaultValue`, `onValueChange`, `onAutoSubmit`, `autoSubmit` (default false), `validationType` (`'numeric'` | `'alpha'` | `'alphanumeric'` | `'none'`), `type` (`'password'` | `'text'`), `autoComplete`, `disabled`, `readOnly`, `placeholder`, `orientation`, `dir`, `name`, `form`.

## Leptos API

```rust
// Root component — wraps inputs in CollectionProvider + RovingFocusGroup
#[component]
pub fn OneTimePasswordField(
    value: MaybeProp<String>,
    default_value: MaybeProp<String>,
    on_value_change: Option<Callback<String>>,
    auto_submit: MaybeProp<bool>,
    on_auto_submit: Option<Callback<String>>,
    disabled: MaybeProp<bool>,
    read_only: MaybeProp<bool>,
    auto_complete: MaybeProp<AutoComplete>,
    auto_focus: MaybeProp<bool>,
    form: MaybeProp<String>,
    name: MaybeProp<String>,
    placeholder: MaybeProp<String>,
    r#type: MaybeProp<InputType>,
    orientation: MaybeProp<Orientation>,
    dir: MaybeProp<Direction>,
    validation_type: MaybeProp<InputValidationType>,
    sanitize_value: Option<Callback<String, String>>,
    on_paste: Option<Callback<ev::ClipboardEvent>>,
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView

// Individual input — wrapped in CollectionItemSlot + RovingFocusGroupItem
#[component]
pub fn OneTimePasswordFieldInput(
    on_invalid_change: Option<Callback<String>>,
    index: MaybeProp<usize>,
    on_focus: Option<Callback<ev::FocusEvent>>,
    on_cut: Option<Callback<ev::ClipboardEvent>>,
    on_input: Option<Callback<ev::Event>>,
    on_change: Option<Callback<ev::Event>>,
    on_key_down: Option<Callback<ev::KeyboardEvent>>,
    on_pointer_down: Option<Callback<ev::PointerEvent>>,
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    children: Option<Children>,
) -> impl IntoView

// Hidden input for form submission
#[component]
pub fn OneTimePasswordFieldHiddenInput(
    name: MaybeProp<String>,
    node_ref: AnyNodeRef,
) -> impl IntoView

// Enums
pub enum InputValidationType { Alpha, Numeric (default), Alphanumeric, None }
pub enum InputType { Password, Text (default) }
pub enum AutoComplete { Off, OneTimeCode (default) }
```

## React Implementation Notes

- ~965 lines.
- `Collection` + `RovingFocusGroup` for input management and keyboard navigation.
- Action system: `SET_CHAR`, `CLEAR_CHAR`, `PASTE` actions drive input state changes.
- Validation map with regex patterns and input modes per validation type.
- Paste event handling: Distributes pasted characters across inputs.
- Password manager integration detection via `RovingFocusGroup.Item` render callback (`hasTabStop`, `isCurrentTabStop`).
- Custom sanitization function support.
- Form submit on Enter key.
- Hidden input for form submission with concatenated value.
- Auto-focus movement: Moves to next input on character entry, previous on backspace.
- `CLEAR` action with reasons: `Reset`, `Backspace`, `Delete`.
- Form reset event listener to clear value when associated form resets.
- Re-validate effect when `validationType` changes.

## Leptos Implementation Notes

### Roving Focus Context Exposure

The React component uses a render callback from `RovingFocusGroup.Item` to receive `hasTabStop` and `isCurrentTabStop` — used to determine which input gets `autocomplete="one-time-code"` vs password manager ignore attributes. To support this in Leptos, two public context types were added to `radix-leptos-roving-focus`:
- `RovingFocusGroupContext { has_tab_stop: Signal<bool> }` — provided by `RovingFocusGroupImpl`
- `RovingFocusGroupItemContext { is_current_tab_stop: Signal<bool> }` — provided by `RovingFocusGroupItem`

The OTP input component reads these contexts with `use_context::<...>()`.

### Component Split for Context Reading

`OneTimePasswordFieldInput` delegates rendering to an inner `OneTimePasswordFieldInputInner` component. This split is necessary because `RovingFocusGroupItemContext` is provided by `RovingFocusGroupItem`, which wraps the inner component — the inner component can then read the context that the outer component set up.

### Omissions

- **`useEffectEvent`**: Not needed. Leptos closures capture signals by value; no stale-closure problem.
- **`flushSync`**: Not needed. Signal updates are synchronous in event handlers.
- **`useIsHydrated`**: Not needed for CSR-only target. Removed from dependency list.
- **Scoped context (`__scopeOneTimePasswordField`)**: Leptos uses `provide_context`/`expect_context` instead.
- **`PastedAndDeletedControlled` / `PastedAndDeletedUncontrolled` stories**: Skipped. These use Storybook's `play` testing API which has no Leptos equivalent.

### Key Decisions

- **Value representation**: Internal state is `Vec<String>` (each element = one char). Public API uses `String` (joined). `use_controllable_state` wraps `Option<Vec<String>>`.
- **Collection helpers**: Local helper functions (`collection_at`, `collection_index_of`, `collection_from`, `collection_element`) replicate React's collection API methods.
- **Dispatch pattern**: All value mutations go through a `dispatch` closure stored in `StoredValue<SendWrapper<Box<dyn Fn(UpdateAction)>>>`, matching React's action-based approach.
- **JS interop**: Uses `js_sys::RegExp` for validation (matching React behavior exactly), `Closure::once_into_js` for fire-and-forget timeouts/rAF, and `SendWrapper` for non-Send DOM types in signals.
- **`OneTimePasswordFieldHiddenInput` props**: Simplified to just `name` and `node_ref` (no `#[prop(attrs)]` spread) since hidden inputs don't need arbitrary attribute passthrough.
- **`OneTimePasswordFieldInput` children**: Uses `Option<Children>` (not `TypedChildrenFn`) so callers can use `<OneTimePasswordFieldInput />` without providing children.
