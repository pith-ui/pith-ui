use leptos::prelude::*;
use radix_leptos_primitives::direction::{Direction, DirectionProvider};
use radix_leptos_primitives::menubar::*;

#[component]
pub fn MenubarPage() -> impl IntoView {
    let (last_action, set_last_action) = signal(String::new());
    let (bookmarks, set_bookmarks) = signal(true);
    let (urls, set_urls) = signal(false);
    let (zoom, set_zoom) = signal("normal".to_string());
    let (disabled, set_disabled) = signal(false);
    let (rtl, set_rtl) = signal(false);
    let dir = Signal::derive(move || {
        if rtl.get() {
            Direction::Rtl
        } else {
            Direction::Ltr
        }
    });

    view! {
        <DirectionProvider direction=dir>
        <div dir=move || dir.get().to_string()>
        <Menubar class:menubar-root=true dir=dir>
            <MenubarMenu value="file".to_string()>
                <MenubarTrigger class:menubar-trigger=true>"File"</MenubarTrigger>
                <MenubarPortal>
                    <MenubarContent class:menubar-content=true side_offset=5.0 avoid_collisions=Signal::derive(move || !rtl.get())>
                        <MenubarItem
                            class:menubar-item=true
                            on_select=Callback::new({
                                let set_last_action = set_last_action.clone();
                                move |_: web_sys::Event| set_last_action.set("New Tab".into())
                            })
                        >
                            "New Tab"
                        </MenubarItem>
                        <MenubarItem
                            class:menubar-item=true
                            on_select=Callback::new({
                                let set_last_action = set_last_action.clone();
                                move |_: web_sys::Event| set_last_action.set("New Window".into())
                            })
                        >
                            "New Window"
                        </MenubarItem>
                        <MenubarItem
                            class:menubar-item=true
                            on_select=Callback::new({
                                let set_last_action = set_last_action.clone();
                                move |_: web_sys::Event| set_last_action.set("Print".into())
                            })
                        >
                            "Print"
                        </MenubarItem>
                    </MenubarContent>
                </MenubarPortal>
            </MenubarMenu>

            <MenubarMenu value="edit".to_string()>
                <MenubarTrigger class:menubar-trigger=true>"Edit"</MenubarTrigger>
                <MenubarPortal>
                    <MenubarContent class:menubar-content=true side_offset=5.0 avoid_collisions=Signal::derive(move || !rtl.get())>
                        <MenubarItem
                            class:menubar-item=true
                            disabled=disabled
                            on_select=Callback::new({
                                let set_last_action = set_last_action.clone();
                                move |_: web_sys::Event| set_last_action.set("Undo".into())
                            })
                        >
                            "Undo"
                        </MenubarItem>
                        <MenubarItem
                            class:menubar-item=true
                            on_select=Callback::new({
                                let set_last_action = set_last_action.clone();
                                move |_: web_sys::Event| set_last_action.set("Redo".into())
                            })
                        >
                            "Redo"
                        </MenubarItem>
                        <MenubarSeparator class:menubar-separator=true />
                        <MenubarSub>
                            <MenubarSubTrigger class:menubar-item=true class:menubar-sub-trigger=true>
                                "Find \u{2192}"
                            </MenubarSubTrigger>
                            <MenubarPortal>
                                <MenubarSubContent class:menubar-content=true side_offset=2.0>
                                    <MenubarItem
                                        class:menubar-item=true
                                        on_select=Callback::new({
                                            let set_last_action = set_last_action.clone();
                                            move |_: web_sys::Event| set_last_action.set("Search the web\u{2026}".into())
                                        })
                                    >
                                        "Search the web\u{2026}"
                                    </MenubarItem>
                                    <MenubarItem
                                        class:menubar-item=true
                                        on_select=Callback::new({
                                            let set_last_action = set_last_action.clone();
                                            move |_: web_sys::Event| set_last_action.set("Find\u{2026}".into())
                                        })
                                    >
                                        "Find\u{2026}"
                                    </MenubarItem>
                                    <MenubarItem
                                        class:menubar-item=true
                                        on_select=Callback::new({
                                            let set_last_action = set_last_action.clone();
                                            move |_: web_sys::Event| set_last_action.set("Find Next".into())
                                        })
                                    >
                                        "Find Next"
                                    </MenubarItem>
                                </MenubarSubContent>
                            </MenubarPortal>
                        </MenubarSub>
                        <MenubarSeparator class:menubar-separator=true />
                        <MenubarItem
                            class:menubar-item=true
                            on_select=Callback::new({
                                let set_last_action = set_last_action.clone();
                                move |_: web_sys::Event| set_last_action.set("Cut".into())
                            })
                        >
                            "Cut"
                        </MenubarItem>
                    </MenubarContent>
                </MenubarPortal>
            </MenubarMenu>

            <MenubarMenu value="view".to_string()>
                <MenubarTrigger class:menubar-trigger=true>"View"</MenubarTrigger>
                <MenubarPortal>
                    <MenubarContent class:menubar-content=true side_offset=5.0 avoid_collisions=Signal::derive(move || !rtl.get())>
                        <MenubarCheckboxItem
                            class:menubar-item=true
                            checked=Signal::derive(move || CheckedState::from(bookmarks.get()))
                            on_checked_change=Callback::new(move |v: bool| set_bookmarks.set(v))
                        >
                            <MenubarItemIndicator class:menubar-indicator=true>
                                "\u{2713}"
                            </MenubarItemIndicator>
                            "Always Show Bookmarks Bar"
                        </MenubarCheckboxItem>
                        <MenubarCheckboxItem
                            class:menubar-item=true
                            checked=Signal::derive(move || CheckedState::from(urls.get()))
                            on_checked_change=Callback::new(move |v: bool| set_urls.set(v))
                        >
                            <MenubarItemIndicator class:menubar-indicator=true>
                                "\u{2713}"
                            </MenubarItemIndicator>
                            "Always Show Full URLs"
                        </MenubarCheckboxItem>
                        <MenubarSeparator class:menubar-separator=true />
                        <MenubarLabel class:menubar-label=true>"Zoom"</MenubarLabel>
                        <MenubarRadioGroup
                            value=zoom
                            on_value_change=Callback::new(move |v: String| set_zoom.set(v))
                        >
                            <MenubarRadioItem class:menubar-item=true value="compact">
                                <MenubarItemIndicator class:menubar-indicator=true>
                                    "\u{25cf}"
                                </MenubarItemIndicator>
                                "Compact"
                            </MenubarRadioItem>
                            <MenubarRadioItem class:menubar-item=true value="normal">
                                <MenubarItemIndicator class:menubar-indicator=true>
                                    "\u{25cf}"
                                </MenubarItemIndicator>
                                "Normal"
                            </MenubarRadioItem>
                        </MenubarRadioGroup>
                    </MenubarContent>
                </MenubarPortal>
            </MenubarMenu>
        </Menubar>
        </div>
        </DirectionProvider>

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
        <label>
            <input
                type="checkbox"
                prop:checked=move || rtl.get()
                on:change=move |ev| set_rtl.set(event_target_checked(&ev))
            />
            " rtl"
        </label>

        <br />
        <br />

        <span data-testid="last-action">{move || last_action.get()}</span>
        <br />
        <span data-testid="checkbox-bookmarks">{move || if bookmarks.get() { "true" } else { "false" }}</span>
        <br />
        <span data-testid="checkbox-urls">{move || if urls.get() { "true" } else { "false" }}</span>
        <br />
        <span data-testid="radio-size">{move || zoom.get()}</span>

        <br />
        <br />

        <button data-testid="outside-button">"outside"</button>
        <input data-testid="outside-input" placeholder="name" />
    }
}
