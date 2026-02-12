---
react_location: "[[reference/react-radix-primitives/packages/react/toggle-group/src/toggle-group.tsx|toggle-group]]"
rust_location: "[[packages/primitives/leptos/toggle-group/src/toggle_group.rs|toggle-group]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/toggle-group.stories.tsx|toggle-group]]"
rust_story: "[[stories/leptos/src/primitives/toggle_group.rs|toggle-group]]"
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-direction]]"
  - "[[leptos-primitive]]"
  - "[[leptos-roving-focus]]"
  - "[[leptos-use-controllable-state]]"
ported: true
tested: false
tested_story: true
---
## Intent

A group of toggle buttons with single or multiple selection mode. Supports roving focus (keyboard navigation), disabled state, and visual/semantic state representation.

## React API

```ts
// ToggleGroup — routes to Single or Multiple based on `type` prop
interface ToggleGroupSingleProps extends ToggleGroupImplProps {
  type: 'single';
  value?: string;
  defaultValue?: string;
  onValueChange?(value: string): void;
}
interface ToggleGroupMultipleProps extends ToggleGroupImplProps {
  type: 'multiple';
  value?: string[];
  defaultValue?: string[];
  onValueChange?(value: string[]): void;
}

// Shared impl props
interface ToggleGroupImplProps extends PrimitiveDivProps {
  disabled?: boolean;          // default false
  rovingFocus?: boolean;       // default true
  loop?: boolean;              // from RovingFocusGroup
  orientation?: Orientation;   // from RovingFocusGroup
  dir?: Direction;             // from RovingFocusGroup
}

// ToggleGroupItem
interface ToggleGroupItemProps extends Omit<ToggleProps, 'defaultPressed' | 'onPressedChange'> {
  value: string;  // required, unique within group
}
```

Two contexts:
- `ToggleGroupValueContext`: { type, value: string[], onItemActivate, onItemDeactivate }
- `ToggleGroupContext`: { rovingFocus, disabled }

## Leptos API

```rust
#[component]
fn ToggleGroup(
    r#type: ToggleGroupType,            // Single or Multiple
    value: MaybeProp<Vec<String>>,       // controlled value (single: vec of 0-1, multiple: vec of N)
    default_value: MaybeProp<Vec<String>>,
    on_value_change: Option<Callback<Vec<String>>>,
    disabled: MaybeProp<bool>,
    roving_focus: MaybeProp<bool>,
    r#loop: MaybeProp<bool>,
    orientation: MaybeProp<Orientation>,
    dir: MaybeProp<Direction>,
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView

#[component]
fn ToggleGroupItem(
    value: Signal<String>,               // required
    disabled: MaybeProp<bool>,
    on_click: Option<Callback<ev::MouseEvent>>,
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView
```

## React Implementation Notes

- ~319 lines.
- Built on Toggle primitive for each item.
- `RovingFocusGroup` for keyboard navigation when `rovingFocus=true`.
- Single mode: value is `string`, stored internally as `[value]` or `[]`. Uses `role="radio"` and `aria-checked` (with `aria-pressed` set to `undefined`).
- Multiple mode: value is `string[]`. Uses standard toggle `aria-pressed` semantics.
- `ToggleGroupItem` wraps `Toggle` in `RovingFocusGroup.Item` if roving focus is enabled.
- Disabled at group level propagated to items via context (item disabled = group disabled || own disabled).
- `ToggleGroupItemImpl` calls `onItemActivate`/`onItemDeactivate` from value context based on pressed state changes.
- React has separate `ToggleGroupImplSingle` and `ToggleGroupImplMultiple` components for the two modes, both wrapping `ToggleGroupImpl`.

## Leptos Implementation Notes

- **Unified value type**: Unlike React which uses `string` for single and `string[]` for multiple, the Leptos port uses `Vec<String>` for both modes. For single mode, the vec has 0 or 1 elements. This simplifies the internal implementation while maintaining the same behavior. The `on_value_change` callback always receives `Vec<String>`.
- **ToggleGroupType enum**: Replaces the React `type: 'single' | 'multiple'` string literal with `ToggleGroupType::Single` and `ToggleGroupType::Multiple`.
- **Single component**: Instead of React's separate `ToggleGroupImplSingle` / `ToggleGroupImplMultiple` / `ToggleGroupImpl`, the Leptos port uses a single `ToggleGroup` component that handles both modes via the `ToggleGroupType` enum and conditional logic. A separate `ToggleGroupImpl` sub-component handles the conditional `RovingFocusGroup` wrapping.
- **Context pattern**: Two contexts like React — `ToggleGroupValueContextValue` (type, value, activate/deactivate callbacks) and `ToggleGroupContextValue` (roving_focus, disabled) — provided via Leptos `Provider`.
- **Scope omission**: React's `createContextScope` / `__scopeToggleGroup` pattern is omitted. Leptos uses standard `provide_context` / `expect_context` which is sufficient for the Radix composition model. Same pattern as other ported components (tabs, accordion, etc.).
- **Toggle omission**: React's `ToggleGroupItemImpl` wraps the `Toggle` primitive and overrides `aria-pressed` to `undefined` for single mode via prop spread. In Leptos, we can't override attributes set internally by a child component, so `ToggleGroupItemImpl` renders a `Primitive` button directly instead of wrapping `Toggle`. This gives us full control over ARIA attributes: single mode gets `role="radio"` + `aria-checked`, multiple mode gets `aria-pressed`. The button click, data-state, data-disabled, and disabled behaviors are reimplemented inline (identical to Toggle's logic).
- **AttributeInterceptor pattern**: Uses `AttributeInterceptor let:attrs` + `{..attrs}` for attribute forwarding, consistent with all other Leptos primitives in this codebase.
- Dependencies: `leptos`, `leptos-node-ref`, `radix-leptos-primitive`, `radix-leptos-roving-focus`, `radix-leptos-direction`, `radix-leptos-use-controllable-state` (note: `radix-leptos-toggle` is NOT a dependency since items render buttons directly).
