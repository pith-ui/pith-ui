# Separator

Visually or semantically separates content.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "cardo-ui-book-primitives"
features = ["separator"]
files = ["src/separator.rs"]
```

{{#endtab }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-primitives"
features = ["separator"]
files = ["src/separator.rs"]
```

{{#endtab }}
{{#endtabs }}

## Features

-   Supports horizontal and vertical orientations.

## Installation

Install the component from your command line.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```shell
cargo add cardo-ui-separator
```

-   [View on crates.io](https://crates.io/crates/cardo-ui-separator)
-   [View on docs.rs](https://docs.rs/cardo-ui-separator/latest/cardo_ui_separator/)
-   [View source](https://github.com/cardo-ui/cardo-ui/tree/main/packages/primitives/leptos/separator)

{{#endtab }}
{{#tab name="Yew" }}

```shell
cargo add radix-yew-separator
```

-   [View on crates.io](https://crates.io/crates/radix-yew-separator)
-   [View on docs.rs](https://docs.rs/radix-yew-separator/latest/radix_yew_separator/)
-   [View source](https://github.com/cardo-ui/cardo-ui/tree/main/packages/primitives/yew/separator)

{{#endtab }}
{{#endtabs }}

## Anatomy

Import the component.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```rust,ignore
use leptos::*;
use cardo_ui_separator::*;

#[component]
fn Anatomy() -> impl IntoView {
    view! {
        <Separator />
    }
}
```

{{#endtab }}
{{#tab name="Yew" }}

```rust,ignore
use radix_yew_separator::*;
use yew::prelude::*;

#[component]
fn Anatomy() -> Html {
    html! {
        <Separator />
    }
}
```

{{#endtab }}
{{#endtabs }}

## API Reference

### Root

The separator.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

| Prop          | Type                     | Default                   |
| ------------- | ------------------------ | ------------------------- |
| `as_child`    | `MaybeProp<bool>`        | `false`                   |
| `orientation` | `MaybeProp<Orientation>` | `Orientation::Horizontal` |
| `decorative`  | `MaybeProp<bool>`        | `false`                   |

{{#endtab }}
{{#tab name="Yew" }}

| Prop          | Type                                          | Default                   |
| ------------- | --------------------------------------------- | ------------------------- |
| `as_child`    | `Option<Callback<SeparatorChildProps, Html>>` | -                         |
| `orientation` | `Orientation`                                 | `Orientation::Horizontal` |
| `decorative`  | `bool`                                        | `false`                   |

{{#endtab }}
{{#endtabs }}

<div style="height: 1em;"></div>

| Data attribute       | Values                       |
| -------------------- | ---------------------------- |
| `[data-orientation]` | `"horizontal" \| "vertical"` |

## Accessibility

Adheres to the [`separator` role requirements](https://www.w3.org/TR/wai-aria-1.2/#separator).

## See Also

-   [Radix documentation](https://www.radix-ui.com/primitives/docs/components/separator)
