# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Rust port of [Radix UI](https://www.radix-ui.com) — a component library for building accessible UI. Part of the [Rust for Web](https://github.com/RustForWeb) initiative. Implements components across three Rust web frameworks: **Leptos**, **Yew**, and **Dioxus**.

The project targets `wasm32-unknown-unknown` and uses Rust edition 2024.

## Build & Development Commands

```bash
# Lint (CI runs with RUSTFLAGS='-Dwarnings')
cargo clippy --all-features --locked

# Format
cargo fmt --all
cargo fmt --all --check       # check only

# Test
cargo test --all-features --locked --release

# Dependency checks
cargo deny check              # security/license audit
cargo machete                 # unused dependencies
cargo sort --workspace --check # manifest ordering
```

To run a single package's tests: `cargo test -p <package-name> --all-features --locked --release`

## Architecture

### Workspace Layout

The Cargo workspace is a monorepo with four main areas under `packages/`:

- **`colors/`** — Framework-agnostic color constants (generated from Radix Colors). Modules: `light`, `dark`, `black_a`, `white_a`.
- **`icons/{leptos,yew,dioxus}/`** — SVG icon components generated per-framework from `@radix-ui/react-icons`.
- **`primitives/`** — The main component library:
  - `core/` — Framework-agnostic utilities (`number`, `rect`, `primitive`)
  - `leptos/` — Leptos implementations (partially available; Leptos 0.7→0.8 migration in progress, many packages commented out in workspace members)
  - `yew/` — Yew implementations (fully available)
  - Dioxus primitives are planned but not yet implemented
- **`themes/{yew,leptos}/`** — Higher-level themed component wrappers (Yew has the most progress)

### Package Naming Convention

Packages follow the pattern `radix-{framework}-{component}`, e.g., `radix-yew-checkbox`, `radix-leptos-label`.

### Supporting Directories

- **`scripts/`** — Code generation for colors and icons (scrapes upstream Radix repos)
- **`book/`** — mdBook documentation site
- **`book-examples/`** — Working code examples used in documentation
- **`stories/`** — Storybook-like component demos per framework (uses Trunk dev server + Tailwind CSS)
- **`reference/`** — Reference implementations for porting:
  - `react-radix-primitives/` — Git submodule of the original React Radix Primitives (primary reference)
  - `leptos-radix-rough-draft/` — A previous Leptos implementation of these primitives. Not API-strict with React, but useful as a reference for how patterns were solved in Leptos. **Use for implementation hints only; always follow the React API per Rule 4.**
- **`reference_app/`** — Cross-framework E2E testing harness (see **Reference App Rules** below):
  - `react/` — React app using real `@radix-ui/react-*` packages (source of truth)
  - `leptos/` — Leptos app using `radix-leptos-*` packages (must match React behavior)
  - `cypress/` — Shared Cypress E2E tests that run against either app

### Key Dependencies

- Yew uses a **custom fork** (`RustForWeb/yew`, branch `feature/use-composed-ref`) patched in `[patch.crates-io]`
- `tailwind_fuse` with variant support for CSS class management
- `leptos-node-ref`, `leptos-style`, `leptos-maybe-callback` for Leptos component patterns

## Porting Focus

We are currently only porting to **Leptos**. Yew and Dioxus are out of scope. Research notes for framework-specific primitives should use "Leptos API" / "Leptos Implementation Notes" headings.

## Porting Rules

### Rule 1: Document Research Before Porting

Before porting a component, create a research note at `notes/<package-name>.md` (e.g., `notes/core-number.md`, `notes/leptos-checkbox.md`). Read the React source in `reference/react-radix-primitives/` and the existing Rust implementation (if any) to fill it out.

**Template:**

```markdown
---
react_location: "[[reference/react-radix-primitives/packages/.../src/source-file.ts|display-name]]"
rust_location: "[[packages/primitives/.../src/source_file.rs|display_name]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/component.stories.tsx|component]]"
rust_story: "[[stories/leptos/src/primitives/component.rs|component]]"
dependencies: []
ported: false
tested: false
tested_story: false
unstable: false
---
## Intent

What the component/utility does and why it exists.

## React API

Function signatures and exports.

## {Rust|Leptos|Yew} API

Equivalent Rust signatures. Use "Rust API" for framework-agnostic `core/` packages.
Use "Leptos API", "Yew API", etc. for framework-specific packages.

## React Implementation Notes

Key implementation details, patterns, edge cases.

## {Rust|Leptos|Yew} Implementation Notes

How the Rust port differs, idiomatic adaptations, missing coverage.
Use "Rust Implementation Notes" for `core/` packages.
Use "Leptos Implementation Notes", "Yew Implementation Notes", etc. for framework-specific packages.
```

**Metadata fields:**
- `react_location` / `rust_location` — Obsidian wikilinks using full repo-relative paths with a display alias (e.g., `"[[reference/react-radix-primitives/packages/core/number/src/number.ts|number]]"`)
- `react_story` / `rust_story` — Obsidian wikilinks to the React storybook file and Leptos story file, respectively. Use `""` (empty string) if no story exists for this component.
- `dependencies` — list of internal radix packages this component depends on; `[]` if none
- `ported` — `true` if the Rust implementation exists and matches the React source
- `tested` — `true` if tests exist with reasonable coverage
- `tested_story` — `false` by default; set to `true` only by the user after they have manually verified the story's functionality against the React reference
- `unstable` — `true` if the React source marks the component as unstable/preview; these are deferred per Rule 2

**Content guidelines:**
- Notes should be complete but not overly verbose
- Focus on what matters for porting: intent, API shape, non-obvious implementation details
- Call out missing test coverage or known divergences from the React source

### Rule 2: Follow the Dependency Graph

Use `scripts/topo_sort.py` to determine porting order. Run it to see all components sorted topologically with their ported and story-tested status. A dependency is considered **complete** when it is `ported: true` AND its story has been tested (`tested_story: true`), or it has no story (`react_story: ""`). The next item to port is the first entry where `ported` is `false` and all of its dependencies are complete. **Never port a component before all its dependencies are both ported and story-tested (when applicable).**

**Unstable/preview items last:** Components marked as `unstable` in the React source (preview APIs, experimental features) should be deferred until all stable components are ported and story-tested. When determining the next component to port, skip any item with `unstable: true` in its research note unless no stable items remain.

```bash
python3 scripts/topo_sort.py
```

### Rule 3: Reference React Only, Never Yew

When porting, reference the React source in `reference/react-radix-primitives/` and the existing Leptos code. **Never consult the Yew implementation** — it may contain stale patterns or divergences that should not propagate. The inputs are:

1. React source (primary): `reference/react-radix-primitives/packages/<area>/<component>/src/`
2. Existing Leptos code (if any): `packages/primitives/leptos/<component>/src/`
3. Rough draft Leptos reference (optional): `reference/leptos-radix-rough-draft/src/primitives/<component>.rs` — A prior Leptos implementation that may not match the React API exactly, but can provide useful implementation hints for Leptos-specific patterns. Use for guidance, not as an authoritative API source.

### Rule 4: Follow the React API, Write Idiomatic Leptos

- Match the **public React API** — same component names, same prop names (converted to snake_case), same compositional structure.
- **Structure the Rust code similarly to the React source** for maintainability, ease of review, and future syncs with upstream changes.
- Use **idiomatic Leptos patterns** for the implementation: signals, `#[component]` functions, `view!` macros, context providers, `NodeRef`, etc. Do not transliterate React hooks 1:1 when a cleaner Leptos idiom exists.

### Rule 5: Verify with Stories

Create a Leptos story in `stories/leptos/src/primitives/<component>.rs` that closely mirrors the React reference stories at `reference/react-radix-primitives/apps/storybook/stories/<component>.stories.tsx`.

- Wire the new story module into `stories/leptos/src/primitives.rs`, `stories/leptos/src/app.rs` (router + nav), and `stories/leptos/Cargo.toml`.
- **Copy the CSS module from the React reference directly.** Copy `reference/react-radix-primitives/apps/storybook/stories/<component>.stories.module.css` to `stories/leptos/src/primitives/<component>.stories.module.css`. The file must be an exact copy — **never modify or "improve" the borrowed styles.** Any required CSS variables (e.g., Radix Colors) that the module references must be available at runtime. New color packages should be added as `<link data-trunk rel="css" href="/node_modules/@radix-ui/colors/<color>.css">` entries in `stories/leptos/index.html` — this is how Trunk loads them into the build. Do **not** add colors via `@import` in `preview.css`.
- **Use stylance to import CSS modules.** In the Rust story file, import the CSS module classes using `stylance::import_crate_style!`:
  ```rust
  stylance::import_crate_style!(classes, "src/primitives/<component>.stories.module.css");
  ```
  Then reference classes via `attr:class=classes::root`, `attr:class=classes::trigger`, etc. See `stories/leptos/src/primitives/accordion.rs` and its corresponding `accordion.stories.module.css` for the reference pattern.
- Each exported story in the React file (e.g., `Styled`, `Controlled`, `Chromatic`) should have a corresponding Leptos `#[component]` function.

### Rule 6: Document Omissions and Decisions

During porting, two things must be documented in the research note's **Leptos Implementation Notes** section:

1. **Omissions** — If a piece of React code is intentionally left out because it doesn't apply to Leptos (e.g., a React-specific reconciliation workaround, an `effectEvent` shim, a ref-forwarding pattern handled differently), document *what* was omitted and *why*.
2. **Assumptions and key decisions** — Any non-obvious choice made during the port (e.g., choosing `RwSignal` vs `ReadSignal`/`WriteSignal`, how context is structured, how a React callback pattern was adapted). These notes help reviewers and future contributors understand intent.

### Rule 7: Definition of Done

A port is complete when:

1. **All public API** from the React source is implemented in Leptos (same components, same props, same behavior).
2. **Any omitted functionality** is documented per Rule 6 and explicitly approved by the user.
3. **Stories** exist per Rule 5 that cover the same scenarios as the React reference.
4. The research note's `ported` field is set to `true` only after the above are satisfied.

Do not mark a component as ported if functionality is missing without documented justification and user approval.

### Rule 8: Escape Key UX in Dismissable Layers

Components that use `DismissableLayer` (dialogs, popovers, dropdowns, etc.) must implement the **"two escapes" UX pattern**: when a text-editing element (text input, textarea, contenteditable) inside the layer has focus, the first Escape press moves focus to the layer container rather than dismissing. The second Escape dismisses the layer.

This behavior is implemented in `DismissableLayer`'s escape handler via `is_text_input()`. When porting new components that compose `DismissableLayer`, verify this works correctly with the component's stories — especially stories that contain form inputs. If a component manages its own escape handling outside of `DismissableLayer`, it must implement the same pattern.

## Known Dependency Issues

### floating-ui-leptos / floating-ui-core v0.6.0

The Rust port of floating-ui has bugs that diverge from the JavaScript original. When debugging positioning issues, **compare the Rust source in `~/.cargo/registry/src/` against the JS source on GitHub** (`https://github.com/floating-ui/floating-ui`) line-by-line before assuming the library works correctly.

Known bugs and workarounds:

1. **LimitShift cross_axis uses wrong dimension** — In `floating-ui-core`'s `LimitShift` limiter (`shift.rs`), the `check_cross_axis` block uses `main_axis.length()` instead of `cross_axis.length()`. This causes incorrect clamping — e.g., for `Side::Right`, it constrains the X coordinate using height values instead of width values. **Workaround:** When configuring `LimitShift`, always pass `.cross_axis(false)` since `Shift` itself uses `cross_axis: false`, making `LimitShift`'s cross-axis check unnecessary.

### Debugging strategy for third-party Rust WASM crate issues

When a component's visual output is wrong and the cause is unclear:

1. **Check the rough-draft reference first** (`reference/leptos-radix-rough-draft/`) to see how a working Leptos implementation solved the same problem. This narrows whether the issue is in our code or in a dependency.
2. **Read the upstream Rust crate source** in `~/.cargo/registry/src/` and diff it against the original JS. The Rust ports of JS libraries may have subtle translation bugs.
3. **Avoid prolonged printf-debugging in the browser.** If adding log statements to middleware pipelines doesn't yield a clear answer within a couple of iterations, pivot to source-level comparison of the Rust port vs. the JS original.

## Reference App Rules

The `reference_app/` directory contains a cross-framework E2E testing harness: a React app and a Leptos app that implement identical pages for each Radix primitive, validated by a shared Cypress test suite. This proves behavioral parity between the original React Radix and our Leptos port.

### Directory Layout

```
reference_app/
├── react/                    # React app (Vite + React)
│   ├── src/
│   │   ├── pages/            # One file per component
│   │   │   ├── Dialog.jsx
│   │   │   ├── Form.jsx
│   │   │   └── ...
│   │   ├── App.jsx           # Router with routes for each page
│   │   └── main.jsx          # Entry point
│   ├── package.json
│   ├── vite.config.js
│   └── index.html
├── leptos/                   # Leptos app (Trunk)
│   ├── src/
│   │   ├── pages/            # One file per component
│   │   │   ├── dialog.rs
│   │   │   ├── form.rs
│   │   │   └── ...
│   │   ├── pages.rs          # Module index
│   │   ├── app.rs            # Router with routes for each page
│   │   └── main.rs           # Entry point
│   ├── Cargo.toml
│   ├── index.html
│   └── Trunk.toml
├── cypress/                  # Shared Cypress E2E tests
│   ├── e2e/
│   │   ├── dialog.cy.js
│   │   ├── form.cy.js
│   │   └── ...
│   └── support/
│       ├── commands.js
│       └── e2e.js
├── cypress.config.js         # Cypress config (baseUrl swapped per target)
└── package.json              # Shared test dependencies
```

### RA Rule 1: Identical Routes, Identical Semantics

Both apps must expose the **same routes** with the **same DOM semantics**. Cypress tests are framework-agnostic — they interact with the DOM, not the implementation.

- **Routes**: `/<component>` (e.g., `/dialog`, `/form`). If a component needs multiple scenarios, use `/<component>/<scenario>` (e.g., `/dialog/modal`, `/dialog/non-modal`).
- **Text content**: Buttons, labels, headings, and other visible text must be identical across both apps. Cypress tests locate elements by text (`cy.findByText`), label (`cy.findByLabelText`), placeholder (`cy.findByPlaceholderText`), and role.
- **ARIA attributes**: Both apps must produce equivalent `aria-*` attributes, roles, and `data-state` attributes since tests assert on these.
- **`data-testid`**: Use `data-testid` attributes when text/aria selectors are insufficient. Both apps must use the same `data-testid` values.
- **Control elements**: Test fixtures (checkboxes to toggle state, reset buttons, output displays) must use identical labels and text across both apps.

### RA Rule 2: Docs-Driven TDD — Tests First, React Validates, Leptos Must Pass

The **Radix documentation** is the source of truth for what to test. The **React app** (using real `@radix-ui/react-*` packages) is the oracle that validates test correctness. Follow this order:

1. **Write the Cypress test first**, derived from the Radix docs. The docs' data-attribute tables, Keyboard Interactions table, and Accessibility section define every assertion. See the **E2E Testing Methodology** section below for the full process.
2. **Write the React page second** — a minimal test fixture with variant toggles and outside interaction targets.
3. **Run the tests against React to validate them.** This catches cases where the docs are ambiguous or underspecified (exact tab order, what gets auto-focused, animation timing). Fix tests only if they assert something the docs don't actually require; add tests for undocumented behaviors discovered during validation.
4. **Write the Leptos page last**, making it produce equivalent DOM output.
5. **Run the same Cypress test against Leptos.** It must pass without modification.

Writing tests from the docs (not from the React implementation) prevents tests from just confirming "what React happens to do" instead of "what the spec says should happen."

If a Cypress test needs to be modified to pass against Leptos, that indicates a bug in the Leptos implementation, not a problem with the test. The only exception is timing — Leptos WASM apps may need slightly longer waits, which should be handled via Cypress retry-ability (`.should()` assertions) rather than hard-coded waits.

### RA Rule 3: Keep Pages Minimal and Focused

Reference app pages are **test fixtures**, not demos or showcases. They exist solely to exercise component behavior for Cypress.

- Each page should test the **core interactive behavior** of one primitive: open/close, focus management, keyboard navigation, form validation, accessibility attributes, etc.
- Include **control elements** (checkboxes, buttons) to toggle component state for different test scenarios, matching the pattern used in `stories/leptos/src/primitives/dialog.rs` (the `Cypress` story).
- Do **not** add visual polish, multiple variants, or showcase-style layouts. The existing `stories/` directory serves that purpose.
- Keep the React implementation simple and idiomatic — no abstractions, no shared utilities. Each page should be self-contained.

### RA Rule 4: Shared CSS

Both apps must use the **same CSS** for component styling within test pages, so layout-dependent behavior (scroll, overflow, overlay clicks) is identical.

- Place shared CSS files in `reference_app/shared/` (e.g., `reference_app/shared/dialog.css`).
- Both apps import from this shared location. React uses standard CSS imports; Leptos uses `<link data-trunk rel="css" ...>` in `index.html` or inline styles.
- Keep styles minimal — just enough to make interactive behavior testable (e.g., overlays need some opacity/positioning so pointer-outside-dismiss works; scrollable content needs a height).

### RA Rule 5: Running the Apps

Both apps must serve on `http://localhost:3000` (configurable via environment variable). Only one app runs at a time.

```bash
# React
cd reference_app/react && npm run dev          # Vite dev server on :3000

# Leptos
cd reference_app/leptos && trunk serve --port 3000

# Cypress (in a separate terminal, against whichever app is running)
cd reference_app && npx cypress open
cd reference_app && npx cypress run
cd reference_app && npx cypress run --spec cypress/e2e/dialog.cy.js
```

Add npm scripts in `reference_app/package.json`:

```json
{
    "scripts": {
        "react:dev": "cd react && npm run dev",
        "leptos:dev": "cd leptos && trunk serve --port 3000",
        "cy:open": "cypress open",
        "cy:run": "cypress run",
        "cy:run:react": "start-server-and-test react:dev http://localhost:3000 cy:run",
        "cy:run:leptos": "start-server-and-test leptos:dev http://localhost:3000 cy:run"
    }
}
```

### RA Rule 6: Cypress Test Conventions

Tests must be framework-agnostic and follow these conventions:

- **Use `@testing-library/cypress` queries** (`findByText`, `findByLabelText`, `findByRole`, `findByPlaceholderText`, `findByTestId`) for element selection. Never use framework-specific selectors (React component names, Leptos class paths, etc.).
- **Use `cypress-real-events`** (`realPress`, `realClick`, `realType`, `realTouch`) for interactions that must match real browser behavior (focus, keyboard events).
- **Never use hard-coded waits** (`cy.wait()`). Rely on Cypress's built-in retry-ability via `.should()` assertions.
- **Reset state explicitly.** Since `testIsolation: false` is used for performance, each test must reset to a known state in `beforeEach` (e.g., clicking a reset button, revisiting the page).
- **File naming**: `cypress/e2e/<component>.cy.js` — lowercase, matching the route name.
- **Structure**: One `describe('<Component>')` block per file, with nested `describe` blocks for scenarios and `it` blocks for individual assertions.

### RA Rule 7: Adding a New Component

When adding a new component to the reference apps:

1. **Check that the primitive is ported.** Only add components whose research note has `ported: true`.
2. **Read the Radix docs** at `https://www.radix-ui.com/primitives/docs/components/<component>`. Extract data-attribute tables, Keyboard Interactions table, and Accessibility section.
3. **Write the Cypress test first** at `reference_app/cypress/e2e/<component>.cy.js`, following the E2E Testing Methodology below. Every documented behavior becomes a test assertion.
4. **Create the React page** at `reference_app/react/src/pages/<Component>.jsx`. Use `@radix-ui/react-<component>` and write a minimal test fixture with variant toggles and outside interaction targets.
5. **Add shared CSS** at `reference_app/shared/<component>.css` if needed.
6. **Add the route** in `reference_app/react/src/App.jsx`.
7. **Run the Cypress tests against the React app.** Fix tests only if they assert something the docs don't actually require; add tests for undocumented behaviors discovered during validation.
8. **Create the Leptos page** at `reference_app/leptos/src/pages/<component>.rs`. Wire it into `pages.rs`, `app.rs`, and `Cargo.toml`. Add the route.
9. **Run the same Cypress test against the Leptos app.** It must pass without test modifications.
10. **Update the research note** — set `tested_e2e: true` once the Cypress suite passes against both apps.

### RA Rule 8: What NOT to Test Here

The reference app Cypress tests focus on **behavioral parity** — proving the Leptos port behaves identically to React. They do **not** replace:

- **Unit tests** (`cargo test`) for internal logic.
- **Story-based manual testing** (`stories/`) for visual fidelity and edge-case demos.
- **Accessibility audits** beyond what ARIA attribute assertions cover.

Keep Cypress tests focused on interactive behavior: open/close, focus management, keyboard navigation, form submission, dismiss-on-outside-click, state attributes, etc.

### E2E Testing Methodology

Each Cypress test file must provide **comprehensive behavioral coverage** of a Radix primitive, derived directly from the React Radix documentation and source code. The Radix docs at `https://www.radix-ui.com/primitives/docs/components/<component>` define the contract. Every documented behavior is a test case.

#### Step 1: Read the Radix Documentation

Before writing any test, read the component's Radix documentation page. Extract the following:

1. **API Reference data-attribute tables** — Each sub-component documents its data attributes (e.g., `[data-state]`, `[data-disabled]`, `[data-orientation]`, `[data-side]`, `[data-align]`). These are the component's public contract for state visibility.
2. **Accessibility section** — Identifies the WAI-ARIA design pattern the component adheres to and any specific ARIA roles, properties, or states it must expose.
3. **Keyboard Interactions table** — Lists every key and its expected behavior. This is the definitive spec for keyboard tests.

Also read the React source at `reference/react-radix-primitives/packages/react/<component>/src/` to understand behaviors not fully described in the docs (edge cases, focus management, dismiss behavior, etc.).

#### Step 2: Structure the Test File

Every test file follows this structure, organized by **test category**:

```javascript
describe('<Component>', () => {
    // ── Helpers ──────────────────────────────────────────────
    // Reusable assertion functions (shouldBeOpen, shouldBeClosed, etc.)

    // ── 1. Accessibility Semantics ──────────────────────────
    describe('accessibility', () => {
        // ARIA roles, properties, label associations
    });

    // ── 2. Data Attributes ──────────────────────────────────
    describe('data attributes', () => {
        // data-state, data-disabled, data-orientation, etc.
    });

    // ── 3. Keyboard Navigation ──────────────────────────────
    describe('keyboard navigation', () => {
        // Every key from the Keyboard Interactions table
    });

    // ── 4. Pointer Interaction ──────────────────────────────
    describe('pointer interaction', () => {
        // Click, touch, outside-click dismiss, etc.
    });

    // ── 5. Focus Management ─────────────────────────────────
    describe('focus management', () => {
        // Focus trap, focus restore, auto-focus, tab order
    });

    // ── 6. Variants ─────────────────────────────────────────
    describe('variants', () => {
        // Controlled/uncontrolled, modal/non-modal, single/multiple,
        // disabled, orientation, collapsible, etc.
    });
});
```

Not every component will have all categories. Omit empty categories — but **never omit a category that the docs specify behavior for.**

#### Step 3: Cover Every Test Category

##### 3a. Accessibility Semantics

Test every ARIA attribute and role documented for the component. The Radix docs reference specific WAI-ARIA design patterns — the test must verify compliance.

```javascript
// Roles
cy.findByRole('dialog').should('exist');
cy.findByRole('button', {name: 'open'}).should('exist');

// ARIA properties and states
cy.findByRole('button', {name: 'open'}).should('have.attr', 'aria-haspopup', 'dialog');
cy.findByRole('button', {name: 'open'}).should('have.attr', 'aria-expanded', 'false');

// Label associations (aria-labelledby, aria-describedby)
cy.findByRole('dialog').should('have.attr', 'aria-labelledby');
cy.findByRole('dialog')
    .invoke('attr', 'aria-labelledby')
    .then((labelId) => {
        cy.get(`#${labelId}`).should('have.text', 'Title');
    });

cy.findByRole('dialog').should('have.attr', 'aria-describedby');
cy.findByRole('dialog')
    .invoke('attr', 'aria-describedby')
    .then((descId) => {
        cy.get(`#${descId}`).should('have.text', 'Description');
    });
```

##### 3b. Data Attributes

Test every `data-*` attribute listed in the component's API Reference tables. Assert both the attribute's presence and its value transitions through all documented states.

```javascript
// data-state transitions
cy.findByRole('button', {name: 'open'}).should('have.attr', 'data-state', 'closed');
cy.findByRole('button', {name: 'open'}).click();
cy.findByRole('button', {name: 'open'}).should('have.attr', 'data-state', 'open');

// data-disabled (boolean attribute — present or absent)
cy.findByRole('button', {name: 'item'}).should('not.have.attr', 'data-disabled');
// after disabling:
cy.findByRole('button', {name: 'item'}).should('have.attr', 'data-disabled');

// data-orientation
cy.get('[data-orientation]').should('have.attr', 'data-orientation', 'vertical');

// Positioned content (popover, tooltip, dropdown)
cy.findByRole('dialog').should('have.attr', 'data-side', 'bottom');
cy.findByRole('dialog').should('have.attr', 'data-align', 'center');
```

Every sub-component's data attributes must be tested independently. If the docs list `data-state` on Trigger, Overlay, and Content — test all three.

##### 3c. Keyboard Navigation

Test **every row** in the component's Keyboard Interactions table from the docs. Each key listed must have at least one test assertion.

```javascript
// From the Dialog keyboard table:
// Space → Opens/closes the dialog
it('Space opens and closes', () => {
    cy.findByRole('button', {name: 'open'}).focus();
    cy.realPress('Space');
    shouldBeOpen();
    cy.findByRole('button', {name: 'close'}).focus();
    cy.realPress('Space');
    shouldBeClosed();
});

// Enter → Opens/closes the dialog
it('Enter opens and closes', () => {
    cy.findByRole('button', {name: 'open'}).focus();
    cy.realPress('Enter');
    shouldBeOpen();
});

// Esc → Closes the dialog, moves focus to Trigger
it('Escape closes and restores focus', () => {
    cy.findByRole('button', {name: 'open'}).click();
    shouldBeOpen();
    cy.realPress('Escape');
    shouldBeClosed();
    cy.findByRole('button', {name: 'open'}).should('be.focused');
});

// Tab / Shift+Tab → Focus navigation
it('Tab cycles through focusable elements', () => {
    cy.findByRole('button', {name: 'open'}).click();
    cy.realPress('Tab');
    // assert next focusable element is focused
    cy.realPress(['Shift', 'Tab']);
    // assert previous element is focused
});

// Arrow keys (for roving-focus components like Accordion, Tabs, RadioGroup):
it('ArrowDown moves to next trigger', () => {
    cy.findByRole('button', {name: 'Item 1'}).focus();
    cy.realPress('ArrowDown');
    cy.findByRole('button', {name: 'Item 2'}).should('be.focused');
});

// Home/End (for roving-focus components):
it('Home moves focus to first trigger', () => {
    cy.findByRole('button', {name: 'Item 3'}).focus();
    cy.realPress('Home');
    cy.findByRole('button', {name: 'Item 1'}).should('be.focused');
});
```

##### 3d. Pointer Interaction

Test mouse click and touch interactions. For dismissable components, test both inside and outside interactions.

```javascript
// Open/close with pointer
it('click opens and closes', () => {
    cy.findByRole('button', {name: 'open'}).click();
    shouldBeOpen();
    cy.findByRole('button', {name: 'close'}).click();
    shouldBeClosed();
});

// Outside click dismiss (modal vs non-modal behave differently)
it('clicking outside closes', () => {
    cy.findByRole('button', {name: 'open'}).click();
    shouldBeOpen();
    cy.get('body').click(0, 0);
    shouldBeClosed();
});

// Touch interactions
it('touch outside closes', () => {
    cy.findByRole('button', {name: 'open'}).click();
    shouldBeOpen();
    cy.findByTestId('outside-element').realTouch();
    shouldBeClosed();
});
```

##### 3e. Focus Management

Test focus trapping, focus restoration, and auto-focus behavior.

```javascript
// Auto-focus on open
it('focuses first focusable element on open', () => {
    cy.findByRole('button', {name: 'open'}).click();
    cy.findByRole('button', {name: 'close'}).should('be.focused');
});

// Focus restore on close
it('restores focus to trigger on close', () => {
    cy.findByRole('button', {name: 'open'}).click();
    cy.realPress('Escape');
    cy.findByRole('button', {name: 'open'}).should('be.focused');
});

// Focus trap (modal dialogs)
it('traps focus within dialog', () => {
    cy.findByRole('button', {name: 'open'}).click();
    // Tab to last element, then Tab should wrap to first
    cy.realPress('Tab');
    cy.realPress('Tab');
    cy.realPress('Tab');
    cy.findByRole('button', {name: 'close'}).should('be.focused');
});

// Focus trap with removed elements
it('handles focus when focused element is removed', () => {
    cy.findByRole('button', {name: 'open'}).click();
    cy.findByText('destroy me').click(); // removes itself
    cy.realPress('Tab');
    cy.findByRole('button', {name: 'close'}).should('be.focused');
});
```

##### 3f. Variants

Each component variant that changes behavior must be tested independently. Common variants:

| Variant | Components | What changes |
|---------|-----------|-------------|
| `modal` / non-modal | Dialog, AlertDialog | Focus trap, outside interaction blocking, scroll lock |
| `controlled` / `uncontrolled` | All stateful components | State is driven by props vs internal |
| `disabled` | Accordion, Checkbox, RadioGroup, etc. | Prevents interaction, sets `data-disabled` |
| `orientation` (`vertical`/`horizontal`) | Accordion, Tabs, Slider, etc. | Arrow key direction, `data-orientation` |
| `type` (`single`/`multiple`) | Accordion | One vs many items open |
| `collapsible` | Accordion (`single` type) | Whether last open item can be closed |
| `required` | Checkbox, RadioGroup, Select | Form validation behavior |
| `dir` (`ltr`/`rtl`) | Components with directional arrow keys | Arrow key mapping flips |

For each variant that the component supports:
1. Set up the variant (via control toggle or route parameter).
2. Re-test the behaviors that the variant affects — do not just test that the attribute is present.

```javascript
describe('non-modal variant', () => {
    beforeEach(() => {
        cy.findByLabelText('modal').click(); // toggle to non-modal
    });

    it('does not trap focus', () => {
        cy.findByRole('button', {name: 'open'}).click();
        // Tab should leave the dialog
        cy.realPress('Tab');
        cy.realPress('Tab');
        cy.findByTestId('outside-input').should('be.focused');
    });

    it('allows outside interaction', () => {
        cy.findByRole('button', {name: 'open'}).click();
        cy.findByTestId('outside-button').click();
        // outside button should receive the click (not blocked)
    });
});
```

#### Step 4: Test Page Requirements

The React and Leptos pages must provide sufficient DOM structure to exercise all the above categories. Each page needs:

1. **The component** in its default configuration.
2. **Variant toggles** — labeled checkboxes/radio buttons for each behavioral variant (e.g., `modal`, `disabled`, `orientation`). Use `<label>` wrapping `<input type="checkbox">` so `cy.findByLabelText()` works.
3. **Outside interaction targets** — elements outside the component (buttons, inputs) with stable labels, for testing dismiss-on-outside-click and focus escape.
4. **State readouts** — visible text that reflects internal state, so tests can assert without inspecting framework internals. E.g., a counter button labeled `count up` that tests can click and verify, or a `<pre>` block showing submitted form data.
5. **A reset mechanism** — a `reset` button or page revisit strategy so `beforeEach` can return to a known state.

#### Step 5: Coverage Checklist

Before considering a component's E2E test complete, verify:

- [ ] Every data attribute from every sub-component's API Reference table is asserted
- [ ] Every key from the Keyboard Interactions table has at least one test
- [ ] ARIA roles match the WAI-ARIA pattern referenced in the Accessibility section
- [ ] `aria-labelledby` / `aria-describedby` associations are verified (where applicable)
- [ ] All behavioral variants listed in the API Reference are tested
- [ ] Focus auto-focus, trap, and restore behaviors are tested (for overlay/layer components)
- [ ] Pointer interactions (click and touch) are tested
- [ ] Outside-interaction behavior matches docs (blocked for modal, allowed for non-modal)
- [ ] Disabled state prevents interaction and sets `data-disabled`
- [ ] The test passes against both the React app and the Leptos app without modification

## Conventions

- Conventional commit messages: `fix:`, `feat:`, `chore:`, `docs:`
- All workspace members inherit `version`, `edition`, `license`, `authors`, `repository` from the root `Cargo.toml`
- CI lints with warnings-as-errors (`RUSTFLAGS='-Dwarnings'`)
- Prettier (width 120, tab width 4, single quotes, no bracket spacing) for JS/CSS files
