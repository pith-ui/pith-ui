---
react_location: "[[reference/react-radix-primitives/packages/react/toolbar/src/toolbar.tsx|toolbar]]"
rust_location: "[[packages/primitives/leptos/toolbar/src/toolbar.rs|toolbar]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/toolbar.stories.tsx|toolbar]]"
rust_story: "[[stories/leptos/src/primitives/toolbar.rs|toolbar]]"
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-direction]]"
  - "[[leptos-primitive]]"
  - "[[leptos-roving-focus]]"
  - "[[leptos-separator]]"
  - "[[leptos-toggle-group]]"
ported: true
tested: false
tested_story: true
---
## Intent

A flexible container for grouping related interactive controls (buttons, links, toggle groups) with roving focus and optional visual separators. Commonly used for editor toolbars, formatting controls.

## React API

```ts
// 6 sub-components:
Toolbar         // Root container with role="toolbar", wraps RovingFocusGroup
ToolbarButton   // Button wrapped in RovingFocusGroup.Item
ToolbarLink     // Anchor wrapped in RovingFocusGroup.Item, space key triggers click
ToolbarSeparator // Separator with auto-flipped orientation
ToolbarToggleGroup // ToggleGroup with rovingFocus=false (toolbar handles focus)
ToolbarToggleItem  // ToggleGroupItem wrapped in ToolbarButton
```

Props on `Toolbar`: `orientation` (default `'horizontal'`), `dir`, `loop` (default `true`).

## Leptos API

```rust
// Toolbar (root)
pub fn Toolbar(
    orientation: Option<Orientation>,       // default Horizontal
    dir: MaybeProp<Direction>,
    r#loop: MaybeProp<bool>,                // default true
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView>,
)

// ToolbarButton
pub fn ToolbarButton(
    disabled: MaybeProp<bool>,
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView>,
)

// ToolbarLink
pub fn ToolbarLink(
    on_key_down: Option<Callback<ev::KeyboardEvent>>,
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView>,
)

// ToolbarSeparator
pub fn ToolbarSeparator(
    decorative: MaybeProp<bool>,
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    children: Option<ChildrenFn>,
)

// ToolbarToggleGroup
pub fn ToolbarToggleGroup(
    r#type: ToggleGroupType,
    value: MaybeProp<Vec<String>>,
    default_value: MaybeProp<Vec<String>>,
    on_value_change: Option<Callback<Vec<String>>>,
    disabled: MaybeProp<bool>,
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView>,
)

// ToolbarToggleItem
pub fn ToolbarToggleItem(
    value: Signal<String>,
    disabled: MaybeProp<bool>,
    on_click: Option<Callback<ev::MouseEvent>>,
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView>,
)
```

## React Implementation Notes

- ~250 lines. Pure composition of `RovingFocusGroup` + `ToggleGroup` + `Separator` primitives.
- All children (Button, Link, ToggleGroup items) are `RovingFocusGroup.Item`s.
- `ToolbarLink` adds space key handling (converts to click) via `composeEventHandlers`.
- `ToolbarSeparator` auto-flips orientation perpendicular to toolbar.
- `ToolbarToggleGroup` sets `rovingFocus={false}` to prevent double focus handling (toolbar handles focus).
- `ToolbarToggleItem` bridges: wraps `ToggleGroupItem` in `ToolbarButton` using `asChild`.
- Keyboard navigation handled entirely by `RovingFocusGroup`.
- Uses React scope system (`createToolbarScope`) for context isolation.

## Leptos Implementation Notes

### Omissions
- **Scope system**: React's `createToolbarScope`/`__scopeToolbar` pattern for context isolation is omitted. Leptos uses standard `Provider`/`expect_context` which provides sufficient isolation for component trees. The scope system is a React-specific pattern for allowing multiple independent instances of the same component type to coexist with separate contexts — Leptos's context system handles this naturally through component tree nesting.

### Assumptions and key decisions
- **Orientation type**: The toolbar context stores `radix_leptos_roving_focus::Orientation` and converts to `radix_leptos_separator::Orientation` for `ToolbarSeparator` since these are separate types in the Leptos implementation.
- **Callback forwarding**: `Option<Callback<T>>` props can't be forwarded directly to child components with `#[prop(into, optional)]` due to `IntoReactiveValue` trait bounds. Instead, optional callbacks are wrapped in a non-optional `Callback` that conditionally invokes the inner callback.
- **Orientation as non-signal**: Toolbar's `orientation` is taken as `Option<Orientation>` (not reactive) since it's set once at creation and used to configure both the context and child components. This matches the React pattern where orientation is a plain prop.
- **DropdownMenu omission in stories**: The React storybook integrates a DropdownMenu inside the toolbar example. This is omitted from the Leptos story since DropdownMenu is not yet ported.
