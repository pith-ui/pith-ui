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

## Conventions

- Conventional commit messages: `fix:`, `feat:`, `chore:`, `docs:`
- All workspace members inherit `version`, `edition`, `license`, `authors`, `repository` from the root `Cargo.toml`
- CI lints with warnings-as-errors (`RUSTFLAGS='-Dwarnings'`)
- Prettier (width 120, tab width 4, single quotes, no bracket spacing) for JS/CSS files
