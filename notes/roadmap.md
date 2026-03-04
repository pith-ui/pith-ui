# Refactoring Roadmap: Leptos Primitives

## 1. Extract `wrap_callback` to shared crate

**Priority:** 1 | **Effort:** Trivial

The identical function is copy-pasted in 5 files (`menu.rs`, `dropdown_menu.rs`, `context_menu.rs`, `menubar.rs`, `alert_dialog.rs`):

```rust
fn wrap_callback<T: 'static>(cb: Option<Callback<T>>) -> Callback<T> {
    match cb {
        Some(cb) => cb,
        None => Callback::new(|_| {}),
    }
}
```

**Fix:** Move to `radix_leptos_primitive` and re-export.

## 2. Extract `get_state(bool)` helper

**Priority:** 2 | **Effort:** Trivial

5 identical copies across `collapsible.rs`, `dialog.rs`, `accordion.rs`, `hover_card.rs`, `popover.rs` (plus variants named `get_open_state` in tooltip/menu/dropdown-menu):

```rust
fn get_state(open: bool) -> &'static str {
    match open { true => "open", false => "closed" }
}
```

**Fix:** Add `pub fn open_closed_state(open: bool) -> &'static str` to `radix_leptos_primitive`.

## 3. Simplify `use_controllable_state` callback wrapping

**Priority:** 3 | **Effort:** Small

Every component using `use_controllable_state` with an external callback writes this 6-line adapter (~12 occurrences):

```rust
on_change: on_open_change.map(|cb| {
    Callback::new(move |value: Option<bool>| {
        if let Some(value) = value { cb.run(value); }
    })
}),
```

**Fix:** Add a helper function:

```rust
pub fn adapt_callback<T: 'static>(cb: Option<Callback<T>>) -> Option<Callback<Option<T>>> {
    cb.map(|cb| Callback::new(move |v: Option<T>| { if let Some(v) = v { cb.run(v); } }))
}
```

Or redesign `use_controllable_state` so `on_change` takes `Callback<T>` directly instead of `Callback<Option<T>>`, pushing the `Option` unwrap inside the hook.

## 4. `MaybeProp<bool>` → `Signal<bool>` conversion helper

**Priority:** 4 | **Effort:** Small

67 occurrences of `Signal::derive(move || prop.get().unwrap_or(default))`.

**Fix:**

```rust
pub fn prop_or<T: Clone + Send + Sync + 'static>(prop: MaybeProp<T>, default: T) -> Signal<T> {
    Signal::derive(move || prop.get().unwrap_or(default))
}
```

Each call site becomes `let disabled = prop_or(disabled, false);`.

## 5. `data-disabled` attribute helper

**Priority:** 5 | **Effort:** Small

54 occurrences of `attr:data-disabled=move || disabled.get().then_some("")`.

**Fix:**

```rust
pub fn data_attr(signal: Signal<bool>) -> impl Fn() -> Option<&'static str> {
    move || signal.get().then_some("")
}
```

Call sites become `attr:data-disabled=data_attr(disabled)`.

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

| #   | Refactor                                       | Effort       | Impact                                   |
| --- | ---------------------------------------------- | ------------ | ---------------------------------------- |
| 1   | Extract `wrap_callback`                        | Trivial      | Removes 5 duplicates                     |
| 2   | Extract `get_state`/`open_closed_state`        | Trivial      | Removes 8+ duplicates                    |
| 3   | `adapt_callback` for controllable state        | Small        | Cleans up 12 call sites                  |
| 4   | `prop_or` helper                               | Small        | Cleans up 67 call sites                  |
| 5   | `data_attr` helper + pub(crate) contexts       | Small        | Cleans up 54 sites, enables unit testing |
| 6   | Generic `ScopedPortal`                         | Medium       | Removes ~100 lines of portal boilerplate |
| 7   | Extract state machines + consistent Impl split | Medium-Large | Major testability win                    |

Items 1–4 are low-risk, mechanical, and could be done in a single PR. Items 5–7 are more architectural and would benefit from incremental rollout.

## Notes

- The `StoredValue::new(children)` pattern (216 occurrences) is a Leptos framework limitation rather than something to solve in this codebase — low priority.
- The `children.with_value(|c| c())` pattern (235 occurrences) is similarly a framework-level concern.
