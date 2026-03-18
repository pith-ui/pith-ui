# AccessibleIcon

A utility component that makes icons accessible by hiding the icon from screen readers and providing a visually hidden label instead.

## Anatomy

AccessibleIcon is a single-part utility. There is no nesting structure -- it wraps a single icon element.

```
AccessibleIcon
└── <icon element> (e.g., <svg>)
```

### React

```tsx
<AccessibleIcon.Root label="Close">
  <CrossIcon />
</AccessibleIcon.Root>
```

### Leptos

```rust
<AccessibleIcon label="Close">
    <CrossIcon />
</AccessibleIcon>
```

## React Signature

```typescript
interface AccessibleIconProps {
  children?: React.ReactNode;
  /**
   * The accessible label for the icon. This label will be visually hidden but announced to screen
   * reader users, similar to `alt` text for `img` tags.
   */
  label: string;
}

const AccessibleIcon: React.FC<AccessibleIconProps> = ({ children, label }) => { ... }
```

Note: This component is an `React.FC`, not a `forwardRef` -- it does not accept a `ref` prop.

## Leptos Signature

```rust
#[component]
pub fn AccessibleIcon(
    /// The accessible label for the icon. This label will be visually hidden but announced to
    /// screen reader users, similar to `alt` text for `img` tags.
    #[prop(into)]
    label: Signal<String>,
    children: TypedChildren<impl IntoView + 'static>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `label` | `label` | `string` (required) | `Signal<String>` (required, `#[prop(into)]`) | The accessible label for the icon. Visually hidden but announced to screen readers, similar to `alt` text for `<img>` tags. In Leptos, accepts `&str`, `String`, `Signal<String>`, etc. via `into`. |
| `children` | `children` | `React.ReactNode` | `TypedChildren<impl IntoView + 'static>` | The icon element to wrap. React enforces exactly one child via `React.Children.only()` (throws if zero or multiple children). Leptos does not enforce this constraint at the type level -- see note below. |
| *(no ref)* | *(no node_ref)* | -- | -- | This component does not accept a ref/node_ref in either framework. |
| *(no asChild)* | *(no as_child)* | -- | -- | This component does not support `asChild`/`as_child` in either framework. |

### Data attributes (rendered on DOM)

AccessibleIcon does not render its own DOM element and therefore sets no `data-*` attributes. It renders a fragment containing the child (with added attributes) and a `VisuallyHidden` element.

### Implicit behavior

- **`aria-hidden="true"`** is added to the child icon element, hiding it from screen readers.
- **`focusable="false"`** is added to the child icon element, preventing SVGs from being focusable in IE/Edge. See [allyjs SVG focus guide](https://allyjs.io/tutorials/focusing-in-svg.html#making-svg-elements-focusable).
- A **`VisuallyHidden`** element is rendered as a sibling to the child, containing the `label` text. This text is announced by screen readers but not visible on screen.
- **React enforces exactly one child** via `React.Children.only()`. Leptos uses `TypedChildren` which allows multiple children -- all children receive the `aria-hidden` and `focusable` attributes. This is a framework-level difference (no equivalent to `React.Children.only` in Leptos) and is tracked as a known difference.

## Usage Examples

### Basic usage (inside a button)

#### React

```tsx
<button type="button">
  <AccessibleIcon.Root label="Close">
    <CrossIcon />
  </AccessibleIcon.Root>
</button>
```

#### Leptos

```rust
<button r#type="button">
    <AccessibleIcon label="Close">
        <CrossIcon />
    </AccessibleIcon>
</button>
```

### Inline with text

#### React

```tsx
<p>
  Some text with an inline accessible icon{' '}
  <AccessibleIcon.Root label="Close">
    <CrossIcon />
  </AccessibleIcon.Root>
</p>
```

#### Leptos

```rust
<p>
    "Some text with an inline accessible icon "
    <AccessibleIcon label="Close">
        <CrossIcon />
    </AccessibleIcon>
</p>
```

### Reactive label (Leptos only)

The Leptos `label` prop accepts a `Signal<String>`, enabling dynamic labels:

```rust
let (label, set_label) = signal("Close".to_string());

<AccessibleIcon label=label>
    <CrossIcon />
</AccessibleIcon>
```

## Accessibility

AccessibleIcon implements a common accessibility pattern for decorative icons: the icon itself is hidden from the accessibility tree, and a visually hidden text label provides the accessible name.

### How it works

1. The icon child receives `aria-hidden="true"`, removing it from the accessibility tree entirely.
2. The icon child receives `focusable="false"`, preventing SVG elements from receiving focus in IE/Edge.
3. A `VisuallyHidden` sibling renders the `label` text -- invisible on screen but announced by screen readers.

This pattern is equivalent to using an `<img alt="Close">` tag: the visual element is decorative, and the text alternative conveys meaning.

### Behavioral Notes

- The component itself does not render a wrapping DOM element -- it outputs a fragment. This means it does not interfere with layout or styling of the parent.
- When used inside interactive elements (buttons, links), the `label` text becomes part of the interactive element's accessible name.
- No keyboard interactions are added by this component. Keyboard behavior is determined by the parent element (e.g., `<button>`).
