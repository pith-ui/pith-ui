use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use radix_leptos_toast::*;
use web_sys::wasm_bindgen::JsCast;

#[component]
pub fn ToastPage() -> impl IntoView {
    let (open, set_open) = signal(false);
    let (auto_dismiss, set_auto_dismiss) = signal(false);
    let (count, set_count) = signal(0u32);
    let timer_ref: StoredValue<Option<i32>> = StoredValue::new(None);

    let add_toast = move |_: web_sys::MouseEvent| {
        set_open.set(false);
        if let Some(id) = timer_ref.get_value() {
            web_sys::window()
                .expect("Window should exist.")
                .clear_timeout_with_handle(id);
        }
        let closure = web_sys::wasm_bindgen::closure::Closure::once_into_js(move || {
            set_count.update(|c| *c += 1);
            set_open.set(true);
        });
        let id = web_sys::window()
            .expect("Window should exist.")
            .set_timeout_with_callback_and_timeout_and_arguments_0(closure.unchecked_ref(), 100)
            .expect("setTimeout should succeed.");
        timer_ref.set_value(Some(id));
    };

    let duration = Signal::derive(move || if auto_dismiss.get() { 2000 } else { 1_000_000 });

    // node_ref for the viewport <ol> element, used to set data-testid on the correct element
    let viewport_ref = AnyNodeRef::new();
    Effect::new(move |_| {
        if let Some(el) = viewport_ref.get() {
            let el: &web_sys::HtmlElement = el.unchecked_ref();
            let _ = el.set_attribute("data-testid", "toast-viewport");
        }
    });

    view! {
        <ToastProvider duration=duration swipe_direction=Signal::derive(|| SwipeDirection::Right)>
            <button on:click=add_toast data-testid="add-toast">
                "Add toast"
            </button>

            <br />
            <br />

            <label>
                <input
                    type="checkbox"
                    prop:checked=move || auto_dismiss.get()
                    on:change=move |ev| {
                        let target: web_sys::HtmlInputElement = ev.target().unwrap().unchecked_into();
                        set_auto_dismiss.set(target.checked());
                    }
                />
                " auto-dismiss"
            </label>

            <br />
            <br />

            <span data-testid="toast-count">{move || count.get()}</span>

            <br />
            <br />

            <button data-testid="outside-button">"outside"</button>

            <Toast
                attr:class="toast-root"
                open=open
                on_open_change=Callback::new(move |value: bool| set_open.set(value))
            >
                <ToastTitle attr:class="toast-title">"Toast title"</ToastTitle>
                <ToastDescription attr:class="toast-description">"Toast description"</ToastDescription>
                <ToastAction attr:class="toast-action" alt_text="Undo the action" as_child=true>
                    <button>"Undo"</button>
                </ToastAction>
                <ToastClose attr:class="toast-close" as_child=true>
                    <button>"×"</button>
                </ToastClose>
            </Toast>

            <ToastViewport node_ref=viewport_ref attr:class="toast-viewport" label="Notifications" />
        </ToastProvider>
    }
}
