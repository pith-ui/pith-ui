# {ComponentPart}

<!--
  This template is for a SINGLE component part (e.g. AccordionItem, DialogTrigger).
  Create one file per part at: notes/API/<component>/<component-part>.md

  The ROOT part file (e.g. accordion.md for AccordionRoot / Accordion) also includes
  the component-level sections marked [ROOT ONLY] below. Non-root parts omit those.
-->

## Anatomy

<!-- [ROOT ONLY] Show the expected nesting structure for the whole component. -->

The expected component nesting structure:

```
{RootComponent}
└── ...
```

### React

```tsx
<!-- Minimal skeleton showing component composition -->
```

### Leptos

```rust
<!-- Minimal skeleton showing component composition -->
```

## React Signature

```typescript
<!-- Full forwardRef signature and prop interface(s). Include inherited/extended types. -->
```

## Leptos Signature

```rust
<!-- Full #[component] function signature with all props and their attributes. -->
```

## Prop Mapping

<!--
  One table per logical prop group. For most parts there is one table.
  Root components may split into multiple tables (e.g. single-mode, multiple-mode, shared).
-->

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| | | | | |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| | | |

### Implicit behavior

<!--
  Only include if the part has behavior not visible through props:
  - Auto-generated IDs / ARIA wiring
  - Inherited context values
  - Conditional ARIA attributes
  Omit this section if there is nothing to document.
-->

## Usage Examples

<!--
  [ROOT ONLY] Side-by-side React/Leptos snippets for key usage patterns.
  Cover at minimum: basic uncontrolled, controlled, and any component-specific
  patterns (e.g. animated content, disabled items).

  Keep examples minimal — just enough to show the API shape, not full apps.
  Leptos examples must use correct prop syntax:
    - Props with #[prop(into)] accept &str directly
    - Props without #[prop(into)] (bare String) need .to_string()
    - Vec<String> values need .into() per element
-->

### {Pattern name}

#### React

```tsx
```

#### Leptos

```rust
```

## Accessibility

<!--
  [ROOT ONLY] Document the WAI-ARIA pattern implemented and link to the spec.
-->

### Keyboard Interactions

| Key | Description |
|---|---|
| | |

### ARIA Attributes

| Element | Attribute | Value | Notes |
|---|---|---|---|
| | | | |

### Behavioral Notes

<!-- Gotchas, edge cases, and non-obvious interactions. -->

## CSS Custom Properties

<!--
  [ROOT ONLY] Only include if the component exposes CSS custom properties.
  Document the property name, its source, and what it represents.
-->

| Property | Source | Description |
|---|---|---|
| | | |
