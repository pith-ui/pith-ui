<p align="center">
    <a href="./logo.svg">
        <img src="./logo.svg" width="300" height="200" alt="Rust Radix Logo">
    </a>
</p>

<h1 align="center">Rust Radix</h1>

A Rust port of [Radix Primitives](https://www.radix-ui.com/primitives) for [Leptos](https://leptos.dev) — unstyled, accessible, headless UI components for the Rust/WASM ecosystem.

## Background

This project is a fork of [RustForWeb/radix](https://github.com/RustForWeb/radix), originally created by [Daniëlle Huisman](https://github.com/DanielleHuisman) as part of the [Rust for Web](https://github.com/RustForWeb) initiative. The original project laid the groundwork for porting Radix UI across multiple Rust web frameworks (Leptos, Yew, and Dioxus), but is currently unmaintained.

This fork narrows the focus to **Leptos only**, with the goal of delivering a complete, production-quality headless component library that faithfully follows the [Radix Primitives](https://www.radix-ui.com/primitives) API and accessibility standards.

## Status

All 61 Radix primitive components and utilities have been ported to Leptos, including:

- **Layout & Display** — Aspect Ratio, Scroll Area, Separator, Progress, Visually Hidden
- **Overlays & Layers** — Dialog, Alert Dialog, Popover, Hover Card, Tooltip, Toast
- **Menus** — Dropdown Menu, Context Menu, Menubar, Navigation Menu
- **Forms** — Checkbox, Radio Group, Switch, Slider, Select, Toggle, Toggle Group, Label, Form
- **Disclosure** — Accordion, Collapsible, Tabs
- **Utilities** — Portal, Presence, Slot, Arrow, Accessible Icon, Collection, Roving Focus, Dismissable Layer, Focus Scope, Popper, Direction, ID
- **Experimental** — One-Time Password Field, Password Toggle Field

Every component has a corresponding Leptos story, and 34 Cypress E2E test suites validate behavioral parity between the Leptos implementation and the original React Radix packages.

### Headless Principles

This port follows the headless UI philosophy of the original Radix Primitives:

- **Unstyled** — Components ship with zero CSS. You bring your own styles.
- **Accessible** — Full WAI-ARIA compliance, keyboard navigation, and focus management out of the box.
- **Composable** — Small, focused components that compose together via the same patterns as React Radix (Root/Trigger/Content/etc.).
- **Controlled & Uncontrolled** — All stateful components support both modes.
- **API-faithful** — Component names, prop names (in snake_case), and compositional structure match the React Radix API.

## Roadmap

### Near-term: Feature Complete

- [ ] Complete E2E test coverage for all components (ongoing — 34 components have Cypress suites). Utilities are tested indirectly through the components that use them.
- [ ] Refactor internals for better unit testability (extract shared helpers, reduce duplication — see `notes/roadmap.md`)
- [ ] Bug fixes and behavioral parity gaps identified through testing
- [ ] Stabilize the public API surface

### Medium-term: Publish & Document

- [ ] Publish crates to [crates.io](https://crates.io)
- [ ] Update [the Rust Radix book](https://radix.rustforweb.org) with current Leptos API documentation, usage examples, and getting-started guide

### Long-term

- [ ] Track the [Base UI](https://base-ui.com/) component library for new additions beyond the original Radix Primitives
- [ ] SSR / hydration support

## Development

```bash
# Lint
cargo clippy --all-features --locked

# Format
cargo fmt --all

# Test
cargo test --all-features --locked --release
```

### Storybooks

The Leptos storybook uses [Trunk](https://trunkrs.dev/):

```bash
just serve_leptos_storybook
```

The React reference storybook (from the upstream Radix Primitives submodule) uses Storybook:

```bash
just serve_react_storybook
```

### E2E Testing

A [Justfile](Justfile) drives the strangler-pattern E2E test harness in `reference_app/`. Shared Cypress tests run against both a React reference app (using real `@radix-ui/react-*` packages) and the Leptos app, proving behavioral parity.

```bash
just test_react_component dialog       # Test one component against React
just test_leptos_component dialog      # Test the same component against Leptos
just test_react                        # Run all E2E tests against React
just test_leptos                       # Run all E2E tests against Leptos
```

See [CLAUDE.md](.claude/CLAUDE.md) for detailed build commands, architecture documentation, and contribution guidelines.

## Credits

This project is a fork of [RustForWeb/radix](https://github.com/RustForWeb/radix), created by [Daniëlle Huisman](https://github.com/DanielleHuisman) and contributors as part of the [Rust for Web](https://github.com/RustForWeb) initiative.

The logo is a combination of the [Radix logo](https://github.com/radix-ui/website/blob/main/components/RadixLogo.tsx) and [Ferris the Rustacean](https://rustacean.net/).

## License

This project is available under the [MIT license](LICENSE.md).
