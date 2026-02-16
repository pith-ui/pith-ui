use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use radix_leptos_popover::*;
use radix_leptos_popper::{Align, Side};
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;

stylance::import_crate_style!(classes, "src/primitives/popover.stories.module.css");

fn content_class(animated: bool) -> String {
    if animated {
        format!("{} {}", classes::content, classes::animatedContent)
    } else {
        classes::content.to_string()
    }
}

#[component]
pub fn Styled() -> impl IntoView {
    view! {
        <div style="display: flex; align-items: center; justify-content: center; height: 200vh;">
            <Popover>
                <PopoverTrigger attr:class=classes::trigger>"open"</PopoverTrigger>
                <PopoverPortal>
                    <PopoverContent attr:class=classes::content side_offset=5.0>
                        <PopoverClose attr:class=classes::close>"close"</PopoverClose>
                        <PopoverArrow attr:class=classes::arrow width=20.0 height=10.0 />
                    </PopoverContent>
                </PopoverPortal>
            </Popover>
            <input />
        </div>
    }
}

#[component]
pub fn Boundary() -> impl IntoView {
    let boundary_ref = NodeRef::<leptos::html::Div>::new();

    let collision_boundary = Signal::derive(move || match boundary_ref.get() {
        Some(el) => {
            let el: web_sys::Element = el.unchecked_into();
            SendWrapper::new(vec![el])
        }
        None => SendWrapper::new(vec![]),
    });

    view! {
        <div
            style="border: 3px dashed red; width: 200px; height: 200px;"
            node_ref=boundary_ref
        >
            <Popover>
                <PopoverTrigger as_child=true>
                    <button>"open"</button>
                </PopoverTrigger>
                <PopoverPortal>
                    <PopoverContent
                        attr:style="box-sizing: border-box; border-radius: 8px; padding: 8px; color: white; background-color: black; width: var(--radix-popper-available-width); height: var(--radix-popper-available-height);"
                        side_offset=5.0
                        collision_boundary=collision_boundary
                    >
                        "out of bound out of bound out of bound out of bound out of bound out of bound out of bound out of bound out of bound"
                    </PopoverContent>
                </PopoverPortal>
            </Popover>
        </div>
    }
}

#[component]
pub fn Modality() -> impl IntoView {
    view! {
        <div style="display: flex; align-items: center; justify-content: center; height: 110vh;">
            <div style="display: grid; gap: 50px;">
                <div style="display: inline-flex; align-items: center; flex-direction: column;">
                    <h1>"Non modal (default)"</h1>
                    <Popover>
                        <PopoverTrigger attr:class=classes::trigger>"open"</PopoverTrigger>
                        <PopoverPortal>
                            <PopoverContent attr:class=classes::content side_offset=5.0>
                                <PopoverClose attr:class=classes::close>"close"</PopoverClose>
                                <PopoverArrow attr:class=classes::arrow width=20.0 height=10.0 />
                            </PopoverContent>
                        </PopoverPortal>
                    </Popover>
                    <textarea
                        style="width: 500px; height: 100px; margin-top: 10px;"
                    >"Lorem ipsum dolor sit amet consectetur adipisicing elit. Quaerat nobis at ipsa, nihil tempora debitis maxime dignissimos non amet."</textarea>
                </div>
                <div style="display: inline-flex; align-items: center; flex-direction: column;">
                    <h1>"Modal"</h1>
                    <Popover modal=true>
                        <PopoverTrigger attr:class=classes::trigger>"open"</PopoverTrigger>
                        <PopoverPortal>
                            <PopoverContent attr:class=classes::content side_offset=5.0>
                                <PopoverClose attr:class=classes::close>"close"</PopoverClose>
                                <PopoverArrow attr:class=classes::arrow width=20.0 height=10.0 />
                            </PopoverContent>
                        </PopoverPortal>
                    </Popover>
                    <textarea
                        style="width: 500px; height: 100px; margin-top: 10px;"
                    >"Lorem ipsum dolor sit amet consectetur adipisicing elit. Quaerat nobis at ipsa, nihil tempora debitis maxime dignissimos non amet."</textarea>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Controlled() -> impl IntoView {
    let (open, set_open) = signal(false);

    view! {
        <div style="display: flex; align-items: center; justify-content: center; height: 50vh;">
            <Popover
                open=MaybeProp::derive(move || Some(open.get()))
                on_open_change=Callback::new(move |value: bool| set_open.set(value))
            >
                <PopoverTrigger attr:class=classes::trigger>
                    {move || if open.get() { "close" } else { "open" }}
                </PopoverTrigger>
                <PopoverPortal>
                    <PopoverContent attr:class=classes::content>
                        <PopoverClose attr:class=classes::close>"close"</PopoverClose>
                        <PopoverArrow attr:class=classes::arrow width=20.0 height=10.0 />
                    </PopoverContent>
                </PopoverPortal>
            </Popover>
        </div>
    }
}

#[component]
pub fn Animated() -> impl IntoView {
    view! {
        <div style="display: flex; align-items: center; justify-content: center; height: 200vh;">
            <Popover>
                <PopoverTrigger attr:class=classes::trigger>"open"</PopoverTrigger>
                <PopoverPortal>
                    <PopoverContent attr:class=content_class(true) side_offset=10.0>
                        <PopoverClose attr:class=classes::close>"close"</PopoverClose>
                        <PopoverArrow attr:class=classes::arrow width=20.0 height=10.0 />
                    </PopoverContent>
                </PopoverPortal>
            </Popover>
        </div>
    }
}

#[component]
pub fn ForcedMount() -> impl IntoView {
    view! {
        <div style="display: flex; align-items: center; justify-content: center; height: 200vh;">
            <Popover>
                <PopoverTrigger attr:class=classes::trigger>"open"</PopoverTrigger>
                <PopoverPortal force_mount=true>
                    <PopoverContent attr:class=classes::content side_offset=10.0>
                        <PopoverClose attr:class=classes::close>"close"</PopoverClose>
                        <PopoverArrow attr:class=classes::arrow width=20.0 height=10.0 />
                    </PopoverContent>
                </PopoverPortal>
            </Popover>
        </div>
    }
}

#[component]
pub fn Nested() -> impl IntoView {
    let button_ref = AnyNodeRef::new();

    view! {
        <button
            type="button"
            style="position: fixed; top: 10px; left: 10px;"
            on:click=move |_| {
                if let Some(el) = button_ref.get() {
                    let el: &web_sys::HtmlElement = el.unchecked_ref();
                    let _ = el.focus();
                }
            }
        >
            "Focus popover button"
        </button>

        <div style="height: 300vh; width: 300vw; display: flex; align-items: center; justify-content: center;">
            <Popover>
                <PopoverTrigger attr:class=classes::trigger node_ref=button_ref>
                    "Open popover"
                </PopoverTrigger>

                <PopoverPortal>
                    <PopoverContent
                        attr:class=classes::content
                        side_offset=5.0
                        attr:style="background-color: crimson;"
                    >
                        <Popover>
                            <PopoverTrigger attr:class=classes::trigger>"Open nested popover"</PopoverTrigger>
                            <PopoverPortal>
                                <PopoverContent
                                    attr:class=classes::content
                                    side=Side::Top
                                    align=Align::Center
                                    side_offset=5.0
                                    attr:style="background-color: green;"
                                >
                                    <PopoverClose attr:class=classes::close>"close"</PopoverClose>
                                    <PopoverArrow
                                        attr:class=classes::arrow
                                        width=20.0
                                        height=10.0
                                        attr:style="fill: green;"
                                    />
                                </PopoverContent>
                            </PopoverPortal>
                        </Popover>

                        <PopoverClose attr:class=classes::close attr:style="margin-left: 10px;">
                            "close"
                        </PopoverClose>
                        <PopoverArrow
                            attr:class=classes::arrow
                            width=20.0
                            height=10.0
                            attr:style="fill: crimson;"
                        />
                    </PopoverContent>
                </PopoverPortal>
            </Popover>
        </div>
    }
}

#[component]
pub fn CustomAnchor() -> impl IntoView {
    view! {
        <Popover>
            <PopoverAnchor
                attr:style="display: flex; justify-content: space-between; align-items: center; width: 250px; padding: 20px; margin: 100px; background-color: #eee;"
            >
                "Item "
                <PopoverTrigger attr:class=classes::trigger>"open"</PopoverTrigger>
            </PopoverAnchor>
            <PopoverPortal>
                <PopoverContent
                    attr:class=classes::content
                    side=Side::Right
                    side_offset=1.0
                    align=Align::Start
                    attr:style="border-radius: 0; width: 200px; height: 100px;"
                >
                    <PopoverClose>"close"</PopoverClose>
                </PopoverContent>
            </PopoverPortal>
        </Popover>
    }
}

#[component]
pub fn WithSlottedTrigger() -> impl IntoView {
    view! {
        <Popover>
            <PopoverTrigger as_child=true>
                <button class=classes::trigger on:click=move |_| {
                    leptos::logging::log!("StyledTrigger click");
                }>"open"</button>
            </PopoverTrigger>
            <PopoverPortal>
                <PopoverContent attr:class=classes::content side_offset=5.0>
                    <PopoverClose attr:class=classes::close>"close"</PopoverClose>
                    <PopoverArrow attr:class=classes::arrow width=20.0 height=10.0 />
                </PopoverContent>
            </PopoverPortal>
        </Popover>
    }
}

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
            <Popover>
                <PopoverTrigger attr:class=classes::trigger>"open"</PopoverTrigger>
                <PopoverPortal>
                    <PopoverContent attr:class=classes::content side_offset=5.0>
                        <PopoverClose attr:class=classes::close>"close"</PopoverClose>
                        <PopoverArrow attr:class=classes::arrow width=20.0 height=10.0 />
                    </PopoverContent>
                </PopoverPortal>
            </Popover>

            <h2>"Open"</h2>
            <Popover default_open=true>
                <PopoverTrigger attr:class=classes::trigger>"open"</PopoverTrigger>
                <PopoverPortal>
                    <PopoverContent
                        attr:class=classes::content
                        side_offset=5.0
                        on_focus_outside=Callback::new(|event: web_sys::CustomEvent| {
                            event.prevent_default();
                        })
                    >
                        <PopoverClose attr:class=classes::close>"close"</PopoverClose>
                        <PopoverArrow attr:class=classes::arrow width=20.0 height=10.0 />
                    </PopoverContent>
                </PopoverPortal>
            </Popover>

            <h2 style="margin-top: 100px;">"Open with reordered parts"</h2>
            <Popover default_open=true>
                <PopoverPortal>
                    <PopoverContent
                        attr:class=classes::content
                        side_offset=5.0
                        on_focus_outside=Callback::new(|event: web_sys::CustomEvent| {
                            event.prevent_default();
                        })
                    >
                        <PopoverClose attr:class=classes::close>"close"</PopoverClose>
                        <PopoverArrow attr:class=classes::arrow width=20.0 height=10.0 />
                    </PopoverContent>
                </PopoverPortal>
                <PopoverTrigger attr:class=classes::trigger>"open"</PopoverTrigger>
            </Popover>

            <h1 style="margin-top: 100px;">"Controlled"</h1>
            <h2>"Closed"</h2>
            <Popover open=false>
                <PopoverTrigger attr:class=classes::trigger>"open"</PopoverTrigger>
                <PopoverPortal>
                    <PopoverContent attr:class=classes::content side_offset=5.0>
                        <PopoverClose attr:class=classes::close>"close"</PopoverClose>
                        <PopoverArrow attr:class=classes::arrow width=20.0 height=10.0 />
                    </PopoverContent>
                </PopoverPortal>
            </Popover>

            <h2>"Open"</h2>
            <Popover open=true>
                <PopoverTrigger attr:class=classes::trigger>"open"</PopoverTrigger>
                <PopoverPortal>
                    <PopoverContent attr:class=classes::content side_offset=5.0>
                        <PopoverClose attr:class=classes::close>"close"</PopoverClose>
                        <PopoverArrow attr:class=classes::arrow width=20.0 height=10.0 />
                    </PopoverContent>
                </PopoverPortal>
            </Popover>

            <h1 style="margin-top: 100px;">"Force mounted content"</h1>
            <Popover>
                <PopoverTrigger attr:class=classes::trigger>"open"</PopoverTrigger>
                <PopoverPortal force_mount=true>
                    <PopoverContent attr:class=classes::content side_offset=5.0>
                        <PopoverClose attr:class=classes::close>"close"</PopoverClose>
                        <PopoverArrow attr:class=classes::arrow width=20.0 height=10.0 />
                    </PopoverContent>
                </PopoverPortal>
            </Popover>

            <h1 style="margin-top: 100px;">"Anchor"</h1>
            <h2>"Controlled"</h2>
            <Popover open=true>
                <PopoverAnchor attr:style="padding: 20px; background: gainsboro;">
                    <PopoverTrigger attr:class=classes::trigger>"open"</PopoverTrigger>
                </PopoverAnchor>
                <PopoverPortal>
                    <PopoverContent attr:class=classes::content>
                        <PopoverClose attr:class=classes::close>"close"</PopoverClose>
                        <PopoverArrow attr:class=classes::arrow width=20.0 height=10.0 />
                    </PopoverContent>
                </PopoverPortal>
            </Popover>

            <h2>"Uncontrolled"</h2>
            <Popover default_open=true>
                <PopoverAnchor attr:style="padding: 20px; background: gainsboro;">
                    <PopoverTrigger attr:class=classes::trigger>"open"</PopoverTrigger>
                </PopoverAnchor>
                <PopoverPortal>
                    <PopoverContent
                        attr:class=classes::content
                        on_focus_outside=Callback::new(|event: web_sys::CustomEvent| {
                            event.prevent_default();
                        })
                    >
                        <PopoverClose attr:class=classes::close>"close"</PopoverClose>
                        <PopoverArrow attr:class=classes::arrow width=20.0 height=10.0 />
                    </PopoverContent>
                </PopoverPortal>
            </Popover>

            <h1 style="margin-top: 100px;">"Positioning"</h1>
            <h2>"No collisions"</h2>
            <h3>"Side & Align"</h3>
            <div class=classes::grid>
                {SIDES.iter().flat_map(|side| {
                    ALIGNS.iter().map(move |align| {
                        let side = *side;
                        let align = *align;
                        view! {
                            <Popover open=true>
                                <PopoverTrigger attr:class=classes::chromaticTrigger />
                                <PopoverPortal>
                                    <PopoverContent
                                        attr:class=classes::chromaticContent
                                        side=side
                                        align=align
                                        avoid_collisions=false
                                    >
                                        <p style="text-align: center;">
                                            {side_name(side)}<br />{align_name(align)}
                                        </p>
                                        <PopoverArrow attr:class=classes::chromaticArrow width=20.0 height=10.0 />
                                    </PopoverContent>
                                </PopoverPortal>
                            </Popover>
                        }
                    })
                }).collect_view()}
            </div>

            <h3>"Side offset"</h3>
            <h4>"Positive"</h4>
            <div class=classes::grid>
                {SIDES.iter().flat_map(|side| {
                    ALIGNS.iter().map(move |align| {
                        let side = *side;
                        let align = *align;
                        view! {
                            <Popover open=true>
                                <PopoverTrigger attr:class=classes::chromaticTrigger />
                                <PopoverPortal>
                                    <PopoverContent
                                        attr:class=classes::chromaticContent
                                        side=side
                                        side_offset=5.0
                                        align=align
                                        avoid_collisions=false
                                    >
                                        <p style="text-align: center;">
                                            {side_name(side)}<br />{align_name(align)}
                                        </p>
                                        <PopoverArrow attr:class=classes::chromaticArrow width=20.0 height=10.0 />
                                    </PopoverContent>
                                </PopoverPortal>
                            </Popover>
                        }
                    })
                }).collect_view()}
            </div>
            <h4>"Negative"</h4>
            <div class=classes::grid>
                {SIDES.iter().flat_map(|side| {
                    ALIGNS.iter().map(move |align| {
                        let side = *side;
                        let align = *align;
                        view! {
                            <Popover open=true>
                                <PopoverTrigger attr:class=classes::chromaticTrigger />
                                <PopoverPortal>
                                    <PopoverContent
                                        attr:class=classes::chromaticContent
                                        side=side
                                        side_offset=-10.0
                                        align=align
                                        avoid_collisions=false
                                    >
                                        <p style="text-align: center;">
                                            {side_name(side)}<br />{align_name(align)}
                                        </p>
                                        <PopoverArrow attr:class=classes::chromaticArrow width=20.0 height=10.0 />
                                    </PopoverContent>
                                </PopoverPortal>
                            </Popover>
                        }
                    })
                }).collect_view()}
            </div>

            <h3>"Align offset"</h3>
            <h4>"Positive"</h4>
            <div class=classes::grid>
                {SIDES.iter().flat_map(|side| {
                    ALIGNS.iter().map(move |align| {
                        let side = *side;
                        let align = *align;
                        view! {
                            <Popover open=true>
                                <PopoverTrigger attr:class=classes::chromaticTrigger />
                                <PopoverPortal>
                                    <PopoverContent
                                        attr:class=classes::chromaticContent
                                        side=side
                                        align=align
                                        align_offset=20.0
                                        avoid_collisions=false
                                    >
                                        <p style="text-align: center;">
                                            {side_name(side)}<br />{align_name(align)}
                                        </p>
                                        <PopoverArrow attr:class=classes::chromaticArrow width=20.0 height=10.0 />
                                    </PopoverContent>
                                </PopoverPortal>
                            </Popover>
                        }
                    })
                }).collect_view()}
            </div>
            <h4>"Negative"</h4>
            <div class=classes::grid>
                {SIDES.iter().flat_map(|side| {
                    ALIGNS.iter().map(move |align| {
                        let side = *side;
                        let align = *align;
                        view! {
                            <Popover open=true>
                                <PopoverTrigger attr:class=classes::chromaticTrigger />
                                <PopoverPortal>
                                    <PopoverContent
                                        attr:class=classes::chromaticContent
                                        side=side
                                        align=align
                                        align_offset=-10.0
                                        avoid_collisions=false
                                    >
                                        <p style="text-align: center;">
                                            {side_name(side)}<br />{align_name(align)}
                                        </p>
                                        <PopoverArrow attr:class=classes::chromaticArrow width=20.0 height=10.0 />
                                    </PopoverContent>
                                </PopoverPortal>
                            </Popover>
                        }
                    })
                }).collect_view()}
            </div>

            <h2>"Collisions"</h2>
            <p>"See instances on the periphery of the page."</p>
            {SIDES.iter().flat_map(|side| {
                ALIGNS.iter().map(move |align| {
                    let side = *side;
                    let align = *align;
                    let trigger_style = StoredValue::new(get_collision_trigger_style(side, align));
                    view! {
                        <Popover open=true>
                            <PopoverTrigger
                                attr:class=classes::chromaticTrigger
                                attr:style=move || trigger_style.get_value()
                            />
                            <PopoverPortal>
                                <PopoverContent
                                    attr:class=classes::chromaticContent
                                    side=side
                                    align=align
                                >
                                    <p style="text-align: center;">
                                        {side_name(side)}<br />{align_name(align)}
                                    </p>
                                    <PopoverArrow attr:class=classes::chromaticArrow width=20.0 height=10.0 />
                                </PopoverContent>
                            </PopoverPortal>
                        </Popover>
                    }
                })
            }).collect_view()}

            <h2>"Relative parent (non-portalled)"</h2>
            <div style="position: relative;">
                <Popover open=true>
                    <PopoverTrigger as_child=true>
                        <button class=classes::trigger>"open"</button>
                    </PopoverTrigger>
                    <PopoverContent attr:class=classes::content side_offset=5.0>
                        <PopoverClose attr:class=classes::close>"close"</PopoverClose>
                        <PopoverArrow attr:class=classes::arrow width=20.0 height=10.0 />
                    </PopoverContent>
                </Popover>
            </div>

            <h1 style="margin-top: 100px;">"With slotted trigger"</h1>
            <Popover open=true>
                <PopoverTrigger as_child=true>
                    <button class=classes::trigger>"open"</button>
                </PopoverTrigger>
                <PopoverPortal>
                    <PopoverContent attr:class=classes::content side_offset=5.0>
                        <PopoverClose attr:class=classes::close>"close"</PopoverClose>
                        <PopoverArrow attr:class=classes::arrow width=20.0 height=10.0 />
                    </PopoverContent>
                </PopoverPortal>
            </Popover>

            <h1 style="margin-top: 100px;">"State attributes"</h1>
            <h2>"Closed"</h2>
            <Popover open=false>
                <PopoverTrigger attr:class=classes::triggerAttr>"open"</PopoverTrigger>
                <PopoverPortal>
                    <PopoverContent attr:class=classes::contentAttr side_offset=5.0 avoid_collisions=false>
                        <PopoverClose attr:class=classes::closeAttr>"close"</PopoverClose>
                        <PopoverArrow attr:class=classes::arrowAttr width=20.0 height=10.0 />
                    </PopoverContent>
                </PopoverPortal>
            </Popover>

            <h2>"Open"</h2>
            <Popover open=true>
                <PopoverTrigger attr:class=classes::triggerAttr>"open"</PopoverTrigger>
                <PopoverPortal>
                    <PopoverContent
                        attr:class=classes::contentAttr
                        side=Side::Right
                        side_offset=5.0
                        avoid_collisions=false
                    >
                        <PopoverClose attr:class=classes::closeAttr>"close"</PopoverClose>
                        <PopoverArrow attr:class=classes::arrowAttr width=20.0 height=10.0 />
                    </PopoverContent>
                </PopoverPortal>
            </Popover>
        </div>
    }
}

fn get_collision_trigger_style(side: Side, align: Align) -> String {
    let mut style = "position: absolute; ".to_string();

    match side {
        Side::Top => style.push_str("top: 10px; "),
        Side::Right => style.push_str("right: 10px; "),
        Side::Bottom => style.push_str("bottom: 10px; "),
        Side::Left => style.push_str("left: 10px; "),
    }

    match side {
        Side::Right | Side::Left => match align {
            Align::Start => style.push_str("bottom: 10px; "),
            Align::Center => style.push_str("top: calc(50% - 15px); "),
            Align::End => style.push_str("top: 10px; "),
        },
        Side::Top | Side::Bottom => match align {
            Align::Start => style.push_str("right: 10px; "),
            Align::Center => style.push_str("left: calc(50% - 15px); "),
            Align::End => style.push_str("left: 10px; "),
        },
    }

    style
}
