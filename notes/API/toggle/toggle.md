# Toggle

## Anatomy

Toggle is a single-part component -- there is no nesting structure. It renders a `<button>` element with pressed state.

```
Toggle
```

### React

```tsx
<Toggle.Root>Bold</Toggle.Root>
```

### Leptos

```rust
<Toggle>"Bold"</Toggle>
```

## React Signature

```typescript
type ToggleElement = React.ComponentRef<typeof Primitive.button>;
type PrimitiveButtonProps = React.ComponentPropsWithoutRef<typeof Primitive.button>;

interface ToggleProps extends PrimitiveButtonProps {
  /**
   * The controlled state of the toggle.
   */
  pressed?: boolean;
  /**
   * The state of the toggle when initially rendered. Use `defaultPressed`
   * if you do not need to control the state of the toggle.
   * @defaultValue false
   */
  defaultPressed?: boolean;
  /**
   * The callback that fires when the state of the toggle changes.
   */
  onPressedChange?(pressed: boolean): void;
}

const Toggle = React.forwardRef<ToggleElement, ToggleProps>((props, forwardedRef) => ...)
```

## Leptos Signature

```rust
pub fn Toggle(
    /// The controlled state of the toggle.
    #[prop(into, optional)]
    pressed: MaybeProp<bool>,
    /// The state of the toggle when initially rendered. Use `default_pressed`
    /// if you do not need to control the state of the toggle. Defaults to `false`.
    #[prop(into, optional)]
    default_pressed: MaybeProp<bool>,
    /// The callback that fires when the state of the toggle changes.
    #[prop(into, optional)]
    on_pressed_change: Option<Callback<bool>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop       | Leptos Prop        | Type (React)                  | Type (Leptos)              | Description                                                                                                                    |
| ---------------- | ------------------ | ----------------------------- | -------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| `pressed`        | `pressed`          | `boolean \| undefined`        | `MaybeProp<bool>`          | The controlled pressed state of the toggle. When set, the component becomes controlled and `on_pressed_change` must be used to update state. |
| `defaultPressed` | `default_pressed`  | `boolean` (default `false`)   | `MaybeProp<bool>` (default `false`) | The initial pressed state when uncontrolled. Use when you do not need to control the toggle state externally.                  |
| `onPressedChange`| `on_pressed_change`| `(pressed: boolean) => void`  | `Option<Callback<bool>>`   | Callback fired when the pressed state changes. Receives the new boolean value.                                                 |
| `disabled`       | `disabled`         | `boolean`                     | `MaybeProp<bool>`          | When `true`, the toggle does not respond to clicks and renders with `data-disabled` and `disabled` attributes. In React, this comes through `PrimitiveButtonProps` spread; in Leptos, it is an explicit prop. |
| `onClick`        | `on_click`         | `(event: MouseEvent) => void` | `Option<Callback<ev::MouseEvent>>` | Click handler composed with the internal toggle handler. The user's handler runs first; the toggle state change follows. In React, this comes through `PrimitiveButtonProps` spread; in Leptos, it is an explicit prop. |
| `ref`            | `node_ref`         | `React.Ref`                   | `AnyNodeRef`               | Ref to the root DOM element (`<button>`).                                                                                      |
| `asChild`        | `as_child`         | `boolean`                     | `MaybeProp<bool>`          | When `true`, the component renders its child directly instead of wrapping in a `<button>`, merging props and refs onto the child. |
| *(spread)*       | --                 | `...PrimitiveButtonProps`     | --                         | React allows spreading any `<button>` HTML attribute. Leptos uses `attr:` directives on the call site instead.                 |

### Data attributes (rendered on DOM)

| Attribute       | Value                                   | Description                                                              |
| --------------- | --------------------------------------- | ------------------------------------------------------------------------ |
| `data-state`    | `"on" \| "off"`                         | Reflects the pressed state: `"on"` when pressed, `"off"` when not.       |
| `data-disabled` | `""` (present) or absent                | Present when the toggle is disabled.                                     |

### Implicit behavior

- The component renders `type="button"` on the underlying `<button>` element to prevent form submission.
- `aria-pressed` is always rendered, reflecting the current pressed state as `"true"` or `"false"`.
- The `disabled` HTML attribute is set on the button when the `disabled` prop is true, preventing native focus and interaction.
- Click handling is composed: the user-supplied `on_click` fires first, then the internal handler toggles state (unless disabled).

## Usage Examples

### Uncontrolled

#### React

```tsx
<Toggle.Root defaultPressed>
  Bold
</Toggle.Root>
```

#### Leptos

```rust
<Toggle default_pressed=true>
  "Bold"
</Toggle>
```

### Controlled

#### React

```tsx
const [pressed, setPressed] = React.useState(true);

<Toggle.Root pressed={pressed} onPressedChange={setPressed}>
  {pressed ? 'On' : 'Off'}
</Toggle.Root>
```

#### Leptos

```rust
let (pressed, set_pressed) = signal(true);

<Toggle
  pressed=pressed
  on_pressed_change=move |value: bool| set_pressed.set(value)
>
  {move || match pressed.get() {
      true => "On",
      false => "Off",
  }}
</Toggle>
```

### Disabled

#### React

```tsx
<Toggle.Root disabled>
  Toggle
</Toggle.Root>
```

#### Leptos

```rust
<Toggle disabled=true>
  "Toggle"
</Toggle>
```

## Accessibility

Toggle implements a two-state toggle button. There is no single WAI-ARIA "toggle" pattern, but the component follows the [WAI-ARIA Button pattern](https://www.w3.org/WAI/ARIA/apd/patterns/button/) with the addition of `aria-pressed` to indicate toggle state, as recommended by the [ARIA Authoring Practices: Toggle Button](https://www.w3.org/WAI/ARIA/apg/patterns/button/).

### Keyboard Interactions

| Key               | Description                                                  |
| ----------------- | ------------------------------------------------------------ |
| `Enter` / `Space` | Toggles the pressed state (native `<button>` click behavior). |

### ARIA Attributes

| Element  | Attribute      | Value              | Notes                                                                                                 |
| -------- | -------------- | ------------------ | ----------------------------------------------------------------------------------------------------- |
| `Toggle` | `aria-pressed` | `"true" / "false"` | Always rendered. Reflects the current pressed state. Screen readers announce this as "pressed" or "not pressed". |

### Behavioral Notes

- When `disabled` is `true`, clicking the toggle does not change its state. The `disabled` HTML attribute is set on the `<button>`, so it is excluded from tab order and cannot be activated via keyboard.
- `data-disabled` is set as an empty string (present) when disabled, or absent when not. This is separate from the native `disabled` attribute and allows CSS styling without relying on `:disabled`.
- `data-state` toggles between `"on"` and `"off"` and can be used for CSS styling (e.g., different background colors for each state).
- There is no `orientation` or directional keyboard navigation -- Toggle is a single button, not a group.
