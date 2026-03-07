# Refactor: Reorganize Leptos Primitives Crate

## Context

The Leptos primitives crate (`packages/primitives/leptos/src/`) has ~31K lines across 57 files, all flat in `src/`. We want to:
1. **Phase 1**: Split large single-file components into per-part files (agent-friendly, better diffs)
2. **Phase 2**: Group into `components/` and `support/` subdirectories

Public API stays unchanged — `radix_leptos_primitives::dialog::Dialog` still works.

## Phase 1: Split Large Modules

**Threshold**: Split modules with **600+ lines AND 5+ components**, or **500+ lines AND natural part boundaries**.

Each split module's `mod.rs` retains shared types (contexts, constants, helpers, enums) and re-exports all public items via `pub use`. Sub-files use `use super::*` for shared items.

### Modules to Split (17 modules)

| Module | Lines | Components | Approx Files |
|--------|-------|-----------|------|
| select | 2,469 | 16+4 internal | ~8 |
| menu | 2,297 | 16+4 internal | ~7 |
| navigation_menu | 2,204 | 9+6 internal | ~7 |
| scroll_area | 2,033 | 5+7 internal | ~5 |
| one_time_password_field | 1,493 | 3 | ~3 |
| tooltip | 1,444 | 6 | ~4 |
| toast | 1,363 | 7 | ~4 |
| slider | 1,338 | 4 | ~3 |
| form | 1,285 | 7 | ~4 |
| menubar | 881 | 17 | ~4 |
| popover | 838 | 7 | ~3 |
| hover_card | 736 | 5 | ~3 |
| dialog | 734 | 8+4 internal | ~4 |
| accordion | 636 | 5 | ~3 |
| context_menu | 635 | 16 | ~3 |
| dropdown_menu | 609 | 16 | ~3 |
| password_toggle_field | 532 | 3 | ~3 |

### Modules to Keep Single-File (15 modules)

toggle_group (411), tabs (406), checkbox (364), collapsible (305), alert_dialog (268), toolbar (250), switch (230), avatar (212), progress (206), toggle (73), separator (61), label (42), aspect_ratio (41), accessible_icon (26). radio_group (536, already split) — refine if needed.

### Split Pattern

**`mod.rs`** retains:
- All shared imports (crate utilities, leptos, web_sys, etc.)
- Context structs, constants, helper functions, enums, item data types
- `mod` declarations + `pub use subfile::*;` for each sub-file

**Each sub-file** contains:
- `use super::*;` to access shared items from mod.rs
- One (or a few tightly-coupled) `#[component]` function(s)

### Execution Order (smallest → largest, build confidence first)

1. password_toggle_field
2. accordion
3. dropdown_menu
4. context_menu
5. dialog
6. hover_card
7. popover
8. menubar
9. form
10. slider
11. toast
12. tooltip
13. one_time_password_field
14. scroll_area
15. navigation_menu
16. menu
17. select

Run `cargo clippy --all-features --locked` after each split to catch issues early.

## Phase 2: Bundle into `components/` and `support/`

### Target Structure

```
src/
├── lib.rs              # re-exports for backward compat
├── components/
│   ├── mod.rs          # feature-gated pub mod for each component
│   └── <33 component dirs>/
└── support/
    ├── mod.rs          # pub mod for each utility
    └── <22 utility dirs>/
```

### lib.rs re-exports (preserves public API)

```rust
mod support;
mod components;

// Re-export support at crate root
pub use support::announce;
pub use support::aria_hidden;
// ... all 22 support modules

// Re-export components at crate root
#[cfg(feature = "dialog")]
pub use components::dialog;
// ... all 33 components
```

### Import path updates

All `crate::<util>::` paths → `crate::support::<util>::` (full explicit paths).
All cross-component `crate::<component>::` paths → `crate::components::<component>::`.

### Execution

1. `mkdir -p src/support src/components`
2. `git mv` all 22 utility dirs to `src/support/`
3. `git mv` all 33 component dirs to `src/components/`
4. Create `src/support/mod.rs` and `src/components/mod.rs`
5. Update `src/lib.rs` with re-exports
6. Find-and-replace all `crate::` import paths in component + utility files
7. Verify

## Verification

After Phase 1 completion:
```bash
cargo clippy --all-features --locked
cargo fmt --all --check
just test_leptos_unit
just test_leptos_wasm
```

After Phase 2 completion (full suite):
```bash
cargo clippy --all-features --locked
cargo fmt --all --check
just test_leptos_all    # unit + wasm + e2e — ALL must pass
```
