# Refactoring Roadmap: Leptos Primitives

## 1. ~~Extract `wrap_callback` to shared crate~~ ✅ DONE

Consolidated into `support::primitive::wrap_callback`. All components import from the shared module.

## 2. ~~Extract `get_state(bool)` helper~~ ✅ DONE

Consolidated as `support::primitive::open_closed_state`. Last remaining `get_open_state` in navigation_menu replaced.

## 3. ~~Simplify `use_controllable_state` callback wrapping~~ ✅ DONE

Added `support::primitive::adapt_callback` and migrated all ~22 adapter pattern call sites.

## 4. `MaybeProp<bool>` → `Signal<bool>` conversion helper — PARTIALLY DONE

`prop_or` and `prop_or_default` helpers exist in `support::primitive` and are used in 11+ components. Popper props in `menu_content.rs` migrated.

**Remaining:** ~22 inline `Signal::derive(move || x.get().unwrap_or(...))` patterns remain, but most operate on `Signal<Option<T>>` (from `use_controllable_state`), not `MaybeProp<T>`. These can't use the current `prop_or` without making it generic over `Get<Value = Option<T>>`. The remaining compound patterns (e.g., `force_mount.get().unwrap_or(false) || context.open.get()`) can't be simplified with `prop_or`.

**Possible future improvement:** Make `prop_or` generic or redesign `use_controllable_state` to return `Signal<T>` instead of `Signal<Option<T>>`.

## 5. ~~`data-disabled` attribute helper~~ ✅ DONE

`support::primitive::data_attr` exists and is used across all components. No remaining inline patterns.

## 6. Make context types `pub(crate)`

**Priority:** 5 | **Effort:** Small

Context structs are all private, making it impossible to write unit tests that provide mock contexts to inner components. Making context value types `pub(crate)` (or `pub`) would enable isolated component testing.

## 7. Generic `ScopedPortal` component

**Priority:** 6 | **Effort:** Medium

Every Portal component (Dialog, Popover, HoverCard, Tooltip, Select — 5 implementations) is structurally identical: capture contexts outside the portal, re-provide them inside. Each creates a trivial `*PortalContextValue` struct that only holds `force_mount: Signal<bool>`.

**Fix:** A generic `ScopedPortal<T: PortalContext>` component that:
- Accepts a list of context values to forward
- Provides a standard `PortalContextValue { force_mount }` inside
- Handles `popper_scope` and `collection_scope` forwarding automatically

Eliminates ~20 lines per portal component and prevents bugs when new contexts are added but forgotten in portals.

## 8. Extract state machines from component functions

**Priority:** 7 | **Effort:** Medium-Large

Components like `Accordion` and `Tabs` have non-trivial state machines (which items are open, roving focus index) buried inside `#[component]` functions. Extracting these into standalone state-machine structs/functions that can be tested without mounting a view would improve unit test coverage.

## 9. Consistent `Impl` component split

**Priority:** 7 | **Effort:** Medium

The "logic shell" / "rendered impl" split is used inconsistently — some components use it (Dialog, Collapsible, Accordion) and others don't (Checkbox, Toggle). Consistently separating these across all components would make the codebase more predictable.

## Summary

| #   | Refactor                                       | Effort       | Status                                  |
| --- | ---------------------------------------------- | ------------ | --------------------------------------- |
| 1   | Extract `wrap_callback`                        | Trivial      | ✅ Done                                 |
| 2   | Extract `get_state`/`open_closed_state`        | Trivial      | ✅ Done                                 |
| 3   | `adapt_callback` for controllable state        | Small        | ✅ Done (22 sites migrated)             |
| 4   | `prop_or` helper                               | Small        | Partially done (MaybeProp sites done)   |
| 5   | `data_attr` helper                             | Small        | ✅ Done                                 |
| 6   | `pub(crate)` contexts                          | Small        | Not started                             |
| 7   | Generic `ScopedPortal`                         | Medium       | Not started                             |
| 8   | Extract state machines + consistent Impl split | Medium-Large | Not started                             |

Items 1–3, 5 are complete. Item 4 is partially complete. Items 6–8 are more architectural and would benefit from incremental rollout.

## Notes

- The `StoredValue::new(children)` pattern (216 occurrences) is a Leptos framework limitation rather than something to solve in this codebase — low priority.
- The `children.with_value(|c| c())` pattern (235 occurrences) is similarly a framework-level concern.
