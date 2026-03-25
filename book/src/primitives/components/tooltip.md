# Tooltip

A popup that displays information related to an element when the element receives keyboard focus or the mouse hovers over it.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

<div class="warning">

This component is not yet updated to Leptos 0.7+.

</div>

<!-- ```toml,trunk
package = "pith-ui-book-primitives"
features = ["tooltip"]
files = ["src/tooltip.rs"]
``` -->

{{#endtab }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-primitives"
features = ["tooltip"]
files = ["src/tooltip.rs"]
```

{{#endtab }}
{{#endtabs }}

## Features

-   Provider to control display delay globally.
-   Opens when the trigger is focused or hovered.
-   Closes when the trigger is activated or when pressing escape.
-   Supports custom timings.

## Installation

Install the component from your command line.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```shell
cargo add pith-ui-tooltip
```

-   [View source](https://github.com/pith-ui/pith-ui/tree/main/packages/primitives/leptos/src/components/tooltip)

{{#endtab }}
{{#tab name="Yew" }}

```shell
cargo add radix-yew-tooltip
```

-   [View on crates.io](https://crates.io/crates/radix-yew-tooltip)
-   [View on docs.rs](https://docs.rs/radix-yew-tooltip/latest/radix_yew_tooltip/)
-   [View source](https://github.com/pith-ui/pith-ui/tree/main/packages/primitives/yew/tooltip)

{{#endtab }}
{{#endtabs }}

## Anatomy

Import all parts and piece them together.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```rust,ignore
use leptos::prelude::*;
use pith_ui::tooltip::*;

#[component]
fn Anatomy() -> impl IntoView {
    view! {
        <TooltipProvider>
            <Tooltip>
                <TooltipTrigger />
                <TooltipPortal>
                    <TooltipContent>
                        <TooltipArrow />
                    </TooltipContent>
                </TooltipPortal>
            </Tooltip>
        </TooltipProvider>
    }
}
```

{{#endtab }}
{{#tab name="Yew" }}

```rust,ignore
use radix_yew_tooltip::*;
use yew::prelude::*;

#[component]
fn Anatomy() -> Html {
    html! {
        <TooltipProvider>
            <Tooltip>
                <TooltipTrigger />
                <TooltipPortal>
                    <TooltipContent>
                        <TooltipArrow />
                    </TooltipContent>
                </TooltipPortal>
            </Tooltip>
        </TooltipProvider>
    }
}
```

{{#endtab }}
{{#endtabs }}

## API Reference

### Provider

Wraps your app to provide global functionality to your tooltips.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

| Prop                        | Type              | Default |
| --------------------------- | ----------------- | ------- |
| `delay_duration`            | `MaybeProp<f64>`  | `700.0` |
| `skip_delay_duration`       | `MaybeProp<f64>`  | `300.0` |
| `disable_hoverable_content` | `MaybeProp<bool>` | -       |

{{#endtab }}
{{#tab name="Yew" }}

| Prop                        | Type           | Default |
| --------------------------- | -------------- | ------- |
| `delay_duration`            | `i32`          | `700`   |
| `skip_delay_duration`       | `i32`          | `300`   |
| `disable_hoverable_content` | `Option<bool>` | -       |

{{#endtab }}
{{#endtabs }}

### Root

Contains all the parts of a tooltip.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

| Prop                        | Type                   | Default |
| --------------------------- | ---------------------- | ------- |
| `default_open`              | `MaybeProp<bool>`      | -       |
| `open`                      | `MaybeProp<bool>`      | -       |
| `on_open_change`            | `Option<Callback<bool>>` | -     |
| `delay_duration`            | `MaybeProp<f64>`       | -       |
| `disable_hoverable_content` | `MaybeProp<bool>`      | -       |

{{#endtab }}
{{#tab name="Yew" }}

| Prop                        | Type             | Default |
| --------------------------- | ---------------- | ------- |
| `default_open`              | `Option<bool>`   | -       |
| `open`                      | `Option<bool>`   | -       |
| `on_open_change`            | `Callback<bool>` | -       |
| `delay_duration`            | `Option<i32>`    | -       |
| `disable_hoverable_content` | `Option<bool>`   | -       |

{{#endtab }}
{{#endtabs }}

### Trigger

The button that toggles the tooltip. By default, the `TooltipContent` will position itself against the trigger.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

| Prop               | Type                                  | Default |
| ------------------ | ------------------------------------- | ------- |
| `as_child`         | `MaybeProp<bool>`                     | `false` |
| `on_pointer_move`  | `Option<Callback<ev::PointerEvent>>`  | -       |
| `on_pointer_leave` | `Option<Callback<ev::PointerEvent>>`  | -       |
| `on_pointer_down`  | `Option<Callback<ev::PointerEvent>>`  | -       |
| `on_focus`         | `Option<Callback<ev::FocusEvent>>`    | -       |
| `on_blur`          | `Option<Callback<ev::FocusEvent>>`    | -       |
| `on_click`         | `Option<Callback<ev::MouseEvent>>`    | -       |

{{#endtab }}
{{#tab name="Yew" }}

| Prop       | Type                                       | Default |
| ---------- | ------------------------------------------ | ------- |
| `as_child` | `Callback<TooltipTriggerChildProps, Html>` | -       |

{{#endtab }}
{{#endtabs }}

<div style="height: 1em;"></div>

| Data attribute | Values                                         |
| -------------- | ---------------------------------------------- |
| `[data-state]` | `"closed" \| "delayed-open" \| "instant-open"` |

### Portal

When used, portals the content part into the `body`.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

| Prop            | Type                                      | Default |
| --------------- | ----------------------------------------- | ------- |
| `force_mount`   | `MaybeProp<bool>`                         | -       |
| `container`     | `MaybeProp<SendWrapper<web_sys::Element>>` | -      |
| `container_ref` | `AnyNodeRef`                              | -       |

{{#endtab }}
{{#tab name="Yew" }}

| Prop            | Type                       | Default |
| --------------- | -------------------------- | ------- |
| `force_mount`   | `Option<bool>`             | -       |
| `container`     | `Option<web_sys::Element>` | -       |
| `container_ref` | `Option<NodeRef>`          | -       |

{{#endtab }}
{{#endtabs }}

### Content

The component that pops out when the tooltip is open.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

| Prop                         | Type                                                 | Default             |
| ---------------------------- | ---------------------------------------------------- | ------------------- |
| `as_child`                   | `MaybeProp<bool>`                                    | `false`             |
| `aria_label`                 | `MaybeProp<String>`                                  | -                   |
| `on_escape_key_down`         | `Option<Callback<web_sys::KeyboardEvent>>`           | -                   |
| `on_pointer_down_outside`    | `Option<Callback<web_sys::CustomEvent>>`             | -                   |
| `force_mount`                | `MaybeProp<bool>`                                    | -                   |
| `side`                       | `Signal<Side>`                                       | `Side::Top`         |
| `side_offset`                | `Signal<f64>`                                        | `0.0`               |
| `align`                      | `Signal<Align>`                                      | `Align::Center`     |
| `align_offset`               | `Signal<f64>`                                        | `0.0`               |
| `avoid_collisions`           | `Signal<bool>`                                       | `true`              |
| `collision_boundary`         | `Signal<SendWrapper<Vec<web_sys::Element>>>`         | `vec![]`            |
| `collision_padding`          | `Signal<Padding>`                                    | `Padding::All(0.0)` |
| `sticky`                     | `Signal<Sticky>`                                     | `Sticky::Partial`   |
| `hide_when_detached`         | `Signal<bool>`                                       | `false`             |
| `update_position_strategy`   | `Signal<UpdatePositionStrategy>`                     | `UpdatePositionStrategy::Optimized` |

{{#endtab }}
{{#tab name="Yew" }}

| Prop                      | Type                                               | Default             |
| ------------------------- | -------------------------------------------------- | ------------------- |
| `as_child`                | `Option<Callback<TooltipContentChildProps, Html>>` | -                   |
| `on_escape_key_down`      | `Callback<KeyboardEvent>`                          | -                   |
| `on_pointer_down_outside` | `Callback<PointerDownOutsideEvent>`                | -                   |
| `force_mount`             | `Option<bool>`                                     | -                   |
| `side`                    | `Side`                                             | `Side::Top`         |
| `side_offset`             | `f64`                                              | `0.0`               |
| `align`                   | `Align`                                            | `Align::Center`     |
| `align_offset`            | `f64`                                              | `0.0`               |
| `avoid_collisions`        | `bool`                                             | `true`              |
| `collision_boundary`      | `Vec<web_sys::Element>`                            | `vec![]`            |
| `collision_padding`       | `Padding`                                          | `Padding::All(0.0)` |
| `sticky`                  | `Sticky`                                           | `Sticky::Partial`   |
| `hide_when_detatched`     | `bool`                                             | `false`             |

{{#endtab }}
{{#endtabs }}

<div style="height: 1em;"></div>

| Data attribute | Values                                         |
| -------------- | ---------------------------------------------- |
| `[data-state]` | `"closed" \| "delayed-open" \| "instant-open"` |
| `[data-side]`  | `"left" \| "right" \| "bottom" \| "top"`       |
| `[data-align]` | `"start" \| "end" \| "center"`                 |

<div style="height: 1em;"></div>

| CSS Variable                               | Description                                                                   |
| ------------------------------------------ | ----------------------------------------------------------------------------- |
| `--radix-tooltip-content-transform-origin` | The `transform-origin` computed from the content and arrow positions/offsets. |
| `--radix-tooltip-content-available-width`  | The remaining width between the trigger and the boundary edge.                |
| `--radix-tooltip-content-available-height` | The remaining height between the trigger and the boundary edge.               |
| `--radix-tooltip-trigger-width`            | The width of the trigger.                                                     |
| `--radix-tooltip-trigger-height`           | The height of the trigger.                                                    |

### Arrow

An optional arrow element to render alongside the tooltip. This can be used to help visually link the trigger with the `TooltipContent`. Must be rendered inside `TooltipContent`.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

| Prop       | Type              | Default |
| ---------- | ----------------- | ------- |
| `as_child` | `MaybeProp<bool>` | `false` |
| `width`    | `MaybeProp<f64>`  | -       |
| `height`   | `MaybeProp<f64>`  | -       |

{{#endtab }}
{{#tab name="Yew" }}

| Prop       | Type                                             | Default |
| ---------- | ------------------------------------------------ | ------- |
| `as_child` | `Option<Callback<TooltipArrowChildProps, Html>>` | -       |
| `width`    | `f64`                                            | `10.0`  |
| `height`   | `f64`                                            | `5.0`   |

{{#endtab }}
{{#endtabs }}

## Examples

### Basic usage

```rust,ignore
use leptos::prelude::*;
use pith_ui::tooltip::*;

#[component]
fn BasicTooltip() -> impl IntoView {
    view! {
        <TooltipProvider>
            <Tooltip>
                <TooltipTrigger>
                    <button>"Hover me"</button>
                </TooltipTrigger>
                <TooltipPortal>
                    <TooltipContent side_offset=5.0>
                        "Tooltip content"
                        <TooltipArrow />
                    </TooltipContent>
                </TooltipPortal>
            </Tooltip>
        </TooltipProvider>
    }
}
```

### Controlled

Use the `open` and `on_open_change` props to control the tooltip state programmatically.

```rust,ignore
use leptos::prelude::*;
use pith_ui::tooltip::*;

#[component]
fn ControlledTooltip() -> impl IntoView {
    let (open, set_open) = signal(false);

    view! {
        <TooltipProvider>
            <Tooltip
                open=open
                on_open_change=Callback::new(move |val: bool| set_open.set(val))
            >
                <TooltipTrigger>
                    <button>"Controlled"</button>
                </TooltipTrigger>
                <TooltipPortal>
                    <TooltipContent>
                        "Controlled tooltip"
                        <TooltipArrow />
                    </TooltipContent>
                </TooltipPortal>
            </Tooltip>
        </TooltipProvider>
    }
}
```

### Custom delay

Override the global delay for a specific tooltip using the `delay_duration` prop.

```rust,ignore
use leptos::prelude::*;
use pith_ui::tooltip::*;

#[component]
fn InstantTooltip() -> impl IntoView {
    view! {
        <TooltipProvider>
            <Tooltip delay_duration=0.0>
                <TooltipTrigger>
                    <button>"Instant"</button>
                </TooltipTrigger>
                <TooltipPortal>
                    <TooltipContent>
                        "Shows instantly"
                        <TooltipArrow />
                    </TooltipContent>
                </TooltipPortal>
            </Tooltip>
        </TooltipProvider>
    }
}
```

## Accessibility

Adheres to the [Tooltip WAI-ARIA design pattern](https://www.w3.org/WAI/ARIA/apg/patterns/tooltip/).

### Keyboard Interactions

| Key      | Description                                |
| -------- | ------------------------------------------ |
| `Tab`    | Opens/closes the tooltip without delay.    |
| `Space`  | If open, closes the tooltip without delay. |
| `Enter`  | If open, closes the tooltip without delay. |
| `Escape` | If open, closes the tooltip without delay. |

## See Also

-   [Radix documentation](https://www.radix-ui.com/primitives/docs/components/tooltip)
