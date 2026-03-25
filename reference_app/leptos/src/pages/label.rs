use leptos::prelude::*;
use pith_ui::label::*;

#[component]
pub fn LabelPage() -> impl IntoView {
    view! {
        <Label attr:data-testid="basic-label" attr:r#for="basic-input">
            "Basic Label"
        </Label>
        <input id="basic-input" data-testid="basic-input" type="text" />

        <br />
        <br />

        <Label attr:data-testid="label-with-button">
            "Label with button "
            <button data-testid="nested-button">"Click me"</button>
        </Label>

        <br />
        <br />

        <Label attr:data-testid="label-with-input">
            "Label with input "
            <input data-testid="nested-input" type="text" />
        </Label>
    }
}
