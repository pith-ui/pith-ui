# Stories (Leptos)

Instructions for creating Leptos component stories that mirror the React Radix storybook.

## Creating a New Story

1. Create `stories/leptos/src/primitives/<component>.rs` mirroring the React stories at `reference/react-radix-primitives/apps/storybook/stories/<component>.stories.tsx`.
2. Wire the module into `stories/leptos/src/primitives.rs`, `stories/leptos/src/app.rs` (router + nav), and `stories/leptos/Cargo.toml`.
3. Each exported story in the React file (e.g., `Styled`, `Controlled`, `Chromatic`) should have a corresponding Leptos `#[component]` function.

## CSS Modules

**Copy the CSS module from the React reference directly.** Copy `reference/react-radix-primitives/apps/storybook/stories/<component>.stories.module.css` to `stories/leptos/src/primitives/<component>.stories.module.css`. The file must be an exact copy — **never modify or "improve" the borrowed styles.**

Any required CSS variables (e.g., Radix Colors) that the module references must be available at runtime. New color packages should be added as `<link data-trunk rel="css" href="/node_modules/@radix-ui/colors/<color>.css">` entries in `stories/leptos/index.html` — this is how Trunk loads them into the build. Do **not** add colors via `@import` in `preview.css`.

## Stylance Imports

Use `stylance::import_crate_style!` to import CSS module classes:

```rust
stylance::import_crate_style!(classes, "src/primitives/<component>.stories.module.css");
```

Then reference classes via `attr:class=classes::root`, `attr:class=classes::trigger`, etc.

See `stories/leptos/src/primitives/accordion.rs` and its corresponding `accordion.stories.module.css` for the reference pattern.
