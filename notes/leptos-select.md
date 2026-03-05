---
react_location: "[[reference/react-radix-primitives/packages/react/select/src/select.tsx|select]]"
rust_location: "[[packages/primitives/leptos/src/select/mod.rs|select]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/select.stories.tsx|select]]"
rust_story: "[[stories/leptos/src/primitives/select.rs|select]]"
dependencies:
  - "[[core-number]]"
  - "[[core-primitive]]"
  - "[[leptos-collection]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-direction]]"
  - "[[leptos-dismissable-layer]]"
  - "[[leptos-focus-guards]]"
  - "[[leptos-focus-scope]]"
  - "[[leptos-id]]"
  - "[[leptos-popper]]"
  - "[[leptos-portal]]"
  - "[[leptos-primitive]]"
  - "[[leptos-slot]]"
  - "[[leptos-use-callback-ref]]"
  - "[[leptos-use-controllable-state]]"
  - "[[leptos-use-previous]]"
  - "[[leptos-visually-hidden]]"
ported: true
tested: true
tested_story: true
---
## Intent

A dropdown select component with typeahead search, keyboard navigation, flexible positioning (item-aligned or popper mode), and native form integration. The largest Radix primitive.

## React API

```ts
// 16 sub-components:
Select, SelectTrigger, SelectValue, SelectIcon, SelectPortal,
SelectContent, SelectViewport, SelectGroup, SelectLabel, SelectItem,
SelectItemText, SelectItemIndicator, SelectScrollUpButton,
SelectScrollDownButton, SelectSeparator, SelectArrow
```

Props: `value`, `defaultValue`, `onValueChange`, `open`, `defaultOpen`, `onOpenChange`, `dir`, `name`, `form`, `disabled`, `required`. Content: `position` (`'item-aligned'` | `'popper'`).

## React Implementation Notes

- ~1842 lines — the largest single primitive.
- Two positioning modes: `item-aligned` (selected item overlaps trigger) and `popper` (Floating UI positioning).
- Typeahead search: Buffers key presses with 1-second timeout to jump to matching items.
- Scroll buttons for long lists with auto-scroll on hover.
- Pointer move delta tracking to prevent accidental selection on open.
- Native `<select>` rendered for form submission.
- `FocusScope` for focus management, `DismissableLayer` for dismiss.
- `aria-hidden` others integration.
- Pointer up debouncing: On touch, waits for pointer up before activating selection to prevent accidental taps.

## Leptos API

```rust
// 16 public components (matching React):
Select, SelectTrigger, SelectValue, SelectIcon, SelectPortal,
SelectContent, SelectViewport, SelectGroup, SelectLabel, SelectItem,
SelectItemText, SelectItemIndicator, SelectScrollUpButton,
SelectScrollDownButton, SelectSeparator, SelectArrow
```

All props converted to snake_case. `MaybeProp<T>` / `Signal<T>` for optional/reactive props. `Callback<T>` for event handlers.

## Leptos Implementation Notes

### Omissions

1. **Item-aligned positioning mode**: The `SelectItemAlignedPosition` component (React lines 827–1037) is not implemented. `SelectContentImpl` always renders `SelectPopperPosition` regardless of the `position` prop value. The `position` prop is stored but never used for conditional rendering. Users get popper positioning in all cases.

2. **Native option collection**: React's `SelectNativeOptionsContext` (which dynamically collects `<option>` elements from `SelectItemText` children) is not implemented. `SelectBubbleInput` renders a static hidden `<select>` with a single empty option. Form autofill based on actual option values will not work.

3. **`hide_others` (aria-hidden siblings)**: The `hide_others()` function exists (lines 1748–1794) but is marked `#[allow(dead_code)]` and never called. React calls `hideOthers(content)` on mount to mark non-Select siblings as `aria-hidden="true"` for screen readers.

4. **RemoveScroll**: React wraps content in `<RemoveScroll>` to prevent background scrolling while the Select is open. No equivalent in the Leptos port; `DismissableLayer` blocks pointer events but does not prevent scroll.

5. **Pointer move delta tracking**: React tracks pointer movement from the trigger `pointerdown` event and only activates item hover after the pointer moves >10px, preventing accidental selection. Not implemented.

6. **Window blur/resize close**: React closes the Select on `window.blur` and `window.resize` events. Not implemented.

7. **Context scope threading** (`__scopeSelect`): React's `createContextScope` pattern for namespace isolation is not implemented. Uses flat Leptos context providers. Standard omission for Leptos ports — nested Select instances could theoretically conflict but this is uncommon in practice.

### Key Decisions

1. **Signal types**: `Signal<Option<String>>` for value (derived via `Signal::derive`), `StoredValue<T>` for mutable refs and timers, `Callback<T>` for event handlers.

2. **Text portaling**: React uses `ReactDOM.createPortal()` to teleport selected item text into `SelectValue`. Leptos uses an `Effect` that manually copies `textContent` between DOM elements via `set_text_content()`.

3. **Scroll button increment**: Uses a fixed 32px scroll increment in auto-scroll, whereas React uses `selectedItem.offsetHeight` for adaptive scrolling. Minor behavioral difference.

4. **Collection pattern**: Uses the shared `radix-leptos-collection` crate with `CollectionProvider`, `CollectionSlot`, and `CollectionItemSlot` for item tracking, matching the pattern established by other ported components.

5. **Deferred close on selection**: `handle_select` defers `on_open_change(false)` to the next macrotask via `setTimeout`. This lets reactive effects triggered by the value change (e.g., text copying in `SelectItemText`) settle before the content is unmounted by Presence. Without deferral, synchronous close would dispose child scopes while queued effects still reference their `StoredValue`s.

6. **Inlined event handlers in SelectItem**: SelectItem event handlers (`on:focus`, `on:blur`, `on:click`, etc.) are inlined closures instead of using `compose_callbacks(user_cb, Callback::new(our_fn), None)`. `Callback::new(...)` creates a `StoredValue` in the reactive scope; when the scope is disposed during unmount, browser events (e.g., `blur` from FocusScope focus restoration) fire after disposal and try to invoke the disposed Callback, causing a WASM `unreachable` panic. Inlined closures avoid creating the intermediate `StoredValue`.

7. **Defensive `try_get_value` / `try_set_value` throughout**: All `StoredValue` accesses in `select.rs` use `try_get_value`, `try_set_value`, and `try_with_value` instead of panicking variants. This guards against disposal timing edge cases where a reactive subscriber or event handler accesses a `StoredValue` after its owning scope has been disposed.

8. **Inlined PopperContent**: `SelectContentImpl` renders `PopperContent` as a direct child of `DismissableLayer` (matching Popover's pattern), rather than wrapping it in a separate `SelectPopperPosition` component. This ensures the container ref chain (`FocusScope` → `DismissableLayer` → `PopperContent`) resolves correctly via `add_any_attr(any_node_ref(...))`.

### E2E Testing

- **39/39 Cypress tests pass** against both React and Leptos reference apps.
- Test coverage: accessibility semantics (7), data attributes (9), keyboard navigation (11), pointer interaction (3), focus management (2), value display (3), disabled state (2), groups (1).
- Reference app pages: `reference_app/react/src/pages/Select.jsx`, `reference_app/leptos/src/pages/select.rs`.
- Cypress test: `reference_app/cypress/e2e/select.cy.js`.
