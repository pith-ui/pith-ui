use leptos::prelude::*;
use cardo_ui::direction::{Direction, DirectionProvider};
use cardo_ui::menubar::*;

stylance::import_crate_style!(
    #[allow(dead_code)]
    classes,
    "src/primitives/menubar.stories.module.css"
);

struct FoodGroup {
    label: Option<&'static str>,
    foods: &'static [Food],
}

struct Food {
    #[allow(dead_code)]
    value: &'static str,
    label: &'static str,
    disabled: bool,
}

const FOOD_GROUPS: &[FoodGroup] = &[
    FoodGroup {
        label: Some("Fruits"),
        foods: &[
            Food {
                value: "apple",
                label: "Apple",
                disabled: false,
            },
            Food {
                value: "banana",
                label: "Banana",
                disabled: false,
            },
            Food {
                value: "blueberry",
                label: "Blueberry",
                disabled: false,
            },
            Food {
                value: "grapes",
                label: "Grapes",
                disabled: false,
            },
            Food {
                value: "pineapple",
                label: "Pineapple",
                disabled: false,
            },
        ],
    },
    FoodGroup {
        label: Some("Vegetables"),
        foods: &[
            Food {
                value: "aubergine",
                label: "Aubergine",
                disabled: false,
            },
            Food {
                value: "broccoli",
                label: "Broccoli",
                disabled: false,
            },
            Food {
                value: "carrot",
                label: "Carrot",
                disabled: true,
            },
            Food {
                value: "courgette",
                label: "Courgette",
                disabled: false,
            },
            Food {
                value: "leek",
                label: "Leek",
                disabled: false,
            },
        ],
    },
    FoodGroup {
        label: Some("Meat"),
        foods: &[
            Food {
                value: "beef",
                label: "Beef",
                disabled: false,
            },
            Food {
                value: "chicken",
                label: "Chicken",
                disabled: false,
            },
            Food {
                value: "lamb",
                label: "Lamb",
                disabled: false,
            },
            Food {
                value: "pork",
                label: "Pork",
                disabled: false,
            },
        ],
    },
];

fn tick_icon() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 32 32"
            width="12"
            height="12"
            fill="none"
            stroke="currentcolor"
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="3"
        >
            <path d="M2 20 L12 28 30 4" />
        </svg>
    }
}

#[component]
pub fn Styled() -> impl IntoView {
    let (loop_enabled, set_loop) = signal(false);
    let (rtl, set_rtl) = signal(false);
    let dir = Signal::derive(move || {
        if rtl.get() {
            Direction::Rtl
        } else {
            Direction::Ltr
        }
    });

    let check_options = [
        "Always Show Bookmarks Bar",
        "Always Show Toolbar in Fullscreen",
        "Always Show Full URLs",
    ];
    let (checked_selection, set_checked_selection) = signal(vec![check_options[1].to_string()]);

    let radio_options = ["Andy", "Benoît", "Colm", "Jenna", "Pedro"];
    let (radio_selection, set_radio_selection) = signal(radio_options[1].to_string());

    view! {
        <div style="display: flex; align-items: center; justify-content: center; flex-direction: column; padding-top: 50px;">
            <div style="display: flex; gap: 25px; margin-bottom: 20px;">
                <label>
                    <input
                        type="checkbox"
                        prop:checked=move || rtl.get()
                        on:change=move |ev| {
                            set_rtl.set(event_target_checked(&ev));
                        }
                    />
                    "Right-to-left"
                </label>
                <label>
                    <input
                        type="checkbox"
                        prop:checked=move || loop_enabled.get()
                        on:change=move |ev| {
                            set_loop.set(event_target_checked(&ev));
                        }
                    />
                    "Loop"
                </label>
            </div>

            <DirectionProvider direction=dir>
                <div dir=move || dir.get().to_string()>
                <Menubar
                    attr:class=classes::root
                    r#loop=loop_enabled
                    dir=dir
                >
                    <MenubarMenu>
                        <MenubarTrigger attr:class=classes::trigger>"File"</MenubarTrigger>
                        <MenubarPortal>
                            <MenubarContent attr:class=classes::content side_offset=2.0>
                                <MenubarItem attr:class=classes::item>"New Tab"</MenubarItem>
                                <MenubarItem attr:class=classes::item>"New Window"</MenubarItem>
                                <MenubarItem attr:class=classes::item>"New Incognito Window"</MenubarItem>
                                <MenubarSeparator attr:class=classes::separator />
                                <MenubarSub>
                                    <MenubarSubTrigger attr:class=format!("{} {}", classes::item, classes::subTrigger)>
                                        "Share " <span>"→"</span>
                                    </MenubarSubTrigger>
                                    <MenubarPortal>
                                        <MenubarSubContent attr:class=classes::content>
                                            <MenubarItem attr:class=classes::item>"Email Link"</MenubarItem>
                                            <MenubarItem attr:class=classes::item>"Messages"</MenubarItem>
                                            <MenubarItem attr:class=classes::item>"Airdrop"</MenubarItem>
                                        </MenubarSubContent>
                                    </MenubarPortal>
                                </MenubarSub>
                                <MenubarSeparator attr:class=classes::separator />
                                <MenubarItem attr:class=classes::item>"Print…"</MenubarItem>
                            </MenubarContent>
                        </MenubarPortal>
                    </MenubarMenu>

                    <MenubarMenu>
                        <MenubarTrigger attr:class=classes::trigger>"Edit"</MenubarTrigger>
                        <MenubarPortal>
                            <MenubarContent attr:class=classes::content side_offset=2.0>
                                <MenubarItem attr:class=classes::item>"Undo"</MenubarItem>
                                <MenubarItem attr:class=classes::item>"Redo"</MenubarItem>
                                <MenubarSeparator attr:class=classes::separator />
                                <MenubarSub>
                                    <MenubarSubTrigger attr:class=format!("{} {}", classes::item, classes::subTrigger)>
                                        "Find " <span>"→"</span>
                                    </MenubarSubTrigger>
                                    <MenubarPortal>
                                        <MenubarSubContent attr:class=classes::content>
                                            <MenubarItem attr:class=classes::item>"Search the web…"</MenubarItem>
                                            <MenubarSeparator attr:class=classes::separator />
                                            <MenubarItem attr:class=classes::item>"Find…"</MenubarItem>
                                            <MenubarItem attr:class=classes::item>"Find Next"</MenubarItem>
                                            <MenubarItem attr:class=classes::item>"Find Previous"</MenubarItem>
                                            <MenubarSub>
                                                <MenubarSubTrigger attr:class=format!("{} {}", classes::item, classes::subTrigger)>
                                                    "Advanced " <span>"→"</span>
                                                </MenubarSubTrigger>
                                                <MenubarPortal>
                                                    <MenubarSubContent attr:class=classes::content>
                                                        <MenubarItem attr:class=classes::item>"Regex"</MenubarItem>
                                                        <MenubarItem attr:class=classes::item>"Replace"</MenubarItem>
                                                    </MenubarSubContent>
                                                </MenubarPortal>
                                            </MenubarSub>
                                        </MenubarSubContent>
                                    </MenubarPortal>
                                </MenubarSub>
                                <MenubarSeparator attr:class=classes::separator />
                                <MenubarItem attr:class=classes::item>"Cut"</MenubarItem>
                                <MenubarItem attr:class=classes::item>"Copy"</MenubarItem>
                                <MenubarItem attr:class=classes::item>"Paste"</MenubarItem>
                            </MenubarContent>
                        </MenubarPortal>
                    </MenubarMenu>

                    <MenubarMenu>
                        <MenubarTrigger attr:class=classes::trigger>"View"</MenubarTrigger>
                        <MenubarPortal>
                            <MenubarContent attr:class=classes::content side_offset=2.0>
                                {check_options.iter().map(|option| {
                                    let option = option.to_string();
                                    let option_for_check = option.clone();
                                    let option_for_change = option.clone();
                                    view! {
                                        <MenubarCheckboxItem
                                            attr:class=classes::item
                                            checked=Signal::derive({
                                                let option = option_for_check.clone();
                                                move || {
                                                    if checked_selection.get().contains(&option) {
                                                        CheckedState::True
                                                    } else {
                                                        CheckedState::False
                                                    }
                                                }
                                            })
                                            on_checked_change=Callback::new({
                                                let option = option_for_change.clone();
                                                move |_: bool| {
                                                    set_checked_selection.update(|current| {
                                                        if current.contains(&option) {
                                                            current.retain(|el| el != &option);
                                                        } else {
                                                            current.push(option.clone());
                                                        }
                                                    });
                                                }
                                            })
                                        >
                                            {option.clone()}
                                            <MenubarItemIndicator attr:style="margin-left: 10px;">
                                                {tick_icon()}
                                            </MenubarItemIndicator>
                                        </MenubarCheckboxItem>
                                    }
                                }).collect_view()}
                                <MenubarSeparator attr:class=classes::separator />
                                <MenubarItem attr:class=classes::item>"Reload"</MenubarItem>
                                <MenubarItem attr:class=classes::item>"Force Reload"</MenubarItem>
                                <MenubarSeparator attr:class=classes::separator />
                                <MenubarItem attr:class=classes::item>"Toggle Fullscreen"</MenubarItem>
                                <MenubarSeparator attr:class=classes::separator />
                                <MenubarItem attr:class=classes::item>"Hide Sidebar"</MenubarItem>
                            </MenubarContent>
                        </MenubarPortal>
                    </MenubarMenu>

                    <MenubarMenu>
                        <MenubarTrigger attr:class=classes::trigger>"Profiles"</MenubarTrigger>
                        <MenubarPortal>
                            <MenubarContent attr:class=classes::content side_offset=2.0>
                                <MenubarRadioGroup
                                    value=Signal::derive(move || radio_selection.get())
                                    on_value_change=Callback::new(move |value: String| {
                                        set_radio_selection.set(value);
                                    })
                                >
                                    {radio_options.iter().map(|option| {
                                        let option = option.to_string();
                                        view! {
                                            <MenubarRadioItem
                                                attr:class=classes::item
                                                value=option.clone()
                                            >
                                                {option.clone()}
                                                <MenubarItemIndicator attr:style="margin-left: 10px;">
                                                    {tick_icon()}
                                                </MenubarItemIndicator>
                                            </MenubarRadioItem>
                                        }
                                    }).collect_view()}
                                </MenubarRadioGroup>
                            </MenubarContent>
                        </MenubarPortal>
                    </MenubarMenu>

                    <MenubarMenu>
                        <MenubarTrigger attr:class=classes::trigger>"History"</MenubarTrigger>
                        <MenubarPortal>
                            <MenubarContent attr:class=classes::content side_offset=2.0>
                                <MenubarLabel attr:class=classes::label>"Work"</MenubarLabel>
                                <MenubarItem attr:class=classes::item>"Radix"</MenubarItem>
                                <MenubarItem attr:class=classes::item>"Github"</MenubarItem>
                                <MenubarItem attr:class=classes::item>"WorkOS"</MenubarItem>
                                <MenubarLabel attr:class=classes::label>"Community"</MenubarLabel>
                                <MenubarItem attr:class=classes::item>"Twitter"</MenubarItem>
                                <MenubarItem attr:class=classes::item>"Discord"</MenubarItem>
                                <MenubarItem attr:class=classes::item>"Slack"</MenubarItem>
                            </MenubarContent>
                        </MenubarPortal>
                    </MenubarMenu>
                </Menubar>
                </div>
            </DirectionProvider>
        </div>
    }
}

#[component]
pub fn Cypress() -> impl IntoView {
    let (loop_enabled, set_loop) = signal(false);
    let (rtl, set_rtl) = signal(false);
    let (portalled, set_portalled) = signal(false);
    let dir = Signal::derive(move || {
        if rtl.get() {
            Direction::Rtl
        } else {
            Direction::Ltr
        }
    });

    let sub_trigger_class = StoredValue::new(format!("{} {}", classes::item, classes::subTrigger));

    view! {
        <div style="display: flex; align-items: center; justify-content: center; flex-direction: column; padding-top: 50px;">
            <div style="display: flex; gap: 25px; margin-bottom: 20px;">
                <label>
                    <input
                        type="checkbox"
                        prop:checked=move || rtl.get()
                        on:change=move |ev| set_rtl.set(event_target_checked(&ev))
                    />
                    "Right-to-left"
                </label>
                <label>
                    <input
                        type="checkbox"
                        prop:checked=move || loop_enabled.get()
                        on:change=move |ev| set_loop.set(event_target_checked(&ev))
                    />
                    "Loop"
                </label>
                <label>
                    <input
                        type="checkbox"
                        prop:checked=move || portalled.get()
                        on:change=move |ev| set_portalled.set(event_target_checked(&ev))
                    />
                    "Portalled"
                </label>
            </div>
            <DirectionProvider direction=dir>
                <div dir=move || dir.get().to_string()>
                <Menubar attr:class=classes::root r#loop=loop_enabled dir=dir>
                    <MenubarMenu>
                        <MenubarTrigger attr:class=classes::trigger>"File"</MenubarTrigger>
                        <CypressPortal portalled=portalled>
                            <MenubarContent attr:class=classes::content side_offset=2.0>
                                <MenubarItem attr:class=classes::item>"New Tab"</MenubarItem>
                                <MenubarItem attr:class=classes::item>"New Window"</MenubarItem>
                                <MenubarItem attr:class=classes::item>"New Incognito Window"</MenubarItem>
                                <MenubarSeparator attr:class=classes::separator />
                                <MenubarSub>
                                    <MenubarSubTrigger attr:class=sub_trigger_class.get_value()>
                                        "Share " <span>"→"</span>
                                    </MenubarSubTrigger>
                                    <MenubarSubContent attr:class=classes::content>
                                        <MenubarItem attr:class=classes::item>"Email Link"</MenubarItem>
                                        <MenubarItem attr:class=classes::item>"Messages"</MenubarItem>
                                        <MenubarItem attr:class=classes::item>"Airdrop"</MenubarItem>
                                    </MenubarSubContent>
                                </MenubarSub>
                                <MenubarSeparator attr:class=classes::separator />
                                <MenubarItem attr:class=classes::item>"Print…"</MenubarItem>
                            </MenubarContent>
                        </CypressPortal>
                    </MenubarMenu>

                    <MenubarMenu>
                        <MenubarTrigger attr:class=classes::trigger>"Edit"</MenubarTrigger>
                        <CypressPortal portalled=portalled>
                            <MenubarContent attr:class=classes::content side_offset=2.0>
                                <MenubarItem attr:class=classes::item disabled=true>"Undo"</MenubarItem>
                                <MenubarItem attr:class=classes::item>"Redo"</MenubarItem>
                                <MenubarSeparator attr:class=classes::separator />
                                <MenubarSub>
                                    <MenubarSubTrigger attr:class=sub_trigger_class.get_value()>
                                        "Find " <span>"→"</span>
                                    </MenubarSubTrigger>
                                    <CypressPortal portalled=portalled>
                                        <MenubarSubContent attr:class=classes::content>
                                            <MenubarItem attr:class=classes::item>"Search the web…"</MenubarItem>
                                            <MenubarSeparator attr:class=classes::separator />
                                            <MenubarItem attr:class=classes::item>"Find…"</MenubarItem>
                                            <MenubarItem attr:class=classes::item>"Find Next"</MenubarItem>
                                            <MenubarItem attr:class=classes::item>"Find Previous"</MenubarItem>
                                            <MenubarSub>
                                                <MenubarSubTrigger attr:class=sub_trigger_class.get_value()>
                                                    "Advanced " <span>"→"</span>
                                                </MenubarSubTrigger>
                                                <CypressPortal portalled=portalled>
                                                    <MenubarSubContent attr:class=classes::content>
                                                        <MenubarItem attr:class=classes::item>"Regex"</MenubarItem>
                                                        <MenubarItem attr:class=classes::item>"Replace"</MenubarItem>
                                                    </MenubarSubContent>
                                                </CypressPortal>
                                            </MenubarSub>
                                        </MenubarSubContent>
                                    </CypressPortal>
                                </MenubarSub>

                                <MenubarSub>
                                    <MenubarSubTrigger attr:class=sub_trigger_class.get_value() disabled=true>
                                        "Speech " <span>"→"</span>
                                    </MenubarSubTrigger>
                                    <CypressPortal portalled=portalled>
                                        <MenubarSubContent attr:class=classes::content>
                                            <MenubarItem attr:class=classes::item>"Start Speaking"</MenubarItem>
                                            <MenubarItem attr:class=classes::item>"Stop Speaking"</MenubarItem>
                                        </MenubarSubContent>
                                    </CypressPortal>
                                </MenubarSub>

                                <MenubarSub>
                                    <MenubarSubTrigger attr:class=sub_trigger_class.get_value()>
                                        "Substitutions " <span>"→"</span>
                                    </MenubarSubTrigger>
                                    <CypressPortal portalled=portalled>
                                        <MenubarSubContent attr:class=classes::content>
                                            <MenubarItem attr:class=classes::item>"Smart Quotes"</MenubarItem>
                                            <MenubarItem attr:class=classes::item>"Smart Dashes"</MenubarItem>
                                        </MenubarSubContent>
                                    </CypressPortal>
                                </MenubarSub>
                                <MenubarSeparator attr:class=classes::separator />
                                <MenubarItem attr:class=classes::item>"Cut"</MenubarItem>
                                <MenubarItem attr:class=classes::item>"Copy"</MenubarItem>
                                <MenubarItem attr:class=classes::item>"Paste"</MenubarItem>
                            </MenubarContent>
                        </CypressPortal>
                    </MenubarMenu>

                    <MenubarMenu>
                        <MenubarTrigger attr:class=classes::trigger>"History"</MenubarTrigger>
                        <CypressPortal portalled=portalled>
                            <MenubarContent attr:class=classes::content side_offset=2.0>
                                <MenubarItem attr:class=classes::item>"Radix"</MenubarItem>
                                <MenubarItem attr:class=classes::item>"Github"</MenubarItem>
                                <MenubarItem attr:class=classes::item>"WorkOS"</MenubarItem>
                            </MenubarContent>
                        </CypressPortal>
                    </MenubarMenu>
                </Menubar>
                </div>
            </DirectionProvider>
        </div>
    }
}

/// Helper component to conditionally wrap children in a MenubarPortal.
#[component]
fn CypressPortal(portalled: ReadSignal<bool>, children: ChildrenFn) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <Show
            when=move || portalled.get()
            fallback=move || children.with_value(|children| children())
        >
            <MenubarPortal>
                {children.with_value(|children| children())}
            </MenubarPortal>
        </Show>
    }
}

#[component]
pub fn Chromatic() -> impl IntoView {
    let checkbox_items = [
        ("Bold", signal(false), false),
        ("Italic", signal(true), false),
        ("Underline", signal(false), false),
        ("Strikethrough", signal(false), true),
    ];
    let files = ["README.md", "index.js", "page.css"];
    let (file, set_file) = signal(files[1].to_string());

    view! {
        <div style="padding: 200px; padding-top: 50px; padding-bottom: 800px;">
            <h1>"Uncontrolled"</h1>
            <h2>"Closed"</h2>
            <Menubar attr:class=classes::root>
                <MenubarMenu>
                    <MenubarTrigger attr:class=classes::trigger>"File"</MenubarTrigger>
                    <MenubarPortal>
                        <MenubarContent attr:class=classes::content side_offset=2.0>
                            <MenubarItem attr:class=classes::item>"New Tab"</MenubarItem>
                            <MenubarItem attr:class=classes::item>"New Window"</MenubarItem>
                            <MenubarItem attr:class=classes::item>"New Incognito Window"</MenubarItem>
                            <MenubarSeparator attr:class=classes::separator />
                            <MenubarItem attr:class=classes::item>"Print…"</MenubarItem>
                        </MenubarContent>
                    </MenubarPortal>
                </MenubarMenu>
                <MenubarMenu>
                    <MenubarTrigger attr:class=classes::trigger>"Edit"</MenubarTrigger>
                    <MenubarPortal>
                        <MenubarContent attr:class=classes::content side_offset=2.0>
                            <MenubarItem attr:class=classes::item disabled=true>"Undo"</MenubarItem>
                            <MenubarItem attr:class=classes::item>"Redo"</MenubarItem>
                            <MenubarSeparator attr:class=classes::separator />
                            <MenubarItem attr:class=classes::item>"Cut"</MenubarItem>
                            <MenubarItem attr:class=classes::item>"Copy"</MenubarItem>
                            <MenubarItem attr:class=classes::item>"Paste"</MenubarItem>
                        </MenubarContent>
                    </MenubarPortal>
                </MenubarMenu>
            </Menubar>

            <h2>"Open"</h2>
            <Menubar default_value="file".to_string() attr:class=classes::root>
                <MenubarMenu value="file".to_string()>
                    <MenubarTrigger attr:class=classes::trigger>"File"</MenubarTrigger>
                    <MenubarPortal>
                        <MenubarContent
                            attr:class=classes::content
                            on_focus_outside=Callback::new(|event: web_sys::CustomEvent| event.prevent_default())
                            side_offset=2.0
                        >
                            <MenubarItem attr:class=classes::item>"New Tab"</MenubarItem>
                            <MenubarItem attr:class=classes::item>"New Window"</MenubarItem>
                            <MenubarItem attr:class=classes::item>"New Incognito Window"</MenubarItem>
                            <MenubarSeparator attr:class=classes::separator />
                            <MenubarItem attr:class=classes::item>"Print…"</MenubarItem>
                        </MenubarContent>
                    </MenubarPortal>
                </MenubarMenu>
                <MenubarMenu value="edit".to_string()>
                    <MenubarTrigger attr:class=classes::trigger>"Edit"</MenubarTrigger>
                    <MenubarPortal>
                        <MenubarContent attr:class=classes::content side_offset=2.0>
                            <MenubarItem attr:class=classes::item disabled=true>"Undo"</MenubarItem>
                            <MenubarItem attr:class=classes::item>"Redo"</MenubarItem>
                            <MenubarSeparator attr:class=classes::separator />
                            <MenubarItem attr:class=classes::item>"Cut"</MenubarItem>
                            <MenubarItem attr:class=classes::item>"Copy"</MenubarItem>
                            <MenubarItem attr:class=classes::item>"Paste"</MenubarItem>
                        </MenubarContent>
                    </MenubarPortal>
                </MenubarMenu>
            </Menubar>

            <h1 style="margin-top: 180px;">"Controlled"</h1>
            <h2>"Closed"</h2>
            <Menubar value="".to_string() attr:class=classes::root>
                <MenubarMenu value="file".to_string()>
                    <MenubarTrigger attr:class=classes::trigger>"File"</MenubarTrigger>
                    <MenubarPortal>
                        <MenubarContent attr:class=classes::content side_offset=2.0>
                            <MenubarItem attr:class=classes::item>"New Tab"</MenubarItem>
                            <MenubarItem attr:class=classes::item>"New Window"</MenubarItem>
                            <MenubarItem attr:class=classes::item>"New Incognito Window"</MenubarItem>
                            <MenubarSeparator attr:class=classes::separator />
                            <MenubarItem attr:class=classes::item>"Print…"</MenubarItem>
                        </MenubarContent>
                    </MenubarPortal>
                </MenubarMenu>
                <MenubarMenu value="edit".to_string()>
                    <MenubarTrigger attr:class=classes::trigger>"Edit"</MenubarTrigger>
                    <MenubarPortal>
                        <MenubarContent attr:class=classes::content side_offset=2.0>
                            <MenubarItem attr:class=classes::item disabled=true>"Undo"</MenubarItem>
                            <MenubarItem attr:class=classes::item>"Redo"</MenubarItem>
                            <MenubarSeparator attr:class=classes::separator />
                            <MenubarItem attr:class=classes::item>"Cut"</MenubarItem>
                            <MenubarItem attr:class=classes::item>"Copy"</MenubarItem>
                            <MenubarItem attr:class=classes::item>"Paste"</MenubarItem>
                        </MenubarContent>
                    </MenubarPortal>
                </MenubarMenu>
            </Menubar>

            <h2>"Open"</h2>
            <Menubar value="file".to_string() attr:class=classes::root>
                <MenubarMenu value="file".to_string()>
                    <MenubarTrigger attr:class=classes::trigger>"File"</MenubarTrigger>
                    <MenubarPortal>
                        <MenubarContent attr:class=classes::content side_offset=2.0>
                            <MenubarItem attr:class=classes::item>"New Tab"</MenubarItem>
                            <MenubarItem attr:class=classes::item>"New Window"</MenubarItem>
                            <MenubarItem attr:class=classes::item>"New Incognito Window"</MenubarItem>
                            <MenubarSeparator attr:class=classes::separator />
                            <MenubarItem attr:class=classes::item>"Print…"</MenubarItem>
                        </MenubarContent>
                    </MenubarPortal>
                </MenubarMenu>
                <MenubarMenu value="edit".to_string()>
                    <MenubarTrigger attr:class=classes::trigger>"Edit"</MenubarTrigger>
                    <MenubarPortal>
                        <MenubarContent attr:class=classes::content side_offset=2.0>
                            <MenubarItem attr:class=classes::item disabled=true>"Undo"</MenubarItem>
                            <MenubarItem attr:class=classes::item>"Redo"</MenubarItem>
                            <MenubarSeparator attr:class=classes::separator />
                            <MenubarItem attr:class=classes::item>"Cut"</MenubarItem>
                            <MenubarItem attr:class=classes::item>"Copy"</MenubarItem>
                            <MenubarItem attr:class=classes::item>"Paste"</MenubarItem>
                        </MenubarContent>
                    </MenubarPortal>
                </MenubarMenu>
            </Menubar>

            <h1 style="margin-top: 200px;">"Submenus"</h1>
            <Menubar value="edit".to_string() attr:class=classes::root>
                <MenubarMenu value="file".to_string()>
                    <MenubarTrigger attr:class=classes::trigger>"File"</MenubarTrigger>
                    <MenubarPortal>
                        <MenubarContent attr:class=classes::content side_offset=2.0>
                            <MenubarItem attr:class=classes::item>"New Tab"</MenubarItem>
                            <MenubarItem attr:class=classes::item>"New Window"</MenubarItem>
                            <MenubarItem attr:class=classes::item>"New Incognito Window"</MenubarItem>
                            <MenubarSeparator attr:class=classes::separator />
                            <MenubarItem attr:class=classes::item>"Print…"</MenubarItem>
                        </MenubarContent>
                    </MenubarPortal>
                </MenubarMenu>
                <MenubarMenu value="edit".to_string()>
                    <MenubarTrigger attr:class=classes::trigger>"Edit"</MenubarTrigger>
                    <MenubarPortal>
                        <MenubarContent attr:class=classes::content side_offset=2.0>
                            <MenubarItem attr:class=classes::item disabled=true>"Undo"</MenubarItem>
                            <MenubarItem attr:class=classes::item>"Redo"</MenubarItem>
                            <MenubarSeparator attr:class=classes::separator />
                            <MenubarSub open=true>
                                <MenubarSubTrigger attr:class=format!("{} {}", classes::item, classes::subTrigger)>
                                    "Find " <span>"→"</span>
                                </MenubarSubTrigger>
                                <MenubarPortal>
                                    <MenubarSubContent attr:class=classes::content side_offset=10.0>
                                        <MenubarItem attr:class=classes::item>"Search the web…"</MenubarItem>
                                        <MenubarSeparator attr:class=classes::separator />
                                        <MenubarItem attr:class=classes::item>"Find…"</MenubarItem>
                                        <MenubarItem attr:class=classes::item>"Find Next"</MenubarItem>
                                        <MenubarItem attr:class=classes::item>"Find Previous"</MenubarItem>
                                        <MenubarSub open=true>
                                            <MenubarSubTrigger attr:class=format!("{} {}", classes::item, classes::subTrigger)>
                                                "Advanced " <span>"→"</span>
                                            </MenubarSubTrigger>
                                            <MenubarPortal>
                                                <MenubarSubContent attr:class=classes::content side_offset=10.0>
                                                    <MenubarItem attr:class=classes::item>"Regex"</MenubarItem>
                                                    <MenubarItem attr:class=classes::item>"Replace"</MenubarItem>
                                                    <MenubarArrow />
                                                </MenubarSubContent>
                                            </MenubarPortal>
                                        </MenubarSub>
                                        <MenubarArrow />
                                    </MenubarSubContent>
                                </MenubarPortal>
                            </MenubarSub>
                            <MenubarSeparator attr:class=classes::separator />
                            <MenubarItem attr:class=classes::item>"Cut"</MenubarItem>
                            <MenubarItem attr:class=classes::item>"Copy"</MenubarItem>
                            <MenubarItem attr:class=classes::item>"Paste"</MenubarItem>
                        </MenubarContent>
                    </MenubarPortal>
                </MenubarMenu>
            </Menubar>

            <h2 style="margin-top: 250px;">"RTL"</h2>
            <DirectionProvider direction=Direction::Rtl>
                <div dir="rtl">
                <Menubar value="edit".to_string() attr:class=classes::root dir=Direction::Rtl>
                    <MenubarMenu value="file".to_string()>
                        <MenubarTrigger attr:class=classes::trigger>"File"</MenubarTrigger>
                        <MenubarPortal>
                            <MenubarContent attr:class=classes::content avoid_collisions=false side_offset=2.0>
                                <MenubarItem attr:class=classes::item>"New Tab"</MenubarItem>
                                <MenubarItem attr:class=classes::item>"New Window"</MenubarItem>
                                <MenubarItem attr:class=classes::item>"New Incognito Window"</MenubarItem>
                                <MenubarSeparator attr:class=classes::separator />
                                <MenubarItem attr:class=classes::item>"Print…"</MenubarItem>
                            </MenubarContent>
                        </MenubarPortal>
                    </MenubarMenu>
                    <MenubarMenu value="edit".to_string()>
                        <MenubarTrigger attr:class=classes::trigger>"Edit"</MenubarTrigger>
                        <MenubarPortal>
                            <MenubarContent attr:class=classes::content avoid_collisions=false side_offset=2.0>
                                <MenubarItem attr:class=classes::item disabled=true>"Undo"</MenubarItem>
                                <MenubarItem attr:class=classes::item>"Redo"</MenubarItem>
                                <MenubarSeparator attr:class=classes::separator />
                                <MenubarSub open=true>
                                    <MenubarSubTrigger attr:class=format!("{} {}", classes::item, classes::subTrigger)>
                                        "Find " <span>"→"</span>
                                    </MenubarSubTrigger>
                                    <MenubarPortal>
                                        <MenubarSubContent attr:class=classes::content avoid_collisions=false side_offset=10.0 align_offset=-6.0>
                                            <MenubarItem attr:class=classes::item>"Search the web…"</MenubarItem>
                                            <MenubarSeparator attr:class=classes::separator />
                                            <MenubarItem attr:class=classes::item>"Find…"</MenubarItem>
                                            <MenubarItem attr:class=classes::item>"Find Next"</MenubarItem>
                                            <MenubarItem attr:class=classes::item>"Find Previous"</MenubarItem>
                                            <MenubarSub open=true>
                                                <MenubarSubTrigger attr:class=format!("{} {}", classes::item, classes::subTrigger)>
                                                    "Advanced " <span>"→"</span>
                                                </MenubarSubTrigger>
                                                <MenubarPortal>
                                                    <MenubarSubContent attr:class=classes::content avoid_collisions=false side_offset=10.0 align_offset=-6.0>
                                                        <MenubarItem attr:class=classes::item>"Regex"</MenubarItem>
                                                        <MenubarItem attr:class=classes::item>"Replace"</MenubarItem>
                                                        <MenubarArrow />
                                                    </MenubarSubContent>
                                                </MenubarPortal>
                                            </MenubarSub>
                                            <MenubarArrow />
                                        </MenubarSubContent>
                                    </MenubarPortal>
                                </MenubarSub>
                                <MenubarSeparator attr:class=classes::separator />
                                <MenubarItem attr:class=classes::item>"Cut"</MenubarItem>
                                <MenubarItem attr:class=classes::item>"Copy"</MenubarItem>
                                <MenubarItem attr:class=classes::item>"Paste"</MenubarItem>
                            </MenubarContent>
                        </MenubarPortal>
                    </MenubarMenu>
                </Menubar>
                </div>
            </DirectionProvider>

            <h2 style="margin-top: 250px;">"With labels"</h2>
            <Menubar value="food".to_string() attr:class=classes::root>
                <MenubarMenu value="food".to_string()>
                    <MenubarTrigger attr:class=classes::trigger>"Food"</MenubarTrigger>
                    <MenubarPortal>
                        <MenubarContent attr:class=classes::content side_offset=2.0>
                            {FOOD_GROUPS.iter().enumerate().map(|(index, food_group)| {
                                view! {
                                    <MenubarGroup>
                                        {food_group.label.map(|label| view! {
                                            <MenubarLabel attr:class=classes::label>{label}</MenubarLabel>
                                        })}
                                        {food_group.foods.iter().map(|food| {
                                            view! {
                                                <MenubarItem
                                                    attr:class=classes::item
                                                    disabled=food.disabled
                                                >
                                                    {food.label}
                                                </MenubarItem>
                                            }
                                        }).collect_view()}
                                        {(index < FOOD_GROUPS.len() - 1).then(|| view! {
                                            <MenubarSeparator attr:class=classes::separator />
                                        })}
                                    </MenubarGroup>
                                }
                            }).collect_view()}
                        </MenubarContent>
                    </MenubarPortal>
                </MenubarMenu>
                <MenubarMenu value="edit".to_string()>
                    <MenubarTrigger attr:class=classes::trigger>"Edit"</MenubarTrigger>
                    <MenubarPortal>
                        <MenubarContent attr:class=classes::content side_offset=2.0>
                            <MenubarItem attr:class=classes::item disabled=true>"Undo"</MenubarItem>
                            <MenubarItem attr:class=classes::item>"Redo"</MenubarItem>
                            <MenubarSeparator attr:class=classes::separator />
                            <MenubarItem attr:class=classes::item>"Cut"</MenubarItem>
                            <MenubarItem attr:class=classes::item>"Copy"</MenubarItem>
                            <MenubarItem attr:class=classes::item>"Paste"</MenubarItem>
                        </MenubarContent>
                    </MenubarPortal>
                </MenubarMenu>
            </Menubar>

            <h2 style="margin-top: 600px;">"With checkbox and radio items"</h2>
            <Menubar value="items".to_string() attr:class=classes::root>
                <MenubarMenu value="items".to_string()>
                    <MenubarTrigger attr:class=classes::trigger>"Items"</MenubarTrigger>
                    <MenubarPortal>
                        <MenubarContent attr:class=classes::content side_offset=2.0>
                            <MenubarItem attr:class=classes::item>"Show fonts"</MenubarItem>
                            <MenubarItem attr:class=classes::item>"Bigger"</MenubarItem>
                            <MenubarItem attr:class=classes::item>"Smaller"</MenubarItem>
                            <MenubarSeparator attr:class=classes::separator />
                            {checkbox_items.iter().map(|(label, (checked, set_checked), disabled)| {
                                let label = *label;
                                let checked = *checked;
                                let set_checked = *set_checked;
                                let disabled = *disabled;
                                view! {
                                    <MenubarCheckboxItem
                                        attr:class=classes::item
                                        checked=Signal::derive(move || {
                                            if checked.get() { CheckedState::True } else { CheckedState::False }
                                        })
                                        on_checked_change=Callback::new(move |value: bool| {
                                            set_checked.set(value);
                                        })
                                        disabled=disabled
                                    >
                                        {label}
                                        <MenubarItemIndicator>
                                            {tick_icon()}
                                        </MenubarItemIndicator>
                                    </MenubarCheckboxItem>
                                }
                            }).collect_view()}
                            <MenubarSeparator attr:class=classes::separator />
                            <MenubarRadioGroup
                                value=Signal::derive(move || file.get())
                                on_value_change=Callback::new(move |value: String| set_file.set(value))
                            >
                                {files.iter().map(|f| {
                                    let f = f.to_string();
                                    view! {
                                        <MenubarRadioItem attr:class=classes::item value=f.clone()>
                                            {f.clone()}
                                            <MenubarItemIndicator>
                                                {tick_icon()}
                                            </MenubarItemIndicator>
                                        </MenubarRadioItem>
                                    }
                                }).collect_view()}
                            </MenubarRadioGroup>
                        </MenubarContent>
                    </MenubarPortal>
                </MenubarMenu>
                <MenubarMenu value="edit".to_string()>
                    <MenubarTrigger attr:class=classes::trigger>"Edit"</MenubarTrigger>
                    <MenubarPortal>
                        <MenubarContent attr:class=classes::content side_offset=2.0>
                            <MenubarItem attr:class=classes::item disabled=true>"Undo"</MenubarItem>
                            <MenubarItem attr:class=classes::item>"Redo"</MenubarItem>
                            <MenubarSeparator attr:class=classes::separator />
                            <MenubarItem attr:class=classes::item>"Cut"</MenubarItem>
                            <MenubarItem attr:class=classes::item>"Copy"</MenubarItem>
                            <MenubarItem attr:class=classes::item>"Paste"</MenubarItem>
                        </MenubarContent>
                    </MenubarPortal>
                </MenubarMenu>
            </Menubar>
        </div>
    }
}
