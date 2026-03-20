use leptos::prelude::*;
use cardo_ui::hover_card::*;

#[component]
pub fn HoverCardPage() -> impl IntoView {
    let (controlled, set_controlled) = signal(false);
    let (controlled_open, set_controlled_open) = signal(false);

    view! {
        <HoverCard open_delay=0.0 close_delay=0.0>
            <HoverCardTrigger
                class:hover-card-trigger=true
                attr:href="#"
                attr:data-testid="hover-card-trigger"
            >
                "trigger"
            </HoverCardTrigger>
            <HoverCardPortal>
                <HoverCardContent
                    class:hover-card-content=true
                    side_offset=5.0
                    attr:data-testid="hover-card-content"
                    style:background="tomato"
                >
                    <p>"Hover card content"</p>
                    <p>"Supplementary information"</p>
                    <HoverCardArrow class:hover-card-arrow=true width=20.0 height=10.0 />
                </HoverCardContent>
            </HoverCardPortal>
        </HoverCard>

        <br />
        <br />

        <label>
            <input
                type="checkbox"
                prop:checked=move || controlled.get()
                on:change=move |ev| set_controlled.set(event_target_checked(&ev))
            />
            " controlled"
        </label>

        <br />
        <br />

        <Show when=move || controlled.get()>
            <button data-testid="open-controlled" on:click=move |_| set_controlled_open.set(true)>
                "open"
            </button>
            <button data-testid="close-controlled" on:click=move |_| set_controlled_open.set(false)>
                "close"
            </button>
            <span data-testid="controlled-state">
                {move || if controlled_open.get() { "open" } else { "closed" }}
            </span>

            <br />
            <br />

            <HoverCard
                open=controlled_open
                on_open_change=Callback::new(move |v: bool| set_controlled_open.set(v))
                open_delay=0.0
                close_delay=0.0
            >
                <HoverCardTrigger
                    class:hover-card-trigger=true
                    attr:href="#"
                    attr:data-testid="controlled-trigger"
                >
                    "controlled trigger"
                </HoverCardTrigger>
                <HoverCardPortal>
                    <HoverCardContent
                        class:hover-card-content=true
                        side_offset=5.0
                        attr:data-testid="controlled-content"
                    >
                        <p>"Controlled hover card content"</p>
                        <HoverCardArrow class:hover-card-arrow=true width=20.0 height=10.0 />
                    </HoverCardContent>
                </HoverCardPortal>
            </HoverCard>

            <br />
            <br />
        </Show>

        <hr />

        <h3>"Delayed"</h3>
        <HoverCard open_delay=500.0 close_delay=300.0>
            <HoverCardTrigger
                class:hover-card-trigger=true
                attr:href="#"
                attr:data-testid="delayed-trigger"
            >
                "delayed trigger"
            </HoverCardTrigger>
            <HoverCardPortal>
                <HoverCardContent
                    class:hover-card-content=true
                    side_offset=5.0
                    attr:data-testid="delayed-content"
                >
                    <p>"Delayed hover card content"</p>
                    <HoverCardArrow class:hover-card-arrow=true width=20.0 height=10.0 />
                </HoverCardContent>
            </HoverCardPortal>
        </HoverCard>

        <br />
        <br />

        <button data-testid="outside-element">"outside"</button>
    }
}
