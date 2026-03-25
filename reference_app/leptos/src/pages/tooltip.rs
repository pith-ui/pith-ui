use leptos::prelude::*;
use pith_ui::tooltip::{
    Tooltip, TooltipArrow, TooltipContent, TooltipPortal, TooltipProvider, TooltipTrigger,
};

#[component]
pub fn TooltipPage() -> impl IntoView {
    let (controlled_open, set_controlled_open) = signal(false);

    view! {
        <TooltipProvider delay_duration=0.0 skip_delay_duration=0.0>
            <Tooltip>
                <TooltipTrigger
                    class:tooltip-trigger=true
                    attr:data-testid="tooltip-trigger-1"
                >
                    "trigger 1"
                </TooltipTrigger>
                <TooltipPortal>
                    <TooltipContent
                        class:tooltip-content=true
                        side_offset=5.0
                        attr:data-testid="tooltip-content-1"
                    >
                        "Tooltip 1"
                        <TooltipArrow class:tooltip-arrow=true width=12.0 height=6.0 />
                    </TooltipContent>
                </TooltipPortal>
            </Tooltip>

            <br />
            <br />

            <Tooltip>
                <TooltipTrigger
                    class:tooltip-trigger=true
                    attr:data-testid="tooltip-trigger-2"
                >
                    "trigger 2"
                </TooltipTrigger>
                <TooltipPortal>
                    <TooltipContent
                        class:tooltip-content=true
                        side_offset=5.0
                        attr:data-testid="tooltip-content-2"
                    >
                        "Tooltip 2"
                        <TooltipArrow class:tooltip-arrow=true width=12.0 height=6.0 />
                    </TooltipContent>
                </TooltipPortal>
            </Tooltip>

            <br />
            <br />

            <Tooltip
                open=Signal::derive(move || controlled_open.get())
                on_open_change=Callback::new(move |open: bool| set_controlled_open.set(open))
            >
                <TooltipTrigger
                    class:tooltip-trigger=true
                    attr:data-testid="tooltip-trigger-controlled"
                >
                    "controlled trigger"
                </TooltipTrigger>
                <TooltipPortal>
                    <TooltipContent
                        class:tooltip-content=true
                        side_offset=5.0
                        attr:data-testid="tooltip-content-controlled"
                    >
                        "Controlled Tooltip"
                        <TooltipArrow class:tooltip-arrow=true width=12.0 height=6.0 />
                    </TooltipContent>
                </TooltipPortal>
            </Tooltip>

            <br />

            <label>
                <input
                    type="checkbox"
                    prop:checked=move || controlled_open.get()
                    on:change=move |ev| {
                        set_controlled_open.set(event_target_checked(&ev));
                    }
                />
                "open controlled"
            </label>
            <button
                type="button"
                data-testid="controlled-external-close"
                on:click=move |_| set_controlled_open.set(false)
            >
                "external close"
            </button>
            <span data-testid="controlled-open-state">
                {move || if controlled_open.get() { "open" } else { "closed" }}
            </span>

            <br />
            <br />

            <button data-testid="outside-button">"outside"</button>
        </TooltipProvider>
    }
}
