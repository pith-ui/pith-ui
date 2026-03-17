# Reference App

Cross-framework E2E testing harness: a React app and a Leptos app that implement identical pages for each Radix primitive, validated by a shared Cypress test suite. This proves behavioral parity between the original React Radix and our Leptos port.

## Directory Layout

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

## RA Rule 1: Identical Routes, Identical Semantics

Both apps must expose the **same routes** with the **same DOM semantics**. Cypress tests are framework-agnostic — they interact with the DOM, not the implementation.

- **Routes**: `/<component>` (e.g., `/dialog`, `/form`). If a component needs multiple scenarios, use `/<component>/<scenario>` (e.g., `/dialog/modal`, `/dialog/non-modal`).
- **Text content**: Buttons, labels, headings, and other visible text must be identical across both apps. Cypress tests locate elements by text (`cy.findByText`), label (`cy.findByLabelText`), placeholder (`cy.findByPlaceholderText`), and role.
- **ARIA attributes**: Both apps must produce equivalent `aria-*` attributes, roles, and `data-state` attributes since tests assert on these.
- **`data-testid`**: Use `data-testid` attributes when text/aria selectors are insufficient. Both apps must use the same `data-testid` values.
- **Control elements**: Test fixtures (checkboxes to toggle state, reset buttons, output displays) must use identical labels and text across both apps.

## RA Rule 2: Docs-Driven TDD — Tests First, React Validates, Leptos Must Pass

The **Radix documentation** is the source of truth for what to test. The **React app** (using real `@radix-ui/react-*` packages) is the oracle that validates test correctness. Follow this order:

1. **Write the Cypress test first**, derived from the Radix docs. The docs' data-attribute tables, Keyboard Interactions table, and Accessibility section define every assertion. See the **E2E Testing Methodology** section below for the full process.
2. **Write the React page second** — a minimal test fixture with variant toggles and outside interaction targets.
3. **Run the tests against React to validate them.** This catches cases where the docs are ambiguous or underspecified (exact tab order, what gets auto-focused, animation timing). Fix tests only if they assert something the docs don't actually require; add tests for undocumented behaviors discovered during validation.
   - When testing, use `just test_react_component <component>` to run the suite of tests against a react component.
4. **Write the Leptos page last**, making it produce equivalent DOM output.
5. **Run the same Cypress test against Leptos.** It must pass without modification.
   - When testing, use `just test_leptos_component <component>` to run the suite of tests against a react component.
   - When you need to compile from scratch, use `trunk clean` and/or `cargo clean`

Writing tests from the docs (not from the React implementation) prevents tests from just confirming "what React happens to do" instead of "what the spec says should happen."

If a Cypress test needs to be modified to pass against Leptos, that indicates a bug in the Leptos implementation, not a problem with the test. The only exception is timing — Leptos WASM apps may need slightly longer waits, which should be handled via Cypress retry-ability (`.should()` assertions) rather than hard-coded waits.

## RA Rule 3: Keep Pages Minimal and Focused

Reference app pages are **test fixtures**, not demos or showcases. They exist solely to exercise component behavior for Cypress.

- Each page should test the **core interactive behavior** of one primitive: open/close, focus management, keyboard navigation, form validation, accessibility attributes, etc.
- Include **control elements** (checkboxes, buttons) to toggle component state for different test scenarios, matching the pattern used in `stories/leptos/src/primitives/dialog.rs` (the `Cypress` story).
- Do **not** add visual polish, multiple variants, or showcase-style layouts. The existing `stories/` directory serves that purpose.
- Keep the React implementation simple and idiomatic — no abstractions, no shared utilities. Each page should be self-contained.

## RA Rule 4: Shared CSS

Both apps must use the **same CSS** for component styling within test pages, so layout-dependent behavior (scroll, overflow, overlay clicks) is identical.

- Place shared CSS files in `reference_app/shared/` (e.g., `reference_app/shared/dialog.css`).
- Both apps import from this shared location. React uses standard CSS imports; Leptos uses `<link data-trunk rel="css" ...>` in `index.html` or inline styles.
- Keep styles minimal — just enough to make interactive behavior testable (e.g., overlays need some opacity/positioning so pointer-outside-dismiss works; scrollable content needs a height).

## RA Rule 5: Running Tests via the Justfile

**Always use the justfile to run E2E tests.** Never run cypress, trunk, or pnpm commands directly.

**Critical constraints:**
- **Never run more than one test command in parallel.** All test commands use port 3000; running two concurrently causes port conflicts and failures.
- **Never use explicit wait times** (`cy.wait()`, `sleep`, etc.) to queue or synchronize test commands. Rely on Cypress retry-ability (`.should()` assertions) and `start-server-and-test` which handles server readiness automatically.

**Test commands reference:**

| Command                                     | When to Use                                                                                                                                                       |
| ------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `just test_react_component <component>`     | Test a single component against the React reference app. Use to **validate that a Cypress test is correct** before running against Leptos (per RA Rule 2 step 3). |
| `just test_leptos_component <component>`    | Test a single component against the Leptos app. Use to **verify the Leptos port matches React behavior** (per RA Rule 2 step 5).                                  |
| `just test_react_components <c1> <c2> ...`  | Test multiple components against React in one server session. Use when validating several related Cypress tests at once.                                          |
| `just test_leptos_components <c1> <c2> ...` | Test multiple components against Leptos in one server session. Use when verifying several Leptos ports at once.                                                   |
| `just test_react`                           | Run **all** Cypress tests against React. Full regression validation.                                                                                              |
| `just test_leptos`                          | Run **all** Cypress tests against Leptos. Full regression validation.                                                                                             |
| `just test_leptos_unit`                     | Run cargo unit tests for leptos primitives. Logic-level testing without a browser.                                                                                |
| `just test_leptos_wasm`                     | Run wasm-bindgen tests in headless Chrome. WASM-specific behavior testing.                                                                                        |
| `just test_leptos_all`                      | Run unit + wasm + all E2E tests for Leptos. Comprehensive pre-merge validation.                                                                                   |

The `<component>` name uses **hyphens** to match the Cypress spec filename (e.g., `dropdown-menu` for `dropdown-menu.cy.js`).

**Dev servers** (for manual testing only — prefer the `just test_*` commands above):

```bash
just dev_react    # Start React dev server on :3000
just dev_leptos   # Start Leptos dev server on :3000
just cy_open      # Open Cypress GUI (server must already be running)
```

**Test output handling:** When running the `just test_{leptos,react}_*` commands, the output is captured and summarized for later consumption within `.results/`.

If you need more detail on failures, query that folder through the `scripts/query_e2e_results.py` script and its commands:

- `history <spec>`: prints out a historical table of react and leptos runs for a particular component
- `list`: shows all specs with their latest pass/fail status per framework

## RA Rule 6: Cypress Test Conventions

Tests must be framework-agnostic and follow these conventions:

- **Use `@testing-library/cypress` queries** (`findByText`, `findByLabelText`, `findByRole`, `findByPlaceholderText`, `findByTestId`) for element selection. Never use framework-specific selectors (React component names, Leptos class paths, etc.).
- **Use `cypress-real-events`** (`realPress`, `realClick`, `realType`, `realTouch`) for interactions that must match real browser behavior (focus, keyboard events).
- **Never use hard-coded waits** (`cy.wait()`). Rely on Cypress's built-in retry-ability via `.should()` assertions.
- **Reset state explicitly.** Since `testIsolation: false` is used for performance, each test must reset to a known state in `beforeEach` (e.g., clicking a reset button, revisiting the page).
- **File naming**: `cypress/e2e/<component>.cy.js` — lowercase, matching the route name.
- **Structure**: One `describe('<Component>')` block per file, with nested `describe` blocks for scenarios and `it` blocks for individual assertions.

## RA Rule 7: Adding a New Component

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

## RA Rule 8: What NOT to Test Here

The reference app Cypress tests focus on **behavioral parity** — proving the Leptos port behaves identically to React. They do **not** replace:

- **Unit tests** (`cargo test`) for internal logic.
- **Story-based manual testing** (`stories/`) for visual fidelity and edge-case demos.
- **Accessibility audits** beyond what ARIA attribute assertions cover.

Keep Cypress tests focused on interactive behavior: open/close, focus management, keyboard navigation, form submission, dismiss-on-outside-click, state attributes, etc.

## E2E Testing Methodology

Each Cypress test file must provide **comprehensive behavioral coverage** of a Radix primitive, derived directly from the React Radix documentation and source code. The Radix docs at `https://www.radix-ui.com/primitives/docs/components/<component>` define the contract. Every documented behavior is a test case.

### Step 1: Read the Radix Documentation

Before writing any test, read the component's Radix documentation page. Extract the following:

1. **API Reference data-attribute tables** — Each sub-component documents its data attributes (e.g., `[data-state]`, `[data-disabled]`, `[data-orientation]`, `[data-side]`, `[data-align]`). These are the component's public contract for state visibility.
2. **Accessibility section** — Identifies the WAI-ARIA design pattern the component adheres to and any specific ARIA roles, properties, or states it must expose.
3. **Keyboard Interactions table** — Lists every key and its expected behavior. This is the definitive spec for keyboard tests.

Also read the React source at `reference/react-radix-primitives/packages/react/<component>/src/` to understand behaviors not fully described in the docs (edge cases, focus management, dismiss behavior, etc.).

### Step 2: Structure the Test File

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

### Step 3: Cover Every Test Category

#### 3a. Accessibility Semantics

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

#### 3b. Data Attributes

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

#### 3c. Keyboard Navigation

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

#### 3d. Pointer Interaction

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

#### 3e. Focus Management

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

#### 3f. Variants

Each component variant that changes behavior must be tested independently. Common variants:

| Variant                                 | Components                             | What changes                                          |
| --------------------------------------- | -------------------------------------- | ----------------------------------------------------- |
| `modal` / non-modal                     | Dialog, AlertDialog                    | Focus trap, outside interaction blocking, scroll lock |
| `controlled` / `uncontrolled`           | All stateful components                | State is driven by props vs internal                  |
| `disabled`                              | Accordion, Checkbox, RadioGroup, etc.  | Prevents interaction, sets `data-disabled`            |
| `orientation` (`vertical`/`horizontal`) | Accordion, Tabs, Slider, etc.          | Arrow key direction, `data-orientation`               |
| `type` (`single`/`multiple`)            | Accordion                              | One vs many items open                                |
| `collapsible`                           | Accordion (`single` type)              | Whether last open item can be closed                  |
| `required`                              | Checkbox, RadioGroup, Select           | Form validation behavior                              |
| `dir` (`ltr`/`rtl`)                     | Components with directional arrow keys | Arrow key mapping flips                               |

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

### Step 4: Test Page Requirements

The React and Leptos pages must provide sufficient DOM structure to exercise all the above categories. Each page needs:

1. **The component** in its default configuration.
2. **Variant toggles** — labeled checkboxes/radio buttons for each behavioral variant (e.g., `modal`, `disabled`, `orientation`). Use `<label>` wrapping `<input type="checkbox">` so `cy.findByLabelText()` works.
3. **Outside interaction targets** — elements outside the component (buttons, inputs) with stable labels, for testing dismiss-on-outside-click and focus escape.
4. **State readouts** — visible text that reflects internal state, so tests can assert without inspecting framework internals. E.g., a counter button labeled `count up` that tests can click and verify, or a `<pre>` block showing submitted form data.
5. **A reset mechanism** — a `reset` button or page revisit strategy so `beforeEach` can return to a known state.

### Step 5: Coverage Checklist

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
