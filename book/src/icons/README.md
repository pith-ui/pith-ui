# Icons

A crisp set of 15×15 icons. All icons are available as individual components.

{{#tabs global="framework" }}
{{#tab name="Dioxus" }}

```toml,trunk
package = "radix-dioxus-book-icons"
features = ["icons"]
files = ["src/icons.rs"]
```

{{#endtab }}
{{#tab name="Leptos" }}

```toml,trunk
package = "cardo-ui-book-icons"
features = ["icons"]
files = ["src/icons.rs"]
```

{{#endtab }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-icons"
features = ["icons"]
files = ["src/icons.rs"]
```

{{#endtab }}
{{#endtabs }}

## Installation

Install the icons from your command line.

{{#tabs global="framework" }}
{{#tab name="Dioxus" }}

```shell
cargo add radix-dioxus-icons
```

-   [View on crates.io](https://crates.io/crates/radix-dioxus-icons)
-   [View on docs.rs](https://docs.rs/radix-dioxus-icons/latest/radix_dioxus_icons/)
-   [View source](https://github.com/cardo-ui/cardo-ui/tree/main/packages/icons/dioxus)

{{#endtab }}
{{#tab name="Leptos" }}

```shell
cargo add cardo-ui-icons
```

-   [View on crates.io](https://crates.io/crates/cardo-ui-icons)
-   [View on docs.rs](https://docs.rs/cardo-ui-icons/latest/cardo_ui_icons/)
-   [View source](https://github.com/cardo-ui/cardo-ui/tree/main/packages/icons/leptos)

{{#endtab }}
{{#tab name="Yew" }}

```shell
cargo add radix-yew-icons
```

-   [View on crates.io](https://crates.io/crates/radix-yew-icons)
-   [View on docs.rs](https://docs.rs/radix-yew-icons/latest/radix_yew_icons/)
-   [View source](https://github.com/cardo-ui/cardo-ui/tree/main/packages/icons/yew)

{{#endtab }}
{{#endtabs }}

## Anatomy

Import the icons.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```rust,ignore
use dioxus::prelude::*;
use radix_dioxus_icons::{FaceIcon, ImageIcon, SunIcon};

#[component]
fn App() -> Element {
    rsx! {
        FaceIcon {}
        ImageIcon {}
        SunIcon {}
    }
}
```

{{#endtab }}
{{#tab name="Leptos" }}

```rust,ignore
use leptos::*;
use cardo_ui_icons::{FaceIcon, ImageIcon, SunIcon};

#[component]
fn Anatomy() -> impl IntoView {
    view! {
        <FaceIcon />
        <SunIcon />
        <ImageIcon />
    }
}
```

{{#endtab }}
{{#tab name="Yew" }}

```rust,ignore
use yew::prelude::*;
use radix_yew_icons::{FaceIcon, ImageIcon, SunIcon};

#[function_component]
fn Anatomy() -> Html {
    html! {
        <>
            <FaceIcon />
            <SunIcon />
            <ImageIcon />
        </>
    }
}
```

{{#endtab }}
{{#endtabs }}

## API Reference

{{#tabs global="framework" }}
{{#tab name="Dioxus" }}

| Prop     | Type     | Default          |
| -------- | -------- | ---------------- |
| `width`  | `usize`  | `15`             |
| `height` | `usize`  | `15`             |
| `color`  | `String` | `"currentColor"` |

{{#endtab }}
{{#tab name="Leptos" }}

| Prop     | Type             | Default          |
| -------- | ---------------- | ---------------- |
| `width`  | `Signal<usize>`  | `15`             |
| `height` | `Signal<usize>`  | `15`             |
| `color`  | `Signal<String>` | `"currentColor"` |

{{#endtab }}
{{#tab name="Yew" }}

| Prop     | Type        | Default          |
| -------- | ----------- | ---------------- |
| `width`  | `usize`     | `15`             |
| `height` | `usize`     | `15`             |
| `color`  | `AttrValue` | `"currentColor"` |

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/icons)
