use leptos::prelude::*;
use radix_leptos_toast::*;
use web_sys::wasm_bindgen::{closure::Closure, JsCast};

stylance::import_crate_style!(classes, "src/primitives/toast.stories.module.css");

#[component]
pub fn Styled() -> impl IntoView {
    view! {
        <ToastProvider>
            <ToastUpgradeAvailable />
            <ToastViewport attr:class=classes::viewport />
        </ToastProvider>
    }
}

#[component]
pub fn Controlled() -> impl IntoView {
    let (has_upgrade, set_has_upgrade) = signal(false);
    let (is_subscribed, set_is_subscribed) = signal(false);
    let (saved_count, set_saved_count) = signal(0u32);
    let (error_count, set_error_count) = signal(0u32);

    // Auto-show upgrade toast after 10 seconds
    Effect::new(move |_| {
        if !has_upgrade.get() {
            let callback: Closure<dyn Fn()> = Closure::new(move || {
                set_has_upgrade.set(true);
            });
            let id = web_sys::window()
                .expect("Window should exist.")
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    callback.as_ref().unchecked_ref(),
                    10000,
                )
                .expect("setTimeout should succeed.");

            std::mem::forget(callback);

            on_cleanup(move || {
                web_sys::window()
                    .expect("Window should exist.")
                    .clear_timeout_with_handle(id);
            });
        }
    });

    view! {
        <ToastProvider>
            <button on:click=move |_| set_is_subscribed.set(true)>"subscribe"</button>
            <button on:click=move |_| set_error_count.update(|c| *c += 1)>"error"</button>
            <button on:click=move |_| set_saved_count.update(|c| *c += 1)>"save"</button>

            <ToastUpgradeAvailable
                open=has_upgrade
                on_open_change=Callback::new(move |open: bool| set_has_upgrade.set(open))
            />
            <ToastSubscribeSuccess
                open=is_subscribed
                on_open_change=Callback::new(move |open: bool| set_is_subscribed.set(open))
            />

            <For
                each=move || 0..error_count.get()
                key=|i| *i
                let:_i
            >
                <Toast
                    attr:class=format!("{} {}", classes::root, classes::errorRoot)
                >
                    <ToastDescription>"There was an error"</ToastDescription>
                    <ToastAction
                        attr:class=classes::button
                        alt_text="Resubmit the form to try again."
                    >
                        "Try again"
                    </ToastAction>
                </Toast>
            </For>

            <For
                each=move || 0..saved_count.get()
                key=|i| *i
                let:_i
            >
                <Toast attr:class=classes::root>
                    <ToastDescription>"Successfully saved"</ToastDescription>
                </Toast>
            </For>

            <ToastViewport attr:class=classes::viewport />
        </ToastProvider>
    }
}

#[component]
pub fn Animated() -> impl IntoView {
    let (open, set_open) = signal(false);
    let (swipe_direction, set_swipe_direction) = signal(SwipeDirection::Right);
    let timer_id = StoredValue::new(None::<i32>);

    let handle_open = move |_: web_sys::MouseEvent| {
        set_open.set(false);
        if let Some(id) = timer_id.get_value() {
            web_sys::window()
                .expect("Window should exist.")
                .clear_timeout_with_handle(id);
        }
        let callback: Closure<dyn Fn()> = Closure::new(move || {
            set_open.set(true);
        });
        let id = web_sys::window()
            .expect("Window should exist.")
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                150,
            )
            .expect("setTimeout should succeed.");
        timer_id.set_value(Some(id));
        std::mem::forget(callback);
    };

    let swipe_threshold = Signal::derive(move || {
        match swipe_direction.get() {
            SwipeDirection::Up | SwipeDirection::Down => 25.0,
            _ => 50.0,
        }
    });

    view! {
        <ToastProvider
            swipe_direction=swipe_direction
            swipe_threshold=swipe_threshold
        >
            <button on:click=handle_open>"Open"</button>
            <select on:change=move |ev| {
                let val = event_target_value(&ev);
                set_swipe_direction.set(match val.as_str() {
                    "left" => SwipeDirection::Left,
                    "up" => SwipeDirection::Up,
                    "down" => SwipeDirection::Down,
                    _ => SwipeDirection::Right,
                });
            }>
                <option value="right">"Slide right"</option>
                <option value="left">"Slide left"</option>
                <option value="up">"Slide up"</option>
                <option value="down">"Slide down"</option>
            </select>
            <Toast
                attr:class=format!("{} {}", classes::root, classes::animatedRoot)
                open=open
                on_open_change=Callback::new(move |o: bool| set_open.set(o))
            >
                <ToastDescription>
                    {move || format!("Swipe me {}", match swipe_direction.get() {
                        SwipeDirection::Right => "right",
                        SwipeDirection::Left => "left",
                        SwipeDirection::Up => "up",
                        SwipeDirection::Down => "down",
                    })}
                </ToastDescription>
                <ToastClose attr:class=classes::button>"Dismiss"</ToastClose>
            </Toast>
            <ToastViewport attr:class=classes::viewport />
        </ToastProvider>
    }
}

#[component]
pub fn Chromatic() -> impl IntoView {
    view! {
        <h1>"Order"</h1>
        <ToastProvider duration=Signal::derive(|| 1_000_000)>
            <Toast attr:class=classes::root>
                <div class=classes::header>
                    <ToastTitle attr:class=classes::title>"Toast 1"</ToastTitle>
                    <ToastClose attr:class=classes::close>"\u{00d7}"</ToastClose>
                </div>
                <ToastDescription attr:class=classes::description>"Description"</ToastDescription>
                <ToastAction
                    alt_text="alternative"
                    attr:class=classes::button
                    attr:style="margin-top: 10px"
                >
                    "Action"
                </ToastAction>
            </Toast>
            <Toast attr:class=classes::root>
                <div class=classes::header>
                    <ToastTitle attr:class=classes::title>"Toast 2"</ToastTitle>
                    <ToastClose attr:class=classes::close>"\u{00d7}"</ToastClose>
                </div>
                <ToastDescription attr:class=classes::description>"Description"</ToastDescription>
                <ToastAction
                    alt_text="alternative"
                    attr:class=classes::button
                    attr:style="margin-top: 10px"
                >
                    "Action"
                </ToastAction>
            </Toast>
            <ToastViewport attr:class=classes::chromaticViewport />
        </ToastProvider>

        <h1>"Uncontrolled"</h1>

        <h2>"Open"</h2>
        <ToastProvider>
            <Toast duration=MaybeProp::from(Some(1_000_000)) attr:class=classes::root>
                <div class=classes::header>
                    <ToastTitle attr:class=classes::title>"Toast"</ToastTitle>
                    <ToastClose attr:class=classes::close>"\u{00d7}"</ToastClose>
                </div>
                <ToastDescription attr:class=classes::description>"Description"</ToastDescription>
                <ToastAction
                    alt_text="alternative"
                    attr:class=classes::button
                    attr:style="margin-top: 10px"
                >
                    "Action"
                </ToastAction>
            </Toast>
            <ToastViewport attr:class=classes::chromaticViewport />
        </ToastProvider>

        <h2>"Closed"</h2>
        <ToastProvider>
            <Toast
                default_open=false
                duration=MaybeProp::from(Some(1_000_000))
                attr:class=classes::root
            >
                <div class=classes::header>
                    <ToastTitle attr:class=classes::title>"Title"</ToastTitle>
                    <ToastClose attr:class=classes::close>"\u{00d7}"</ToastClose>
                </div>
                <ToastDescription attr:class=classes::description>"Uncontrolled foreground closed"</ToastDescription>
                <ToastAction
                    alt_text="alternative"
                    attr:class=classes::button
                    attr:style="margin-top: 10px"
                >
                    "Action"
                </ToastAction>
            </Toast>
            <ToastViewport attr:class=classes::chromaticViewport />
        </ToastProvider>

        <h1>"Controlled"</h1>

        <h2>"Open"</h2>
        <ToastProvider>
            <Toast
                open=true
                duration=MaybeProp::from(Some(1_000_000))
                attr:class=classes::root
            >
                <div class=classes::header>
                    <ToastTitle attr:class=classes::title>"Toast"</ToastTitle>
                    <ToastClose attr:class=classes::close>"\u{00d7}"</ToastClose>
                </div>
                <ToastDescription attr:class=classes::description>"Description"</ToastDescription>
                <ToastAction
                    alt_text="alternative"
                    attr:class=classes::button
                    attr:style="margin-top: 10px"
                >
                    "Action"
                </ToastAction>
            </Toast>
            <ToastViewport attr:class=classes::chromaticViewport />
        </ToastProvider>

        <h2>"Closed"</h2>
        <ToastProvider>
            <Toast
                open=false
                duration=MaybeProp::from(Some(1_000_000))
                attr:class=classes::root
            >
                <div class=classes::header>
                    <ToastTitle attr:class=classes::title>"Toast"</ToastTitle>
                    <ToastClose attr:class=classes::close>"\u{00d7}"</ToastClose>
                </div>
                <ToastDescription attr:class=classes::description>"Description"</ToastDescription>
                <ToastAction
                    alt_text="alternative"
                    attr:class=classes::button
                    attr:style="margin-top: 10px"
                >
                    "Action"
                </ToastAction>
            </Toast>
            <ToastViewport attr:class=classes::chromaticViewport />
        </ToastProvider>
    }
}

/* ─────────────────────────────────────────────────────────────────── */
/* Helper components                                                  */
/* ─────────────────────────────────────────────────────────────────── */

#[component]
fn ToastUpgradeAvailable(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
) -> impl IntoView {
    view! {
        <Toast
            attr:class=classes::root
            open=open
            on_open_change=on_open_change.unwrap_or(Callback::new(|_| {}))
        >
            <div class=classes::header>
                <ToastTitle attr:class=classes::title>"Upgrade available"</ToastTitle>
                <ToastClose attr:class=classes::close attr:aria-label="Close">
                    <span aria-hidden="true">"\u{00d7}"</span>
                </ToastClose>
            </div>
            <ToastDescription attr:class=classes::description>
                "We\u{2019}ve just released Radix version 3.0"
            </ToastDescription>
            <ToastAction
                alt_text="Goto account settings to upgrade"
                attr:class=classes::button
                attr:style="margin-top: 10px"
            >
                "Upgrade"
            </ToastAction>
        </Toast>
    }
}

#[component]
fn ToastSubscribeSuccess(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
) -> impl IntoView {
    view! {
        <Toast
            attr:class=classes::root
            open=open
            on_open_change=on_open_change.unwrap_or(Callback::new(|_| {}))
        >
            <div class=format!("{} {}", classes::header, classes::successHeader)>
                <ToastTitle attr:class=classes::title>"Success!"</ToastTitle>
                <ToastClose attr:class=classes::close attr:aria-label="Close">
                    <span aria-hidden="true">"\u{00d7}"</span>
                </ToastClose>
            </div>
            <ToastDescription attr:class=classes::description>
                "You have subscribed. We\u{2019}ll be in touch."
            </ToastDescription>
        </Toast>
    }
}
