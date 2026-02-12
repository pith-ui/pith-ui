---
react_location: "[[reference/react-radix-primitives/packages/react/tabs/src/tabs.tsx|tabs]]"
rust_location: "[[packages/primitives/leptos/tabs/src/tabs.rs|tabs]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/tabs.stories.tsx|tabs]]"
rust_story: "[[stories/leptos/src/primitives/tabs.rs|tabs]]"
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-direction]]"
  - "[[leptos-id]]"
  - "[[leptos-presence]]"
  - "[[leptos-primitive]]"
  - "[[leptos-roving-focus]]"
  - "[[leptos-use-controllable-state]]"
ported: true
tested: false
tested_story: true
---
## Intent

A tabbed interface for organizing content into panels. Tabs navigate via roving focus; content panels show/hide with optional animation support.

## React API

```ts
// 4 sub-components:
Tabs, TabsList, TabsTrigger, TabsContent

// Tabs (Root)
interface TabsProps extends PrimitiveDivProps {
  value?: string;
  defaultValue?: string;
  onValueChange?: (value: string) => void;
  orientation?: 'horizontal' | 'vertical'; // default 'horizontal'
  dir?: 'ltr' | 'rtl';
  activationMode?: 'automatic' | 'manual'; // default 'automatic'
}

// TabsList
interface TabsListProps extends PrimitiveDivProps {
  loop?: boolean; // default true
}

// TabsTrigger
interface TabsTriggerProps extends PrimitiveButtonProps {
  value: string;
  disabled?: boolean;
}

// TabsContent
interface TabsContentProps extends PrimitiveDivProps {
  value: string;
  forceMount?: true;
}
```

## Leptos API

```rust
// Tabs (Root)
pub fn Tabs(
    value: MaybeProp<String>,
    default_value: MaybeProp<String>,
    on_value_change: Option<Callback<String>>,
    orientation: MaybeProp<Orientation>,  // re-exported from roving-focus
    dir: MaybeProp<Direction>,
    activation_mode: MaybeProp<ActivationMode>,
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView

// TabsList
pub fn TabsList(
    r#loop: MaybeProp<bool>,
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView

// TabsTrigger
pub fn TabsTrigger(
    value: String,
    disabled: MaybeProp<bool>,
    on_mouse_down: Option<Callback<ev::MouseEvent>>,
    on_key_down: Option<Callback<ev::KeyboardEvent>>,
    on_focus: Option<Callback<ev::FocusEvent>>,
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView

// TabsContent
pub fn TabsContent(
    value: String,
    force_mount: MaybeProp<bool>,
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    children: Option<TypedChildrenFn<impl IntoView + 'static>>,
) -> impl IntoView

pub use radix_leptos_roving_focus::Orientation;

pub enum ActivationMode { Automatic, Manual }
```

## React Implementation Notes

- ~299 lines.
- `RovingFocusGroup` for keyboard navigation between tab triggers.
- `Presence` for content mounting/unmounting with animation support.
- `activationMode`: `'automatic'` selects tab on focus, `'manual'` requires click/Enter.
- Generated IDs link triggers to content panels via `aria-controls`/`aria-labelledby`.
- RTL text direction support via `useDirection`.
- `data-state`: `active` | `inactive` on both trigger and content.
- `isMountAnimationPreventedRef` suppresses entry animation on initial mount for the initially selected tab.

## Leptos Implementation Notes

- **Orientation re-export:** `Orientation` is re-exported from `radix_leptos_roving_focus` rather than defining a separate type. This avoids type conversion issues when passing orientation to `RovingFocusGroup`. Default horizontal is set in the component via `unwrap_or(Orientation::Horizontal)`.
- **Context scope:** React uses `createContextScope` for nesting isolation. The Leptos port uses `Provider` from `leptos::context`, following the same pattern as Accordion.
- **Presence pattern:** `TabsContent` uses `Presence` with `present = force_mount || is_selected`. Since the Leptos `Presence` component uses `<Show>` (not a render prop like React), the `hidden` attribute from React's implementation is omitted — Presence handles mount/unmount directly. This matches the Collapsible pattern.
- **Mount animation prevention:** Uses `RwSignal<bool>` + `requestAnimationFrame` to suppress animation on first mount. The initially-selected content gets `animation-duration: 0s` for one frame, then the style is removed.
- **Event composition:** Uses `compose_callbacks` from `radix-leptos-primitive` to merge user-provided event handlers with internal handlers.
- **TabsContentImpl:** Separated from `TabsContent` so Presence can control its mount lifecycle, following the CollapsibleContentImpl pattern.
