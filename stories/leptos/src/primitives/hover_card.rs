use leptos::prelude::*;
use radix_leptos_dialog::{Dialog, DialogClose, DialogContent, DialogTitle, DialogTrigger};
use radix_leptos_hover_card::*;
use radix_leptos_popper::{Align, Side};
use web_sys::wasm_bindgen::JsCast;

stylance::import_crate_style!(classes, "src/primitives/hover_card.stories.module.css");

fn content_class(animated: bool) -> String {
    if animated {
        format!("{} {}", classes::content, classes::animatedContent)
    } else {
        classes::content.to_string()
    }
}

#[component]
pub fn Basic() -> impl IntoView {
    view! {
        <div style="padding: 50px; display: flex; justify-content: center;">
            <HoverCard>
                <HoverCardTrigger attr:href="/" attr:class="trigger">
                    "trigger"
                </HoverCardTrigger>
                <HoverCardPortal>
                    <HoverCardContent attr:class=classes::content side_offset=5.0>
                        <HoverCardArrow attr:class=classes::arrow width=20.0 height=10.0 />
                        <CardContentPlaceholder />
                    </HoverCardContent>
                </HoverCardPortal>
            </HoverCard>
        </div>
    }
}

#[component]
pub fn ContainTextSelection() -> impl IntoView {
    view! {
        <div style="padding: 50px; display: flex; justify-content: center; flex-direction: column; align-items: center;">
            <div style="display: flex; gap: 30px;">
                <HoverCard>
                    <HoverCardTrigger attr:href="/" attr:class="trigger">
                        "single"
                    </HoverCardTrigger>
                    <HoverCardPortal>
                        <HoverCardContent attr:class=content_class(true) side_offset=5.0>
                            <HoverCardArrow attr:class=classes::arrow width=20.0 height=10.0 />
                            <div style="max-width: 400px;">
                                "Text selections will be contained within the content. While a selection is active the content will not dismiss unless the selection is cleared or an outside interaction is performed."
                            </div>
                        </HoverCardContent>
                    </HoverCardPortal>
                </HoverCard>

                <HoverCard>
                    <HoverCardTrigger attr:href="/" attr:class="trigger">
                        "nested"
                    </HoverCardTrigger>
                    <HoverCardPortal>
                        <HoverCardContent attr:class=content_class(true) side_offset=5.0>
                            <HoverCardArrow attr:class=classes::arrow width=20.0 height=10.0 />
                            <div style="max-width: 400px;">
                                "Text selections will be contained within the content. While a selection is active the content will not dismiss unless the selection is cleared or an outside interaction is performed."
                            </div>

                            <HoverCard>
                                <HoverCardTrigger attr:href="/" attr:class="trigger">
                                    "nested trigger"
                                </HoverCardTrigger>
                                <HoverCardPortal>
                                    <HoverCardContent
                                        attr:class=content_class(true)
                                        side_offset=5.0
                                        attr:style="background-color: crimson;"
                                    >
                                        <HoverCardArrow attr:class=classes::arrow width=20.0 height=10.0 attr:style="fill: crimson;" />
                                        <div style="max-width: 400px;">
                                            "Text selections will be contained within the content. While a selection is active the content will not dismiss unless the selection is cleared or an outside interaction is performed."
                                        </div>

                                        <HoverCard>
                                            <HoverCardTrigger attr:href="/" attr:class="trigger">
                                                "nested trigger"
                                            </HoverCardTrigger>
                                            <HoverCardPortal>
                                                <HoverCardContent
                                                    attr:class=content_class(true)
                                                    side_offset=5.0
                                                    attr:style="background-color: green;"
                                                >
                                                    <HoverCardArrow attr:class=classes::arrow width=20.0 height=10.0 attr:style="fill: green;" />
                                                    <div style="max-width: 400px;">
                                                        "Text selections will be contained within the content. While a selection is active the content will not dismiss unless the selection is cleared or an outside interaction is performed."
                                                    </div>
                                                </HoverCardContent>
                                            </HoverCardPortal>
                                        </HoverCard>
                                    </HoverCardContent>
                                </HoverCardPortal>
                            </HoverCard>
                        </HoverCardContent>
                    </HoverCardPortal>
                </HoverCard>
            </div>
            <div style="max-width: 800px;">
                <p>
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer feugiat mattis malesuada. Fusce elementum vulputate aliquet. Integer fringilla porta eros. Ut ultricies mattis nisi. Sed et tempor massa. Sed non arcu ut velit scelerisque bibendum tempor sed mi. In non consequat sapien. Donec sollicitudin eget tellus ut venenatis. Donec posuere sem ante, nec iaculis arcu varius sit amet. Praesent non tortor quam. Curabitur dapibus justo a commodo ornare."
                </p>
                <p>
                    "Suspendisse eleifend consequat iaculis. Nunc bibendum velit felis, nec vulputate purus egestas quis. Integer mauris dui, pulvinar non metus id, tristique dignissim elit. Vivamus massa tellus, porttitor id lorem non, molestie aliquam dolor. Pellentesque erat quam, pellentesque non metus id, tempus sagittis massa."
                </p>
                <p>
                    "Sed at elementum sem, non venenatis leo. Ut vulputate consectetur finibus. Sed nunc lectus, accumsan in nisl et, vehicula pretium nisi. Vivamus vestibulum ante quis urna consequat, ultrices condimentum sem commodo. Pellentesque eget orci laoreet, feugiat purus sed, maximus nisi. Suspendisse commodo venenatis facilisis."
                </p>
            </div>
        </div>
    }
}

#[component]
pub fn AsyncUpdate() -> impl IntoView {
    let (open, set_open) = signal(false);
    let (content_loaded, set_content_loaded) = signal(false);
    let timer_ref: StoredValue<Option<i32>> = StoredValue::new(None);

    let handle_open_change = Callback::new(move |new_open: bool| {
        if let Some(id) = timer_ref.get_value() {
            web_sys::window()
                .expect("Window should exist.")
                .clear_timeout_with_handle(id);
        }

        if new_open {
            let closure = web_sys::wasm_bindgen::closure::Closure::once_into_js(move || {
                set_content_loaded.set(true);
            });
            let id = web_sys::window()
                .expect("Window should exist.")
                .set_timeout_with_callback_and_timeout_and_arguments_0(closure.unchecked_ref(), 500)
                .expect("setTimeout should succeed.");
            timer_ref.set_value(Some(id));
        } else {
            set_content_loaded.set(false);
        }

        set_open.set(new_open);
    });

    on_cleanup(move || {
        if let Some(id) = timer_ref.get_value() {
            web_sys::window()
                .expect("Window should exist.")
                .clear_timeout_with_handle(id);
        }
    });

    view! {
        <div style="padding: 50px; display: flex; justify-content: center;">
            <HoverCard open=open on_open_change=handle_open_change>
                <HoverCardTrigger attr:href="/" attr:class="trigger">
                    "trigger"
                </HoverCardTrigger>
                <HoverCardPortal>
                    <HoverCardContent attr:class=classes::content side_offset=5.0>
                        <HoverCardArrow attr:class=classes::arrow width=20.0 height=10.0 />
                        {move || if content_loaded.get() {
                            view! { <CardContentPlaceholder /> }.into_any()
                        } else {
                            view! { "Loading..." }.into_any()
                        }}
                    </HoverCardContent>
                </HoverCardPortal>
            </HoverCard>
        </div>
    }
}

#[component]
pub fn CustomDurations() -> impl IntoView {
    view! {
        <div>
            <h1>"Delay duration"</h1>
            <h2>"Default (700ms open, 300ms close)"</h2>

            <HoverCard>
                <HoverCardTrigger attr:href="/" attr:class="trigger">
                    "trigger"
                </HoverCardTrigger>
                <HoverCardPortal>
                    <HoverCardContent attr:class=classes::content>
                        <CardContentPlaceholder />
                    </HoverCardContent>
                </HoverCardPortal>
            </HoverCard>

            <h2>"Custom (instant, 0ms open, 0ms close)"</h2>
            <HoverCard open_delay=0.0 close_delay=0.0>
                <HoverCardTrigger attr:href="/" attr:class="trigger">
                    "trigger"
                </HoverCardTrigger>
                <HoverCardPortal>
                    <HoverCardContent attr:class=classes::content>
                        <CardContentPlaceholder />
                    </HoverCardContent>
                </HoverCardPortal>
            </HoverCard>

            <h2>"Custom (300ms open, 100ms close)"</h2>

            <HoverCard open_delay=300.0 close_delay=100.0>
                <HoverCardTrigger attr:href="/" attr:class="trigger">
                    "trigger"
                </HoverCardTrigger>
                <HoverCardPortal>
                    <HoverCardContent attr:class=classes::content>
                        <CardContentPlaceholder />
                    </HoverCardContent>
                </HoverCardPortal>
            </HoverCard>
        </div>
    }
}

#[component]
pub fn Controlled() -> impl IntoView {
    let (open, set_open) = signal(false);

    view! {
        <div style="padding: 50px; display: flex; justify-content: center;">
            <HoverCard open=open on_open_change=Callback::new(move |value: bool| set_open.set(value))>
                <HoverCardTrigger attr:href="/" attr:class="trigger">
                    "trigger"
                </HoverCardTrigger>
                <HoverCardPortal>
                    <HoverCardContent attr:class=classes::content>
                        <HoverCardArrow attr:class=classes::arrow width=20.0 height=10.0 />
                        <CardContentPlaceholder />
                    </HoverCardContent>
                </HoverCardPortal>
            </HoverCard>
        </div>
    }
}

#[component]
pub fn Layerable() -> impl IntoView {
    view! {
        <div style="padding: 50px; display: flex; justify-content: center;">
            <Dialog>
                <DialogTrigger>"Open"</DialogTrigger>
                <DialogContent
                    attr:style="position: fixed; top: 50%; left: 50%; transform: translate(-50%, -50%); background: white; border: 1px solid; border-radius: 4px; padding: 20px;"
                >
                    <DialogTitle>"Some dialog title"</DialogTitle>
                    "Some dialog content with a "
                    <HoverCard>
                        <HoverCardTrigger attr:href="/" attr:class="trigger">
                            "trigger"
                        </HoverCardTrigger>
                        <HoverCardPortal>
                            <HoverCardContent attr:class=classes::content side_offset=5.0>
                                <HoverCardArrow attr:class=classes::arrow width=20.0 height=10.0 />
                                <CardContentPlaceholder />
                            </HoverCardContent>
                        </HoverCardPortal>
                    </HoverCard>
                    " "
                    <DialogClose>"Close"</DialogClose>
                </DialogContent>
            </Dialog>
        </div>
    }
}

#[component]
pub fn Animated() -> impl IntoView {
    view! {
        <div style="padding: 50px; display: flex; justify-content: center;">
            <HoverCard>
                <HoverCardTrigger attr:href="/" attr:class="trigger">
                    "trigger"
                </HoverCardTrigger>
                <HoverCardPortal>
                    <HoverCardContent attr:class=content_class(true) side_offset=10.0>
                        <HoverCardArrow attr:class=classes::arrow width=20.0 height=10.0 />
                        <CardContentPlaceholder />
                    </HoverCardContent>
                </HoverCardPortal>
            </HoverCard>
        </div>
    }
}

#[component]
pub fn ForcedMount() -> impl IntoView {
    view! {
        <div style="padding: 50px; display: flex; justify-content: center;">
            <HoverCard>
                <HoverCardTrigger attr:href="/" attr:class="trigger">
                    "trigger"
                </HoverCardTrigger>
                <HoverCardPortal force_mount=true>
                    <HoverCardContent attr:class=classes::content side_offset=10.0>
                        <HoverCardArrow attr:class=classes::arrow width=20.0 height=10.0 />
                        <CardContentPlaceholder />
                    </HoverCardContent>
                </HoverCardPortal>
            </HoverCard>
        </div>
    }
}

#[component]
pub fn Nested() -> impl IntoView {
    view! {
        <HoverCard>
            <HoverCardTrigger attr:href="/" attr:class="trigger">
                "trigger level 1"
            </HoverCardTrigger>

            <HoverCardPortal>
                <HoverCardContent
                    attr:class=classes::content
                    side_offset=5.0
                    attr:style="background-color: crimson;"
                >
                    <HoverCard>
                        <HoverCardTrigger attr:href="/" attr:class="trigger">
                            "trigger level 2"
                        </HoverCardTrigger>
                        <HoverCardPortal>
                            <HoverCardContent
                                attr:class=classes::content
                                side=Side::Top
                                align=Align::Center
                                side_offset=5.0
                                attr:style="background-color: green;"
                            >
                                <HoverCardArrow attr:class=classes::arrow width=20.0 height=10.0 attr:style="fill: green;" />
                                <HoverCard>
                                    <HoverCardTrigger attr:href="/" attr:class="trigger">
                                        "trigger level 3"
                                    </HoverCardTrigger>
                                    <HoverCardPortal>
                                        <HoverCardContent
                                            attr:class=classes::content
                                            side=Side::Bottom
                                            align=Align::Start
                                            side_offset=5.0
                                            attr:style="background-color: purple;"
                                        >
                                            <HoverCardArrow attr:class=classes::arrow width=20.0 height=10.0 attr:style="fill: purple;" />
                                            "level 3"
                                        </HoverCardContent>
                                    </HoverCardPortal>
                                </HoverCard>
                            </HoverCardContent>
                        </HoverCardPortal>
                    </HoverCard>

                    <HoverCardArrow attr:class=classes::arrow width=20.0 height=10.0 attr:style="fill: crimson;" />
                </HoverCardContent>
            </HoverCardPortal>
        </HoverCard>
    }
}

#[component]
pub fn NonPortal() -> impl IntoView {
    view! {
        <div>
            <button>"button"</button>
            <HoverCard>
                <HoverCardTrigger attr:href="/" attr:class="trigger">
                    "trigger"
                </HoverCardTrigger>
                <HoverCardContent attr:class=classes::content side_offset=5.0>
                    <HoverCardArrow attr:class=classes::arrow width=20.0 height=10.0 />
                    <a href="#link">"Should not be able to focus me"</a>
                    <CardContentPlaceholder />
                </HoverCardContent>
            </HoverCard>
            <button>"button"</button>
        </div>
    }
}

#[component]
pub fn WithSlottedTrigger() -> impl IntoView {
    view! {
        <HoverCard>
            <HoverCardTrigger as_child=true>
                <button class="trigger" on:click=move |_| {
                    web_sys::console::log_1(&"StyledTrigger click".into());
                }>
                    "trigger"
                </button>
            </HoverCardTrigger>
            <HoverCardPortal>
                <HoverCardContent attr:class=classes::content side_offset=5.0>
                    <HoverCardArrow attr:class=classes::arrow width=20.0 height=10.0 />
                    <CardContentPlaceholder />
                </HoverCardContent>
            </HoverCardPortal>
        </HoverCard>
    }
}

#[component]
pub fn WithSlottedContent() -> impl IntoView {
    view! {
        <HoverCard>
            <HoverCardTrigger attr:href="/" attr:class="trigger">
                "trigger"
            </HoverCardTrigger>
            <HoverCardPortal>
                <HoverCardContent as_child=true side_offset=5.0>
                    <div class=classes::content>
                        <HoverCardArrow attr:class=classes::arrow width=20.0 height=10.0 />
                        <CardContentPlaceholder />
                    </div>
                </HoverCardContent>
            </HoverCardPortal>
        </HoverCard>
    }
}

// Change order slightly for more pleasing visual
const SIDES: [Side; 4] = [Side::Top, Side::Right, Side::Left, Side::Bottom];
const ALIGNS: [Align; 3] = [Align::Start, Align::Center, Align::End];

fn side_name(side: Side) -> &'static str {
    match side {
        Side::Top => "top",
        Side::Right => "right",
        Side::Bottom => "bottom",
        Side::Left => "left",
    }
}

fn align_name(align: Align) -> &'static str {
    match align {
        Align::Start => "start",
        Align::Center => "center",
        Align::End => "end",
    }
}

#[component]
pub fn Chromatic() -> impl IntoView {
    view! {
        <div style="padding: 200px; padding-bottom: 500px;">
            <h1>"Uncontrolled"</h1>
            <h2>"Closed"</h2>
            <HoverCard>
                <HoverCardTrigger attr:class="trigger">"open"</HoverCardTrigger>
                <HoverCardPortal>
                    <HoverCardContent attr:class=classes::content side_offset=5.0>
                        <HoverCardArrow attr:class=classes::arrow width=20.0 height=10.0 />
                        "Some content"
                    </HoverCardContent>
                </HoverCardPortal>
            </HoverCard>

            <h2>"Open"</h2>
            <HoverCard default_open=true>
                <HoverCardTrigger attr:class="trigger">"open"</HoverCardTrigger>
                <HoverCardPortal>
                    <HoverCardContent attr:class=classes::content side_offset=5.0>
                        <HoverCardArrow attr:class=classes::arrow width=20.0 height=10.0 />
                        "Some content"
                    </HoverCardContent>
                </HoverCardPortal>
            </HoverCard>

            <h2 style="margin-top: 60px;">"Open with reordered parts"</h2>
            <HoverCard default_open=true>
                <HoverCardPortal>
                    <HoverCardContent attr:class=classes::content side_offset=5.0>
                        "Some content"
                        <HoverCardArrow attr:class=classes::arrow />
                    </HoverCardContent>
                </HoverCardPortal>
                <HoverCardTrigger attr:class="trigger">"open"</HoverCardTrigger>
            </HoverCard>

            <h1 style="margin-top: 100px;">"Controlled"</h1>
            <h2>"Closed"</h2>
            <HoverCard open=false>
                <HoverCardTrigger attr:class="trigger">"open"</HoverCardTrigger>
                <HoverCardPortal>
                    <HoverCardContent attr:class=classes::content side_offset=5.0>
                        <HoverCardArrow attr:class=classes::arrow width=20.0 height=10.0 />
                        "Some content"
                    </HoverCardContent>
                </HoverCardPortal>
            </HoverCard>

            <h2>"Open"</h2>
            <HoverCard open=true>
                <HoverCardTrigger attr:class="trigger">"open"</HoverCardTrigger>
                <HoverCardPortal>
                    <HoverCardContent attr:class=classes::content side_offset=5.0>
                        <HoverCardArrow attr:class=classes::arrow width=20.0 height=10.0 />
                        "Some content"
                    </HoverCardContent>
                </HoverCardPortal>
            </HoverCard>

            <h2 style="margin-top: 60px;">"Open with reordered parts"</h2>
            <HoverCard open=true>
                <HoverCardPortal>
                    <HoverCardContent attr:class=classes::content side_offset=5.0>
                        "Some content"
                        <HoverCardArrow attr:class=classes::arrow />
                    </HoverCardContent>
                </HoverCardPortal>
                <HoverCardTrigger attr:class="trigger">"open"</HoverCardTrigger>
            </HoverCard>

            <h1 style="margin-top: 100px;">"Force mounted content"</h1>
            <HoverCard>
                <HoverCardTrigger attr:class="trigger">"open"</HoverCardTrigger>
                <HoverCardPortal force_mount=true>
                    <HoverCardContent attr:class=classes::content side_offset=5.0>
                        <HoverCardArrow attr:class=classes::arrow width=20.0 height=10.0 />
                        "Some content"
                    </HoverCardContent>
                </HoverCardPortal>
            </HoverCard>

            <h1 style="margin-top: 100px;">"Positioning"</h1>
            <h2>"No collisions"</h2>
            <h3>"Side & Align"</h3>
            <div class=classes::grid>
                {SIDES.iter().flat_map(|side| {
                    let side = *side;
                    ALIGNS.iter().map(move |align| {
                        let align = *align;
                        view! {
                            <HoverCard open=true>
                                <HoverCardTrigger attr:class=classes::chromaticTrigger />
                                <HoverCardPortal>
                                    <HoverCardContent
                                        attr:class=classes::chromaticContent
                                        side=side
                                        align=align
                                        avoid_collisions=false
                                    >
                                        <p style="text-align: center;">
                                            {side_name(side)}<br />{align_name(align)}
                                        </p>
                                        <HoverCardArrow attr:class=classes::chromaticArrow width=20.0 height=10.0 />
                                    </HoverCardContent>
                                </HoverCardPortal>
                            </HoverCard>
                        }
                    })
                }).collect_view()}
            </div>

            <h3>"Side offset"</h3>
            <h4>"Positive"</h4>
            <div class=classes::grid>
                {SIDES.iter().flat_map(|side| {
                    let side = *side;
                    ALIGNS.iter().map(move |align| {
                        let align = *align;
                        view! {
                            <HoverCard open=true>
                                <HoverCardTrigger attr:class=classes::chromaticTrigger />
                                <HoverCardPortal>
                                    <HoverCardContent
                                        attr:class=classes::chromaticContent
                                        side=side
                                        side_offset=5.0
                                        align=align
                                        avoid_collisions=false
                                    >
                                        <p style="text-align: center;">
                                            {side_name(side)}<br />{align_name(align)}
                                        </p>
                                        <HoverCardArrow attr:class=classes::chromaticArrow width=20.0 height=10.0 />
                                    </HoverCardContent>
                                </HoverCardPortal>
                            </HoverCard>
                        }
                    })
                }).collect_view()}
            </div>
            <h4>"Negative"</h4>
            <div class=classes::grid>
                {SIDES.iter().flat_map(|side| {
                    let side = *side;
                    ALIGNS.iter().map(move |align| {
                        let align = *align;
                        view! {
                            <HoverCard open=true>
                                <HoverCardTrigger attr:class=classes::chromaticTrigger />
                                <HoverCardPortal>
                                    <HoverCardContent
                                        attr:class=classes::chromaticContent
                                        side=side
                                        side_offset=-10.0
                                        align=align
                                        avoid_collisions=false
                                    >
                                        <p style="text-align: center;">
                                            {side_name(side)}<br />{align_name(align)}
                                        </p>
                                        <HoverCardArrow attr:class=classes::chromaticArrow width=20.0 height=10.0 />
                                    </HoverCardContent>
                                </HoverCardPortal>
                            </HoverCard>
                        }
                    })
                }).collect_view()}
            </div>

            <h3>"Align offset"</h3>
            <h4>"Positive"</h4>
            <div class=classes::grid>
                {SIDES.iter().flat_map(|side| {
                    let side = *side;
                    ALIGNS.iter().map(move |align| {
                        let align = *align;
                        view! {
                            <HoverCard open=true>
                                <HoverCardTrigger attr:class=classes::chromaticTrigger />
                                <HoverCardPortal>
                                    <HoverCardContent
                                        attr:class=classes::chromaticContent
                                        side=side
                                        align=align
                                        align_offset=20.0
                                        avoid_collisions=false
                                    >
                                        <p style="text-align: center;">
                                            {side_name(side)}<br />{align_name(align)}
                                        </p>
                                        <HoverCardArrow attr:class=classes::chromaticArrow width=20.0 height=10.0 />
                                    </HoverCardContent>
                                </HoverCardPortal>
                            </HoverCard>
                        }
                    })
                }).collect_view()}
            </div>
            <h4>"Negative"</h4>
            <div class=classes::grid>
                {SIDES.iter().flat_map(|side| {
                    let side = *side;
                    ALIGNS.iter().map(move |align| {
                        let align = *align;
                        view! {
                            <HoverCard open=true>
                                <HoverCardTrigger attr:class=classes::chromaticTrigger />
                                <HoverCardPortal>
                                    <HoverCardContent
                                        attr:class=classes::chromaticContent
                                        side=side
                                        align=align
                                        align_offset=-10.0
                                        avoid_collisions=false
                                    >
                                        <p style="text-align: center;">
                                            {side_name(side)}<br />{align_name(align)}
                                        </p>
                                        <HoverCardArrow attr:class=classes::chromaticArrow width=20.0 height=10.0 />
                                    </HoverCardContent>
                                </HoverCardPortal>
                            </HoverCard>
                        }
                    })
                }).collect_view()}
            </div>

            <h2>"Collisions"</h2>
            <p>"See instances on the periphery of the page."</p>
            {SIDES.iter().flat_map(|side| {
                let side = *side;
                ALIGNS.iter().map(move |align| {
                    let align = *align;
                    let trigger_style = {
                        let mut s = format!("position: absolute; {}: 10px;", side_name(side));
                        match side {
                            Side::Right | Side::Left => {
                                match align {
                                    Align::Start => s.push_str(" bottom: 10px;"),
                                    Align::Center => s.push_str(" top: calc(50% - 15px);"),
                                    Align::End => s.push_str(" top: 10px;"),
                                }
                            }
                            Side::Top | Side::Bottom => {
                                match align {
                                    Align::Start => s.push_str(" right: 10px;"),
                                    Align::Center => s.push_str(" left: calc(50% - 15px);"),
                                    Align::End => s.push_str(" left: 10px;"),
                                }
                            }
                        }
                        s
                    };
                    view! {
                        <HoverCard open=true>
                            <HoverCardTrigger
                                attr:class=classes::chromaticTrigger
                                attr:style=trigger_style.clone()
                            />
                            <HoverCardPortal>
                                <HoverCardContent attr:class=classes::chromaticContent side=side align=align>
                                    <p style="text-align: center;">
                                        {side_name(side)}<br />{align_name(align)}
                                    </p>
                                    <HoverCardArrow attr:class=classes::chromaticArrow width=20.0 height=10.0 />
                                </HoverCardContent>
                            </HoverCardPortal>
                        </HoverCard>
                    }
                })
            }).collect_view()}

            <h2>"Relative parent (non-portalled)"</h2>
            <div style="position: relative;">
                <HoverCard open=true>
                    <HoverCardTrigger attr:href="/" attr:class="trigger">
                        "trigger"
                    </HoverCardTrigger>
                    <HoverCardContent attr:class=classes::content side_offset=5.0>
                        <HoverCardArrow attr:class=classes::arrow width=20.0 height=10.0 />
                        "Some content"
                    </HoverCardContent>
                </HoverCard>
            </div>

            <h1 style="margin-top: 100px;">"With slotted trigger"</h1>
            <HoverCard open=true>
                <HoverCardTrigger as_child=true>
                    <button class="trigger">"open"</button>
                </HoverCardTrigger>
                <HoverCardPortal>
                    <HoverCardContent attr:class=classes::content side_offset=5.0>
                        <HoverCardArrow attr:class=classes::arrow width=20.0 height=10.0 />
                        "Some content"
                    </HoverCardContent>
                </HoverCardPortal>
            </HoverCard>

            <h1 style="margin-top: 100px;">"State attributes"</h1>
            <h2>"Closed"</h2>
            <HoverCard open=false>
                <HoverCardTrigger attr:class=classes::triggerAttr>"open"</HoverCardTrigger>
                <HoverCardPortal>
                    <HoverCardContent attr:class=classes::contentAttr side_offset=5.0 avoid_collisions=false>
                        <HoverCardArrow attr:class=classes::arrowAttr width=20.0 height=10.0 />
                        "Some content"
                    </HoverCardContent>
                </HoverCardPortal>
            </HoverCard>

            <h2>"Open"</h2>
            <HoverCard open=true>
                <HoverCardTrigger attr:class=classes::triggerAttr>"open"</HoverCardTrigger>
                <HoverCardPortal>
                    <HoverCardContent
                        attr:class=classes::contentAttr
                        side=Side::Right
                        side_offset=5.0
                        avoid_collisions=false
                    >
                        <HoverCardArrow attr:class=classes::arrowAttr width=20.0 height=10.0 />
                        "Some content"
                    </HoverCardContent>
                </HoverCardPortal>
            </HoverCard>
        </div>
    }
}

#[component]
fn CardContentPlaceholder() -> impl IntoView {
    view! {
        <div style="max-width: 400px; display: flex; align-items: center;">
            <div style="width: 60px; height: 60px; background-color: white; border-radius: 100px;" />
            <div style="margin-left: 14px;">
                <div style="width: 200px; background-color: white; height: 14px; border-radius: 100px;" />
                <div style="width: 150px; background-color: white; height: 14px; border-radius: 100px; margin-top: 10px;" />
            </div>
        </div>
    }
}
