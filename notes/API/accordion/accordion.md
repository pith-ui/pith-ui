# Accordion (Root)

## Anatomy

The expected component nesting structure:

```
Accordion (or AccordionSingle / AccordionMultiple)
└── AccordionItem (one per section, each with a unique `value`)
    ├── AccordionHeader
    │   └── AccordionTrigger
    └── AccordionContent
```

### React

```tsx
<Accordion.Root type="single">
  <Accordion.Item value="item-1">
    <Accordion.Header>
      <Accordion.Trigger>...</Accordion.Trigger>
    </Accordion.Header>
    <Accordion.Content>...</Accordion.Content>
  </Accordion.Item>
</Accordion.Root>
```

### Leptos

```rust
<AccordionSingle>
  <AccordionItem value="item-1">
    <AccordionHeader>
      <AccordionTrigger>"..."</AccordionTrigger>
    </AccordionHeader>
    <AccordionContent>"..."</AccordionContent>
  </AccordionItem>
</AccordionSingle>
```

## React Signature

React exposes a single `Accordion` component that dispatches on a `type` discriminator:

```typescript
const Accordion = React.forwardRef<
  AccordionElement,
  AccordionSingleProps | AccordionMultipleProps
>(...)

// When type === 'single':
interface AccordionSingleProps extends AccordionImplSingleProps {
  type: 'single';
}
interface AccordionImplSingleProps extends AccordionImplProps {
  value?: string;
  defaultValue?: string;
  onValueChange?(value: string): void;
  collapsible?: boolean;
}

// When type === 'multiple':
interface AccordionMultipleProps extends AccordionImplMultipleProps {
  type: 'multiple';
}
interface AccordionImplMultipleProps extends AccordionImplProps {
  value?: string[];
  defaultValue?: string[];
  onValueChange?(value: string[]): void;
}

// Shared base:
interface AccordionImplProps extends PrimitiveDivProps {
  disabled?: boolean;
  orientation?: React.AriaAttributes['aria-orientation'];
  dir?: Direction; // 'ltr' | 'rtl'
}
```

## Leptos Signatures

Leptos splits this into three components: `AccordionSingle`, `AccordionMultiple`, and a convenience `Accordion` wrapper.

### AccordionSingle

```rust
pub fn AccordionSingle(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] collapsible: MaybeProp<bool>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

### AccordionMultiple

```rust
pub fn AccordionMultiple(
    #[prop(into, optional)] values: MaybeProp<Vec<String>>,
    #[prop(into, optional)] default_values: MaybeProp<Vec<String>>,
    #[prop(into, optional)] on_values_change: Option<Callback<Vec<String>>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

### Accordion (convenience wrapper)

```rust
pub fn Accordion(
    r#type: AccordionType,
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] collapsible: MaybeProp<bool>,
    #[prop(into, optional)] values: MaybeProp<Vec<String>>,
    #[prop(into, optional)] default_values: MaybeProp<Vec<String>>,
    #[prop(into, optional)] on_values_change: Option<Callback<Vec<String>>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

### Single-mode props

| React Prop      | Leptos Prop       | Type (React)                | Type (Leptos)                       | Description                                                                                                                               |
| --------------- | ----------------- | --------------------------- | ----------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| `type`          | `r#type`          | `'single'`                  | `AccordionType::Single`             | Discriminator selecting single-item mode. In Leptos, only present on the `Accordion` wrapper; `AccordionSingle` is used directly instead. |
| `value`         | `value`           | `string \| undefined`       | `MaybeProp<String>`                 | The controlled value of the currently expanded item. When set, the component becomes controlled.                                          |
| `defaultValue`  | `default_value`   | `string \| undefined`       | `MaybeProp<String>`                 | The value of the item expanded on initial render. Use when you do not need to control accordion state.                                    |
| `onValueChange` | `on_value_change` | `(value: string) => void`   | `Option<Callback<String>>`          | Callback fired when the expanded item changes. Receives the new value string.                                                             |
| `collapsible`   | `collapsible`     | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | Whether the open item can be collapsed by clicking its trigger again. When `false`, at least one item stays open once opened.             |

### Multiple-mode props

| React Prop      | Leptos Prop        | Type (React)                | Type (Leptos)                   | Description                                                                                                                                                         |
| --------------- | ------------------ | --------------------------- | ------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`          | `r#type`           | `'multiple'`                | `AccordionType::Multiple`       | Discriminator selecting multi-item mode. In Leptos, only present on the `Accordion` wrapper; `AccordionMultiple` is used directly instead.                          |
| `value`         | `values`           | `string[] \| undefined`     | `MaybeProp<Vec<String>>`        | The controlled list of expanded item values. When set, the component becomes controlled. Note: Leptos renames to `values` (plural) to distinguish from single-mode. |
| `defaultValue`  | `default_values`   | `string[] \| undefined`     | `MaybeProp<Vec<String>>`        | The values of the items expanded on initial render. Note: Leptos renames to `default_values` (plural).                                                              |
| `onValueChange` | `on_values_change` | `(value: string[]) => void` | `Option<Callback<Vec<String>>>` | Callback fired when the set of expanded items changes. Receives the full updated list. Note: Leptos renames to `on_values_change` (plural).                         |

### Shared props (both modes)

| React Prop    | Leptos Prop   | Type (React)                                        | Type (Leptos)                                 | Description                                                                                                                                                                      |
| ------------- | ------------- | --------------------------------------------------- | --------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `disabled`    | `disabled`    | `boolean` (default `false`)                         | `MaybeProp<bool>` (default `false`)           | Disables the entire accordion. When `true`, all items ignore user interaction and keyboard navigation is suppressed.                                                             |
| `orientation` | `orientation` | `'horizontal' \| 'vertical'` (default `'vertical'`) | `MaybeProp<Orientation>` (default `Vertical`) | The layout axis the accordion operates along. Controls which arrow keys navigate between triggers (`ArrowUp`/`ArrowDown` for vertical, `ArrowLeft`/`ArrowRight` for horizontal). |
| `dir`         | `dir`         | `'ltr' \| 'rtl'`                                    | `MaybeProp<Direction>`                        | The reading direction. Affects horizontal arrow-key navigation: in RTL, `ArrowRight` moves to the previous trigger and `ArrowLeft` to the next.                                  |
| `ref`         | `node_ref`    | `React.Ref`                                         | `AnyNodeRef`                                  | Ref to the root DOM element (`<div>`).                                                                                                                                           |
| `asChild`     | `as_child`    | `boolean`                                           | `MaybeProp<bool>`                             | When `true`, the component renders its child directly instead of wrapping in a `<div>`, merging props and refs onto the child.                                                   |
| *(spread)*    | —             | `...PrimitiveDivProps`                              | —                                             | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives on the call site instead.                                                                      |

### Leptos-only: `AccordionType` enum

```rust
pub enum AccordionType {
    Single,
    Multiple,
}
```

### Data attributes (rendered on DOM)

| Attribute          | Value                        | Description                      |
| ------------------ | ---------------------------- | -------------------------------- |
| `data-orientation` | `"horizontal" \| "vertical"` | Reflects the `orientation` prop. |

## Usage Examples

### Single mode (uncontrolled)

#### React

```tsx
<Accordion.Root type="single" defaultValue="one" collapsible>
  <Accordion.Item value="one">
    <Accordion.Header>
      <Accordion.Trigger>Section One</Accordion.Trigger>
    </Accordion.Header>
    <Accordion.Content>Content for section one.</Accordion.Content>
  </Accordion.Item>
  <Accordion.Item value="two">
    <Accordion.Header>
      <Accordion.Trigger>Section Two</Accordion.Trigger>
    </Accordion.Header>
    <Accordion.Content>Content for section two.</Accordion.Content>
  </Accordion.Item>
</Accordion.Root>
```

#### Leptos

```rust
<AccordionSingle default_value="one" collapsible=true>
  <AccordionItem value="one">
    <AccordionHeader>
      <AccordionTrigger>"Section One"</AccordionTrigger>
    </AccordionHeader>
    <AccordionContent>"Content for section one."</AccordionContent>
  </AccordionItem>
  <AccordionItem value="two">
    <AccordionHeader>
      <AccordionTrigger>"Section Two"</AccordionTrigger>
    </AccordionHeader>
    <AccordionContent>"Content for section two."</AccordionContent>
  </AccordionItem>
</AccordionSingle>
```

### Single mode (controlled)

#### React

```tsx
const [value, setValue] = React.useState('one');

<Accordion.Root type="single" value={value} onValueChange={setValue}>
  {/* ...items */}
</Accordion.Root>
```

#### Leptos

```rust
let (value, set_value) = signal("one".to_string());

<AccordionSingle
  value=value
  on_value_change=Callback::new(move |v: String| set_value.set(v))
>
  // ...items
</AccordionSingle>
```

### Multiple mode (uncontrolled)

#### React

```tsx
<Accordion.Root type="multiple" defaultValue={['one', 'two']}>
  {/* ...items */}
</Accordion.Root>
```

#### Leptos

```rust
<AccordionMultiple default_values=vec!["one".into(), "two".into()]>
  // ...items
</AccordionMultiple>
```

### Multiple mode (controlled)

#### React

```tsx
const [value, setValue] = React.useState(['one', 'two']);

<Accordion.Root type="multiple" value={value} onValueChange={setValue}>
  {/* ...items */}
</Accordion.Root>
```

#### Leptos

```rust
let (value, set_value) = signal(vec!["one".into(), "two".into()]);

<AccordionMultiple
  values=value
  on_values_change=Callback::new(move |v: Vec<String>| set_value.set(v))
>
  // ...items
</AccordionMultiple>
```

### Disabled item

#### React

```tsx
<Accordion.Item value="three" disabled>
  {/* ...header, trigger, content */}
</Accordion.Item>
```

#### Leptos

```rust
<AccordionItem value="three" disabled=true>
  // ...header, trigger, content
</AccordionItem>
```

### Animated content (CSS keyframes)

Use the `--radix-accordion-content-height` and `--radix-accordion-content-width` CSS custom properties to animate open/close transitions:

```css
.content {
  overflow: hidden;
}

.content[data-state='open'] {
  animation: slideDown 300ms ease-out;
}

.content[data-state='closed'] {
  animation: slideUp 300ms ease-out;
}

@keyframes slideDown {
  from { height: 0; }
  to   { height: var(--radix-accordion-content-height); }
}

@keyframes slideUp {
  from { height: var(--radix-accordion-content-height); }
  to   { height: 0; }
}
```

## Accessibility

Implements the [WAI-ARIA Accordion pattern](https://www.w3.org/WAI/ARIA/apd/patterns/accordion/).

### Keyboard Interactions

| Key               | Description                                                                                                     |
| ----------------- | --------------------------------------------------------------------------------------------------------------- |
| `Enter` / `Space` | When focus is on a trigger, toggles the associated content section open/closed.                                 |
| `ArrowDown`       | When `orientation="vertical"` (default): moves focus to the next trigger. Wraps from last to first.             |
| `ArrowUp`         | When `orientation="vertical"`: moves focus to the previous trigger. Wraps from first to last.                   |
| `ArrowRight`      | When `orientation="horizontal"`: moves focus to the next trigger (LTR) or previous trigger (RTL). Wraps around. |
| `ArrowLeft`       | When `orientation="horizontal"`: moves focus to the previous trigger (LTR) or next trigger (RTL). Wraps around. |
| `Home`            | Moves focus to the first trigger.                                                                               |
| `End`             | Moves focus to the last trigger.                                                                                |

### ARIA Attributes

| Element            | Attribute         | Value             | Notes                                                                                                                                                  |
| ------------------ | ----------------- | ----------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `AccordionTrigger` | `aria-expanded`   | `true` / `false`  | Inherited from `CollapsibleTrigger`. Reflects the item's open state.                                                                                   |
| `AccordionTrigger` | `aria-controls`   | `string`          | Inherited from `CollapsibleTrigger`. Points to the content element's `id`.                                                                             |
| `AccordionTrigger` | `aria-disabled`   | `"true"` / absent | Set when the item is open and the accordion is not collapsible (single mode, `collapsible=false`), indicating the trigger cannot collapse its section. |
| `AccordionContent` | `role`            | `"region"`        | Identifies the content as a landmark region.                                                                                                           |
| `AccordionContent` | `aria-labelledby` | `string`          | Points to the trigger's auto-generated `id`, linking the region to its heading.                                                                        |

### Behavioral Notes

- Disabled triggers are skipped during keyboard navigation.
- Arrow key navigation prevents page scroll (`event.preventDefault()`).
- In single mode with `collapsible=false` (the default), once an item is opened another must be opened to close it — there is always one item expanded.
- In multiple mode, the accordion is always implicitly collapsible — any combination of items can be open or closed.

## CSS Custom Properties

These properties are set on `AccordionContent` and alias the underlying `Collapsible` values. Use them in CSS animations or transitions to animate to/from the content's natural dimensions.

| Property                           | Source                                            | Description                                             |
| ---------------------------------- | ------------------------------------------------- | ------------------------------------------------------- |
| `--radix-accordion-content-height` | Aliased from `--radix-collapsible-content-height` | The measured height of the content when fully expanded. |
| `--radix-accordion-content-width`  | Aliased from `--radix-collapsible-content-width`  | The measured width of the content when fully expanded.  |

These properties update dynamically if the content size changes (e.g., from dynamic content being added). See the "Animated content" usage example above for a complete CSS keyframe pattern.
