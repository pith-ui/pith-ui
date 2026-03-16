# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Leptos port of [Radix UI](https://www.radix-ui.com) — a component library for building accessible UI.

## Build & Development Commands

```bash
# Lint (CI runs with RUSTFLAGS='-Dwarnings')
cargo clippy --all-features --locked

# Format
cargo fmt --all
cargo fmt --all --check       # check only

# Test
just test_leptos_unit

# Dependency checks
cargo deny check              # security/license audit
cargo machete                 # unused dependencies
cargo sort --workspace --check # manifest ordering
```

## Architecture

### Workspace Layout

The Cargo workspace is a monorepo under `packages/`:

- **`colors/`** — Framework-agnostic color constants (generated from Radix Colors).
- **`icons/{leptos,yew,dioxus}/`** — SVG icon components generated per-framework from `@radix-ui/react-icons`.
- **`primitives/`** — The main component library:
  - `core/` — Framework-agnostic utilities (`number`, `rect`, `primitive`)
  - `leptos/` — Leptos implementations (active porting target)
  - `yew/` — Yew implementations (out of scope, do not reference)
- **`themes/{yew,leptos}/`** — Higher-level themed component wrappers.

### Supporting Directories

- **`scripts/`** — Code generation for colors and icons; includes `topo_sort.py` for porting order.
- **`stories/`** — Storybook-like component demos (uses Trunk + Tailwind). See `stories/leptos/CLAUDE.md` for story creation instructions.
- **`reference/`** — Reference implementations:
  - `react-radix-primitives/` — Git submodule of the original React Radix Primitives (primary reference)
  - `leptos-radix-rough-draft/` — A previous Leptos implementation. **Use for implementation hints only; always follow the React API per Rule 4.**
- **`reference_app/`** — Cross-framework E2E testing harness (React + Leptos + Cypress). See `reference_app/CLAUDE.md` for full rules and testing methodology.
- **`reference_experiments/`** — Leptos-only experiment sandbox. See `reference_experiments/CLAUDE.md` for details and `just test_experiment <name>` to run.

### Key Dependencies

- `tailwind_fuse` with variant support for CSS class management
- `leptos-node-ref`, `leptos-style`, `leptos-maybe-callback` for Leptos component patterns

## Porting Focus

We are currently only porting to **Leptos**. Yew and Dioxus are out of scope. Research notes for framework-specific primitives should use "Leptos API" / "Leptos Implementation Notes" headings.

## Porting Rules

### Rule 1: Document Research Before Porting

Before porting a component, create a research note at `notes/<package-name>.md` (e.g., `notes/core-number.md`, `notes/leptos-checkbox.md`). Read the React source in `reference/react-radix-primitives/` and the existing Rust implementation (if any) to fill it out.

Use the template at `notes/TEMPLATE.md`. See any existing note in `notes/` for a filled-out example.

**Metadata fields:**
- `react_location` / `rust_location` — Obsidian wikilinks using full repo-relative paths with a display alias
- `react_story` / `rust_story` — Obsidian wikilinks to the React storybook file and Leptos story file. Use `""` if no story exists.
- `dependencies` — list of internal radix packages this component depends on; `[]` if none
- `ported` — `true` if the Rust implementation exists and matches the React source
- `tested` — `true` if tests exist with reasonable coverage
- `tested_story` — `false` by default; set to `true` only by the user after manual verification
- `unstable` — `true` if the React source marks the component as unstable/preview; deferred per Rule 2

### Rule 2: Follow the Dependency Graph

Use `scripts/topo_sort.py` to determine porting order. A dependency is **complete** when `ported: true` AND `tested_story: true` (or `react_story: ""`). The next item to port is the first entry where `ported` is `false` and all dependencies are complete. **Never port a component before all its dependencies are complete.**

**Unstable/preview items last:** Components with `unstable: true` should be deferred until all stable components are ported and story-tested.

```bash
python3 scripts/topo_sort.py
```

### Rule 3: Reference React Only, Never Yew

When porting, reference the React source and existing Leptos code. **Never consult the Yew implementation** — it may contain stale patterns. The inputs are:

1. React source (primary): `reference/react-radix-primitives/packages/<area>/<component>/src/`
2. Existing Leptos code (if any): `packages/primitives/leptos/<component>/src/`
3. Rough draft Leptos reference (optional): `reference/leptos-radix-rough-draft/src/primitives/<component>.rs` — Use for guidance, not as an authoritative API source.

### Rule 4: Follow the React API, Write Idiomatic Leptos

- Match the **public React API** — same component names, same prop names (converted to snake_case), same compositional structure.
- **Structure the Rust code similarly to the React source** for maintainability and ease of review.
- Use **idiomatic Leptos patterns**: signals, `#[component]` functions, `view!` macros, context providers, `NodeRef`, etc. Do not transliterate React hooks 1:1 when a cleaner Leptos idiom exists.

### Rule 5: Verify with Stories

Create a Leptos story mirroring the React storybook. See `stories/leptos/CLAUDE.md` for detailed instructions on CSS modules, stylance imports, and wiring.

### Rule 6: Document Omissions and Decisions

During porting, document in the research note's **Leptos Implementation Notes** section:

1. **Omissions** — React code intentionally left out (e.g., React-specific workarounds). Document *what* and *why*.
2. **Assumptions and key decisions** — Non-obvious choices (signal types, context structure, callback adaptations).

### Rule 7: Definition of Done

A port is complete when:

1. **All public API** from the React source is implemented in Leptos.
2. **Any omitted functionality** is documented per Rule 6 and approved by the user.
3. **Stories** exist per Rule 5 covering the same scenarios as the React reference.
4. The research note's `ported` field is set to `true` only after the above are satisfied.

### Rule 8: Escape Key UX in Dismissable Layers

Components using `DismissableLayer` must implement the **"two escapes" pattern**: when a text-editing element inside the layer has focus, the first Escape moves focus to the layer container; the second Escape dismisses. This is implemented in `DismissableLayer`'s escape handler via `is_text_input()`. Verify this works with stories containing form inputs.

## Known Dependency Issues

### floating-ui-leptos / floating-ui-core v0.6.0

The Rust port of floating-ui has bugs diverging from the JavaScript original. When debugging positioning issues, **compare the Rust source in `~/.cargo/registry/src/` against the JS source on GitHub** line-by-line.

Known bugs:

1. **LimitShift cross_axis uses wrong dimension** — `LimitShift`'s `check_cross_axis` block uses `main_axis.length()` instead of `cross_axis.length()`. **Workaround:** Always pass `.cross_axis(false)` when configuring `LimitShift`.

### Debugging strategy for third-party Rust WASM crate issues

1. **Check the rough-draft reference first** (`reference/leptos-radix-rough-draft/`) to narrow whether the issue is in our code or a dependency.
2. **Read the upstream Rust crate source** in `~/.cargo/registry/src/` and diff against the original JS.
3. **Avoid prolonged printf-debugging.** If log statements don't yield a clear answer within a couple of iterations, pivot to source-level comparison.

## Conventions

- Conventional commit messages: `fix:`, `feat:`, `chore:`, `docs:`
- All workspace members inherit `version`, `edition`, `license`, `authors`, `repository` from the root `Cargo.toml`
- CI lints with warnings-as-errors (`RUSTFLAGS='-Dwarnings'`)
- Prettier (width 120, tab width 4, single quotes, no bracket spacing) for JS/CSS files
