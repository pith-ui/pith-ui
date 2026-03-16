use leptos::prelude::*;
use radix_leptos_primitives::context_menu::*;

#[component]
pub fn ContextMenuPage() -> impl IntoView {
    let (last_action, set_last_action) = signal(String::new());
    let (checked, set_checked) = signal(false);
    let (radio_value, set_radio_value) = signal("radio1".to_string());
    let (disabled, set_disabled) = signal(false);

    view! {
        <ContextMenu>
            <ContextMenuTrigger class:context-trigger=true attr:data-testid="context-trigger">
                "Right click here"
            </ContextMenuTrigger>
            <ContextMenuPortal>
                <ContextMenuContent class:context-content=true>
                    <ContextMenuLabel class:context-label=true>"Actions"</ContextMenuLabel>
                    <ContextMenuItem
                        class:context-item=true
                        on_select=Callback::new({
                            let set_last_action = set_last_action.clone();
                            move |_: web_sys::Event| set_last_action.set("Item 1".into())
                        })
                    >
                        "Item 1"
                    </ContextMenuItem>
                    <ContextMenuItem
                        class:context-item=true
                        disabled=disabled
                        on_select=Callback::new({
                            let set_last_action = set_last_action.clone();
                            move |_: web_sys::Event| set_last_action.set("Item 2".into())
                        })
                    >
                        "Item 2"
                    </ContextMenuItem>
                    <ContextMenuItem
                        class:context-item=true
                        on_select=Callback::new({
                            let set_last_action = set_last_action.clone();
                            move |_: web_sys::Event| set_last_action.set("Item 3".into())
                        })
                    >
                        "Item 3"
                    </ContextMenuItem>

                    <ContextMenuSeparator class:context-separator=true />

                    <ContextMenuItem
                        class:context-item=true
                        on_select=Callback::new({
                            let set_last_action = set_last_action.clone();
                            move |_: web_sys::Event| set_last_action.set("Apple".into())
                        })
                    >
                        "Apple"
                    </ContextMenuItem>
                    <ContextMenuItem
                        class:context-item=true
                        on_select=Callback::new({
                            let set_last_action = set_last_action.clone();
                            move |_: web_sys::Event| set_last_action.set("Banana".into())
                        })
                    >
                        "Banana"
                    </ContextMenuItem>

                    <ContextMenuSeparator class:context-separator=true />

                    <ContextMenuCheckboxItem
                        class:context-item=true
                        checked=Signal::derive(move || CheckedState::from(checked.get()))
                        on_checked_change=Callback::new(move |v: bool| set_checked.set(v))
                    >
                        <ContextMenuItemIndicator class:context-indicator=true>
                            "\u{2713}"
                        </ContextMenuItemIndicator>
                        "Check me"
                    </ContextMenuCheckboxItem>

                    <ContextMenuSeparator class:context-separator=true />

                    <ContextMenuRadioGroup
                        value=radio_value
                        on_value_change=Callback::new(move |v: String| set_radio_value.set(v))
                    >
                        <ContextMenuRadioItem class:context-item=true value="radio1">
                            <ContextMenuItemIndicator class:context-indicator=true>
                                "\u{25cf}"
                            </ContextMenuItemIndicator>
                            "Radio 1"
                        </ContextMenuRadioItem>
                        <ContextMenuRadioItem class:context-item=true value="radio2">
                            <ContextMenuItemIndicator class:context-indicator=true>
                                "\u{25cf}"
                            </ContextMenuItemIndicator>
                            "Radio 2"
                        </ContextMenuRadioItem>
                    </ContextMenuRadioGroup>

                    <ContextMenuSeparator class:context-separator=true />

                    <ContextMenuSub>
                        <ContextMenuSubTrigger class:context-item=true class:context-sub-trigger=true>
                            "Submenu \u{2192}"
                        </ContextMenuSubTrigger>
                        <ContextMenuPortal>
                            <ContextMenuSubContent class:context-content=true side_offset=2.0>
                                <ContextMenuItem
                                    class:context-item=true
                                    on_select=Callback::new({
                                        let set_last_action = set_last_action.clone();
                                        move |_: web_sys::Event| set_last_action.set("Sub item 1".into())
                                    })
                                >
                                    "Sub item 1"
                                </ContextMenuItem>
                                <ContextMenuItem
                                    class:context-item=true
                                    on_select=Callback::new({
                                        let set_last_action = set_last_action.clone();
                                        move |_: web_sys::Event| set_last_action.set("Sub item 2".into())
                                    })
                                >
                                    "Sub item 2"
                                </ContextMenuItem>
                            </ContextMenuSubContent>
                        </ContextMenuPortal>
                    </ContextMenuSub>
                </ContextMenuContent>
            </ContextMenuPortal>
        </ContextMenu>

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

        // Styled trigger for internal styles testing
        <ContextMenu>
            <ContextMenuTrigger
                class:context-trigger=true
                attr:data-testid="styled-context-trigger"
                style:background="tomato"
            >
                "Styled trigger"
            </ContextMenuTrigger>
        </ContextMenu>

        <br />

        <button data-testid="outside-button">"outside"</button>
        <input data-testid="outside-input" placeholder="name" />
    }
}
