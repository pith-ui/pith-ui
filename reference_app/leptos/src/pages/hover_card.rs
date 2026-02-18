use leptos::prelude::*;
use radix_leptos_hover_card::*;

#[component]
pub fn HoverCardPage() -> impl IntoView {
    view! {
        <HoverCard open_delay=0.0 close_delay=0.0>
            <HoverCardTrigger
                attr:class="hover-card-trigger"
                attr:href="#"
                attr:data-testid="hover-card-trigger"
            >
                "trigger"
            </HoverCardTrigger>
            <HoverCardPortal>
                <HoverCardContent
                    attr:class="hover-card-content"
                    side_offset=5.0
                    attr:data-testid="hover-card-content"
                >
                    <p>"Hover card content"</p>
                    <p>"Supplementary information"</p>
                    <HoverCardArrow attr:class="hover-card-arrow" width=20.0 height=10.0 />
                </HoverCardContent>
            </HoverCardPortal>
        </HoverCard>

        <br />
        <br />

        <button data-testid="outside-element">"outside"</button>
    }
}
