use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use cardo_ui::dialog::{
    Dialog, DialogClose, DialogContent, DialogDescription, DialogTitle, DialogTrigger,
};
use cardo_ui::tooltip::{
    Align, Side, Tooltip, TooltipArrow, TooltipContent, TooltipPortal, TooltipProvider,
    TooltipTrigger,
};
use wasm_bindgen::JsCast;

stylance::import_crate_style!(classes, "src/primitives/tooltip.stories.module.css");

fn simple_tooltip(
    children: ChildrenFn,
    label: &'static str,
    aria_label: Option<&'static str>,
    class_name: Option<String>,
    side: Option<Side>,
    align: Option<Align>,
    open: Option<Signal<bool>>,
    on_open_change: Option<Callback<bool>>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let content_class =
        StoredValue::new(class_name.unwrap_or_else(|| classes::content.to_string()));
    let side = side.unwrap_or(Side::Top);
    let align = align.unwrap_or(Align::Center);
    let on_open_change = on_open_change.unwrap_or(Callback::new(|_| {}));

    let open_prop: MaybeProp<bool> = open
        .map(|s| MaybeProp::from(Signal::derive(move || Some(s.get()))))
        .unwrap_or_default();

    let aria_label_prop: MaybeProp<String> = aria_label
        .map(|s| MaybeProp::from(s.to_string()))
        .unwrap_or_default();

    view! {
        <Tooltip
            open=open_prop
            on_open_change=on_open_change
        >
            {children.with_value(|children| children())}
            <TooltipPortal>
                <TooltipContent
                    attr:class=move || content_class.get_value()
                    side_offset=5.0
                    aria_label=aria_label_prop
                    side=side
                    align=align
                >
                    {label}
                    <TooltipArrow attr:class=classes::arrow />
                </TooltipContent>
            </TooltipPortal>
        </Tooltip>
    }
}

#[component]
pub fn Styled() -> impl IntoView {
    view! {
        <TooltipProvider>
            <Tooltip>
                <TooltipTrigger attr:class=classes::trigger>
                    "Hover or Focus me"
                </TooltipTrigger>
                <TooltipPortal>
                    <TooltipContent attr:class=classes::content side_offset=5.0>
                        "Nicely done!"
                        <TooltipArrow attr:class=classes::arrow />
                    </TooltipContent>
                </TooltipPortal>
            </Tooltip>
        </TooltipProvider>
    }
}

#[component]
pub fn Controlled() -> impl IntoView {
    let (open, set_open) = signal(true);

    view! {
        <TooltipProvider>
            <Tooltip
                open=open
                on_open_change=Callback::new(move |val: bool| set_open.set(val))
            >
                <TooltipTrigger attr:style="margin: 100px;">
                    {move || if open.get() {
                        "I'm controlled, look I'm open"
                    } else {
                        "I'm controlled, look I'm closed"
                    }}
                </TooltipTrigger>
                <TooltipPortal>
                    <TooltipContent attr:class=classes::content side_offset=5.0>
                        "Nicely done!"
                        <TooltipArrow attr:class=classes::arrow />
                    </TooltipContent>
                </TooltipPortal>
            </Tooltip>
        </TooltipProvider>
    }
}

fn tooltip_item() -> impl IntoView {
    view! {
        <Tooltip>
            <TooltipTrigger attr:class=classes::trigger>"Hover me"</TooltipTrigger>
            <TooltipPortal>
                <TooltipContent attr:class=classes::content side_offset=5.0>
                    "Nicely done!"
                    <TooltipArrow attr:class=classes::arrow />
                </TooltipContent>
            </TooltipPortal>
        </Tooltip>
    }
}

#[component]
pub fn CustomDurations() -> impl IntoView {
    view! {
        <TooltipProvider>
            <h1>"Delay duration"</h1>
            <h2>"Default (700ms)"</h2>
            <div style="display: flex; gap: 50px;">
                {tooltip_item()}
                {tooltip_item()}
                {tooltip_item()}
            </div>

            <h2>"Custom (0ms = instant open)"</h2>
            <div style="display: flex; gap: 50px;">
                <TooltipProvider delay_duration=0.0>
                    {tooltip_item()}
                    {tooltip_item()}
                    {tooltip_item()}
                </TooltipProvider>
            </div>

            <h2>"Custom (2s)"</h2>
            <div style="display: flex; gap: 50px;">
                <TooltipProvider delay_duration=2000.0>
                    {tooltip_item()}
                    {tooltip_item()}
                    {tooltip_item()}
                </TooltipProvider>
            </div>

            <h1>"Skip delay duration"</h1>
            <h2>"Default (300ms to move from one to another tooltip)"</h2>
            <div style="display: flex; gap: 50px;">
                {tooltip_item()}
                {tooltip_item()}
                {tooltip_item()}
            </div>

            <h2>"Custom (0ms to move from one to another tooltip = never skip)"</h2>
            <div style="display: flex; gap: 50px;">
                <TooltipProvider skip_delay_duration=0.0>
                    {tooltip_item()}
                    {tooltip_item()}
                    {tooltip_item()}
                </TooltipProvider>
            </div>

            <h2>"Custom (5s to move from one to another tooltip)"</h2>
            <div style="display: flex; gap: 50px;">
                <TooltipProvider skip_delay_duration=5000.0>
                    {tooltip_item()}
                    {tooltip_item()}
                    {tooltip_item()}
                </TooltipProvider>
            </div>
        </TooltipProvider>
    }
}

#[component]
pub fn Positions() -> impl IntoView {
    view! {
        <TooltipProvider>
            <div style="display: flex; width: 100vw; height: 100vh; align-items: center; justify-content: center;">
                <div style="display: grid; grid-template-columns: repeat(5, 1fr); grid-template-rows: repeat(5, 50px);">
                    {position_tooltip("Top start", Side::Top, Align::Start, "grid-column: 2; grid-row: 1;")}
                    {position_tooltip("Top center", Side::Top, Align::Center, "grid-column: 3; grid-row: 1;")}
                    {position_tooltip("Top end", Side::Top, Align::End, "grid-column: 4; grid-row: 1;")}
                    {position_tooltip("Right start", Side::Right, Align::Start, "grid-column: 5; grid-row: 2;")}
                    {position_tooltip("Right center", Side::Right, Align::Center, "grid-column: 5; grid-row: 3;")}
                    {position_tooltip("Right end", Side::Right, Align::End, "grid-column: 5; grid-row: 4;")}
                    {position_tooltip("Bottom end", Side::Bottom, Align::End, "grid-column: 4; grid-row: 5;")}
                    {position_tooltip("Bottom center", Side::Bottom, Align::Center, "grid-column: 3; grid-row: 5;")}
                    {position_tooltip("Bottom start", Side::Bottom, Align::Start, "grid-column: 2; grid-row: 5;")}
                    {position_tooltip("Left end", Side::Left, Align::End, "grid-column: 1; grid-row: 4;")}
                    {position_tooltip("Left center", Side::Left, Align::Center, "grid-column: 1; grid-row: 3;")}
                    {position_tooltip("Left start", Side::Left, Align::Start, "grid-column: 1; grid-row: 2;")}
                </div>
            </div>
        </TooltipProvider>
    }
}

fn position_tooltip(
    label: &'static str,
    side: Side,
    align: Align,
    style: &'static str,
) -> impl IntoView {
    simple_tooltip(
        ChildrenFn::to_children(move || {
            view! {
                <TooltipTrigger
                    attr:class=classes::positionButton
                    attr:style=style
                >
                    {label}
                </TooltipTrigger>
            }
            .into_view()
        }),
        label,
        None,
        None,
        Some(side),
        Some(align),
        None,
        None,
    )
}

#[component]
pub fn CustomContent() -> impl IntoView {
    view! {
        <TooltipProvider>
            <div style="display: flex; gap: 20px; padding: 100px;">
                <Tooltip>
                    <TooltipTrigger>"Heading"</TooltipTrigger>
                    <TooltipPortal>
                        <TooltipContent attr:class=classes::content side_offset=5.0>
                            <h1>"Some heading"</h1>
                            <TooltipArrow attr:class=classes::arrow />
                        </TooltipContent>
                    </TooltipPortal>
                </Tooltip>

                <Tooltip>
                    <TooltipTrigger>"Paragraph"</TooltipTrigger>
                    <TooltipPortal>
                        <TooltipContent attr:class=classes::content side_offset=5.0>
                            <p>"Some paragraph"</p>
                            <TooltipArrow attr:class=classes::arrow />
                        </TooltipContent>
                    </TooltipPortal>
                </Tooltip>

                <Tooltip>
                    <TooltipTrigger>"List"</TooltipTrigger>
                    <TooltipPortal>
                        <TooltipContent attr:class=classes::content side_offset=5.0>
                            <ul>
                                <li>"One"</li>
                                <li>"Two"</li>
                                <li>"Three"</li>
                            </ul>
                            <TooltipArrow attr:class=classes::arrow />
                        </TooltipContent>
                    </TooltipPortal>
                </Tooltip>

                <Tooltip>
                    <TooltipTrigger>"Article"</TooltipTrigger>
                    <TooltipPortal>
                        <TooltipContent attr:class=classes::content side_offset=5.0>
                            <article>
                                "Lorem ipsum dolor sit amet consectetur, adipisicing elit. Harum, quae qui. Magnam delectus ex totam repellat amet distinctio unde, porro architecto voluptatibus nemo et nisi, voluptatem eligendi earum autem fugit."
                            </article>
                            <TooltipArrow attr:class=classes::arrow />
                        </TooltipContent>
                    </TooltipPortal>
                </Tooltip>

                <Tooltip>
                    <TooltipTrigger>"Figure"</TooltipTrigger>
                    <TooltipPortal>
                        <TooltipContent attr:class=classes::content side_offset=5.0>
                            <figure style="margin: 0;">
                                <img
                                    src="https://pbs.twimg.com/profile_images/864164353771229187/Catw6Nmh_400x400.jpg"
                                    alt=""
                                    width="100"
                                />
                                <figcaption>"Colm Tuite"</figcaption>
                            </figure>
                            <TooltipArrow attr:class=classes::arrow />
                        </TooltipContent>
                    </TooltipPortal>
                </Tooltip>

                <Tooltip>
                    <TooltipTrigger>"Time"</TooltipTrigger>
                    <TooltipPortal>
                        <TooltipContent attr:class=classes::content side_offset=5.0>
                            <time datetime="2017-10-31T11:21:00+02:00">"Tuesday, 31 October 2017"</time>
                            <TooltipArrow attr:class=classes::arrow />
                        </TooltipContent>
                    </TooltipPortal>
                </Tooltip>

                <Tooltip>
                    <TooltipTrigger>"Link"</TooltipTrigger>
                    <TooltipPortal>
                        <TooltipContent attr:class=classes::content side_offset=5.0>
                            "View in "
                            <a href="https://workos.com">"WorkOS"</a>
                            <TooltipArrow attr:class=classes::arrow />
                        </TooltipContent>
                    </TooltipPortal>
                </Tooltip>

                <Tooltip>
                    <TooltipTrigger>"Form"</TooltipTrigger>
                    <TooltipPortal>
                        <TooltipContent attr:class=classes::content side_offset=5.0>
                            <form>
                                <label for_="fname">"First name:"</label>
                                <br />
                                <input type="text" id="fname" name="fname" />
                                <br />
                                <label for_="lname">"Last name:"</label>
                                <br />
                                <input type="text" id="lname" name="lname" />
                            </form>
                            <TooltipArrow attr:class=classes::arrow />
                        </TooltipContent>
                    </TooltipPortal>
                </Tooltip>

                <Tooltip>
                    <TooltipTrigger>"Mini layout"</TooltipTrigger>
                    <TooltipPortal>
                        <TooltipContent attr:class=classes::content side_offset=5.0>
                            <p style="margin: 0; text-align: center; font-family: apple-system, BlinkMacSystemFont, helvetica, arial, sans-serif; font-size: 14px;">
                                "Start video call"
                                <span style="display: block; color: #999;">
                                    "press "
                                    <kbd
                                        style="font-family: apple-system, BlinkMacSystemFont, helvetica, arial, sans-serif; font-weight: bold; color: white;"
                                        aria-label="c key"
                                    >
                                        "c"
                                    </kbd>
                                </span>
                            </p>
                            <TooltipArrow attr:class=classes::arrow />
                        </TooltipContent>
                    </TooltipPortal>
                </Tooltip>
            </div>
        </TooltipProvider>
    }
}

#[component]
pub fn AriaLabel() -> impl IntoView {
    view! {
        <TooltipProvider>
            <p>"The first button will display AND enunciate the label."</p>
            <p>"The second button will display the label, but enunciate the aria label."</p>
            <div style="display: flex;">
                {simple_tooltip(
                    ChildrenFn::to_children(move || view! {
                        <TooltipTrigger attr:style="margin: 5px;">
                            <span aria-hidden="true">{"\u{1F514}(3)"}</span>
                        </TooltipTrigger>
                    }.into_view()),
                    "Notifications",
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                )}
                {simple_tooltip(
                    ChildrenFn::to_children(move || view! {
                        <TooltipTrigger attr:style="margin: 5px;">
                            <span aria-hidden="true">{"\u{1F514}(3)"}</span>
                        </TooltipTrigger>
                    }.into_view()),
                    "Notifications",
                    Some("3 notifications"),
                    None,
                    None,
                    None,
                    None,
                    None,
                )}
            </div>
        </TooltipProvider>
    }
}

#[component]
pub fn WithText() -> impl IntoView {
    view! {
        <TooltipProvider>
            <p>
                "Hello this is a test with "
                {simple_tooltip(
                    ChildrenFn::to_children(move || view! {
                        <TooltipTrigger as_child=true>
                            <a href="https://workos.com">"Tooltip.Root"</a>
                        </TooltipTrigger>
                    }.into_view()),
                    "This is a tooltip",
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                )}
                " inside a Text Component "
                {simple_tooltip(
                    ChildrenFn::to_children(move || view! {
                        <TooltipTrigger as_child=true>
                            <a href="https://workos.com">"Tooltip.Root"</a>
                        </TooltipTrigger>
                    }.into_view()),
                    "This is a tooltip",
                    None,
                    None,
                    Some(Side::Top),
                    None,
                    None,
                    None,
                )}
                " some more text "
                {simple_tooltip(
                    ChildrenFn::to_children(move || view! {
                        <TooltipTrigger as_child=true>
                            <a href="https://workos.com">"Tooltip.Root"</a>
                        </TooltipTrigger>
                    }.into_view()),
                    "This is a tooltip",
                    None,
                    None,
                    Some(Side::Right),
                    Some(Align::Center),
                    None,
                    None,
                )}
            </p>
        </TooltipProvider>
    }
}

#[component]
pub fn WithExternalRef() -> impl IntoView {
    let button_ref = AnyNodeRef::new();

    Effect::new(move |_| {
        if let Some(el) = button_ref.get() {
            let el: web_sys::HtmlElement = el.unchecked_into();
            let _ = el.style().set_property("box-shadow", "0 0 0 2px red");
        }
    });

    view! {
        <TooltipProvider>
            {simple_tooltip(
                ChildrenFn::to_children(move || view! {
                    <TooltipTrigger node_ref=button_ref attr:r#type="button" attr:style="margin: 100px;">
                        "Save"
                    </TooltipTrigger>
                }.into_view()),
                "Save document",
                None,
                None,
                Some(Side::Bottom),
                Some(Align::End),
                None,
                None,
            )}
        </TooltipProvider>
    }
}

#[component]
pub fn Unmount() -> impl IntoView {
    let (is_mounted, set_is_mounted) = signal(true);

    view! {
        <TooltipProvider>
            <ul>
                <li>"Focus the first button (tooltip 1 shows)"</li>
                <li>"Focus the second button (tooltip 2 shows)"</li>
                <li>"Press escape (second button unmounts)"</li>
                <li>"Focus the first button (tooltip 1 should still show)"</li>
            </ul>
            {simple_tooltip(
                ChildrenFn::to_children(move || view! {
                    <TooltipTrigger attr:style="align-self: flex-start; margin: 0vmin;">
                        "Tool 1"
                    </TooltipTrigger>
                }.into_view()),
                "tooltip 1",
                None,
                None,
                None,
                None,
                None,
                None,
            )}
            {move || is_mounted.get().then(|| {
                simple_tooltip(
                    ChildrenFn::to_children(move || view! {
                        <TooltipTrigger
                            attr:style="align-self: flex-start; margin: 0vmin;"
                            on:keydown=move |event: leptos::ev::KeyboardEvent| {
                                if event.key() == "Escape" {
                                    set_is_mounted.set(false);
                                }
                            }
                        >
                            "Tool 2"
                        </TooltipTrigger>
                    }.into_view()),
                    "tooltip 2",
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                )
            })}
        </TooltipProvider>
    }
}

#[component]
pub fn Animated() -> impl IntoView {
    let content_class = format!("{} {}", classes::content, classes::animatedContent);

    view! {
        <TooltipProvider>
            <div style="padding: 100px;">
                {simple_tooltip(
                    ChildrenFn::to_children(move || view! {
                        <TooltipTrigger attr:style="margin-right: 10px;">"Hello 1"</TooltipTrigger>
                    }.into_view()),
                    "Hello world 1",
                    None,
                    Some(content_class.clone()),
                    None,
                    None,
                    None,
                    None,
                )}
                {simple_tooltip(
                    ChildrenFn::to_children(move || view! {
                        <TooltipTrigger>"Hello 2"</TooltipTrigger>
                    }.into_view()),
                    "Hello world 2",
                    None,
                    Some(content_class.clone()),
                    Some(Side::Top),
                    None,
                    None,
                    None,
                )}
            </div>
        </TooltipProvider>
    }
}

#[component]
pub fn SlottableContent() -> impl IntoView {
    view! {
        <TooltipProvider>
            <Tooltip>
                <TooltipTrigger attr:class=classes::trigger>
                    "Hover or Focus me"
                </TooltipTrigger>
                <TooltipPortal>
                    <TooltipContent as_child=true side_offset=5.0>
                        <div class=classes::content>
                            "Nicely done!"
                            <TooltipArrow attr:class=classes::arrow />
                        </div>
                    </TooltipContent>
                </TooltipPortal>
            </Tooltip>
        </TooltipProvider>
    }
}

#[component]
pub fn WithinDialog() -> impl IntoView {
    view! {
        <TooltipProvider>
            <Dialog>
                <DialogTrigger>"Open dialog"</DialogTrigger>
                <DialogContent>
                    <DialogTitle>"Dialog title"</DialogTitle>
                    <DialogDescription>"Dialog description"</DialogDescription>
                    <DialogClose>"Close dialog"</DialogClose>
                    <Tooltip>
                        <TooltipTrigger attr:class=classes::trigger>
                            "Hover or Focus me"
                        </TooltipTrigger>
                        <TooltipPortal>
                            <TooltipContent attr:class=classes::content side_offset=5.0>
                                "Nicely done!"
                                <TooltipArrow attr:class=classes::arrow />
                            </TooltipContent>
                        </TooltipPortal>
                    </Tooltip>
                </DialogContent>
            </Dialog>
        </TooltipProvider>
    }
}

#[component]
pub fn KeepOpenOnActivation() -> impl IntoView {
    view! {
        <TooltipProvider>
            <Tooltip>
                <TooltipTrigger
                    attr:class=classes::trigger
                    on_click=Callback::new(|event: leptos::ev::MouseEvent| {
                        event.prevent_default();
                    })
                >
                    "Hover or Focus me"
                </TooltipTrigger>
                <TooltipPortal>
                    <TooltipContent
                        attr:class=classes::content
                        side_offset=5.0
                        on_pointer_down_outside=Callback::new(|event: web_sys::CustomEvent| {
                            event.prevent_default();
                        })
                    >
                        "Nicely done!"
                        <TooltipArrow attr:class=classes::arrow />
                    </TooltipContent>
                </TooltipPortal>
            </Tooltip>
        </TooltipProvider>
    }
}

#[component]
pub fn WithinScrollable() -> impl IntoView {
    view! {
        <TooltipProvider>
            <div style="position: absolute; top: 0; left: 0; height: 500px; width: 300px; border: 1px solid black; overflow: auto;">
                <div style="display: flex; align-items: center; justify-content: center; height: 600px;">
                    <Tooltip>
                        <TooltipTrigger attr:class=classes::trigger>"Hover or Focus me"</TooltipTrigger>
                        <TooltipPortal>
                            <TooltipContent attr:class=classes::content side_offset=5.0>
                                "Nicely done!"
                                <TooltipArrow attr:class=classes::arrow />
                            </TooltipContent>
                        </TooltipPortal>
                    </Tooltip>
                </div>
            </div>
            <div style="display: flex; align-items: center; justify-content: center; height: 150vh;">
                <Tooltip>
                    <TooltipTrigger attr:class=classes::trigger>"Hover or Focus me"</TooltipTrigger>
                    <TooltipPortal>
                        <TooltipContent attr:class=classes::content side_offset=5.0>
                            "Nicely done!"
                            <TooltipArrow attr:class=classes::arrow />
                        </TooltipContent>
                    </TooltipPortal>
                </Tooltip>
            </div>
        </TooltipProvider>
    }
}

#[component]
pub fn DisableHoverableContent() -> impl IntoView {
    view! {
        <>
            <h1>"Hoverable content (Default)"</h1>
            <p>"Content remains open while moving pointer to it"</p>
            <div style="display: flex; gap: 50px;">
                <TooltipProvider skip_delay_duration=1000.0>
                    {tooltip_item()}
                </TooltipProvider>
            </div>

            <h1>"Disable hoverable content"</h1>
            <p>"Tooltip closes when pointer leaves the trigger"</p>
            <h2>"Inherited from provider"</h2>
            <div style="display: flex; gap: 50px;">
                <TooltipProvider delay_duration=0.0 disable_hoverable_content=true>
                    {tooltip_item()}
                </TooltipProvider>
            </div>
            <h2>"Inherited value overridden by prop on tooltip"</h2>
            <div style="display: flex; gap: 50px;">
                <TooltipProvider delay_duration=0.0 disable_hoverable_content=true>
                    <Tooltip>
                        <TooltipTrigger attr:class=classes::trigger>"Disabled hoverable content"</TooltipTrigger>
                        <TooltipPortal>
                            <TooltipContent attr:class=classes::content side_offset=5.0>
                                "Nicely done!"
                                <TooltipArrow attr:class=classes::arrow />
                            </TooltipContent>
                        </TooltipPortal>
                    </Tooltip>
                    <Tooltip disable_hoverable_content=false>
                        <TooltipTrigger attr:class=classes::trigger>"Hoverable content"</TooltipTrigger>
                        <TooltipPortal>
                            <TooltipContent attr:class=classes::content side_offset=5.0>
                                "Nicely done!"
                                <TooltipArrow attr:class=classes::arrow />
                            </TooltipContent>
                        </TooltipPortal>
                    </Tooltip>
                </TooltipProvider>
            </div>
        </>
    }
}

#[component]
pub fn Chromatic() -> impl IntoView {
    view! {
        <TooltipProvider>
            <div style="padding: 200px;">
                <h1>"Uncontrolled"</h1>
                <h2>"Closed"</h2>
                <Tooltip>
                    <TooltipTrigger attr:class=classes::trigger>"open"</TooltipTrigger>
                    <TooltipPortal>
                        <TooltipContent attr:class=classes::content side_offset=5.0>
                            "Some content"
                            <TooltipArrow attr:class=classes::arrow />
                        </TooltipContent>
                    </TooltipPortal>
                </Tooltip>

                <h2 style="margin-bottom: 60px;">"Open"</h2>
                <Tooltip default_open=true>
                    <TooltipTrigger attr:class=classes::trigger>"open"</TooltipTrigger>
                    <TooltipPortal>
                        <TooltipContent attr:class=classes::content side_offset=5.0>
                            "Some content"
                            <TooltipArrow attr:class=classes::arrow />
                        </TooltipContent>
                    </TooltipPortal>
                </Tooltip>

                <h2 style="margin-top: 60px; margin-bottom: 60px;">"Open with reordered parts"</h2>
                <Tooltip default_open=true>
                    <TooltipPortal>
                        <TooltipContent attr:class=classes::content side_offset=5.0>
                            "Some content"
                            <TooltipArrow attr:class=classes::arrow />
                        </TooltipContent>
                    </TooltipPortal>
                    <TooltipTrigger attr:class=classes::trigger>"open"</TooltipTrigger>
                </Tooltip>

                <h1 style="margin-top: 100px;">"Controlled"</h1>
                <h2>"Closed"</h2>
                <Tooltip open=false>
                    <TooltipTrigger attr:class=classes::trigger>"open"</TooltipTrigger>
                    <TooltipPortal>
                        <TooltipContent attr:class=classes::content side_offset=5.0>
                            "Some content"
                            <TooltipArrow attr:class=classes::arrow />
                        </TooltipContent>
                    </TooltipPortal>
                </Tooltip>

                <h2 style="margin-bottom: 60px;">"Open"</h2>
                <Tooltip open=true>
                    <TooltipTrigger attr:class=classes::trigger>"open"</TooltipTrigger>
                    <TooltipPortal>
                        <TooltipContent attr:class=classes::content side_offset=5.0>
                            "Some content"
                            <TooltipArrow attr:class=classes::arrow />
                        </TooltipContent>
                    </TooltipPortal>
                </Tooltip>

                <h2 style="margin-top: 60px; margin-bottom: 60px;">"Open with reordered parts"</h2>
                <Tooltip open=true>
                    <TooltipPortal>
                        <TooltipContent attr:class=classes::content side_offset=5.0>
                            "Some content"
                            <TooltipArrow attr:class=classes::arrow />
                        </TooltipContent>
                    </TooltipPortal>
                    <TooltipTrigger attr:class=classes::trigger>"open"</TooltipTrigger>
                </Tooltip>

                <h1 style="margin-top: 100px;">"Positioning"</h1>
                <h2>"No collisions"</h2>
                <h3>"Side & Align"</h3>
                <div class=classes::grid>
                    {chromatic_grid(None, None)}
                </div>

                <h3>"Side offset"</h3>
                <h4>"Positive"</h4>
                <div class=classes::grid>
                    {chromatic_grid(Some(5.0), None)}
                </div>
                <h4>"Negative"</h4>
                <div class=classes::grid>
                    {chromatic_grid(Some(-10.0), None)}
                </div>

                <h3>"Align offset"</h3>
                <h4>"Positive"</h4>
                <div class=classes::grid>
                    {chromatic_grid(None, Some(20.0))}
                </div>
                <h4>"Negative"</h4>
                <div class=classes::grid>
                    {chromatic_grid(None, Some(-10.0))}
                </div>

                <h2>"Collisions"</h2>
                <p>"See instances on the periphery of the page."</p>
                {chromatic_collision_items()}

                <h2 style="margin-top: 50px; margin-bottom: 60px;">"Relative parent (non-portalled)"</h2>
                <div style="position: relative;">
                    <TooltipProvider>
                        <Tooltip open=true>
                            <TooltipTrigger attr:class=classes::trigger>"Hover or Focus me"</TooltipTrigger>
                            <TooltipContent attr:class=classes::content side_offset=5.0>
                                "Nicely done!"
                                <TooltipArrow attr:class=classes::arrow />
                            </TooltipContent>
                        </Tooltip>
                    </TooltipProvider>
                </div>

                <h1 style="margin-top: 100px; margin-bottom: 60px;">"With slotted trigger"</h1>
                <Tooltip open=true>
                    <TooltipTrigger as_child=true>
                        <button class=classes::trigger>"open"</button>
                    </TooltipTrigger>
                    <TooltipPortal>
                        <TooltipContent attr:class=classes::content side_offset=5.0>
                            "Some content"
                            <TooltipArrow attr:class=classes::arrow width=20.0 height=10.0 />
                        </TooltipContent>
                    </TooltipPortal>
                </Tooltip>

                <h1 style="margin-top: 100px; margin-bottom: 60px;">"With slotted content"</h1>
                <Tooltip open=true>
                    <TooltipTrigger attr:class=classes::trigger>"Hover or Focus me"</TooltipTrigger>
                    <TooltipPortal>
                        <TooltipContent as_child=true side_offset=5.0>
                            <div class=classes::content>
                                "Some content"
                                <TooltipArrow attr:class=classes::arrow />
                            </div>
                        </TooltipContent>
                    </TooltipPortal>
                </Tooltip>
            </div>
        </TooltipProvider>
    }
}

const SIDES: [Side; 4] = [Side::Top, Side::Right, Side::Bottom, Side::Left];
const ALIGNS: [Align; 3] = [Align::Start, Align::Center, Align::End];

fn chromatic_grid(side_offset: Option<f64>, align_offset: Option<f64>) -> impl IntoView {
    SIDES
        .iter()
        .flat_map(|&side| {
            ALIGNS.iter().map(move |&align| {
                let side_label = match side {
                    Side::Top => "top",
                    Side::Right => "right",
                    Side::Bottom => "bottom",
                    Side::Left => "left",
                };
                let align_label = match align {
                    Align::Start => "start",
                    Align::Center => "center",
                    Align::End => "end",
                };
                view! {
                    <Tooltip open=true>
                        <TooltipTrigger attr:class=classes::chromaticTrigger />
                        <TooltipPortal>
                            <TooltipContent
                                attr:class=classes::chromaticContent
                                side=side
                                align=align
                                side_offset=side_offset.unwrap_or(0.0)
                                align_offset=align_offset.unwrap_or(0.0)
                                avoid_collisions=false
                            >
                                <p style="text-align: center;">
                                    {side_label}
                                    <br />
                                    {align_label}
                                </p>
                                <TooltipArrow
                                    attr:class=classes::chromaticArrow
                                    width=20.0
                                    height=10.0
                                />
                            </TooltipContent>
                        </TooltipPortal>
                    </Tooltip>
                }
            })
        })
        .collect_view()
}

fn chromatic_collision_items() -> impl IntoView {
    SIDES
        .iter()
        .flat_map(|&side| {
            ALIGNS.iter().map(move |&align| {
                let side_label = match side {
                    Side::Top => "top",
                    Side::Right => "right",
                    Side::Bottom => "bottom",
                    Side::Left => "left",
                };
                let align_label = match align {
                    Align::Start => "start",
                    Align::Center => "center",
                    Align::End => "end",
                };
                let style = match (side, align) {
                    (Side::Top, Align::Start) => "position: absolute; top: 10px; right: 10px;",
                    (Side::Top, Align::Center) => {
                        "position: absolute; top: 10px; left: calc(50% - 15px);"
                    }
                    (Side::Top, Align::End) => "position: absolute; top: 10px; left: 10px;",
                    (Side::Right, Align::Start) => "position: absolute; right: 10px; bottom: 10px;",
                    (Side::Right, Align::Center) => {
                        "position: absolute; right: 10px; top: calc(50% - 15px);"
                    }
                    (Side::Right, Align::End) => "position: absolute; right: 10px; top: 10px;",
                    (Side::Bottom, Align::Start) => {
                        "position: absolute; bottom: 10px; right: 10px;"
                    }
                    (Side::Bottom, Align::Center) => {
                        "position: absolute; bottom: 10px; left: calc(50% - 15px);"
                    }
                    (Side::Bottom, Align::End) => "position: absolute; bottom: 10px; left: 10px;",
                    (Side::Left, Align::Start) => "position: absolute; left: 10px; bottom: 10px;",
                    (Side::Left, Align::Center) => {
                        "position: absolute; left: 10px; top: calc(50% - 15px);"
                    }
                    (Side::Left, Align::End) => "position: absolute; left: 10px; top: 10px;",
                };
                view! {
                    <Tooltip open=true>
                        <TooltipTrigger
                            attr:class=classes::chromaticTrigger
                            attr:style=style
                        />
                        <TooltipPortal>
                            <TooltipContent
                                attr:class=classes::chromaticContent
                                side=side
                                align=align
                            >
                                <p style="text-align: center;">
                                    {side_label}
                                    <br />
                                    {align_label}
                                </p>
                                <TooltipArrow
                                    attr:class=classes::chromaticArrow
                                    width=20.0
                                    height=10.0
                                />
                            </TooltipContent>
                        </TooltipPortal>
                    </Tooltip>
                }
            })
        })
        .collect_view()
}
