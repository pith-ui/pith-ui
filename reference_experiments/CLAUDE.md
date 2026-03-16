# Reference Experiments

## Purpose

This directory is an **isolated experimentation sandbox** for validating assumptions, testing Leptos framework behaviors, and building knowledge about edge cases — without polluting the main `reference_app/`, `stories/`, or component source code.

Unlike `reference_app/` (which proves behavioral parity between React Radix and our Leptos port), experiments here are **Leptos-only**. There is no React counterpart because these tests validate Leptos-specific behaviors, framework capabilities, and implementation hypotheses — not cross-framework parity.

## When to Use This Space

- **Testing assumptions** about how Leptos APIs work (e.g., "can I clone `AnyAttribute` and spread it onto two elements?")
- **Investigating framework edge cases** before committing to an implementation approach in the real components
- **Reproducing bugs** in isolation to understand root causes
- **Prototyping patterns** that may later be adopted in the main component library

## When NOT to Use This Space

- **Behavioral parity testing** (React vs Leptos) — use `reference_app/` instead
- **Component demos/showcases** — use `stories/` instead
- **Unit testing internal logic** — use `cargo test` in the relevant package instead

## Directory Layout

```
reference_experiments/
├── CLAUDE.md               # This file
├── leptos/                 # Leptos app (Trunk, CSR)
│   ├── src/
│   │   ├── main.rs         # Entry point
│   │   ├── app.rs          # Router with routes for each experiment
│   │   ├── pages.rs        # Module index
│   │   └── pages/          # One file per experiment
│   │       └── attribute_clone.rs
│   ├── Cargo.toml
│   ├── Trunk.toml
│   └── index.html
├── cypress/                # Cypress E2E tests
│   ├── e2e/                # Test specs (one per experiment)
│   │   └── attribute-clone.cy.js
│   └── support/
│       └── e2e.js
├── cypress.config.js
└── package.json
```

## Running Experiments

Use the justfile commands:

```bash
# Test a single experiment
just test_experiment <experiment-name>

# Test all experiments
just test_experiments

# Dev server for manual testing
just dev_experiments

# Open Cypress GUI against dev server
just cy_open_experiments
```

The `<experiment-name>` matches the Cypress spec filename (e.g., `attribute-clone` for `attribute-clone.cy.js`).

## Conventions

### Test Philosophy

- **Never delete failing tests** unless they are truly irrelevant or redundant.
- If a test fails and the failure is informative, **skip it** (`it.skip(...)`) and update the test description to explain *why* it fails. This preserves knowledge.
- Each experiment should have a clear hypothesis stated in a comment at the top of both the Leptos page and the Cypress test file.
- Experiments that validate successfully can inform implementation decisions in the main codebase — link to the experiment from research notes or commit messages.

### Naming

- Leptos page files: `snake_case.rs` (e.g., `attribute_clone.rs`)
- Cypress specs: `kebab-case.cy.js` (e.g., `attribute-clone.cy.js`)
- Routes: `/<kebab-case>` (e.g., `/attribute-clone`)

### Experiment Pages

Each page should include:
1. A visible heading stating the experiment name
2. `data-testid` attributes on elements that need assertion
3. Minimal DOM — just enough to test the hypothesis
