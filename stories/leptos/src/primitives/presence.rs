use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use radix_leptos_presence::Presence;
use send_wrapper::SendWrapper;
use tailwind_fuse::*;
use web_sys::wasm_bindgen::{JsCast, closure::Closure};

#[component]
pub fn Basic() -> impl IntoView {
    let (open, set_open) = signal(true);

    view! {
        <button on:click=move |_| set_open.update(|open| *open = !*open)>toggle</button>

        <Presence present=open>
            <div>Content</div>
        </Presence>
    }
}

#[component]
pub fn WithMountAnimation() -> impl IntoView {
    let class = MountAnimationClass::default().to_class();

    view! {
        <Animation class=class />
    }
}

#[component]
pub fn WithUnmountAnimation() -> impl IntoView {
    let class = UnmountAnimationClass::default().to_class();

    view! {
        <Animation class=class />
    }
}

#[component]
pub fn WithMultipleMountAnimations() -> impl IntoView {
    let class = MultipleMountAnimationsClass::default().to_class();

    view! {
        <Animation class=class />
    }
}

#[component]
pub fn WithOpenAndCloseAnimation() -> impl IntoView {
    let class = OpenAndCloseAnimationClass::default().to_class();

    view! {
        <Animation class=class />
    }
}

#[component]
pub fn WithMultipleOpenAndCloseAnimations() -> impl IntoView {
    let class = MultipleOpenAndCloseAnimationsClass::default().to_class();

    view! {
        <Animation class=class />
    }
}

#[component]
pub fn WithDeferredMountAnimation() -> impl IntoView {
    let mount_animation_class = StoredValue::new(MountAnimationClass::default().to_class());

    let node_ref = AnyNodeRef::new();
    let timer = RwSignal::new(0);
    let (open, set_open) = signal(false);
    let (animate, set_animate) = signal(false);

    let handler: SendWrapper<Closure<dyn Fn()>> = SendWrapper::new(Closure::new(move || {
        set_animate.set(true);
    }));
    let handler = StoredValue::new(handler);

    Effect::new(move |_| {
        let window = web_sys::window().expect("Window should exist.");
        if open.get() {
            handler.with_value(|handler| {
                timer.set(
                    window
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            handler.as_ref().unchecked_ref(),
                            150,
                        )
                        .expect("Timeout should be set."),
                );
            });
        } else {
            set_animate.set(false);
            window.clear_timeout_with_handle(timer.get());
        }
    });

    Owner::on_cleanup(move || {
        web_sys::window()
            .expect("Window should exist.")
            .clear_timeout_with_handle(timer.get());
    });

    view! {
        <p>
            Deferred animation should unmount correctly when toggled. Content will flash briefly while
            we wait for animation to be applied.
        </p>
        <Toggles
            open=open
            on_open_change=Callback::new(move |value| set_open.set(value))
            node_ref=node_ref
        />
        <Presence present=open node_ref=node_ref>
            <div
                node_ref=node_ref
                class=move || animate.get().then(|| mount_animation_class.get_value())
            >
                Content
            </div>
        </Presence>
    }
}

#[component]
fn Animation(#[prop(into, optional)] class: String) -> impl IntoView {
    let class = StoredValue::new(class);
    let node_ref = AnyNodeRef::new();
    let (open, set_open) = signal(false);

    view! {
        <Toggles
            open=open
            on_open_change=Callback::new(move |value| set_open.set(value))
            node_ref=node_ref
        />
        <Presence present=open node_ref=node_ref>
            <div
                node_ref=node_ref
                class=move || class.get_value()
                data-state=move || match open.get() {
                    true => "open",
                    false => "closed",
                }
            >
                Content
            </div>
        </Presence>
    }
}

#[component]
fn Toggles(
    #[prop(into)] open: Signal<bool>,
    on_open_change: Callback<bool>,
    node_ref: AnyNodeRef,
) -> impl IntoView {
    let handle_toggle_visibility = move |_| {
        if let Some(node) = node_ref.get() {
            let node: &web_sys::HtmlElement = node.unchecked_ref();
            let style = node.style();
            if style.get_property_value("display").ok() == Some("none".into()) {
                style
                    .set_property("display", "block")
                    .expect("Style should be updated.");
            } else {
                style
                    .set_property("display", "none")
                    .expect("Style should be updated.");
            }
        }
    };

    view! {
        <form style:display="flex" style:margin-bottom="30px">
            <fieldset>
                <legend>Mount</legend>
                <button type="button" on:click=move |_| on_open_change.run(!open.get())>
                    toggle
                </button>
            </fieldset>
            <fieldset>
                <legend>Visibility (triggers cancel event)</legend>
                <button type="button" on:click=handle_toggle_visibility>
                    toggle
                </button>
            </fieldset>
        </form>
    }
}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "animate-[presenceFadeIn_3s_ease-out]")]
struct MountAnimationClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "data-[state=closed]:animate-[presenceFadeOut_3s_ease-in]")]
struct UnmountAnimationClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "animate-[presenceFadeIn_6s_cubic-bezier(0.22,1,0.36,1),presenceSlideUp_6s_cubic-bezier(0.22,1,0.36,1)]"
)]
struct MultipleMountAnimationsClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "data-[state=open]:animate-[presenceFadeIn_3s_ease-out] data-[state=closed]:animate-[presenceFadeOut_3s_ease-in]"
)]
struct OpenAndCloseAnimationClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "data-[state=open]:animate-[presenceFadeIn_3s_cubic-bezier(0.22,1,0.36,1),presenceSlideUp_1s_cubic-bezier(0.22,1,0.36,1)] data-[state=closed]:animate-[presenceFadeOut_3s_cubic-bezier(0.22,1,0.36,1),presenceSlideDown_1s_cubic-bezier(0.22,1,0.36,1)]"
)]
struct MultipleOpenAndCloseAnimationsClass {}
