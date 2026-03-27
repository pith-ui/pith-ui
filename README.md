<p align="center">
    <a href="./logo.svg">
        <img src="./logo.svg" alt="Pith UI Logo">
    </a>
</p>

Unstyled, accessible UI components for [Leptos](https://leptos.dev), inspired by [Radix Primitives](https://www.radix-ui.com/primitives) and [Base-UI](https://github.com/mui/base-ui).

## Features

- **Unstyled** — Ships with zero CSS. Bring your own styles.
- **Accessible** — WAI-ARIA compliant with full keyboard navigation and focus management.
- **Composable** — Small, focused components that compose via familiar Root/Trigger/Content patterns.
- **Controlled & Uncontrolled** — All stateful components support both modes.
- **Feels like React Radix** — Same component names, prop names (in snake_case), and structure as React Radix where applicable.
- **New Components** — Virtualized Lists, Combobox, Calendar, TimeField, and more to come.

## Quick Start

Add the crate with the components you need:

```bash
cargo add pith-ui --features dialog
```

Then use components in your Leptos app:

```rust
use leptos::prelude::*;
use pith_ui::dialog::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <Dialog>
            <DialogTrigger>"Open"</DialogTrigger>
            <DialogPortal>
                <DialogOverlay />
                <DialogContent>
                    <DialogTitle>"Booking info"</DialogTitle>
                    <DialogDescription>
                        "Please enter the info for your booking below."
                    </DialogDescription>
                    <DialogClose>"Close"</DialogClose>
                </DialogContent>
            </DialogPortal>
        </Dialog>
    }
}
```

## Available Components

Each component is gated behind a feature flag. Enable only what you need, or use `features = ["all"]`.

| Category       | Components                                                                       |
| -------------- | -------------------------------------------------------------------------------- |
| **Overlays**   | Dialog, Alert Dialog, Popover, Hover Card, Tooltip, Toast                        |
| **Menus**      | Dropdown Menu, Context Menu, Menubar, Navigation Menu                            |
| **Forms**      | Checkbox, Radio Group, Switch, Slider, Select, Toggle, Toggle Group, Label, Form |
| **Disclosure** | Accordion, Collapsible, Tabs                                                     |
| **Layout**     | Aspect Ratio, Scroll Area, Separator, Progress, Toolbar                          |
| **Utilities**  | Accessible Icon, Avatar, Visually Hidden, Portal                                 |

## Development

```bash
cargo clippy --all-features --locked   # Lint
cargo fmt --all                        # Format
just test_leptos_unit                  # Test

just serve_leptos_storybook            # Run the Leptos storybook
```

See [CLAUDE.md](.claude/CLAUDE.md) for architecture documentation and contribution guidelines.

## Credits

Originally derived from [RustForWeb/radix](https://github.com/RustForWeb/radix) by [Daniëlle Huisman](https://github.com/DanielleHuisman) and contributors.

## License

[MIT](LICENSE.md)
