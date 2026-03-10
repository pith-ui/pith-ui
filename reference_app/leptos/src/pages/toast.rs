use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use radix_leptos_primitives::toast::*;
use web_sys::wasm_bindgen::JsCast;

#[component]
pub fn ToastPage() -> impl IntoView {
    let (open, set_open) = signal(false);
    let (auto_dismiss, set_auto_dismiss) = signal(false);
    let (count, set_count) = signal(0u32);
    let timer_ref: StoredValue<Option<i32>> = StoredValue::new(None);
    let (show_uncontrolled, set_show_uncontrolled) = signal(false);
    let (multi_count, set_multi_count) = signal(0u32);
    let (controlled_open, set_controlled_open) = signal(false);

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

    // node_ref for the viewport <ol> element.
    // attr:class on ToastViewport goes to the outer DismissableLayerBranch div,
    // not the inner <ol>, so we apply class and data-testid via node_ref.
    let viewport_ref = AnyNodeRef::new();
    Effect::new(move |_| {
        if let Some(el) = viewport_ref.get() {
            let el: &web_sys::HtmlElement = el.unchecked_ref();
            let _ = el.set_attribute("class", "toast-viewport");
            let _ = el.set_attribute("data-testid", "toast-viewport");
        }
    });

    view! {
        <ToastProvider duration=duration swipe_direction=Signal::derive(|| SwipeDirection::Right)>
            <h2>"Controlled"</h2>
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
                class="toast-root"
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

            <h2>"Uncontrolled"</h2>
            <button
                data-testid="show-uncontrolled"
                on:click=move |_| set_show_uncontrolled.update(|v| *v = !*v)
            >
                {move || if show_uncontrolled.get() { "Hide uncontrolled" } else { "Show uncontrolled" }}
            </button>

            <Show when=move || show_uncontrolled.get()>
                <UncontrolledToast />
            </Show>

            <h2>"Controlled Mode"</h2>
            <label>
                <input
                    type="checkbox"
                    prop:checked=move || controlled_open.get()
                    on:change=move |ev| {
                        let target: web_sys::HtmlInputElement = ev.target().unwrap().unchecked_into();
                        set_controlled_open.set(target.checked());
                    }
                />
                " open controlled toast"
            </label>

            <Toast
                class="toast-root"
                open=controlled_open
                on_open_change=Callback::new(move |value: bool| set_controlled_open.set(value))
                duration=MaybeProp::from(Some(1_000_000))
            >
                <ToastTitle attr:class="toast-title">"Controlled toast title"</ToastTitle>
                <ToastDescription attr:class="toast-description">"Controlled toast description"</ToastDescription>
                <ToastClose attr:class="toast-close" as_child=true>
                    <button data-testid="controlled-toast-close">"Close controlled"</button>
                </ToastClose>
            </Toast>

            <h2>"Multi-toast tab order"</h2>
            <button data-testid="add-multi-toast" on:click=move |_| set_multi_count.update(|c| *c += 1)>
                "Add multi toast"
            </button>
            <button data-testid="before-viewport">"Before viewport"</button>

            <For
                each=move || 1..=multi_count.get()
                key=|i| *i
                let:id
            >
                <MultiToast id=id />
            </For>

            <ToastViewport node_ref=viewport_ref label="Notifications" />
            <button data-testid="after-viewport">"After viewport"</button>
        </ToastProvider>
    }
}

#[component]
fn MultiToast(id: u32) -> impl IntoView {
    let toast_ref = AnyNodeRef::new();
    let testid = format!("multi-toast-{id}");
    Effect::new(move |_| {
        if let Some(el) = toast_ref.get() {
            let el: &web_sys::HtmlElement = el.unchecked_ref();
            let _ = el.set_attribute("data-testid", &testid);
        }
    });

    view! {
        <Toast class="toast-root" open=true duration=MaybeProp::from(Some(1_000_000)) node_ref=toast_ref>
            <ToastTitle attr:class="toast-title">
                {format!("Multi toast {id}")}
            </ToastTitle>
            <ToastDescription attr:class="toast-description">
                {format!("Description {id}")}
            </ToastDescription>
            <ToastAction alt_text=format!("Action for toast {id}") as_child=true>
                <button data-testid=format!("multi-action-{id}")>
                    {format!("Action {id}")}
                </button>
            </ToastAction>
            <ToastClose as_child=true>
                <button data-testid=format!("multi-close-{id}")>
                    {format!("Close {id}")}
                </button>
            </ToastClose>
        </Toast>
    }
}

#[component]
fn UncontrolledToast() -> impl IntoView {
    view! {
        <Toast class="toast-root" duration=MaybeProp::from(Some(1_000_000))>
            <ToastTitle attr:class="toast-title">"Uncontrolled toast"</ToastTitle>
            <ToastDescription attr:class="toast-description">
                "This toast has no open prop"
            </ToastDescription>
            <ToastClose attr:class="toast-close" as_child=true>
                <button data-testid="uncontrolled-close">"Close uncontrolled"</button>
            </ToastClose>
        </Toast>
    }
}
