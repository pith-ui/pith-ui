use leptos::prelude::*;
use radix_leptos_avatar::*;
use web_sys::wasm_bindgen::{JsCast, closure::Closure};

stylance::import_crate_style!(classes, "src/primitives/avatar.stories.module.css");

const SRC: &str = "https://picsum.photos/id/1005/400/400";
const SRC_ALTERNATIVE: &str = "https://picsum.photos/id/1006/400/400";
const SRC_BROKEN: &str = "https://broken.link.com/broken-pic.jpg";

#[component]
pub fn Styled() -> impl IntoView {
    view! {
        <h1>"Without image & with fallback"</h1>
        <Avatar attr:class=classes::root>
            <AvatarFallback attr:class=classes::fallback>"JS"</AvatarFallback>
        </Avatar>

        <h1>"With image & with fallback"</h1>
        <Avatar attr:class=classes::root>
            <AvatarImage
                src=SRC.to_string()
                attr:alt="John Smith"
                attr:class=classes::image
            />
            <AvatarFallback delay_ms=300 attr:class=classes::fallback>
                "JS"
            </AvatarFallback>
        </Avatar>

        <h1>"With image & with fallback (but broken src)"</h1>
        <Avatar attr:class=classes::root>
            <AvatarImage
                src=SRC_BROKEN.to_string()
                on_loading_status_change=Callback::new(move |status: ImageLoadingStatus| {
                    log::info!("{:?}", status);
                })
                attr:alt="John Smith"
                attr:class=classes::image
            />
            <AvatarFallback attr:class=classes::fallback>
                <AvatarIcon />
            </AvatarFallback>
        </Avatar>

        <h1>"Changing image src"</h1>
        <SourceChanger />
    }
}

#[component]
pub fn Chromatic() -> impl IntoView {
    view! {
        <h1>"Without image & with fallback"</h1>
        <Avatar attr:class=classes::root>
            <AvatarFallback attr:class=classes::fallback>"JS"</AvatarFallback>
        </Avatar>

        <h1>"With image & with fallback"</h1>
        <Avatar attr:class=classes::root>
            <AvatarImage
                src=SRC.to_string()
                attr:alt="John Smith"
                attr:class=classes::image
            />
            <AvatarFallback delay_ms=300 attr:class=classes::fallback>
                "JS"
            </AvatarFallback>
        </Avatar>

        <h1>"With image & with fallback (but broken src)"</h1>
        <Avatar attr:class=classes::root>
            <AvatarImage
                src=SRC_BROKEN.to_string()
                attr:alt="John Smith"
                attr:class=classes::image
            />
            <AvatarFallback attr:class=classes::fallback>
                <AvatarIcon />
            </AvatarFallback>
        </Avatar>

        <h1>"Changing image src"</h1>
        <SourceChanger />
    }
}

/// Cycles through different image sources every 1000ms to test runtime URL changes.
#[component]
fn SourceChanger() -> impl IntoView {
    let sources = [SRC, SRC_ALTERNATIVE, SRC_BROKEN];
    let (src, set_src) = signal(SRC.to_string());

    // Set up interval to cycle through sources
    let interval_id = StoredValue::new(None::<i32>);
    Effect::new(move |_| {
        let callback: Closure<dyn Fn()> = Closure::new(move || {
            set_src.update(|current| {
                let idx = sources
                    .iter()
                    .position(|s| *s == current.as_str())
                    .unwrap_or(0);
                let next = (idx + 1) % sources.len();
                *current = sources[next].to_string();
            });
        });

        let id = web_sys::window()
            .expect("Window should exist.")
            .set_interval_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                1000,
            )
            .expect("setInterval should succeed.");

        interval_id.set_value(Some(id));

        // Keep closure alive for the duration of the effect
        std::mem::forget(callback);
    });

    on_cleanup(move || {
        if let Some(id) = interval_id.get_value() {
            web_sys::window()
                .expect("Window should exist.")
                .clear_interval_with_handle(id);
        }
    });

    view! {
        <Avatar attr:class=classes::root>
            <AvatarImage
                src=Signal::derive(move || Some(src.get()))
                attr:alt="John Smith"
                attr:class=classes::image
            />
            <AvatarFallback delay_ms=300 attr:class=classes::fallback>
                "JS"
            </AvatarFallback>
        </Avatar>
    }
}

#[component]
fn AvatarIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" width="42" height="42">
            <path
                d="M50 51.7a22.1 22.1 0 100-44.2 22.1 22.1 0 000 44.2zM87.9 69.3a27.8 27.8 0 00-21.2-16.1 4 4 0 00-2.8.7 23.5 23.5 0 01-27.6 0 4 4 0 00-2.8-.7 27.5 27.5 0 00-21.2 16.1c-.3.6-.2 1.3.1 1.8a52.8 52.8 0 007 8.9 43.4 43.4 0 0056.9 3.8 56.3 56.3 0 008.9-8.8c.9-1.2 1.8-2.5 2.6-3.9.3-.6.3-1.2.1-1.8z"
                fill="currentColor"
            />
        </svg>
    }
}
