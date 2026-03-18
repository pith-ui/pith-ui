# ToolbarToggleGroup

## React Signature

```typescript
const ToolbarToggleGroup = React.forwardRef<
  ToolbarToggleGroupElement,
  ToolbarToggleGroupSingleProps | ToolbarToggleGroupMultipleProps
>(...)

type ToolbarToggleGroupElement = React.ComponentRef<typeof ToggleGroupPrimitive.Root>;
type ToggleGroupProps = React.ComponentPropsWithoutRef<typeof ToggleGroupPrimitive.Root>;

interface ToolbarToggleGroupSingleProps extends Extract<ToggleGroupProps, { type: 'single' }> {}
interface ToolbarToggleGroupMultipleProps extends Extract<ToggleGroupProps, { type: 'multiple' }> {}

// The underlying ToggleGroup single props:
interface ToggleGroupImplSingleProps extends ToggleGroupImplProps {
  value?: string;
  defaultValue?: string;
  onValueChange?(value: string): void;
}

// The underlying ToggleGroup multiple props:
interface ToggleGroupImplMultipleProps extends ToggleGroupImplProps {
  value?: string[];
  defaultValue?: string[];
  onValueChange?(value: string[]): void;
}

// Shared base:
interface ToggleGroupImplProps extends PrimitiveDivProps {
  disabled?: boolean;
  rovingFocus?: boolean; // forced to false by ToolbarToggleGroup
  loop?: boolean;
  orientation?: RovingFocusGroupProps['orientation'];
  dir?: RovingFocusGroupProps['dir'];
}
```

## Leptos Signature

```rust
pub fn ToolbarToggleGroup(
    r#type: ToggleGroupType,
    #[prop(into, optional)] value: MaybeProp<Vec<String>>,
    #[prop(into, optional)] default_value: MaybeProp<Vec<String>>,
    #[prop(into, optional)] on_value_change: Option<Callback<Vec<String>>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `type` | `r#type` | `'single' \| 'multiple'` (required) | `ToggleGroupType` (required) | Whether the group allows single or multiple items to be pressed at once. |
| `value` | `value` | `string` (single) / `string[]` (multiple) | `MaybeProp<Vec<String>>` | The controlled value of the pressed item(s). In React, the type depends on `type`; in Leptos, it is always `Vec<String>` regardless of mode. For single mode, wrap the value in a one-element vec. |
| `defaultValue` | `default_value` | `string` (single) / `string[]` (multiple) | `MaybeProp<Vec<String>>` | The value of the pressed item(s) on initial render. Same type difference as `value`. |
| `onValueChange` | `on_value_change` | `(value: string) => void` (single) / `(value: string[]) => void` (multiple) | `Option<Callback<Vec<String>>>` | Callback fired when the pressed state changes. In Leptos, always receives a `Vec<String>` regardless of mode. |
| `disabled` | `disabled` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | Disables all toggle items in the group. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| *(spread)* | — | `...ToggleGroupProps` | — | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Leptos-only: `ToggleGroupType` enum

```rust
pub enum ToggleGroupType {
    Single,
    Multiple,
}
```

### Implicit behavior

- Inherits `orientation` and `dir` from the parent `Toolbar` context — these are not exposed as props on `ToolbarToggleGroup`.
- Forces `rovingFocus={false}` on the underlying `ToggleGroup` because the parent `Toolbar`'s `RovingFocusGroup` already handles keyboard navigation. Individual `ToolbarToggleItem`s are each wrapped in a `RovingFocusGroupItem` via `ToolbarButton`.

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-orientation` | `"horizontal" \| "vertical"` | Reflects the parent toolbar's `orientation`. Set explicitly on the element in addition to any attributes from the underlying `ToggleGroup`. |
