use leptos::prelude::*;
use radix_leptos_primitives::dropdown_menu::*;

#[component]
pub fn DropdownMenuPage() -> impl IntoView {
    let (last_action, set_last_action) = signal(String::new());
    let (checked, set_checked) = signal(false);
    let (radio_value, set_radio_value) = signal("radio1".to_string());
    let (disabled, set_disabled) = signal(false);
    let (controlled_open, set_controlled_open) = signal(false);
    let (trigger_click_count, set_trigger_click_count) = signal(0i32);

    view! {
        <DropdownMenu>
            <DropdownMenuTrigger
                attr:class="dropdown-trigger"
                attr:data-custom="dropdown-trigger-custom"
                on:click=move |_| set_trigger_click_count.update(|c| *c += 1)
            >
                "open"
            </DropdownMenuTrigger>
            <DropdownMenuPortal>
                <DropdownMenuContent class="dropdown-content" side_offset=5.0 attr:style="--radix-dropdown-menu-content-transform-origin: 0px 0px">
                    <DropdownMenuLabel attr:class="dropdown-label">"Actions"</DropdownMenuLabel>
                    <DropdownMenuItem
                        attr:class="dropdown-item"
                        on_select=Callback::new({
                            let set_last_action = set_last_action.clone();
                            move |_: web_sys::Event| set_last_action.set("Item 1".into())
                        })
                    >
                        "Item 1"
                    </DropdownMenuItem>
                    <DropdownMenuItem
                        attr:class="dropdown-item"
                        disabled=disabled
                        on_select=Callback::new({
                            let set_last_action = set_last_action.clone();
                            move |_: web_sys::Event| set_last_action.set("Item 2".into())
                        })
                    >
                        "Item 2"
                    </DropdownMenuItem>
                    <DropdownMenuItem
                        attr:class="dropdown-item"
                        on_select=Callback::new({
                            let set_last_action = set_last_action.clone();
                            move |_: web_sys::Event| set_last_action.set("Item 3".into())
                        })
                    >
                        "Item 3"
                    </DropdownMenuItem>

                    <DropdownMenuSeparator attr:class="dropdown-separator" />

                    <DropdownMenuCheckboxItem
                        attr:class="dropdown-item"
                        checked=Signal::derive(move || CheckedState::from(checked.get()))
                        on_checked_change=Callback::new(move |v: bool| set_checked.set(v))
                    >
                        <DropdownMenuItemIndicator attr:class="dropdown-indicator">
                            "\u{2713}"
                        </DropdownMenuItemIndicator>
                        "Check me"
                    </DropdownMenuCheckboxItem>

                    <DropdownMenuSeparator attr:class="dropdown-separator" />

                    <DropdownMenuRadioGroup
                        value=radio_value
                        on_value_change=Callback::new(move |v: String| set_radio_value.set(v))
                    >
                        <DropdownMenuRadioItem attr:class="dropdown-item" value="radio1">
                            <DropdownMenuItemIndicator attr:class="dropdown-indicator">
                                "\u{25cf}"
                            </DropdownMenuItemIndicator>
                            "Radio 1"
                        </DropdownMenuRadioItem>
                        <DropdownMenuRadioItem attr:class="dropdown-item" value="radio2">
                            <DropdownMenuItemIndicator attr:class="dropdown-indicator">
                                "\u{25cf}"
                            </DropdownMenuItemIndicator>
                            "Radio 2"
                        </DropdownMenuRadioItem>
                    </DropdownMenuRadioGroup>

                    <DropdownMenuSeparator attr:class="dropdown-separator" />

                    <DropdownMenuSub>
                        <DropdownMenuSubTrigger attr:class="dropdown-item dropdown-sub-trigger">
                            "Submenu \u{2192}"
                        </DropdownMenuSubTrigger>
                        <DropdownMenuPortal>
                            <DropdownMenuSubContent attr:class="dropdown-content" side_offset=2.0>
                                <DropdownMenuItem
                                    attr:class="dropdown-item"
                                    on_select=Callback::new({
                                        let set_last_action = set_last_action.clone();
                                        move |_: web_sys::Event| set_last_action.set("Sub item 1".into())
                                    })
                                >
                                    "Sub item 1"
                                </DropdownMenuItem>
                                <DropdownMenuItem
                                    attr:class="dropdown-item"
                                    on_select=Callback::new({
                                        let set_last_action = set_last_action.clone();
                                        move |_: web_sys::Event| set_last_action.set("Sub item 2".into())
                                    })
                                >
                                    "Sub item 2"
                                </DropdownMenuItem>
                            </DropdownMenuSubContent>
                        </DropdownMenuPortal>
                    </DropdownMenuSub>

                    <DropdownMenuArrow attr:class="dropdown-arrow" />
                </DropdownMenuContent>
            </DropdownMenuPortal>
        </DropdownMenu>

        <br />
        <br />

        <label>
            <input
                type="checkbox"
                prop:checked=move || disabled.get()
                on:change=move |ev| set_disabled.set(event_target_checked(&ev))
            />
            " disabled"
        </label>

        <br />
        <br />

        <span data-testid="last-action">{move || last_action.get()}</span>
        <br />
        <span data-testid="checkbox-state">{move || if checked.get() { "true" } else { "false" }}</span>
        <br />
        <span data-testid="radio-value">{move || radio_value.get()}</span>

        <br />
        <br />

        <span data-testid="trigger-click-count">{move || trigger_click_count.get()}</span>
        <br />
        <button data-testid="outside-button">"outside"</button>
        <input data-testid="outside-input" placeholder="name" />

        <br />
        <br />
        <hr />

        // Controlled dropdown menu
        <h3>"Controlled"</h3>

        <label>
            <input
                type="checkbox"
                data-testid="controlled-open-checkbox"
                prop:checked=move || controlled_open.get()
                on:change=move |ev| set_controlled_open.set(event_target_checked(&ev))
            />
            " open"
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

        <DropdownMenu
            open=controlled_open
            on_open_change=Callback::new(move |value: bool| set_controlled_open.set(value))
        >
            <DropdownMenuTrigger
                attr:class="controlled-dropdown-trigger"
                attr:data-testid="controlled-dropdown-trigger"
            >
                "controlled open"
            </DropdownMenuTrigger>
            <DropdownMenuPortal>
                <DropdownMenuContent
                    class="dropdown-content"
                    attr:data-testid="controlled-dropdown-content"
                    side_offset=5.0
                >
                    <DropdownMenuItem attr:class="dropdown-item">
                        "Controlled Item 1"
                    </DropdownMenuItem>
                    <DropdownMenuItem attr:class="dropdown-item">
                        "Controlled Item 2"
                    </DropdownMenuItem>
                </DropdownMenuContent>
            </DropdownMenuPortal>
        </DropdownMenu>
    }
}
