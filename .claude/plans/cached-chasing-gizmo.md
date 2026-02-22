# Plan: Port Toast Component to Leptos

## Context

Toast is the next component in the topological porting order. All 10 dependencies are ported and story-tested. The React source is ~983 lines with 7 public sub-components plus 3 internal components. E2E tests (19/19) and a React reference app page already exist. HoverCard is the closest existing Leptos port for timer management patterns; Collection API is needed for tab order management.

## Steps

### 1. Create package scaffolding

Create `packages/primitives/leptos/toast/` with:
- `Cargo.toml` — dependencies modeled after hover-card's Cargo.toml
- `src/lib.rs` — re-exports
- `src/toast.rs` — main implementation file

Add to workspace members in root `Cargo.toml`.

Dependencies: `leptos`, `web-sys`, `wasm-bindgen`, `js-sys`, `send_wrapper`, `radix-leptos-primitive`, `radix-leptos-collection`, `radix-leptos-dismissable-layer`, `radix-leptos-portal`, `radix-leptos-presence`, `radix-leptos-visually-hidden`, `radix-leptos-use-controllable-state`, `radix-leptos-compose-refs`.

### 2. Implement types and enums

- `SwipeDirection` — `Up | Down | Left | Right` (default `Right`)
- `ToastType` — `Foreground | Background` (default `Foreground`)
- `Delta` — `{ x: f64, y: f64 }`
- `SwipeEvent` — `{ current_target, delta, original_event }`

### 3. Implement sub-components

**7 public components:**

1. **`ToastProvider`** — Wraps app area. Manages toast count, viewport ref, shared config (label, duration, swipe_direction, swipe_threshold). Wraps children in `CollectionProvider<()>`. Props: `label` (default "Notification"), `duration` (default 5000), `swipe_direction` (default Right), `swipe_threshold` (default 50).

2. **`ToastViewport`** — The `<ol>` container. Handles F8 hotkey focus, pause/resume on hover/focus via custom DOM events, custom tab management (reverse order, most-recent-first) using Collection API, FocusProxy elements at head/tail. Wrapped in `DismissableLayerBranch`. Props: `hotkey` (default `["F8"]`), `label` (default "Notifications ({hotkey})").

3. **`Toast`** (Root) — Controllable open state via `use_controllable_state` (default_open = true). Wraps `ToastImpl` in `Presence`. Composes swipe event handlers that set `data-swipe` attributes and CSS custom properties. Props: `open`, `default_open`, `on_open_change`, `force_mount`, `type`, `duration`, `on_escape_key_down`, `on_pause`, `on_resume`, `on_swipe_start/move/cancel/end`.

4. **`ToastTitle`** — `Primitive.div` wrapper.

5. **`ToastDescription`** — `Primitive.div` wrapper.

6. **`ToastAction`** — Wraps `ToastClose` in `ToastAnnounceExclude` with `alt_text` for screen readers. Props: `alt_text` (required).

7. **`ToastClose`** — Button that calls `on_close` from `ToastInteractiveContext`. Wrapped in `ToastAnnounceExclude`.

**3 internal components:**

8. **`ToastImpl`** — Core implementation. Manages:
   - Auto-close timers (start/pause/resume via `VIEWPORT_PAUSE`/`VIEWPORT_RESUME` custom events)
   - Swipe gesture detection (pointer capture, directional clamping, threshold-based dismiss)
   - Screen reader announcements via `ToastAnnounce`
   - Portal into viewport element
   - `DismissableLayer` for escape key
   - `CollectionItemSlot` for tab order tracking
   - Toast count tracking (on_toast_add/on_toast_remove)

9. **`ToastAnnounce`** — Temporary VisuallyHidden portal for SR announcements. Double-rAF delay before rendering text, self-removes after 1000ms.

10. **`FocusProxy`** — VisuallyHidden focusable elements at head/tail of viewport for tab focus in/out management.

11. **`ToastAnnounceExclude`** — Wrapper div with `data-radix-toast-announce-exclude` and optional `data-radix-toast-announce-alt`.

**Utility functions:**
- `get_announce_text_content` — Recursively extracts text from toast DOM, respecting exclude markers
- `get_tabbable_candidates` / `focus_first` — Tab management utilities
- `is_delta_in_direction` — Swipe direction check
- `set_timeout` / `clear_timeout` — Timer helpers (hover-card pattern)

### 4. Key implementation patterns

- **Timer management**: `StoredValue<Option<i32>>` for timer IDs + start/remaining time tracking. Same as hover-card.
- **Swipe gestures**: Pointer capture + delta tracking, custom DOM events for swipe lifecycle. `data-swipe` attributes and CSS custom properties set imperatively.
- **Portal into viewport**: Use `Portal` with `container` set to the viewport element from provider context. Re-provide `ToastInteractiveContext` and Collection contexts inside portal boundary.
- **Custom DOM events**: `toast.viewportPause`/`toast.viewportResume` for timer coordination. `addEventListener`/`removeEventListener` in Effects with cleanup.
- **Collection API**: `CollectionProvider<()>` in provider, `CollectionSlot` on viewport, `CollectionItemSlot` on each toast. `use_collection` for sorted item refs in tab management.
- **Mutable refs**: `StoredValue<T>` for non-reactive state (pointer position, swipe delta, timer handles, boolean flags).

### 5. Create stories

**File**: `stories/leptos/src/primitives/toast.rs`

Copy CSS module from React reference to `stories/leptos/src/primitives/toast.stories.module.css`.

Stories to implement (matching React): `Styled`, `Controlled`, `FromDialog`, `Promise`, `KeyChange`, `PauseResumeProps`, `Animated`, `Cypress`, `Chromatic`.

Wire into: `stories/leptos/src/primitives.rs`, `stories/leptos/src/app.rs`, `stories/leptos/Cargo.toml`.

### 6. Create Leptos reference app page

**File**: `reference_app/leptos/src/pages/toast.rs`

Match React page (`reference_app/react/src/pages/Toast.jsx`):
- ToastProvider with duration controlled by auto-dismiss checkbox
- "Add toast" button (`data-testid="add-toast"`)
- Toast with title, description, action (Undo), close (x)
- ToastViewport with `data-testid="toast-viewport"`
- Toast count display (`data-testid="toast-count"`)
- Outside button (`data-testid="outside-button"`)
- Shared CSS from `reference_app/shared/toast.css`

Wire into: `pages.rs`, `app.rs`, `Cargo.toml`, `index.html`.

### 7. Run E2E tests

- `just test_leptos_component toast` — must pass 19/19 without test modifications

### 8. Update research notes

Update `notes/leptos-toast.md` with Leptos API signatures, implementation notes, omissions, and key decisions. Set `ported: true` after E2E passes.

## Files to create

- `packages/primitives/leptos/toast/Cargo.toml`
- `packages/primitives/leptos/toast/src/lib.rs`
- `packages/primitives/leptos/toast/src/toast.rs`
- `stories/leptos/src/primitives/toast.rs`
- `stories/leptos/src/primitives/toast.stories.module.css`
- `reference_app/leptos/src/pages/toast.rs`

## Files to modify

- `Cargo.toml` (root) — add workspace member + dependency
- `stories/leptos/Cargo.toml` — add dependency
- `stories/leptos/src/primitives.rs` — add module
- `stories/leptos/src/app.rs` — add routes + nav
- `reference_app/leptos/Cargo.toml` — add dependency
- `reference_app/leptos/src/pages.rs` — add module
- `reference_app/leptos/src/app.rs` — add route
- `reference_app/leptos/index.html` — add shared CSS link
- `notes/leptos-toast.md` — update with implementation details

## Key reference files

- React source: `reference/react-radix-primitives/packages/react/toast/src/toast.tsx`
- React stories: `reference/react-radix-primitives/apps/storybook/stories/toast.stories.tsx`
- CSS module: `reference/react-radix-primitives/apps/storybook/stories/toast.stories.module.css`
- HoverCard (pattern reference): `packages/primitives/leptos/hover-card/src/hover_card.rs`
- Collection API: `packages/primitives/leptos/collection/src/collection.rs`
- Portal: `packages/primitives/leptos/portal/src/portal.rs`
- E2E tests: `reference_app/cypress/e2e/toast.cy.js`
- React reference page: `reference_app/react/src/pages/Toast.jsx`

## Verification

1. `cargo clippy -p radix-leptos-toast --all-features` — no warnings
2. `cargo fmt --all --check` — passes
3. Stories load and display correctly in Trunk dev server
4. `just test_leptos_component toast` — 19/19 pass without test modifications
