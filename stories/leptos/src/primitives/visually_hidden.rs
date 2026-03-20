use leptos::prelude::*;
use cardo_ui::visually_hidden::*;

#[component]
pub fn Basic() -> impl IntoView {
    view! {
        <button>
            <VisuallyHidden>Save the file</VisuallyHidden>
            <span attr:aria-hidden>{"💾"}</span>
        </button>
    }
}
