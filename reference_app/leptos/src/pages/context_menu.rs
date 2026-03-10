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
            <ContextMenuTrigger attr:class="context-trigger" attr:data-testid="context-trigger">
                "Right click here"
            </ContextMenuTrigger>
            <ContextMenuPortal>
                <ContextMenuContent attr:class="context-content">
                    <ContextMenuLabel attr:class="context-label">"Actions"</ContextMenuLabel>
                    <ContextMenuItem
                        attr:class="context-item"
                        on_select=Callback::new({
                            let set_last_action = set_last_action.clone();
                            move |_: web_sys::Event| set_last_action.set("Item 1".into())
                        })
                    >
                        "Item 1"
                    </ContextMenuItem>
                    <ContextMenuItem
                        attr:class="context-item"
                        disabled=disabled
                        on_select=Callback::new({
                            let set_last_action = set_last_action.clone();
                            move |_: web_sys::Event| set_last_action.set("Item 2".into())
                        })
                    >
                        "Item 2"
                    </ContextMenuItem>
                    <ContextMenuItem
                        attr:class="context-item"
                        on_select=Callback::new({
                            let set_last_action = set_last_action.clone();
                            move |_: web_sys::Event| set_last_action.set("Item 3".into())
                        })
                    >
                        "Item 3"
                    </ContextMenuItem>

                    <ContextMenuSeparator attr:class="context-separator" />

                    <ContextMenuItem
                        attr:class="context-item"
                        on_select=Callback::new({
                            let set_last_action = set_last_action.clone();
                            move |_: web_sys::Event| set_last_action.set("Apple".into())
                        })
                    >
                        "Apple"
                    </ContextMenuItem>
                    <ContextMenuItem
                        attr:class="context-item"
                        on_select=Callback::new({
                            let set_last_action = set_last_action.clone();
                            move |_: web_sys::Event| set_last_action.set("Banana".into())
                        })
                    >
                        "Banana"
                    </ContextMenuItem>

                    <ContextMenuSeparator attr:class="context-separator" />

                    <ContextMenuCheckboxItem
                        attr:class="context-item"
                        checked=Signal::derive(move || CheckedState::from(checked.get()))
                        on_checked_change=Callback::new(move |v: bool| set_checked.set(v))
                    >
                        <ContextMenuItemIndicator attr:class="context-indicator">
                            "\u{2713}"
                        </ContextMenuItemIndicator>
                        "Check me"
                    </ContextMenuCheckboxItem>

                    <ContextMenuSeparator attr:class="context-separator" />

                    <ContextMenuRadioGroup
                        value=radio_value
                        on_value_change=Callback::new(move |v: String| set_radio_value.set(v))
                    >
                        <ContextMenuRadioItem attr:class="context-item" value="radio1">
                            <ContextMenuItemIndicator attr:class="context-indicator">
                                "\u{25cf}"
                            </ContextMenuItemIndicator>
                            "Radio 1"
                        </ContextMenuRadioItem>
                        <ContextMenuRadioItem attr:class="context-item" value="radio2">
                            <ContextMenuItemIndicator attr:class="context-indicator">
                                "\u{25cf}"
                            </ContextMenuItemIndicator>
                            "Radio 2"
                        </ContextMenuRadioItem>
                    </ContextMenuRadioGroup>

                    <ContextMenuSeparator attr:class="context-separator" />

                    <ContextMenuSub>
                        <ContextMenuSubTrigger attr:class="context-item context-sub-trigger">
                            "Submenu \u{2192}"
                        </ContextMenuSubTrigger>
                        <ContextMenuPortal>
                            <ContextMenuSubContent attr:class="context-content" side_offset=2.0>
                                <ContextMenuItem
                                    attr:class="context-item"
                                    on_select=Callback::new({
                                        let set_last_action = set_last_action.clone();
                                        move |_: web_sys::Event| set_last_action.set("Sub item 1".into())
                                    })
                                >
                                    "Sub item 1"
                                </ContextMenuItem>
                                <ContextMenuItem
                                    attr:class="context-item"
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
                attr:class="context-trigger"
                attr:data-testid="styled-context-trigger"
                attr:style="background: tomato"
            >
                "Styled trigger"
            </ContextMenuTrigger>
        </ContextMenu>

        <br />

        <button data-testid="outside-button">"outside"</button>
        <input data-testid="outside-input" placeholder="name" />
    }
}
