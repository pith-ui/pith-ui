# Separator

## Anatomy

Separator is a single-part component — there are no nested children parts.

```
Separator
```

### React

```tsx
<Separator.Root />
```

### Leptos

```rust
<Separator />
```

## React Signature

```typescript
const Separator = React.forwardRef<SeparatorElement, SeparatorProps>(...)

type SeparatorElement = React.ComponentRef<typeof Primitive.div>;
type PrimitiveDivProps = React.ComponentPropsWithoutRef<typeof Primitive.div>;

interface SeparatorProps extends PrimitiveDivProps {
  /**
   * Either `vertical` or `horizontal`. Defaults to `horizontal`.
   */
  orientation?: Orientation;
  /**
   * Whether or not the component is purely decorative. When true, accessibility-related attributes
   * are updated so that that the rendered element is removed from the accessibility tree.
   */
  decorative?: boolean;
}

type Orientation = 'horizontal' | 'vertical';
```

## Leptos Signature

```rust
pub fn Separator(
    #[prop(into, optional)] orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] decorative: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

### Leptos-only: `Orientation` enum

```rust
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `orientation` | `orientation` | `'horizontal' \| 'vertical'` (default `'horizontal'`) | `MaybeProp<Orientation>` (default `Horizontal`) | The axis the separator aligns along. A horizontal separator spans the full width and creates a visual break between vertically stacked content. A vertical separator spans a fixed height and separates horizontally arranged content. |
| `decorative` | `decorative` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | When `true`, the separator is purely visual. The component renders with `role="none"` instead of `role="separator"`, removing it from the accessibility tree. Use this when the separator has no semantic meaning (e.g., a visual flourish between sections that are not structurally distinct). |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the root DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, the component renders its child directly instead of wrapping in a `<div>`, merging props and refs onto the child. |
| *(spread)* | — | `...PrimitiveDivProps` | — | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives on the call site instead. |
| — | `children` | *(implicit via JSX)* | `Option<ChildrenFn>` | Optional children. Separators are typically self-closing with no children, but children are accepted if needed (e.g., for `as_child` usage). |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-orientation` | `"horizontal" \| "vertical"` | Reflects the `orientation` prop. Always present. |

### Implicit behavior

- **Role selection:** When `decorative` is `false` (the default), the element renders with `role="separator"`. When `decorative` is `true`, it renders with `role="none"`, removing it from the accessibility tree entirely.
- **Conditional `aria-orientation`:** When the separator is semantic (`decorative=false`), `aria-orientation` is set to `"vertical"` only when `orientation` is `Vertical`. When `orientation` is `Horizontal`, `aria-orientation` is omitted because the WAI-ARIA spec defaults `aria-orientation` to `horizontal` for the `separator` role. When decorative, `aria-orientation` is omitted entirely.
- **Orientation validation (React only):** The React source validates the `orientation` string at runtime via `isValidOrientation()` and falls back to `'horizontal'` for invalid values. In Leptos, this is handled at compile time by the `Orientation` enum — invalid values are impossible.

## Usage Examples

### Horizontal separator (semantic)

#### React

```tsx
<Separator.Root orientation="horizontal" />
```

#### Leptos

```rust
<Separator orientation=Orientation::Horizontal />
```

### Vertical separator (semantic)

#### React

```tsx
<div style={{ display: 'flex', alignItems: 'center' }}>
  <span>Left content</span>
  <Separator.Root orientation="vertical" />
  <span>Right content</span>
</div>
```

#### Leptos

```rust
<div style:display="flex" style:align-items="center">
    <span>"Left content"</span>
    <Separator orientation=Orientation::Vertical />
    <span>"Right content"</span>
</div>
```

### Decorative separator

#### React

```tsx
<Separator.Root decorative />
```

#### Leptos

```rust
<Separator decorative=true />
```

### Styled separator (with CSS module class)

#### React

```tsx
<Separator.Root className={styles.root} orientation="horizontal" />
```

#### Leptos

```rust
<Separator attr:class=classes::root orientation=Orientation::Horizontal />
```

## Accessibility

Implements the [WAI-ARIA Separator role](https://www.w3.org/TR/wai-aria-1.2/#separator) for non-decorative usage.

### Keyboard Interactions

Separator has no keyboard interactions. It is a static structural element, not an interactive widget. (Note: the WAI-ARIA spec defines keyboard interactions for *focusable* separators used as splitter controls, but Radix's Separator does not implement that pattern.)

### ARIA Attributes

| Element | Attribute | Value | Notes |
|---|---|---|---|
| `Separator` | `role` | `"separator"` | Set when `decorative` is `false` (default). Indicates a structural boundary between sections. |
| `Separator` | `role` | `"none"` | Set when `decorative` is `true`. Removes the element from the accessibility tree. |
| `Separator` | `aria-orientation` | `"vertical"` \| absent | Only rendered when `decorative` is `false` and `orientation` is `vertical`. Omitted for horizontal orientation because the `separator` role defaults to horizontal per the ARIA spec. Omitted entirely when decorative. |

### Behavioral Notes

- **Decorative vs. semantic:** The `decorative` prop is the key accessibility decision. Use `decorative=true` for visual-only dividers (e.g., a colored line between paragraphs that does not represent a structural boundary). Use the default (`decorative=false`) when the separator marks a meaningful boundary between content regions — screen readers will announce it as a separator.
- **No focusable separator support:** Radix's Separator is always non-focusable. The WAI-ARIA spec allows separators to be focusable (for splitter/resize controls), but this component does not implement that pattern.
- **Self-closing usage:** Separators are typically rendered as self-closing elements with no children. The `children` prop exists for `as_child` composition but is not used in standard scenarios.

## CSS Custom Properties

Separator does not expose any CSS custom properties.
