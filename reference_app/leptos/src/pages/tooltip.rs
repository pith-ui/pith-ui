use leptos::prelude::*;
use radix_leptos_tooltip::{
    Tooltip, TooltipArrow, TooltipContent, TooltipPortal, TooltipProvider, TooltipTrigger,
};

#[component]
pub fn TooltipPage() -> impl IntoView {
    view! {
        <TooltipProvider delay_duration=0.0 skip_delay_duration=0.0>
            <Tooltip>
                <TooltipTrigger
                    attr:class="tooltip-trigger"
                    attr:data-testid="tooltip-trigger-1"
                >
                    "trigger 1"
                </TooltipTrigger>
                <TooltipPortal>
                    <TooltipContent
                        attr:class="tooltip-content"
                        side_offset=5.0
                        attr:data-testid="tooltip-content-1"
                    >
                        "Tooltip 1"
                        <TooltipArrow attr:class="tooltip-arrow" width=12.0 height=6.0 />
                    </TooltipContent>
                </TooltipPortal>
            </Tooltip>

            <br />
            <br />

            <Tooltip>
                <TooltipTrigger
                    attr:class="tooltip-trigger"
                    attr:data-testid="tooltip-trigger-2"
                >
                    "trigger 2"
                </TooltipTrigger>
                <TooltipPortal>
                    <TooltipContent
                        attr:class="tooltip-content"
                        side_offset=5.0
                        attr:data-testid="tooltip-content-2"
                    >
                        "Tooltip 2"
                        <TooltipArrow attr:class="tooltip-arrow" width=12.0 height=6.0 />
                    </TooltipContent>
                </TooltipPortal>
            </Tooltip>

            <br />
            <br />

            <button data-testid="outside-button">"outside"</button>
        </TooltipProvider>
    }
}
