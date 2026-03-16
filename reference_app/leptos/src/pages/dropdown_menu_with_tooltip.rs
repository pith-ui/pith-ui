use leptos::prelude::*;
use radix_leptos_primitives::dropdown_menu::*;
use radix_leptos_primitives::tooltip::*;

#[component]
pub fn DropdownMenuWithTooltipPage() -> impl IntoView {
    view! {
        <DropdownMenu>
            <TooltipProvider>
                <Tooltip>
                    <TooltipTrigger as_child=true>
                        <DropdownMenuTrigger class:dropdown-trigger=true>"open"</DropdownMenuTrigger>
                    </TooltipTrigger>
                    <TooltipContent>"Tooltip content"</TooltipContent>
                </Tooltip>
            </TooltipProvider>
            <DropdownMenuPortal>
                <DropdownMenuContent class:dropdown-content=true side_offset=5.0>
                    <DropdownMenuItem class:dropdown-item=true on_select=Callback::new(move |_: web_sys::Event| {})>
                        "Item 1"
                    </DropdownMenuItem>
                    <DropdownMenuItem class:dropdown-item=true on_select=Callback::new(move |_: web_sys::Event| {})>
                        "Item 2"
                    </DropdownMenuItem>
                    <DropdownMenuItem class:dropdown-item=true on_select=Callback::new(move |_: web_sys::Event| {})>
                        "Item 3"
                    </DropdownMenuItem>
                </DropdownMenuContent>
            </DropdownMenuPortal>
        </DropdownMenu>

        <br />
        <br />
        <button data-testid="outside-button">"outside"</button>
    }
}
