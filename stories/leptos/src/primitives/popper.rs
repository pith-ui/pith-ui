use std::time::Duration;

use leptos::prelude::*;
use radix_leptos_popper::*;
use radix_leptos_portal::Portal;
use send_wrapper::SendWrapper;

stylance::import_crate_style!(classes, "src/primitives/popper.stories.module.css");

fn anchor_class(size: &str) -> String {
    let size_class = match size {
        "small" => classes::anchorSmall,
        "large" => classes::anchorLarge,
        _ => "",
    };
    format!("{} {}", classes::anchor, size_class)
}

fn content_class(size: &str) -> String {
    let size_class = match size {
        "small" => classes::contentSmall,
        "large" => classes::contentLarge,
        _ => "",
    };
    format!("{} {}", classes::content, size_class)
}

#[component]
pub fn Styled() -> impl IntoView {
    let (open, set_open) = signal(false);

    view! {
        <Scrollable>
            <Popper>
                <PopperAnchor attr:class=classes::anchor on:click=move |_| set_open.set(true)>
                    open
                </PopperAnchor>

                <Show when=move || open.get()>
                    <PopperContent attr:class=classes::content side_offset=5.0>
                        <button on:click=move |_| set_open.set(false)>close</button>
                        <PopperArrow attr:class=classes::arrow width=20.0 height=10.0 />
                    </PopperContent>
                </Show>
            </Popper>
        </Scrollable>
    }
}

#[component]
pub fn WithCustomArrow() -> impl IntoView {
    let (open, set_open) = signal(false);

    view! {
        <Scrollable>
            <Popper>
                <PopperAnchor attr:class=classes::anchor on:click=move |_| set_open.set(true)>
                    open
                </PopperAnchor>

                <Show when=move || open.get()>
                    <PopperContent attr:class=classes::content side=Side::Right side_offset=5.0>
                        <button on:click=move |_| set_open.set(false)>close</button>
                        <PopperArrow as_child=true>
                            <CustomArrow />
                        </PopperArrow>
                    </PopperContent>
                </Show>
            </Popper>
        </Scrollable>
    }
}

#[component]
pub fn Animated() -> impl IntoView {
    let (open, set_open) = signal(false);
    let animated_content_class =
        Memo::new(move |_| format!("{} {}", classes::content, classes::animatedContent));

    view! {
        <Scrollable>
            <Popper>
                <PopperAnchor attr:class=classes::anchor on:click=move |_| set_open.set(true)>
                    open
                </PopperAnchor>

                <Show when=move || open.get()>
                    <Portal as_child=true>
                        <PopperContent attr:class=animated_content_class side_offset=5.0>
                            <button on:click=move |_| set_open.set(false)>close</button>
                            <PopperArrow attr:class=classes::arrow width=20.0 height=10.0 />
                        </PopperContent>
                    </Portal>
                </Show>
            </Popper>
        </Scrollable>
    }
}

#[component]
pub fn WithPortal() -> impl IntoView {
    let (open, set_open) = signal(false);

    view! {
        <Scrollable>
            <Popper>
                <PopperAnchor attr:class=classes::anchor on:click=move |_| set_open.set(true)>
                    open
                </PopperAnchor>

                <Show when=move || open.get()>
                    <Portal as_child=true>
                        <PopperContent attr:class=classes::content side_offset=5.0>
                            <button on:click=move |_| set_open.set(false)>close</button>
                            <PopperArrow attr:class=classes::arrow width=20.0 height=10.0 />
                        </PopperContent>
                    </Portal>
                </Show>
            </Popper>
        </Scrollable>
    }
}

#[component]
pub fn WithUpdatePositionStrategyAlways() -> impl IntoView {
    let (open, set_open) = signal(false);
    let (left, set_left) = signal(0);

    let handle = set_interval_with_handle(
        move || {
            set_left.update(move |left| {
                *left = (*left + 50) % 300;
            });
        },
        Duration::from_millis(500),
    )
    .expect("Interval should be started");

    on_cleanup(move || {
        handle.clear();
    });

    view! {
        <Scrollable>
            <Popper>
                <PopperAnchor
                    attr:class=classes::anchor
                    on:click=move |_| set_open.set(true)
                    attr:style=move || format!("margin-left: {}px;", left.get())
                >
                    open
                </PopperAnchor>

                <Show when=move || open.get()>
                    <Portal as_child=true>
                        <PopperContent
                            attr:class=classes::content
                            side_offset=5.0
                            update_position_strategy=UpdatePositionStrategy::Always
                        >
                            <button on:click=move |_| set_open.set(false)>close</button>
                            <PopperArrow attr:class=classes::arrow width=20.0 height=10.0 />
                        </PopperContent>
                    </Portal>
                </Show>
            </Popper>
        </Scrollable>
    }
}

#[component]
pub fn Chromatic() -> impl IntoView {
    let small_anchor = Memo::new(move |_| anchor_class("small"));
    let small_content = Memo::new(move |_| content_class("small"));

    let scroll_container1_ref = NodeRef::new();
    let scroll_container1: Signal<SendWrapper<Vec<web_sys::Element>>> = Signal::derive(move || {
        SendWrapper::new(
            scroll_container1_ref
                .get()
                .map(|scroll_container| {
                    let element: &web_sys::HtmlDivElement = &scroll_container;
                    vec![element.clone().into()]
                })
                .unwrap_or(vec![]),
        )
    });
    let scroll_container2_ref = NodeRef::new();
    let scroll_container2: Signal<SendWrapper<Vec<web_sys::Element>>> = Signal::derive(move || {
        SendWrapper::new(
            scroll_container2_ref
                .get()
                .map(|scroll_container| {
                    let element: &web_sys::HtmlDivElement = &scroll_container;
                    vec![element.clone().into()]
                })
                .unwrap_or(vec![]),
        )
    });

    view! {
        <div style:padding-bottom="500px">
            <header
                style:display="flex"
                style:align-items="center"
                style:justify-content="center"
                style:gap="150px"
                style:position="fixed"
                style:top="0px"
                style:left="0px"
                style:right="0px"
                style:background-color="grey"
                style:border="1px solid black"
            >
                <h1>In fixed header</h1>
                <Popper>
                    <PopperAnchor attr:class=small_anchor>1</PopperAnchor>
                    <PopperContent attr:class=small_content side_offset=5.0>
                        <PopperArrow attr:class=classes::arrow width=10.0 height=5.0 />1
                    </PopperContent>
                </Popper>

                <Popper>
                    <PopperAnchor attr:class=small_anchor>2</PopperAnchor>
                    <Portal as_child=true>
                        <PopperContent attr:class=small_content side_offset=5.0>
                            <PopperArrow attr:class=classes::arrow width=10.0 height=5.0 />2 (portalled)
                        </PopperContent>
                    </Portal>
                </Popper>
            </header>

            <div
                style:margin-top="100px"
                style:display="flex"
                style:align-items="center"
                style:justify-content="center"
                style:gap="150px"
                style:border="1px solid black"
            >
                <h1>In normal page flow</h1>
                <Popper>
                    <PopperAnchor attr:class=small_anchor>3</PopperAnchor>
                    <PopperContent attr:class=small_content side_offset=5.0>
                        <PopperArrow attr:class=classes::arrow width=10.0 height=5.0 />3
                    </PopperContent>
                </Popper>

                <Popper>
                    <PopperAnchor attr:class=small_anchor>4</PopperAnchor>
                    <Portal as_child=true>
                        <PopperContent attr:class=small_content side_offset=5.0>
                            <PopperArrow attr:class=classes::arrow width=10.0 height=5.0 />4 (portalled)
                        </PopperContent>
                    </Portal>
                </Popper>
            </div>

            <div
                style:position="relative"
                style:margin-top="50px"
                style:display="flex"
                style:align-items="center"
                style:justify-content="center"
                style:gap="150px"
                style:border="1px solid black"
            >
                <h1>In relative parent</h1>
                <Popper>
                    <PopperAnchor attr:class=small_anchor>5</PopperAnchor>
                    <PopperContent attr:class=small_content side_offset=5.0>
                        <PopperArrow attr:class=classes::arrow width=10.0 height=5.0 />5
                    </PopperContent>
                </Popper>

                <Popper>
                    <PopperAnchor attr:class=small_anchor>6</PopperAnchor>
                    <Portal as_child=true>
                        <PopperContent attr:class=small_content side_offset=5.0>
                            <PopperArrow attr:class=classes::arrow width=10.0 height=5.0 />6 (portalled)
                        </PopperContent>
                    </Portal>
                </Popper>
            </div>

            <div
                style:margin-top="50px"
                style:display="flex"
                style:align-items="center"
                style:justify-content="center"
                style:gap="150px"
                style:border="1px solid black"
                style:transform="translate3d(100px, 0, 0)"
            >
                <h1>In translated parent</h1>
                <Popper>
                    <PopperAnchor attr:class=small_anchor>7</PopperAnchor>
                    <PopperContent attr:class=small_content side_offset=5.0>
                        <PopperArrow attr:class=classes::arrow width=10.0 height=5.0 />7
                    </PopperContent>
                </Popper>

                <Popper>
                    <PopperAnchor attr:class=small_anchor>8</PopperAnchor>
                    <Portal as_child=true>
                        <PopperContent attr:class=small_content side_offset=5.0>
                            <PopperArrow attr:class=classes::arrow width=10.0 height=5.0 />8 (portalled)
                        </PopperContent>
                    </Portal>
                </Popper>
            </div>

            <div style:display="flex" style:gap="100px">
                <div>
                    <h1>In scrolling container</h1>
                    <div
                        node_ref=scroll_container1_ref
                        style:width="400px"
                        style:height="600px"
                        style:overflow="auto"
                        style:border="1px solid black"
                    >
                        <div style:height="2000px">
                            <For
                                each=|| 1..11
                                key=|i| *i
                                children=move |i| view! {
                                    <div
                                        style:display="flex"
                                        style:align-items="center"
                                        style:justify-content="center"
                                        style:gap="150px"
                                        style:padding-bottom="100px"
                                    >
                                        <Popper>
                                            <PopperAnchor attr:class=small_anchor>9.{i + 1}</PopperAnchor>
                                            <PopperContent
                                                attr:class=small_content
                                                side_offset=5.0
                                                hide_when_detached=true
                                                collision_boundary=scroll_container1
                                            >
                                                <PopperArrow attr:class=classes::arrow width=10.0 height=5.0 />
                                                9.{i + 1}
                                            </PopperContent>
                                        </Popper>

                                        <Popper>
                                            <PopperAnchor attr:class=small_anchor>10.{i + 1}</PopperAnchor>
                                            <Portal as_child=true>
                                                <PopperContent
                                                    attr:class=small_content
                                                    side_offset=5.0
                                                    hide_when_detached=true
                                                    collision_boundary=scroll_container1
                                                >
                                                    <PopperArrow attr:class=classes::arrow width=10.0 height=5.0 />
                                                    10.{i + 1}
                                                </PopperContent>
                                            </Portal>
                                        </Popper>
                                    </div>
                                }
                            />
                        </div>
                    </div>
                </div>

                <div>
                    <h1>With position sticky</h1>
                    <div
                        node_ref=scroll_container2_ref
                        style:width="400px"
                        style:height="600px"
                        style:overflow="auto"
                        style:border="1px solid black"
                    >
                        <div style:height="2000px">
                            <For
                                each=|| 1..11
                                key=|i| *i
                                children=move |i| view! {
                                    <div
                                        style:display="flex"
                                        style:align-items="center"
                                        style:justify-content="center"
                                        style:gap="150px"
                                        style:padding-bottom="100px"
                                        style:position="sticky"
                                        style:top="0px"
                                    >
                                        <Popper>
                                            <PopperAnchor attr:class=small_anchor>9.{i + 1}</PopperAnchor>
                                            <PopperContent
                                                attr:class=small_content
                                                side_offset=5.0
                                                hide_when_detached=true
                                                collision_boundary=scroll_container2
                                            >
                                                <PopperArrow attr:class=classes::arrow width=10.0 height=5.0 />
                                                9.{i + 1}
                                            </PopperContent>
                                        </Popper>

                                        <Popper>
                                            <PopperAnchor attr:class=small_anchor>10.{i + 1}</PopperAnchor>
                                            <Portal as_child=true>
                                                <PopperContent
                                                    attr:class=small_content
                                                    side_offset=5.0
                                                    hide_when_detached=true
                                                    collision_boundary=scroll_container2
                                                >
                                                    <PopperArrow attr:class=classes::arrow width=10.0 height=5.0 />
                                                    10.{i + 1}
                                                </PopperContent>
                                            </Portal>
                                        </Popper>
                                    </div>
                                }
                            />
                        </div>
                    </div>
                </div>
            </div>

            <div
                style:margin-top="50px"
                style:display="flex"
                style:align-items="center"
                style:justify-content="center"
                style:gap="150px"
                style:border="1px solid black"
            >
                <h1>Logical "start" alignment (LTR)</h1>
                <Popper>
                    <PopperAnchor attr:class=small_anchor>11</PopperAnchor>
                    <PopperContent attr:class=small_content align=Align::Start side_offset=5.0>
                        <PopperArrow attr:class=classes::arrow width=10.0 height=5.0 />
                        11
                    </PopperContent>
                </Popper>

                <Popper>
                    <PopperAnchor attr:class=small_anchor>12</PopperAnchor>
                    <Portal as_child=true>
                        <PopperContent attr:class=small_content align=Align::Start side_offset=5.0>
                            <PopperArrow attr:class=classes::arrow width=10.0 height=5.0 />
                            12 (portalled)
                        </PopperContent>
                    </Portal>
                </Popper>
            </div>

            <div
                style:margin-top="50px"
                style:display="flex"
                style:align-items="center"
                style:justify-content="center"
                style:gap="150px"
                style:border="1px solid black"
            >
                <h1>Logical "start" alignment (RTL)</h1>
                <Popper>
                    <PopperAnchor attr:class=small_anchor>13</PopperAnchor>
                    <PopperContent attr:class=small_content attr:dir="rtl" align=Align::Start side_offset=5.0>
                        <PopperArrow attr:class=classes::arrow width=10.0 height=5.0 />
                        13
                    </PopperContent>
                </Popper>

                <Popper>
                    <PopperAnchor attr:class=small_anchor>14</PopperAnchor>
                    <Portal as_child=true>
                        <PopperContent attr:class=small_content attr:dir="rtl" align=Align::Start side_offset=5.0>
                            <PopperArrow attr:class=classes::arrow width=10.0 height=5.0 />
                            14 (portalled)
                        </PopperContent>
                    </Portal>
                </Popper>
            </div>
        </div>
    }
}

#[component]
fn Scrollable(children: Children) -> impl IntoView {
    view! {
        <div style="display: flex; align-items: center; justify-content: center; height: 200vh;">
            {children()}
        </div>
    }
}

#[component]
fn CustomArrow() -> impl IntoView {
    view! {
        <div
            style:width="20px"
            style:height="10px"
            style:border-bottom-left-radius="10px"
            style:border-bottom-right-radius="10px"
            style:background-color="tomato"
        />
    }
}
