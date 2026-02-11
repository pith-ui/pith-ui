use leptos::prelude::*;
use radix_leptos_label::*;

stylance::import_crate_style!(label_classes, "src/primitives/label.stories.module.css");

#[component]
pub fn Styled() -> impl IntoView {
    view! {
        <Label attr:class=label_classes::root>Label</Label>
    }
}

#[component]
pub fn WithControl() -> impl IntoView {
    view! {
        <h1>Wrapping control</h1>
        <Label>
            <Control /> " Label"
        </Label>

        <h1>Referencing control</h1>
        <Control attr:id="control" />
        <Label attr:r#for="control">Label</Label>
    }
}

#[component]
pub fn WithInputNumber() -> impl IntoView {
    view! {
        <Label>
            <span>"Name:"</span>
            <input type="number" />
        </Label>
    }
}

#[component]
fn Control() -> impl IntoView {
    view! {
        <button
            class=label_classes::control
            on:click=move |_| window().alert_with_message("clicked").expect("Alert should be successful.")
        >
            "Control"
        </button>
    }
}
