# Plan: Port Remaining Components, E2E Test, and Write Stories

## Context

5 components remain unported: **toast**, **select**, **context-menu**, **dropdown-menu**, and **menubar**. Toast and select are ready to port now (all deps complete). The other 3 are blocked on `leptos-menu` needing story verification. After all are ported and E2E-verified, stories will be written for visual testing.

## Execution Order

```
Phase 1: Port Toast                          [ready now]
Phase 2: Port Select                         [ready now, after toast]
Phase 3: Integrate Menu into workspace/stories [ready now]
   ── USER PAUSE: verify menu stories ──
Phase 4: Port Context Menu                   [after menu story-tested]
Phase 5: Port Dropdown Menu                  [after menu story-tested]
Phase 6: Port Menubar                        [after menu story-tested; write E2E + React page from scratch]
Phase 7: Write stories for all newly ported  [after each port completes]
```

---

## Phase 1: Port Toast

**React source**: `reference/react-radix-primitives/packages/react/toast/src/toast.tsx` (982 lines)
**Rough draft**: `reference/leptos-radix-rough-draft/src/primitives/toast.rs` (641 lines)
**Pattern reference**: `packages/primitives/leptos/hover-card/src/hover_card.rs` (timers, StoredValue)
**Existing plan details**: `.claude/plans/cached-chasing-gizmo.md`

### 1a. Package scaffolding
- Create `packages/primitives/leptos/toast/` with `Cargo.toml`, `src/lib.rs`, `src/toast.rs`
- Add to root `Cargo.toml` workspace members + dependencies
- Deps: leptos, web-sys, wasm-bindgen, js-sys, send_wrapper, radix-leptos-{primitive, collection, dismissable-layer, portal, presence, visually-hidden, use-controllable-state, compose-refs}

### 1b. Implement sub-components
7 public: `ToastProvider`, `ToastViewport`, `Toast`, `ToastTitle`, `ToastDescription`, `ToastAction`, `ToastClose`
3 internal: `ToastImpl`, `ToastAnnounce`/`ToastAnnounceExclude`, `FocusProxy`

Key patterns:
- Timer: `StoredValue<Option<i32>>` + `set_timeout`/`clear_timeout` (hover-card pattern)
- Custom DOM events: `toast.viewportPause`/`toast.viewportResume` for timer coordination
- Swipe gestures via pointer events with CSS variable output
- Collection API for tab order (most-recent-first)
- Portal into viewport element
- F8 hotkey to focus viewport

### 1c. Leptos reference app page
- Create `reference_app/leptos/src/pages/toast.rs` matching `reference_app/react/src/pages/Toast.jsx`
- Required test IDs: `add-toast`, `toast-viewport`, `toast-count`, `outside-button`
- Wire into `pages.rs`, `app.rs`, `Cargo.toml`; add `toast.css` link to `index.html`

### 1d. E2E test iteration
- Run `just test_leptos_component toast` — 19 tests must pass
- Iterate on implementation until all pass

### 1e. Update `notes/leptos-toast.md` — set `ported: true`

---

## Phase 2: Port Select

**React source**: `reference/react-radix-primitives/packages/react/select/src/select.tsx` (1841 lines — largest primitive)

### 2a. Package scaffolding
- Create `packages/primitives/leptos/select/`
- Deps: core-number, core-primitive, leptos-{collection, compose-refs, direction, dismissable-layer, focus-guards, focus-scope, id, popper, portal, primitive, slot, use-callback-ref, use-controllable-state, use-previous, visually-hidden}
- Ensure all deps are in workspace members (some like `core-number`, `leptos-slot`, `leptos-use-callback-ref`, `leptos-use-previous` may need adding)

### 2b. Implement 16+ sub-components
`Select`, `SelectTrigger`, `SelectValue`, `SelectIcon`, `SelectPortal`, `SelectContent`, `SelectViewport`, `SelectGroup`, `SelectLabel`, `SelectItem`, `SelectItemText`, `SelectItemIndicator`, `SelectScrollUpButton`, `SelectScrollDownButton`, `SelectSeparator`, `SelectArrow`

Key challenges:
- Two positioning modes: item-aligned (complex viewport measurement) and popper (Floating UI)
- Typeahead search with 1s buffer timeout
- Scroll buttons with auto-scroll on hover
- Hidden native `<select>` for form submission
- Pointer move delta tracking for accidental selection prevention

### 2c. Leptos reference app page
- Create `reference_app/leptos/src/pages/select.rs` matching `reference_app/react/src/pages/Select.jsx`

### 2d. E2E: 35 tests must pass

### 2e. Update `notes/leptos-select.md`

---

## Phase 3: Unblock Menu Stories

Menu is ported (`packages/primitives/leptos/menu/src/menu.rs`, 1062 lines) but the package and story are **excluded from workspace members**. Context-menu, dropdown-menu, and menubar all depend on menu being story-tested.

### 3a. Add menu package to workspace
- Add `packages/primitives/leptos/menu` to root `Cargo.toml` workspace members + deps
- Verify compilation: `cargo clippy -p radix-leptos-menu --all-features`

### 3b. Enable menu story
- Uncomment `pub mod menu;` in `stories/leptos/src/primitives.rs`
- Uncomment dep + route entries in stories `Cargo.toml` and `app.rs`
- Verify story compiles

### 3c. **USER ACTION**: Visually verify menu stories against React reference, then set `tested_story: true` in `notes/leptos-menu.md`

---

## Phase 4: Port Context Menu (after menu story-tested)

**React source**: `reference/react-radix-primitives/packages/react/context-menu/src/context-menu.tsx` (601 lines — wrapper over Menu)

- 16 sub-components (most are pass-throughs to Menu)
- Unique logic: contextmenu event handling, virtual anchor at pointer, 700ms long-press for touch
- Leptos ref app page matching `reference_app/react/src/pages/ContextMenu.jsx`
- E2E: 27 tests

---

## Phase 5: Port Dropdown Menu (after menu story-tested)

**React source**: `reference/react-radix-primitives/packages/react/dropdown-menu/src/dropdown-menu.tsx` (566 lines — thin wrapper over Menu)

- 16 sub-components (direct delegates to Menu)
- Unique: trigger toggle, Space/Enter/ArrowDown to open, aria-haspopup/expanded/controls
- Leptos ref app page matching `reference_app/react/src/pages/DropdownMenu.jsx`
- E2E: 39 tests

---

## Phase 6: Port Menubar (after menu story-tested)

**React source**: `reference/react-radix-primitives/packages/react/menubar/src/menubar.tsx` (765 lines)

**Unlike other phases, menubar has NO existing E2E test or React reference page.**

### 6a. Read Radix docs at `https://www.radix-ui.com/primitives/docs/components/menubar`
### 6b. Write Cypress test at `reference_app/cypress/e2e/menubar.cy.js` (following E2E Testing Methodology in CLAUDE.md)
### 6c. Create React reference page at `reference_app/react/src/pages/Menubar.jsx` + validate tests against React
### 6d. Port the component (17 sub-components) + create Leptos page + run E2E

---

## Phase 7: Write Stories

For each newly ported component, create stories mirroring React storybook:
1. Copy CSS module from `reference/react-radix-primitives/apps/storybook/stories/<component>.stories.module.css`
2. Create story file using `stylance::import_crate_style!`
3. Wire into `stories/leptos/src/primitives.rs`, `app.rs`, `Cargo.toml`

Components needing stories: **toast, select, context-menu, dropdown-menu, menubar**

Also: integrate avatar (needs API migration to Leptos 0.8 first) and verify OTP field stories.

---

## Verification

After all phases:
- `cargo clippy --all-features --locked` passes
- `cargo fmt --all --check` passes
- E2E passes for all 5 new components
- Stories compile and render: `cd stories/leptos && trunk serve`
- Research notes updated with `ported: true`
