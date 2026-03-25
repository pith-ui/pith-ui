use leptos::prelude::*;
use pith_ui::separator::*;

#[component]
pub fn SeparatorPage() -> impl IntoView {
    view! {
        <p>"Content above horizontal separator"</p>

        <Separator
            class:separator-root=true
            attr:data-testid="horizontal-separator"
            attr:data-custom="user-value"
        />

        <p>"Content below horizontal separator"</p>

        <div class="separator-vertical-container">
            <span>"Left"</span>
            <Separator
                class:separator-root=true
                orientation=Orientation::Vertical
                attr:data-testid="vertical-separator"
            />
            <span>"Right"</span>
        </div>

        <br />

        <Separator
            class:separator-root=true
            decorative=true
            attr:data-testid="decorative-separator"
        />

        <p>"Content below decorative separator"</p>
    }
}
