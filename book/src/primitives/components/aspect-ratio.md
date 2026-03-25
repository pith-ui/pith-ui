# Aspect Ratio

Displays content within a desired ratio.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "pith-ui-book-primitives"
features = ["aspect-ratio"]
files = ["src/aspect_ratio.rs"]
```

{{#endtab }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-primitives"
features = ["aspect-ratio"]
files = ["src/aspect_ratio.rs"]
```

{{#endtab }}
{{#endtabs }}

## Features

-   Accepts any custom ratio.

## Installation

Install the component from your command line.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```shell
cargo add pith-ui-aspect-ratio
```

-   [View on crates.io](https://crates.io/crates/pith-ui-aspect-ratio)
-   [View on docs.rs](https://docs.rs/pith-ui-aspect-ratio/latest/pith_ui_aspect_ratio/)
-   [View source](https://github.com/pith-ui/pith-ui/tree/main/packages/primitives/leptos/aspect-ratio)

{{#endtab }}
{{#tab name="Yew" }}

```shell
cargo add radix-yew-aspect-ratio
```

-   [View on crates.io](https://crates.io/crates/radix-yew-aspect-ratio)
-   [View on docs.rs](https://docs.rs/radix-yew-aspect-ratio/latest/radix_yew_aspect_ratio/)
-   [View source](https://github.com/pith-ui/pith-ui/tree/main/packages/primitives/yew/aspect-ratio)

{{#endtab }}
{{#endtabs }}

## Anatomy

Import the component.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```rust,ignore
use leptos::*;
use pith_ui_aspect_ratio::*;

#[component]
fn Anatomy() -> impl IntoView {
    view! {
        <AspectRatio />
    }
}
```

{{#endtab }}
{{#tab name="Yew" }}

```rust,ignore
use radix_yew_aspect_ratio::*;
use yew::prelude::::*;

#[function_component]
fn Anatomy() -> Html {
    html! {
        <AspectRatio />
    }
}
```

{{#endtab }}
{{#endtabs }}

## API Reference

### Root

Contains the content you want to constrain to a given ratio.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

| Prop       | Type              | Default |
| ---------- | ----------------- | ------- |
| `as_child` | `MaybeProp<bool>` | `false` |
| `ratio`    | `Signal<f64>`     | `1.0`   |

{{#endtab }}
{{#tab name="Yew" }}

| Prop       | Type                                            | Default |
| ---------- | ----------------------------------------------- | ------- |
| `as_child` | `Option<Callback<AspectRatioChildProps, Html>>` | -       |
| `ratio`    | `f64`                                           | `1.0`   |

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/primitives/docs/components/aspect-ratio)
