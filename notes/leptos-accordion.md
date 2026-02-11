---
react_location: "[[reference/react-radix-primitives/packages/react/accordion/src/accordion.tsx|accordion]]"
rust_location: "[[packages/primitives/leptos/accordion/src/accordion.rs|accordion]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/accordion.stories.tsx|accordion]]"
rust_story: "[[stories/leptos/src/primitives/accordion.rs|accordion]]"
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-collapsible]]"
  - "[[leptos-collection]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-direction]]"
  - "[[leptos-id]]"
  - "[[leptos-primitive]]"
  - "[[leptos-use-controllable-state]]"
ported: true
tested: false
tested_story: true
---
## Intent

A composable accordion managing multiple expandable sections. Supports single and multiple open items, keyboard navigation (Home/End/ArrowKeys), optional collapsibility, and RTL support.

## React API

```ts
// 5 sub-components:
Accordion, AccordionItem, AccordionHeader, AccordionTrigger, AccordionContent
```

`type: 'single' | 'multiple'` (required). Single mode: `value?: string`, `collapsible?: boolean`. Multiple mode: `value?: string[]` (always collapsible).

## Leptos API

```rust
#[derive(Clone, Copy)]
enum AccordionType { Single, Multiple }
#[derive(Clone, Copy, Default)] 
enum Orientation { 
	Horizontal, 
	#[default] 
	Vertical
}

#[component] 
fn Accordion(
    r#type: AccordionType,
    // Single mode
    value: MaybeProp<String>,
    default_value: MaybeProp<String>,
    on_value_change: Option<Callback<String>>,
    collapsible: MaybeProp<bool>,
    // Multiple mode
    values: MaybeProp<Vec<String>>,
    default_values: MaybeProp<Vec<String>>,
    on_values_change: Option<Callback<Vec<String>>>,
    // Shared
    disabled: MaybeProp<bool>,
    dir: MaybeProp<Direction>,
    orientation: MaybeProp<Orientation>,
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView>,
) -> impl IntoView

#[component] 
fn AccordionItem(
    value: String,
    disabled: MaybeProp<bool>,
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView>,
) -> impl IntoView

#[component] 
fn AccordionHeader(
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView>,
) -> impl IntoView

#[component] 
fn AccordionTrigger(
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView>,
) -> impl IntoView

#[component] 
fn AccordionContent(
    force_mount: MaybeProp<bool>,
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    children: Option<TypedChildrenFn<impl IntoView>>,
) -> impl IntoView
```

## React Implementation Notes

- ~538 lines.
- Built on Collapsible primitives — each item is a `Collapsible`.
- Uses `Collection` pattern to track trigger elements for keyboard navigation.
- Comprehensive keyboard navigation: Home/End jump to first/last, ArrowDown/Up for vertical, ArrowLeft/Right for horizontal (respects RTL).
- Context tree: `AccordionValueProvider` (state) + `AccordionCollapsibleProvider` (collapsibility config) + `AccordionImplProvider` (shared config).
- Exposes CSS variables through Collapsible: `--radix-accordion-content-height`, `--radix-accordion-content-width`.
- Direction-aware via `useDirection(dir)`.

## Leptos Implementation Notes

- **Hybrid API**: Single `Accordion` component with `AccordionType` enum. Internally dispatches via `setup_single_context()`/`setup_multiple_context()` helper functions that set up `AccordionValueContextValue` and `AccordionCollapsibleContextValue`, then renders shared `AccordionImpl`.
- React's `AccordionImplSingle`/`AccordionImplMultiple` are replaced by these helper functions called directly inside `Accordion`, avoiding the need for intermediate wrapper components and the associated `Option<Callback>` prop issues.
- **Context tree**: `AccordionValueContextValue` (value, on_item_open, on_item_close), `AccordionCollapsibleContextValue` (collapsible), `AccordionImplContextValue` (disabled, orientation).
- **Keyboard navigation**: `AccordionImpl` handles `on:keydown` using `use_collection` to get trigger items in DOM order, filters disabled buttons, navigates Home/End/Arrow keys respecting orientation + RTL direction.
- **AccordionContent** maps CSS variables: `--radix-accordion-content-height: var(--radix-collapsible-content-height)` (same for width).
- **AccordionItem** wraps `Collapsible`, manages open state via the value context.
- **AccordionTrigger** wraps `CollectionItemSlot` > `CollapsibleTrigger`, adds `data-orientation` and `aria-disabled` attributes.
- **AccordionHeader** renders `Primitive::h3` with `data-orientation` and `data-state`.
- Uses `use_controllable_state` for controlled/uncontrolled single and multiple value management.
- Uses `use_id` for generating trigger/content IDs for ARIA relationships.
- **Omissions:**
  - `createAccordionScope` / scoped context providers — React's `createContextScope` pattern not applicable in Leptos. Leptos uses simple `provide_context`/`expect_context`.
  - `useCollapsibleScope` — React composes Collapsible's scope; not needed in Leptos since Collapsible is used directly without scope forwarding.
  - `AccordionImplSingle`/`AccordionImplMultiple` components — replaced by helper functions to avoid `Option<Callback>` prop passing complications in Leptos's type system.
- **Key decisions:**
  - `Orientation` enum defined locally in the accordion module (matches pattern used by separator and scroll-area).
  - `AccordionImpl` stores `use_collection` result in `StoredValue` for `Send + Sync` compatibility.
  - `direction` is used only locally in `AccordionImpl`'s keydown handler, not stored in context (unlike React which passes it through `AccordionImplProvider`).
