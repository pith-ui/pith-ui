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
- **`reference/`** — Git submodule of the original React Radix Primitives for reference during porting

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
dependencies: []
ported: false
tested: false
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
- `dependencies` — list of internal radix packages this component depends on; `[]` if none
- `ported` — `true` if the Rust implementation exists and matches the React source
- `tested` — `true` if tests exist with reasonable coverage

**Content guidelines:**
- Notes should be complete but not overly verbose
- Focus on what matters for porting: intent, API shape, non-obvious implementation details
- Call out missing test coverage or known divergences from the React source

### Rule 2: Follow the Dependency Graph

Use `scripts/topo_sort.py` to determine porting order. Run it to see all components sorted topologically with their ported status. The next item to port is the first entry where `ported` is `false` and all of its dependencies are already `true`. Never port a component before its dependencies are ported.

```bash
python3 scripts/topo_sort.py
```

### Rule 3: Reference React Only, Never Yew

When porting, reference only the React source in `reference/react-radix-primitives/` and the existing Leptos code. **Never consult the Yew implementation** — it may contain stale patterns or divergences that should not propagate. The two inputs are:

1. React source: `reference/react-radix-primitives/packages/<area>/<component>/src/`
2. Existing Leptos code (if any): `packages/primitives/leptos/<component>/src/`

### Rule 4: Follow the React API, Write Idiomatic Leptos

- Match the **public React API** — same component names, same prop names (converted to snake_case), same compositional structure.
- **Structure the Rust code similarly to the React source** for maintainability, ease of review, and future syncs with upstream changes.
- Use **idiomatic Leptos patterns** for the implementation: signals, `#[component]` functions, `view!` macros, context providers, `NodeRef`, etc. Do not transliterate React hooks 1:1 when a cleaner Leptos idiom exists.

### Rule 5: Verify with Stories

Create a Leptos story in `stories/leptos/src/primitives/<component>.rs` that closely mirrors the React reference stories at `reference/react-radix-primitives/apps/storybook/stories/<component>.stories.tsx`.

- Wire the new story module into `stories/leptos/src/primitives.rs`, `stories/leptos/src/app.rs` (router + nav), and `stories/leptos/Cargo.toml`.
- **Borrow CSS from the React reference verbatim.** The React stories use CSS modules (`.stories.module.css`). Convert those styles to equivalent Tailwind classes using `tailwind_fuse` / `TwClass` derive, but preserve the same visual intent — colors, sizes, spacing, data-attribute selectors. **Never modify or "improve" the borrowed styles.**
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

## Conventions

- Conventional commit messages: `fix:`, `feat:`, `chore:`, `docs:`
- All workspace members inherit `version`, `edition`, `license`, `authors`, `repository` from the root `Cargo.toml`
- CI lints with warnings-as-errors (`RUSTFLAGS='-Dwarnings'`)
- Prettier (width 120, tab width 4, single quotes, no bracket spacing) for JS/CSS files
