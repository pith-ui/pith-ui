use leptos::prelude::*;
use radix_leptos_separator::*;

#[component]
pub fn SeparatorPage() -> impl IntoView {
    view! {
        <p>"Content above horizontal separator"</p>

        <Separator
            attr:class="separator-root"
            attr:data-testid="horizontal-separator"
        />

        <p>"Content below horizontal separator"</p>

        <div class="separator-vertical-container">
            <span>"Left"</span>
            <Separator
                attr:class="separator-root"
                orientation=Orientation::Vertical
                attr:data-testid="vertical-separator"
            />
            <span>"Right"</span>
        </div>

        <br />

        <Separator
            attr:class="separator-root"
            decorative=true
            attr:data-testid="decorative-separator"
        />

        <p>"Content below decorative separator"</p>
    }
}
