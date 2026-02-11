use leptos::prelude::*;
use radix_leptos_dismissable_layer::DismissableLayer;
use radix_leptos_focus_guards::FocusGuards;
use radix_leptos_focus_scope::FocusScope;
use radix_leptos_popper::{Popper, PopperAnchor, PopperArrow, PopperContent};
use radix_leptos_portal::Portal;
use web_sys::wasm_bindgen::JsCast;

const SYSTEM_FONT: &str = "-apple-system, BlinkMacSystemFont, \"Segoe UI\", Roboto, Helvetica, Arial, sans-serif, \"Apple Color Emoji\", \"Segoe UI Emoji\", \"Segoe UI Symbol\"";

fn custom_event_target(event: &web_sys::CustomEvent) -> Option<web_sys::EventTarget> {
    event
        .detail()
        .dyn_into::<web_sys::Event>()
        .ok()
        .and_then(|e| e.target())
}

#[component]
pub fn Basic() -> impl IntoView {
    let (open, set_open) = signal(false);
    let open_button_ref = NodeRef::<leptos::html::Button>::new();

    let (dismiss_on_escape, set_dismiss_on_escape) = signal(false);
    let (dismiss_on_pointer_down_outside, set_dismiss_on_pointer_down_outside) = signal(false);
    let (dismiss_on_focus_outside, set_dismiss_on_focus_outside) = signal(false);
    let (disable_outside_pointer_events, set_disable_outside_pointer_events) = signal(false);

    view! {
        <div style="font-family: sans-serif; text-align: center;">
            <h1>"DismissableLayer"</h1>

            <div style="display: inline-block; text-align: left; margin-bottom: 20px;">
                <label style="display: block;">
                    <input
                        type="checkbox"
                        prop:checked=dismiss_on_escape
                        on:change=move |ev| {
                            set_dismiss_on_escape.set(event_target_checked(&ev));
                        }
                    />
                    " Dismiss on escape?"
                </label>

                <label style="display: block;">
                    <input
                        type="checkbox"
                        prop:checked=dismiss_on_pointer_down_outside
                        on:change=move |ev| {
                            set_dismiss_on_pointer_down_outside.set(event_target_checked(&ev));
                        }
                    />
                    " Dismiss on pointer down outside?"
                </label>

                <label style="display: block;">
                    <input
                        type="checkbox"
                        prop:checked=dismiss_on_focus_outside
                        on:change=move |ev| {
                            set_dismiss_on_focus_outside.set(event_target_checked(&ev));
                        }
                    />
                    " Dismiss on focus outside?"
                </label>

                <hr />

                <label style="display: block;">
                    <input
                        type="checkbox"
                        prop:checked=disable_outside_pointer_events
                        on:change=move |ev| {
                            set_disable_outside_pointer_events.set(event_target_checked(&ev));
                        }
                    />
                    " Disable outside pointer events?"
                </label>
            </div>

            <div style="margin-bottom: 20px;">
                <button
                    node_ref=open_button_ref
                    type="button"
                    on:click=move |_| set_open.update(|v| *v = !*v)
                >
                    {move || if open.get() { "Close" } else { "Open" }}
                    " layer"
                </button>
            </div>

            <Show when=move || open.get()>
                <DismissableLayer
                    attr:style="display: inline-flex; justify-content: center; align-items: center; vertical-align: middle; width: 400px; height: 300px; background-color: black; border-radius: 10px; margin-bottom: 20px;"
                    disable_outside_pointer_events=disable_outside_pointer_events
                    on_escape_key_down=Callback::new(move |event: web_sys::KeyboardEvent| {
                        if !dismiss_on_escape.get_untracked() {
                            event.prevent_default();
                        }
                    })
                    on_pointer_down_outside=Callback::new(move |event: web_sys::CustomEvent| {
                        let target = custom_event_target(&event);
                        let button = open_button_ref.get_untracked().map(|el| {
                            let el: web_sys::EventTarget = el.into();
                            el
                        });
                        if !dismiss_on_pointer_down_outside.get_untracked()
                            || target == button
                        {
                            event.prevent_default();
                        }
                    })
                    on_focus_outside=Callback::new(move |event: web_sys::CustomEvent| {
                        if !dismiss_on_focus_outside.get_untracked() {
                            event.prevent_default();
                        }
                    })
                    on_dismiss=Callback::new(move |_| set_open.set(false))
                >
                    <input type="text" />
                </DismissableLayer>
            </Show>

            <div style="margin-bottom: 20px;">
                <input type="text" value="hello" style="margin-right: 20px;" />
                <button type="button" on:mousedown=move |_| {
                    web_sys::window().unwrap().alert_with_message("hey!").ok();
                }>
                    "hey!"
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn Nested() -> impl IntoView {
    view! {
        <div style="font-family: sans-serif; text-align: center;">
            <h1>"DismissableLayer (nested)"</h1>
            <DismissableBox />
        </div>
    }
}

/// Root dismissable box (no dismiss handlers, always present).
#[component]
fn DismissableBox() -> impl IntoView {
    let (open, set_open) = signal(false);
    let open_button_ref = NodeRef::<leptos::html::Button>::new();

    view! {
        <DismissableLayer
            attr:style="display: inline-block; vertical-align: middle; padding: 20px; background-color: rgba(0, 0, 0, 0.2); border-radius: 10px; margin-top: 20px;"
        >
            <div>
                <button
                    node_ref=open_button_ref
                    type="button"
                    on:click=move |_| set_open.update(|v| *v = !*v)
                >
                    {move || if open.get() { "Close" } else { "Open" }}
                    " new layer"
                </button>
            </div>

            <Show when=move || open.get()>
                {dismissable_box_child(open_button_ref, set_open)}
            </Show>
        </DismissableLayer>
    }
}

/// Child dismissable box with dismiss handlers. Uses a plain function returning
/// `AnyView` (concrete type) instead of `impl IntoView` to enable infinite
/// recursion without hitting Rust's recursive opaque type limitation.
fn dismissable_box_child(
    open_button_ref: NodeRef<leptos::html::Button>,
    set_open: WriteSignal<bool>,
) -> AnyView {
    let (inner_open, set_inner_open) = signal(false);
    let inner_button_ref = NodeRef::<leptos::html::Button>::new();

    view! {
        <DismissableLayer
            attr:style="display: inline-block; vertical-align: middle; padding: 20px; background-color: rgba(0, 0, 0, 0.2); border-radius: 10px; margin-top: 20px;"
            on_pointer_down_outside=Callback::new(move |event: web_sys::CustomEvent| {
                let target = custom_event_target(&event);
                let button = open_button_ref.get_untracked().map(|el| {
                    let el: web_sys::EventTarget = el.into();
                    el
                });
                if target == button {
                    event.prevent_default();
                }
            })
            on_focus_outside=Callback::new(move |event: web_sys::CustomEvent| {
                event.prevent_default();
            })
            on_dismiss=Callback::new(move |_| set_open.set(false))
        >
            <div>
                <button
                    node_ref=inner_button_ref
                    type="button"
                    on:click=move |_| set_inner_open.update(|v| *v = !*v)
                >
                    {move || if inner_open.get() { "Close" } else { "Open" }}
                    " new layer"
                </button>
            </div>

            <Show when=move || inner_open.get()>
                {dismissable_box_child(inner_button_ref, set_inner_open)}
            </Show>
        </DismissableLayer>
    }
    .into_any()
}

#[component]
pub fn WithFocusScope() -> impl IntoView {
    let (open, set_open) = signal(false);
    let open_button_ref = NodeRef::<leptos::html::Button>::new();

    view! {
        <div style="font-family: sans-serif; text-align: center;">
            <h1>"DismissableLayer + FocusScope"</h1>
            <div style="margin-bottom: 20px;">
                <button
                    node_ref=open_button_ref
                    type="button"
                    on:click=move |_| set_open.update(|v| *v = !*v)
                >
                    {move || if open.get() { "Close" } else { "Open" }}
                    " layer"
                </button>
            </div>

            <Show when=move || open.get()>
                <DismissableLayer
                    as_child=true
                    disable_outside_pointer_events=true
                    on_escape_key_down=Callback::new(move |event: web_sys::KeyboardEvent| {
                        // If active element is an input/textarea, blur it instead of dismissing
                        if let Some(active) = document().active_element() {
                            let tag = active.tag_name().to_lowercase();
                            if tag == "input" || tag == "textarea" {
                                if let Ok(el) = active.dyn_into::<web_sys::HtmlElement>() {
                                    el.blur().ok();
                                }
                                event.prevent_default();
                            }
                        }
                    })
                    on_pointer_down_outside=Callback::new(move |event: web_sys::CustomEvent| {
                        let target = custom_event_target(&event);
                        let button = open_button_ref.get_untracked().map(|el| {
                            let el: web_sys::EventTarget = el.into();
                            el
                        });
                        if target == button {
                            event.prevent_default();
                        }
                    })
                    on_dismiss=Callback::new(move |_| set_open.set(false))
                >
                    <FocusScope
                        trapped=true
                        attr:style="display: inline-flex; justify-content: center; align-items: center; vertical-align: middle; width: 400px; height: 300px; background-color: black; border-radius: 10px; margin-bottom: 20px;"
                    >
                        <input type="text" />
                    </FocusScope>
                </DismissableLayer>
            </Show>

            <div style="margin-bottom: 20px;">
                <input type="text" value="hello" style="margin-right: 20px;" />
                <button type="button" on:mousedown=move |_| {
                    web_sys::window().unwrap().alert_with_message("hey!").ok();
                }>
                    "hey!"
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn DialogExample() -> impl IntoView {
    view! {
        <div style={format!("height: 300vh; font-family: {SYSTEM_FONT}")}>
            <h1>"Dialog (fully modal example)"</h1>
            <ul style="list-style: none; padding: 0; margin-bottom: 30px;">
                <li>{"\u{2705}"}" focus should move inside Dialog when mounted"</li>
                <li>{"\u{2705}"}" focus should be trapped inside Dialog"</li>
                <li>{"\u{2705}"}" scrolling outside Dialog should be disabled"</li>
                <li>{"\u{2705}"}" should be able to dismiss Dialog on pressing escape"</li>
                <li style="margin-left: 30px;">{"\u{2705}"}" focus should return to the open button"</li>
                <li>
                    {"\u{2705}"}" interacting outside Dialog should be disabled (clicking the \"alert me\" button shouldn't do anything)"
                </li>
                <li>{"\u{2795}"}</li>
                <li>{"\u{2705}"}" should be able to dismiss Dialog when interacting outside"</li>
                <li style="margin-left: 30px;">{"\u{2705}"}" focus should return to the open button"</li>
            </ul>
            <div style="display: flex; gap: 10px;">
                <DummyDialog open_label="Open Dialog" close_label="Close Dialog" />
                <input type="text" value="some input" />
                <button type="button" on:click=move |_| {
                    web_sys::window().unwrap().alert_with_message("clicked!").ok();
                }>
                    "Alert me"
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn PopoverFullyModal() -> impl IntoView {
    view! {
        <div style={format!("height: 300vh; font-family: {SYSTEM_FONT}")}>
            <h1>"Popover (fully modal example)"</h1>
            <ul style="list-style: none; padding: 0; margin-bottom: 30px;">
                <li>{"\u{2705}"}" focus should move inside Popover when mounted"</li>
                <li>{"\u{2705}"}" focus should be trapped inside Popover"</li>
                <li>{"\u{2705}"}" scrolling outside Popover should be disabled"</li>
                <li>{"\u{2705}"}" should be able to dismiss Popover on pressing escape"</li>
                <li style="margin-left: 30px;">{"\u{2705}"}" focus should return to the open button"</li>
                <li>
                    {"\u{2705}"}" interacting outside Popover should be disabled (clicking the \"alert me\" button shouldn't do anything)"
                </li>
                <li>{"\u{2795}"}</li>
                <li>{"\u{2705}"}" should be able to dismiss Popover when interacting outside"</li>
                <li style="margin-left: 30px;">{"\u{2705}"}" focus should return to the open button"</li>
            </ul>
            <div style="display: flex; gap: 10px;">
                <DummyPopover
                    open_label="Open Popover"
                    close_label="Close Popover"
                    disable_outside_pointer_events=true
                />
                <input type="text" value="some input" />
                <button type="button" on:click=move |_| {
                    web_sys::window().unwrap().alert_with_message("clicked!").ok();
                }>
                    "Alert me"
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn PopoverSemiModal() -> impl IntoView {
    let (color, set_color) = signal("royalblue".to_string());
    let change_color_button_ref = NodeRef::<leptos::html::Button>::new();

    view! {
        <div style={format!("height: 300vh; font-family: {SYSTEM_FONT}")}>
            <h1>"Popover (semi-modal example)"</h1>
            <ul style="list-style: none; padding: 0; margin-bottom: 30px;">
                <li>{"\u{2705}"}" focus should move inside Popover when mounted"</li>
                <li>{"\u{2705}"}" focus should be trapped inside Popover"</li>
                <li>{"\u{2705}"}" scrolling outside Popover should be allowed"</li>
                <li>{"\u{2705}"}" should be able to dismiss Popover on pressing escape"</li>
                <li style="margin-left: 30px;">{"\u{2705}"}" focus should return to the open button"</li>
                <li>
                    {"\u{2705}"}" interacting outside Popover should be allowed (clicking the \"alert me\" button should trigger)"
                </li>
                <li>{"\u{2795}"}</li>
                <li>
                    {"\u{2705}"}" should be able to dismiss Popover when interacting outside "
                    <span style="font-weight: 600;">"unless specified (ie. change color button)"</span>
                </li>
                <li style="margin-left: 30px;">
                    {"\u{2705}"}" focus should "<span style="font-weight: 600;">"NOT"</span>" return to the open button when unmounted, natural focus should occur"
                </li>
            </ul>
            <div style="display: flex; gap: 10px;">
                <DummyPopover
                    color=color
                    open_label="Open Popover"
                    close_label="Close Popover"
                    on_pointer_down_outside=Callback::new(move |event: web_sys::CustomEvent| {
                        let target = custom_event_target(&event);
                        let button = change_color_button_ref.get_untracked().map(|el| {
                            let el: web_sys::EventTarget = el.into();
                            el
                        });
                        if target == button {
                            event.prevent_default();
                        }
                    })
                />
                <input type="text" value="some input" />
                <button type="button" on:click=move |_| {
                    web_sys::window().unwrap().alert_with_message("clicked!").ok();
                }>
                    "Alert me"
                </button>
                <button
                    node_ref=change_color_button_ref
                    type="button"
                    on:click=move |_| {
                        set_color.update(|c| {
                            *c = if *c == "royalblue" { "tomato".to_string() } else { "royalblue".to_string() };
                        });
                    }
                >
                    "Change color"
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn PopoverNonModal() -> impl IntoView {
    view! {
        <div style={format!("height: 300vh; font-family: {SYSTEM_FONT}")}>
            <h1>"Popover (non modal example)"</h1>
            <ul style="list-style: none; padding: 0; margin-bottom: 30px;">
                <li>{"\u{2705}"}" focus should move inside Popover when mounted"</li>
                <li>
                    {"\u{2705}"}" focus should "<span style="font-weight: 600;">"NOT"</span>" be trapped inside Popover"
                </li>
                <li>{"\u{2705}"}" scrolling outside Popover should be allowed"</li>
                <li>{"\u{2705}"}" should be able to dismiss Popover on pressing escape"</li>
                <li style="margin-left: 30px;">{"\u{2705}"}" focus should return to the open button"</li>
                <li>
                    {"\u{2705}"}" interacting outside Popover should be allowed (clicking the \"alert me\" button should trigger)"
                </li>
                <li>{"\u{2795}"}</li>
                <li>{"\u{2705}"}" should be able to dismiss Popover when clicking outside"</li>
                <li style="margin-left: 30px;">
                    {"\u{2705}"}" focus should "<span style="font-weight: 600;">"NOT"</span>" return to the open button when unmounted, natural focus should occur"
                </li>
                <li>{"\u{2705}"}" should be able to dismiss Popover when focus leaves it"</li>
                <li style="margin-left: 30px;">
                    {"\u{2753}"}" focus should move to next tabbable element after open button"
                    <div style="font-weight: 600;">
                        <span style="margin-left: 20px;">"notes:"</span>
                        <ul>
                            <li>"I have left this one out for now as I am still unsure in which case it should do this"</li>
                            <li>"for the moment, focus will be returned to the open button when FocusScope unmounts"</li>
                            <li>"Need to do some more thinking, in the meantime, I think this behavior is ok"</li>
                        </ul>
                    </div>
                </li>
            </ul>
            <div style="display: flex; gap: 10px;">
                <DummyPopover
                    open_label="Open Popover"
                    close_label="Close Popover"
                    trapped=false
                />
                <input type="text" value="some input" />
                <button type="button" on:click=move |_| {
                    web_sys::window().unwrap().alert_with_message("clicked!").ok();
                }>
                    "Alert me"
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn PopoverInDialog() -> impl IntoView {
    view! {
        <div style={format!("height: 300vh; font-family: {SYSTEM_FONT}")}>
            <h1>"Popover (semi-modal) in Dialog (fully modal)"</h1>
            <ul style="list-style: none; padding: 0; margin-bottom: 30px;">
                <li>
                    {"\u{2705}"}" dismissing Popover by pressing escape should "
                    <span style="font-weight: 600;">"NOT"</span>" dismiss Dialog"
                </li>
                <li>{"\u{2705}"}" dismissing Popover by clicking outside should also dismiss Dialog"</li>
            </ul>
            <div style="display: flex; gap: 10px;">
                <DummyDialog open_label="Open Dialog" close_label="Close Dialog">
                    <DummyPopover open_label="Open Popover" close_label="Close Popover" />
                </DummyDialog>
                <input type="text" value="some input" />
                <button type="button" on:click=move |_| {
                    web_sys::window().unwrap().alert_with_message("clicked!").ok();
                }>
                    "Alert me"
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn PopoverNested() -> impl IntoView {
    view! {
        <div style={format!("height: 300vh; font-family: {SYSTEM_FONT}")}>
            <h1>"Popover (nested example)"</h1>
            <ul style="list-style: none; padding: 0; margin-bottom: 30px;">
                <li>
                    {"\u{2705}"}" dismissing a Popover by pressing escape should only dismiss that given Popover, not its parents"
                </li>
                <li>
                    {"\u{2705}"}" interacting outside the blue Popover should only dismiss itself and not its parents"
                </li>
                <li>{"\u{2705}"}" interacting outside the red Popover should dismiss itself and the black one"</li>
                <li>{"\u{2705}"}" unless the click wasn't outside the black one"</li>
                <li>
                    {"\u{2705}"}" when the blue Popover is open, there should be "
                    <span style="font-weight: 600;">"NO"</span>" text cursor above the red or black inputs"
                </li>
                <li>
                    {"\u{2705}"}" when the red Popover is open, there should be a text cursor above the black input but not the one on the page behind"
                </li>
            </ul>
            <div style="display: flex; gap: 10px;">
                <DummyPopover
                    disable_outside_pointer_events=true
                    on_interact_outside=Callback::new(move |_: web_sys::CustomEvent| {
                        web_sys::console::log_1(&"interact outside black".into());
                    })
                >
                    <DummyPopover
                        color="tomato"
                        open_label="Open red"
                        close_label="Close red"
                        on_interact_outside=Callback::new(move |_: web_sys::CustomEvent| {
                            web_sys::console::log_1(&"interact outside red".into());
                        })
                    >
                        <DummyPopover
                            color="royalblue"
                            open_label="Open blue"
                            close_label="Close blue"
                            disable_outside_pointer_events=true
                            on_interact_outside=Callback::new(move |_: web_sys::CustomEvent| {
                                web_sys::console::log_1(&"interact outside blue".into());
                            })
                        />
                    </DummyPopover>
                </DummyPopover>
                <input type="text" value="some input" />
                <button type="button" on:click=move |_| {
                    web_sys::window().unwrap().alert_with_message("clicked!").ok();
                }>
                    "Alert me"
                </button>
            </div>
        </div>
    }
}

/* -------------------------------------------------------------------------------------------------
 * Dummy components
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn DummyDialog(
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(into, optional, default = "Open".into())] open_label: String,
    #[prop(into, optional, default = "Close".into())] close_label: String,
) -> impl IntoView {
    let (open, set_open) = signal(false);
    let children: StoredValue<Option<ChildrenFn>> = StoredValue::new(children);
    let open_label = StoredValue::new(open_label);
    let close_label = StoredValue::new(close_label);

    view! {
        <button type="button" on:click=move |_| set_open.update(|v| *v = !*v)>
            {open_label.get_value()}
        </button>
        <Show when=move || open.get()>
            <FocusGuards>
                <Portal as_child=true>
                    <div style="position: fixed; top: 0; right: 0; bottom: 0; left: 0; pointer-events: none; background-color: black; opacity: 0.2;" />
                </Portal>
                <Portal as_child=true>
                    <DismissableLayer
                        as_child=true
                        disable_outside_pointer_events=true
                        on_dismiss=Callback::new(move |_| set_open.set(false))
                    >
                        <FocusScope
                            trapped=true
                            attr:style="box-sizing: border-box; display: flex; align-items: start; gap: 10px; position: fixed; top: 50%; left: 50%; transform: translate(-50%, -50%); background: white; min-width: 300px; min-height: 200px; padding: 40px; border-radius: 10px; background-color: white; box-shadow: 0 2px 10px rgba(0, 0, 0, 0.12);"
                        >
                            {children.with_value(|c| c.as_ref().map(|c| c()))}
                            <button type="button" on:click=move |_| set_open.set(false)>
                                {close_label.get_value()}
                            </button>
                            <input type="text" value="hello world" />
                        </FocusScope>
                    </DismissableLayer>
                </Portal>
            </FocusGuards>
        </Show>
    }
}

#[component]
fn DummyPopover(
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(into, optional, default = "Open".into())] open_label: String,
    #[prop(into, optional, default = "Close".into())] close_label: String,
    #[prop(into, optional)] color: MaybeProp<String>,
    #[prop(into, optional)] trapped: MaybeProp<bool>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] on_interact_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] disable_outside_pointer_events: MaybeProp<bool>,
) -> impl IntoView {
    let color = Signal::derive(move || color.get().unwrap_or_else(|| "#333".to_string()));
    let trapped = Signal::derive(move || trapped.get().unwrap_or(true));
    let disable_outside_pointer_events_val =
        Signal::derive(move || disable_outside_pointer_events.get().unwrap_or(false));

    let (skip_unmount_auto_focus, set_skip_unmount_auto_focus) = signal(false);
    let (open, set_open) = signal(false);
    let open_button_ref = NodeRef::<leptos::html::Button>::new();

    let children: StoredValue<Option<ChildrenFn>> = StoredValue::new(children);
    let open_label = StoredValue::new(open_label);
    let close_label = StoredValue::new(close_label);

    view! {
        <Popper>
            <PopperAnchor as_child=true>
                <button
                    type="button"
                    node_ref=open_button_ref
                    on:click=move |_| set_open.update(|v| *v = !*v)
                >
                    {open_label.get_value()}
                </button>
            </PopperAnchor>
            <Show when=move || open.get()>
                <FocusGuards>
                    <Portal as_child=true>
                        <DismissableLayer
                            as_child=true
                            disable_outside_pointer_events=disable_outside_pointer_events_val
                            on_escape_key_down=Callback::new(move |event: web_sys::KeyboardEvent| {
                                if let Some(handler) = on_escape_key_down {
                                    handler.run(event);
                                }
                            })
                            on_focus_outside=Callback::new(move |event: web_sys::CustomEvent| {
                                if let Some(handler) = on_focus_outside {
                                    handler.run(event);
                                }
                            })
                            on_interact_outside=Callback::new(move |event: web_sys::CustomEvent| {
                                if let Some(handler) = on_interact_outside {
                                    handler.run(event);
                                }
                            })
                            on_pointer_down_outside=Callback::new(move |event: web_sys::CustomEvent| {
                                set_skip_unmount_auto_focus.set(!disable_outside_pointer_events_val.get_untracked());
                                let target = custom_event_target(&event);
                                let button = open_button_ref.get_untracked().map(|el| {
                                    let el: web_sys::EventTarget = el.into();
                                    el
                                });
                                if target == button {
                                    event.prevent_default();
                                } else if let Some(handler) = on_pointer_down_outside {
                                    handler.run(event);
                                }
                            })
                            on_dismiss=Callback::new(move |_| set_open.set(false))
                        >
                            <FocusScope
                                as_child=true
                                trapped=trapped
                                on_unmount_auto_focus=Some(Callback::new(move |event: web_sys::Event| {
                                    if skip_unmount_auto_focus.get_untracked() {
                                        event.prevent_default();
                                    }
                                    set_skip_unmount_auto_focus.set(false);
                                }))
                            >
                                <PopperContent
                                    style:filter="drop-shadow(0 2px 10px rgba(0, 0, 0, 0.12))"
                                    style:display="flex"
                                    style:align-items="flex-start"
                                    style:gap="10px"
                                    style:background="white"
                                    style:min-width="200px"
                                    style:min-height="150px"
                                    style:padding="20px"
                                    style:border-radius="4px"
                                    style:background-color=move || color.get()
                                    side_offset=10.0
                                >
                                    {children.with_value(|c| c.as_ref().map(|c| c()))}
                                    <button type="button" on:click=move |_| set_open.set(false)>
                                        {close_label.get_value()}
                                    </button>
                                    <input type="text" value="hello world" />
                                    <PopperArrow
                                        width=10.0
                                        height=4.0
                                        style:fill=move || color.get()
                                    />
                                </PopperContent>
                            </FocusScope>
                        </DismissableLayer>
                    </Portal>
                </FocusGuards>
            </Show>
        </Popper>
    }
}
