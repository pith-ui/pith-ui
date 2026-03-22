---
react_location: ""
rust_location: "[[packages/primitives/leptos/src/components/combobox/mod.rs|combobox]]"
react_story: ""
rust_story: "[[stories/leptos/src/primitives/combobox.rs|combobox]]"
dependencies: []
ported: true
tested: true
tested_story: false
unstable: false
---
## Intent

Combobox primitive combining a text input for filtering with a dropdown listbox for selection. Supports single-select and multi-select modes. Implements the [WAI-ARIA Combobox pattern](https://www.w3.org/WAI/ARIA/apd/patterns/combobox/).

This component does not exist in React Radix Primitives. It is inspired by [Base UI's Combobox](https://base-ui.com/react/components/combobox) but follows Cardo UI's compositional patterns (as_child, Primitive, Popper, DismissableLayer, Collection).

## React API

N/A — no React Radix equivalent. Reference implementation uses `@base-ui/react` Combobox as the React oracle in the E2E test harness.

## Leptos API

### Components

- `Combobox` — Root. Props: `open`, `default_open`, `on_open_change`, `value`, `default_value`, `on_value_change`, `values`, `default_values`, `on_values_change`, `input_value`, `default_input_value`, `on_input_value_change`, `multiple`, `disabled`, `required`, `name`, `form`, `dir`
- `ComboboxAnchor` — Wraps input + trigger as positioning anchor
- `ComboboxInput` — Text input with `role="combobox"`, `aria-activedescendant`, keyboard nav
- `ComboboxTrigger` — Toggle button (`tabindex="-1"`)
- `ComboboxIcon` — Decorative indicator
- `ComboboxClear` — Clear value button
- `ComboboxPortal` — Renders content in a portal
- `ComboboxContent` — Positioned dropdown (DismissableLayer + PopperContent)
- `ComboboxViewport` — Scrollable container with CollectionSlot
- `ComboboxItem` — `role="option"` with `aria-selected`, `data-state`, `data-highlighted`
- `ComboboxItemText` — Text within an item
- `ComboboxItemIndicator` — Checkmark when selected
- `ComboboxGroup` — `role="group"` with `aria-labelledby`
- `ComboboxLabel` — Group label
- `ComboboxSeparator` — Visual separator
- `ComboboxEmpty` — Shown when no items match
- `ComboboxArrow` — Popup arrow
- `ComboboxChips` — Multi-select chip container
- `ComboboxChip` — Individual chip
- `ComboboxChipRemove` — Remove button on chip

## Leptos Implementation Notes

### Key differences from Select

1. **No focus trapping** — Input retains focus. Items highlighted via `aria-activedescendant`, not DOM focus.
2. **Non-modal popup** — No aria-hidden of siblings, no scroll lock.
3. **Consumer-managed filtering** — `on_input_value_change` callback; consumer controls which items render.
4. **Only Enter selects** — Space types in the input (unlike Select where Space selects).
5. **Multi-select mode** — Values stored as `Vec<String>`, items toggle on click, popup stays open.
6. **Scroll into view** — Explicit `scrollIntoView()` on active descendant change.

### Data attribute conventions

Uses Radix-style data attributes (`data-state="checked"/"unchecked"`, `data-state="open"/"closed"`) rather than Base UI style (`data-selected`, `data-open`). Cypress E2E tests use `Cypress.env('FRAMEWORK')` conditionals where attributes differ between Base UI (React oracle) and Cardo UI (Leptos).

### Omissions

- No virtualization support (Base UI supports `virtualized` prop)
- No grid layout mode (Base UI supports `grid` prop)
- No built-in filtering logic (consumer-managed by design)
- No `Collection` render-function pattern (Base UI's `<Combobox.Collection>`)

#### Virtualization

Base UI's `virtualized: true` delegates DOM rendering to an external library (e.g., TanStack Virtual), bypassing DOM-based item registration. Items must supply explicit `index` props instead of relying on DOM order.

**Why omitted:** Our `Collection` system walks the DOM to register items. Virtualization requires bypassing this entirely and accepting externally-managed indices. Leptos also lacks a mature virtualization ecosystem equivalent to TanStack Virtual.

**Complexity to add:** Medium. Skip `CollectionItemSlot` registration when virtualized, require `index` on each item, and use the index directly for `aria-activedescendant` calculation. The `navigate_items` logic would need an alternative path that works with index + count rather than a materialized item list.

**Benefit:** High for lists with hundreds/thousands of items (country pickers, user searches). Low for typical use cases with <100 items. Worth adding once Leptos has a solid virtual-scroll crate to pair with.

#### Grid layout mode

Base UI's `grid: true` transforms the listbox into a multi-column grid. Left/right arrows move between columns, up/down between rows. Items get `role="gridcell"` instead of `role="option"`, the list gets `role="grid"`, and items must be wrapped in `ComboboxRow` components.

**Why omitted:** Grid comboboxes are uncommon — most combobox UIs are single-column lists. The WAI-ARIA grid pattern adds complexity for a niche use case.

**Complexity to add:** Low-medium. Mostly role changes and extending `navigate_items` to handle 2D navigation (left/right within a row, up/down between rows). A `ComboboxRow` component (simple context provider) would be needed.

**Benefit:** Low. Grid comboboxes appear in specialized UIs (emoji pickers, icon selectors). Defer unless demand arises.

#### Built-in filtering

Base UI provides `useComboboxFilter` with locale-aware matching via `Intl.Collator` (accent-insensitive, supports `contains`/`startsWith`/`endsWith`). Root accepts an `items` array + `filter` function and computes `filteredItems` internally via context.

**Why omitted (by design):** In Leptos, reactive signals make consumer-managed filtering natural and ergonomic — a 2-line `Memo` handles it. Embedding filtering into the component would add API surface (`items`, `filter`, `filteredItems` context) that duplicates what signals already provide, and would constrain the filtering model (consumers may want fuzzy matching, server-side filtering, debounced queries, etc.).

**Complexity to add:** Low. The filter logic itself is simple string matching. The harder part is API design: accepting `items: Vec<T>` on Root, providing `filteredItems` via context, and making items render from the filtered list.

**Benefit:** Moderate convenience — saves consumers from writing their own filter memo. But it trades flexibility for convenience, and Leptos signals already make the consumer path easy. A standalone `use_combobox_filter` utility function (not embedded in the component) could provide the best of both worlds.

#### Collection render-function pattern

Base UI's `<Combobox.List>{(item, index) => ...}</Combobox.List>` detects function children and wraps them in a `ComboboxCollection` that maps filtered items from context.

**Why omitted:** This pattern is React-specific (render props / function-as-children). Leptos's equivalent is a reactive closure: `{move || items.get().into_iter().map(...).collect_view()}`, which is already used in the reference page and stories. There is no ergonomic gap to fill.

**Complexity to add:** Very low technically, but architecturally awkward. Detecting "is this a render function vs static view" isn't idiomatic in Leptos.

**Benefit:** Near zero. Leptos reactive closures already serve this purpose.

#### Summary

| Feature | Complexity | Benefit | Recommendation |
|---------|-----------|---------|----------------|
| Virtualization | Medium | High (large lists) | Add when Leptos virtual-scroll matures |
| Grid mode | Low-medium | Low (niche) | Defer unless demand arises |
| Built-in filtering | Low | Moderate convenience | Consider standalone utility |
| Collection render-function | Very low | Near zero | Skip — closures cover this |
