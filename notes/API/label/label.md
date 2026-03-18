# Label

## Anatomy

Label is a single-part component with no nested sub-parts. It renders as a `<label>` element.

```
Label
```

### React

```tsx
<Label.Root htmlFor="input-id">Username</Label.Root>
```

### Leptos

```rust
<Label attr:r#for="input-id">"Username"</Label>
```

## React Signature

```typescript
const Label = React.forwardRef<LabelElement, LabelProps>(...)

type LabelElement = React.ComponentRef<typeof Primitive.label>;
type PrimitiveLabelProps = React.ComponentPropsWithoutRef<typeof Primitive.label>;
interface LabelProps extends PrimitiveLabelProps {}
```

`LabelProps` extends all native `<label>` HTML attributes (including `htmlFor`, `className`, event handlers, etc.) via `PrimitiveLabelProps`. No additional props are defined beyond what `Primitive.label` provides.

## Leptos Signature

```rust
pub fn Label(
    #[prop(into, optional)] on_mouse_down: MaybeCallback<MouseEvent>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `htmlFor` | *(use `attr:r#for`)* | `string` | — | Associates the label with a form control by ID. In React this is a native `<label>` prop passed via spread. In Leptos, use `attr:r#for="control-id"` on the component call site. |
| `onMouseDown` | `on_mouse_down` | `(event: MouseEvent) => void` | `MaybeCallback<MouseEvent>` | Mouse-down event handler. In React this is an HTML event prop inherited from `PrimitiveLabelProps`. In Leptos it is an explicit prop. The component intercepts this internally: if the click target is not inside a `button`, `input`, `select`, or `textarea`, and the event has `detail > 1` (double-click), `preventDefault()` is called to suppress text selection. The user's handler is called before the prevention check. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the root DOM element (`<label>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, the component renders its child directly instead of wrapping in a `<label>`, merging props and refs onto the child. |
| *(spread)* | — | `...PrimitiveLabelProps` | — | React allows spreading any native `<label>` HTML attribute. Leptos uses `attr:` directives on the call site instead (e.g., `attr:class`, `attr:r#for`). |

### Data attributes (rendered on DOM)

Label does not render any custom `data-*` attributes.

### Implicit behavior

- **Double-click text selection prevention:** On `mousedown`, if the click target is not inside a form control (`button`, `input`, `select`, `textarea`) and `event.detail > 1` (indicating a double-click), the component calls `event.preventDefault()` to prevent text selection. This matches native browser `<label>` UX where double-clicking a label should activate the control rather than select the label text.
- **User handler invocation order:** The user-supplied `onMouseDown` / `on_mouse_down` handler is called *before* the double-click prevention check, giving the user a chance to call `preventDefault()` themselves to override the behavior.

## Usage Examples

### Basic label with `for` attribute

#### React

```tsx
<Label.Root htmlFor="email">Email address</Label.Root>
<input id="email" type="email" />
```

#### Leptos

```rust
<Label attr:r#for="email">"Email address"</Label>
<input id="email" type="email" />
```

### Wrapping a control

#### React

```tsx
<Label.Root>
  <input type="checkbox" /> Accept terms
</Label.Root>
```

#### Leptos

```rust
<Label>
    <input type="checkbox" /> " Accept terms"
</Label>
```

### With a button control inside

#### React

```tsx
<Label.Root>
  <button onClick={() => alert('clicked')}>Control</button>
  {' '}Label text
</Label.Root>
```

#### Leptos

```rust
<Label>
    <button on:click=move |_| window().alert_with_message("clicked").unwrap()>
        "Control"
    </button>
    " Label text"
</Label>
```

## Accessibility

Label implements standard HTML `<label>` semantics. It does not follow a specific WAI-ARIA widget pattern; instead it relies on the native HTML label-control association.

### Keyboard Interactions

Label does not define any keyboard interactions. Native browser behavior applies: clicking or tapping a `<label>` focuses and/or activates its associated form control.

### ARIA Attributes

Label does not set any ARIA attributes. The association between label and control is achieved through the native `<label>` element's `for` attribute (or by nesting the control inside the label).

### Behavioral Notes

- **Double-click prevention:** When a user double-clicks on the label text itself (not on a nested `button`, `input`, `select`, or `textarea`), the component prevents text selection via `preventDefault()`. This preserves the expected UX where double-clicking a label activates or focuses the associated control rather than selecting text.
- **Form control passthrough:** Clicks on nested form controls (`button`, `input`, `select`, `textarea`) are not intercepted -- they behave normally without the text selection prevention logic.
- **Native `<label>` element:** The component renders as a native `<label>` element, so all standard label behaviors (click-to-focus, screen reader association) work out of the box.

## CSS Custom Properties

Label does not expose any CSS custom properties.
