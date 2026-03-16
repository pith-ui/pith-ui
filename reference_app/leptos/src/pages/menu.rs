use leptos::prelude::*;
use radix_leptos_primitives::menu::*;

#[component]
pub fn MenuPage() -> impl IntoView {
    let (last_action, set_last_action) = signal(String::new());
    let (bold_checked, set_bold_checked) = signal(false);
    let (italic_checked, set_italic_checked) = signal(true);
    let (font_size, set_font_size) = signal("Medium".to_string());
    let (sub_open, set_sub_open) = signal(false);
    let (animated, set_animated) = signal(false);

    // sub_content_class removed — using class: directives directly on MenuSubContent

    view! {
        <label>
            <input
                type="checkbox"
                prop:checked=animated
                on:change=move |ev| set_animated.set(event_target_checked(&ev))
                data-testid="animated-toggle"
            />
            " Animated"
        </label>
        <br /><br />
        <Menu open=true modal=false>
            <MenuAnchor class:menu-anchor=true>"Anchor"</MenuAnchor>
            <MenuPortal>
                <MenuContent class:menu-content=true style:background="tomato" style:outline="2px solid rgb(255, 0, 0)">
                    <MenuGroup>
                        <MenuLabel class:menu-label=true>"Fruits"</MenuLabel>
                        <MenuItem
                            class:menu-item=true
                            on_select=Callback::new({
                                let set_last_action = set_last_action.clone();
                                move |_: web_sys::Event| { set_last_action.set("Apple".into()); }
                            })
                        >
                            "Apple"
                        </MenuItem>
                        <MenuItem
                            class:menu-item=true
                            on_select=Callback::new({
                                let set_last_action = set_last_action.clone();
                                move |_: web_sys::Event| { set_last_action.set("Banana".into()); }
                            })
                        >
                            "Banana"
                        </MenuItem>
                        <MenuItem
                            class:menu-item=true
                            on_select=Callback::new({
                                let set_last_action = set_last_action.clone();
                                move |_: web_sys::Event| { set_last_action.set("Blueberry".into()); }
                            })
                        >
                            "Blueberry"
                        </MenuItem>
                    </MenuGroup>

                    <MenuSeparator class:menu-separator=true />

                    <MenuGroup>
                        <MenuLabel class:menu-label=true>"Vegetables"</MenuLabel>
                        <MenuItem
                            class:menu-item=true
                            on_select=Callback::new({
                                let set_last_action = set_last_action.clone();
                                move |_: web_sys::Event| { set_last_action.set("Broccoli".into()); }
                            })
                        >
                            "Broccoli"
                        </MenuItem>
                        <MenuItem
                            class:menu-item=true
                            disabled=true
                            on_select=Callback::new({
                                let set_last_action = set_last_action.clone();
                                move |_: web_sys::Event| { set_last_action.set("Carrot".into()); }
                            })
                        >
                            "Carrot"
                        </MenuItem>
                        <MenuItem
                            class:menu-item=true
                            on_select=Callback::new({
                                let set_last_action = set_last_action.clone();
                                move |_: web_sys::Event| { set_last_action.set("Courgette".into()); }
                            })
                        >
                            "Courgette"
                        </MenuItem>
                    </MenuGroup>

                    <MenuSeparator class:menu-separator=true />

                    <MenuItem
                        class:menu-item=true
                        on_select=Callback::new({
                            let set_last_action = set_last_action.clone();
                            move |_: web_sys::Event| { set_last_action.set("Undo".into()); }
                        })
                    >
                        "Undo"
                    </MenuItem>
                    <MenuItem
                        class:menu-item=true
                        on_select=Callback::new({
                            let set_last_action = set_last_action.clone();
                            move |_: web_sys::Event| { set_last_action.set("Redo".into()); }
                        })
                    >
                        "Redo"
                    </MenuItem>

                    <MenuSeparator class:menu-separator=true />

                    <MenuCheckboxItem
                        class:menu-item=true
                        checked=Signal::derive(move || CheckedState::from(bold_checked.get()))
                        on_checked_change=Callback::new(move |v: bool| set_bold_checked.set(v))
                    >
                        "Bold"
                        <MenuItemIndicator>"\u{2713}"</MenuItemIndicator>
                    </MenuCheckboxItem>
                    <MenuCheckboxItem
                        class:menu-item=true
                        checked=Signal::derive(move || CheckedState::from(italic_checked.get()))
                        on_checked_change=Callback::new(move |v: bool| set_italic_checked.set(v))
                    >
                        "Italic"
                        <MenuItemIndicator>"\u{2713}"</MenuItemIndicator>
                    </MenuCheckboxItem>

                    <MenuSeparator class:menu-separator=true />

                    <MenuRadioGroup value=font_size on_value_change=Callback::new(move |v: String| set_font_size.set(v))>
                        <MenuRadioItem class:menu-item=true value="Small">
                            "Small"
                            <MenuItemIndicator>"\u{2713}"</MenuItemIndicator>
                        </MenuRadioItem>
                        <MenuRadioItem class:menu-item=true value="Medium">
                            "Medium"
                            <MenuItemIndicator>"\u{2713}"</MenuItemIndicator>
                        </MenuRadioItem>
                        <MenuRadioItem class:menu-item=true value="Large">
                            "Large"
                            <MenuItemIndicator>"\u{2713}"</MenuItemIndicator>
                        </MenuRadioItem>
                    </MenuRadioGroup>

                    <MenuSeparator class:menu-separator=true />

                    <MenuGroup>
                        <MenuLabel class:menu-label=true>"Suits (textValue)"</MenuLabel>
                        <MenuItem class:menu-item=true text_value="Hearts">
                            <span role="img" aria-label="Hearts">"\u{2665}\u{fe0f}"</span> "Hearts"
                        </MenuItem>
                        <MenuItem class:menu-item=true text_value="Spades">
                            <span role="img" aria-label="Spades">"\u{2660}\u{fe0f}"</span> "Spades"
                        </MenuItem>
                        <MenuItem class:menu-item=true text_value="Diamonds">
                            <span role="img" aria-label="Diamonds">"\u{2666}\u{fe0f}"</span> "Diamonds"
                        </MenuItem>
                        <MenuItem class:menu-item=true text_value="Clubs">
                            <span role="img" aria-label="Clubs">"\u{2663}\u{fe0f}"</span> "Clubs"
                        </MenuItem>
                    </MenuGroup>

                    <MenuSeparator class:menu-separator=true />

                    <MenuSub open=sub_open on_open_change=Callback::new(move |v: bool| set_sub_open.set(v))>
                        <MenuSubTrigger class:menu-item=true>"More Options..."</MenuSubTrigger>
                        <MenuPortal>
                            <MenuSubContent class:menu-content=true class:menu-content-animated=animated>
                                <MenuItem
                                    class:menu-item=true
                                    on_select=Callback::new({
                                        let set_last_action = set_last_action.clone();
                                        move |_: web_sys::Event| { set_last_action.set("Option A".into()); }
                                    })
                                >
                                    "Option A"
                                </MenuItem>
                                <MenuItem
                                    class:menu-item=true
                                    on_select=Callback::new({
                                        let set_last_action = set_last_action.clone();
                                        move |_: web_sys::Event| { set_last_action.set("Option B".into()); }
                                    })
                                >
                                    "Option B"
                                </MenuItem>
                            </MenuSubContent>
                        </MenuPortal>
                    </MenuSub>
                </MenuContent>
            </MenuPortal>
        </Menu>

        <br />
        <br />

        <span data-testid="last-action">{move || last_action.get()}</span>

        <br />
        <br />

        <button data-testid="outside-button">"outside"</button>

        <br />
        <br />

        <span data-testid="checkbox-state">{move || {
            let mut items = Vec::new();
            if bold_checked.get() { items.push("Bold"); }
            if italic_checked.get() { items.push("Italic"); }
            items.join(",")
        }}</span>

        <br />

        <span data-testid="radio-value">{move || font_size.get()}</span>
    }
}
