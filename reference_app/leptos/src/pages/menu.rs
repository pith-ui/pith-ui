use leptos::prelude::*;
use radix_leptos_menu::*;

#[component]
pub fn MenuPage() -> impl IntoView {
    let (last_action, set_last_action) = signal(String::new());
    let (bold_checked, set_bold_checked) = signal(false);
    let (italic_checked, set_italic_checked) = signal(true);
    let (font_size, set_font_size) = signal("Medium".to_string());
    let (sub_open, set_sub_open) = signal(false);
    let (animated, set_animated) = signal(false);

    let sub_content_class = Signal::derive(move || {
        if animated.get() {
            "menu-content menu-content-animated".to_string()
        } else {
            "menu-content".to_string()
        }
    });

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
            <MenuAnchor attr:class="menu-anchor">"Anchor"</MenuAnchor>
            <MenuPortal>
                <MenuContent attr:class="menu-content">
                    <MenuGroup>
                        <MenuLabel attr:class="menu-label">"Fruits"</MenuLabel>
                        <MenuItem
                            attr:class="menu-item"
                            on_select=Callback::new({
                                let set_last_action = set_last_action.clone();
                                move |_: web_sys::Event| { set_last_action.set("Apple".into()); }
                            })
                        >
                            "Apple"
                        </MenuItem>
                        <MenuItem
                            attr:class="menu-item"
                            on_select=Callback::new({
                                let set_last_action = set_last_action.clone();
                                move |_: web_sys::Event| { set_last_action.set("Banana".into()); }
                            })
                        >
                            "Banana"
                        </MenuItem>
                        <MenuItem
                            attr:class="menu-item"
                            on_select=Callback::new({
                                let set_last_action = set_last_action.clone();
                                move |_: web_sys::Event| { set_last_action.set("Blueberry".into()); }
                            })
                        >
                            "Blueberry"
                        </MenuItem>
                    </MenuGroup>

                    <MenuSeparator attr:class="menu-separator" />

                    <MenuGroup>
                        <MenuLabel attr:class="menu-label">"Vegetables"</MenuLabel>
                        <MenuItem
                            attr:class="menu-item"
                            on_select=Callback::new({
                                let set_last_action = set_last_action.clone();
                                move |_: web_sys::Event| { set_last_action.set("Broccoli".into()); }
                            })
                        >
                            "Broccoli"
                        </MenuItem>
                        <MenuItem
                            attr:class="menu-item"
                            disabled=true
                            on_select=Callback::new({
                                let set_last_action = set_last_action.clone();
                                move |_: web_sys::Event| { set_last_action.set("Carrot".into()); }
                            })
                        >
                            "Carrot"
                        </MenuItem>
                        <MenuItem
                            attr:class="menu-item"
                            on_select=Callback::new({
                                let set_last_action = set_last_action.clone();
                                move |_: web_sys::Event| { set_last_action.set("Courgette".into()); }
                            })
                        >
                            "Courgette"
                        </MenuItem>
                    </MenuGroup>

                    <MenuSeparator attr:class="menu-separator" />

                    <MenuItem
                        attr:class="menu-item"
                        on_select=Callback::new({
                            let set_last_action = set_last_action.clone();
                            move |_: web_sys::Event| { set_last_action.set("Undo".into()); }
                        })
                    >
                        "Undo"
                    </MenuItem>
                    <MenuItem
                        attr:class="menu-item"
                        on_select=Callback::new({
                            let set_last_action = set_last_action.clone();
                            move |_: web_sys::Event| { set_last_action.set("Redo".into()); }
                        })
                    >
                        "Redo"
                    </MenuItem>

                    <MenuSeparator attr:class="menu-separator" />

                    <MenuCheckboxItem
                        attr:class="menu-item"
                        checked=Signal::derive(move || CheckedState::from(bold_checked.get()))
                        on_checked_change=Callback::new(move |v: bool| set_bold_checked.set(v))
                    >
                        "Bold"
                        <MenuItemIndicator>"\u{2713}"</MenuItemIndicator>
                    </MenuCheckboxItem>
                    <MenuCheckboxItem
                        attr:class="menu-item"
                        checked=Signal::derive(move || CheckedState::from(italic_checked.get()))
                        on_checked_change=Callback::new(move |v: bool| set_italic_checked.set(v))
                    >
                        "Italic"
                        <MenuItemIndicator>"\u{2713}"</MenuItemIndicator>
                    </MenuCheckboxItem>

                    <MenuSeparator attr:class="menu-separator" />

                    <MenuRadioGroup value=font_size on_value_change=Callback::new(move |v: String| set_font_size.set(v))>
                        <MenuRadioItem attr:class="menu-item" value="Small">
                            "Small"
                            <MenuItemIndicator>"\u{2713}"</MenuItemIndicator>
                        </MenuRadioItem>
                        <MenuRadioItem attr:class="menu-item" value="Medium">
                            "Medium"
                            <MenuItemIndicator>"\u{2713}"</MenuItemIndicator>
                        </MenuRadioItem>
                        <MenuRadioItem attr:class="menu-item" value="Large">
                            "Large"
                            <MenuItemIndicator>"\u{2713}"</MenuItemIndicator>
                        </MenuRadioItem>
                    </MenuRadioGroup>

                    <MenuSeparator attr:class="menu-separator" />

                    <MenuGroup>
                        <MenuLabel attr:class="menu-label">"Suits (textValue)"</MenuLabel>
                        <MenuItem attr:class="menu-item" text_value="Hearts">
                            <span role="img" aria-label="Hearts">"\u{2665}\u{fe0f}"</span> "Hearts"
                        </MenuItem>
                        <MenuItem attr:class="menu-item" text_value="Spades">
                            <span role="img" aria-label="Spades">"\u{2660}\u{fe0f}"</span> "Spades"
                        </MenuItem>
                        <MenuItem attr:class="menu-item" text_value="Diamonds">
                            <span role="img" aria-label="Diamonds">"\u{2666}\u{fe0f}"</span> "Diamonds"
                        </MenuItem>
                        <MenuItem attr:class="menu-item" text_value="Clubs">
                            <span role="img" aria-label="Clubs">"\u{2663}\u{fe0f}"</span> "Clubs"
                        </MenuItem>
                    </MenuGroup>

                    <MenuSeparator attr:class="menu-separator" />

                    <MenuSub open=sub_open on_open_change=Callback::new(move |v: bool| set_sub_open.set(v))>
                        <MenuSubTrigger attr:class="menu-item">"More Options..."</MenuSubTrigger>
                        <MenuPortal>
                            <MenuSubContent class=sub_content_class>
                                <MenuItem
                                    attr:class="menu-item"
                                    on_select=Callback::new({
                                        let set_last_action = set_last_action.clone();
                                        move |_: web_sys::Event| { set_last_action.set("Option A".into()); }
                                    })
                                >
                                    "Option A"
                                </MenuItem>
                                <MenuItem
                                    attr:class="menu-item"
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
