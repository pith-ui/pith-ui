use leptos::prelude::*;
use radix_leptos_primitives::context_menu::*;
use radix_leptos_primitives::direction::{Direction, DirectionProvider};

stylance::import_crate_style!(
    #[allow(dead_code)]
    classes,
    "src/primitives/context_menu.stories.module.css"
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
                value: "beef-with-sauce",
                label: "Beef with sauce",
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
    FoodGroup {
        label: None,
        foods: &[
            Food {
                value: "candies",
                label: "Candies",
                disabled: false,
            },
            Food {
                value: "chocolates",
                label: "Chocolates",
                disabled: false,
            },
        ],
    },
];

#[component]
pub fn Styled() -> impl IntoView {
    view! {
        <div style="display: flex; align-items: center; justify-content: center; width: 200vw; height: 200vh; gap: 20px;">
            <ContextMenu>
                <ContextMenuTrigger attr:class=classes::trigger>"Right click here"</ContextMenuTrigger>
                <ContextMenuPortal>
                    <ContextMenuContent attr:class=classes::content>
                        <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"undo".into()); })>"Undo"</ContextMenuItem>
                        <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"redo".into()); })>"Redo"</ContextMenuItem>
                        <ContextMenuSeparator attr:class=classes::separator />
                        <ContextMenuItem attr:class=classes::item disabled=true on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"cut".into()); })>"Cut"</ContextMenuItem>
                        <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"copy".into()); })>"Copy"</ContextMenuItem>
                        <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"paste".into()); })>"Paste"</ContextMenuItem>
                    </ContextMenuContent>
                </ContextMenuPortal>
            </ContextMenu>
        </div>
    }
}

#[component]
pub fn Modality() -> impl IntoView {
    view! {
        <div style="display: flex; align-items: center; justify-content: center; height: 110vh;">
            <div style="display: grid; gap: 50px;">
                <div style="display: inline-flex; align-items: center; flex-direction: column;">
                    <h1>"Modal (default)"</h1>
                    <ContextMenu>
                        <ContextMenuTrigger attr:class=classes::trigger>{""}</ContextMenuTrigger>
                        <ContextMenuPortal>
                            <ContextMenuContent attr:class=classes::content>
                                <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"undo".into()); })>"Undo"</ContextMenuItem>
                                <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"redo".into()); })>"Redo"</ContextMenuItem>
                                <ContextMenuSeparator attr:class=classes::separator />
                                <ContextMenuSub>
                                    <ContextMenuSubTrigger attr:class=classes::subTrigger>"Submenu \u{2192}"</ContextMenuSubTrigger>
                                    <ContextMenuPortal>
                                        <ContextMenuSubContent attr:class=classes::content side_offset=12.0>
                                            <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"one".into()); })>"One"</ContextMenuItem>
                                            <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"two".into()); })>"Two"</ContextMenuItem>
                                            <ContextMenuSeparator attr:class=classes::separator />
                                            <ContextMenuSub>
                                                <ContextMenuSubTrigger attr:class=classes::subTrigger>"Submenu \u{2192}"</ContextMenuSubTrigger>
                                                <ContextMenuPortal>
                                                    <ContextMenuSubContent attr:class=classes::content side_offset=12.0>
                                                        <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"one".into()); })>"One"</ContextMenuItem>
                                                        <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"two".into()); })>"Two"</ContextMenuItem>
                                                        <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"three".into()); })>"Three"</ContextMenuItem>
                                                        <ContextMenuArrow />
                                                    </ContextMenuSubContent>
                                                </ContextMenuPortal>
                                            </ContextMenuSub>
                                            <ContextMenuSeparator attr:class=classes::separator />
                                            <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"three".into()); })>"Three"</ContextMenuItem>
                                            <ContextMenuArrow />
                                        </ContextMenuSubContent>
                                    </ContextMenuPortal>
                                </ContextMenuSub>
                                <ContextMenuSeparator attr:class=classes::separator />
                                <ContextMenuItem attr:class=classes::item disabled=true on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"cut".into()); })>"Cut"</ContextMenuItem>
                                <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"copy".into()); })>"Copy"</ContextMenuItem>
                                <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"paste".into()); })>"Paste"</ContextMenuItem>
                            </ContextMenuContent>
                        </ContextMenuPortal>
                    </ContextMenu>
                    <textarea
                        style="width: 500px; height: 100px; margin-top: 10px;"
                    >"Lorem ipsum dolor sit amet consectetur adipisicing elit. Quaerat nobis at ipsa, nihil tempora debitis maxime dignissimos non amet."</textarea>
                </div>
                <div style="display: inline-flex; align-items: center; flex-direction: column;">
                    <h1>"Non modal"</h1>
                    <ContextMenu modal=false>
                        <ContextMenuTrigger attr:class=classes::trigger>{""}</ContextMenuTrigger>
                        <ContextMenuPortal>
                            <ContextMenuContent attr:class=classes::content>
                                <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"undo".into()); })>"Undo"</ContextMenuItem>
                                <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"redo".into()); })>"Redo"</ContextMenuItem>
                                <ContextMenuSeparator attr:class=classes::separator />
                                <ContextMenuSub>
                                    <ContextMenuSubTrigger attr:class=classes::subTrigger>"Submenu \u{2192}"</ContextMenuSubTrigger>
                                    <ContextMenuPortal>
                                        <ContextMenuSubContent attr:class=classes::content side_offset=12.0>
                                            <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"one".into()); })>"One"</ContextMenuItem>
                                            <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"two".into()); })>"Two"</ContextMenuItem>
                                            <ContextMenuSeparator attr:class=classes::separator />
                                            <ContextMenuSub>
                                                <ContextMenuSubTrigger attr:class=classes::subTrigger>"Submenu \u{2192}"</ContextMenuSubTrigger>
                                                <ContextMenuPortal>
                                                    <ContextMenuSubContent attr:class=classes::content side_offset=12.0>
                                                        <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"one".into()); })>"One"</ContextMenuItem>
                                                        <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"two".into()); })>"Two"</ContextMenuItem>
                                                        <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"three".into()); })>"Three"</ContextMenuItem>
                                                        <ContextMenuArrow />
                                                    </ContextMenuSubContent>
                                                </ContextMenuPortal>
                                            </ContextMenuSub>
                                            <ContextMenuSeparator attr:class=classes::separator />
                                            <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"three".into()); })>"Three"</ContextMenuItem>
                                            <ContextMenuArrow />
                                        </ContextMenuSubContent>
                                    </ContextMenuPortal>
                                </ContextMenuSub>
                                <ContextMenuSeparator attr:class=classes::separator />
                                <ContextMenuItem attr:class=classes::item disabled=true on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"cut".into()); })>"Cut"</ContextMenuItem>
                                <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"copy".into()); })>"Copy"</ContextMenuItem>
                                <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"paste".into()); })>"Paste"</ContextMenuItem>
                            </ContextMenuContent>
                        </ContextMenuPortal>
                    </ContextMenu>
                    <textarea
                        style="width: 500px; height: 100px; margin-top: 10px;"
                    >"Lorem ipsum dolor sit amet consectetur adipisicing elit. Quaerat nobis at ipsa, nihil tempora debitis maxime dignissimos non amet."</textarea>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Submenus() -> impl IntoView {
    let (rtl, set_rtl) = signal(false);

    view! {
        <div style="display: flex; align-items: center; justify-content: center; height: 100vh; gap: 20px;">
            <div style="display: flex; flex-direction: column; align-items: center;">
                <label style="margin-bottom: 10px;">
                    <input
                        type="checkbox"
                        prop:checked=rtl
                        on:change=move |ev| set_rtl.set(event_target_checked(&ev))
                    />
                    " Right-to-left"
                </label>
                <DirectionProvider direction=Signal::derive(move || if rtl.get() { Direction::Rtl } else { Direction::Ltr })>
                    <ContextMenu dir=Signal::derive(move || if rtl.get() { Direction::Rtl } else { Direction::Ltr })>
                        <ContextMenuTrigger attr:class=classes::trigger>"Right Click Here"</ContextMenuTrigger>
                        <ContextMenuPortal>
                            <ContextMenuContent attr:class=classes::content>
                                <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"new-tab".into()); })>"New Tab"</ContextMenuItem>
                                <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"new-window".into()); })>"New Window"</ContextMenuItem>
                                <ContextMenuSeparator attr:class=classes::separator />
                                <ContextMenuSub>
                                    <ContextMenuSubTrigger attr:class=classes::subTrigger>"Bookmarks \u{2192}"</ContextMenuSubTrigger>
                                    <ContextMenuPortal>
                                        <ContextMenuSubContent attr:class=classes::content side_offset=12.0>
                                            <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"index".into()); })>"Inbox"</ContextMenuItem>
                                            <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"calendar".into()); })>"Calendar"</ContextMenuItem>
                                            <ContextMenuSeparator attr:class=classes::separator />
                                            <ContextMenuSub>
                                                <ContextMenuSubTrigger attr:class=classes::subTrigger>"WorkOS \u{2192}"</ContextMenuSubTrigger>
                                                <ContextMenuPortal>
                                                    <ContextMenuSubContent attr:class=classes::content side_offset=12.0>
                                                        <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"stitches".into()); })>"Stitches"</ContextMenuItem>
                                                        <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"composer".into()); })>"Composer"</ContextMenuItem>
                                                        <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"radix".into()); })>"Radix"</ContextMenuItem>
                                                        <ContextMenuArrow />
                                                    </ContextMenuSubContent>
                                                </ContextMenuPortal>
                                            </ContextMenuSub>
                                            <ContextMenuSeparator attr:class=classes::separator />
                                            <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"notion".into()); })>"Notion"</ContextMenuItem>
                                            <ContextMenuArrow />
                                        </ContextMenuSubContent>
                                    </ContextMenuPortal>
                                </ContextMenuSub>
                                <ContextMenuSub>
                                    <ContextMenuSubTrigger attr:class=classes::subTrigger disabled=true>"History \u{2192}"</ContextMenuSubTrigger>
                                    <ContextMenuPortal>
                                        <ContextMenuSubContent attr:class=classes::content side_offset=12.0>
                                            <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"github".into()); })>"Github"</ContextMenuItem>
                                            <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"google".into()); })>"Google"</ContextMenuItem>
                                            <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"stack-overflow".into()); })>"Stack Overflow"</ContextMenuItem>
                                            <ContextMenuArrow />
                                        </ContextMenuSubContent>
                                    </ContextMenuPortal>
                                </ContextMenuSub>
                                <ContextMenuSub>
                                    <ContextMenuSubTrigger attr:class=classes::subTrigger>"Tools \u{2192}"</ContextMenuSubTrigger>
                                    <ContextMenuPortal>
                                        <ContextMenuSubContent attr:class=classes::content side_offset=12.0>
                                            <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"extensions".into()); })>"Extensions"</ContextMenuItem>
                                            <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"task-manager".into()); })>"Task Manager"</ContextMenuItem>
                                            <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"developer-tools".into()); })>"Developer Tools"</ContextMenuItem>
                                            <ContextMenuArrow />
                                        </ContextMenuSubContent>
                                    </ContextMenuPortal>
                                </ContextMenuSub>
                                <ContextMenuSeparator attr:class=classes::separator />
                                <ContextMenuItem attr:class=classes::item disabled=true on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"print".into()); })>"Print\u{2026}"</ContextMenuItem>
                                <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"cast".into()); })>"Cast\u{2026}"</ContextMenuItem>
                                <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"find".into()); })>"Find\u{2026}"</ContextMenuItem>
                            </ContextMenuContent>
                        </ContextMenuPortal>
                    </ContextMenu>
                </DirectionProvider>
            </div>
        </div>
    }
}

#[component]
pub fn WithLabels() -> impl IntoView {
    let last_index = FOOD_GROUPS.len() - 1;

    view! {
        <div style="text-align: center; padding: 50px;">
            <ContextMenu>
                <ContextMenuTrigger attr:class=classes::trigger>"Right click here"</ContextMenuTrigger>
                <ContextMenuPortal>
                    <ContextMenuContent attr:class=classes::content>
                        {FOOD_GROUPS.iter().enumerate().map(|(index, food_group)| {
                            view! {
                                <ContextMenuGroup>
                                    {food_group.label.map(|label| view! {
                                        <ContextMenuLabel attr:class=classes::label>{label}</ContextMenuLabel>
                                    })}
                                    {food_group.foods.iter().map(|food| {
                                        let label = food.label;
                                        view! {
                                            <ContextMenuItem
                                                attr:class=classes::item
                                                disabled=food.disabled
                                                on_select=Callback::new(move |_: web_sys::Event| {
                                                    web_sys::console::log_1(&label.into());
                                                })
                                            >
                                                {food.label}
                                            </ContextMenuItem>
                                        }
                                    }).collect_view()}
                                    {(index < last_index).then(|| view! {
                                        <ContextMenuSeparator attr:class=classes::separator />
                                    })}
                                </ContextMenuGroup>
                            }
                        }).collect_view()}
                    </ContextMenuContent>
                </ContextMenuPortal>
            </ContextMenu>
        </div>
    }
}

#[component]
pub fn CheckboxItems() -> impl IntoView {
    let checkbox_items = ["Bold", "Italic", "Underline"];
    let (selection, set_selection) = signal::<Vec<String>>(vec![]);

    view! {
        <div style="text-align: center; padding: 50px;">
            <ContextMenu>
                <ContextMenuTrigger attr:class=classes::trigger>"Right click here"</ContextMenuTrigger>
                <ContextMenuPortal>
                    <ContextMenuContent attr:class=classes::content>
                        <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"show".into()); })>"Show fonts"</ContextMenuItem>
                        <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"bigger".into()); })>"Bigger"</ContextMenuItem>
                        <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"smaller".into()); })>"Smaller"</ContextMenuItem>
                        <ContextMenuSeparator attr:class=classes::separator />
                        {checkbox_items.iter().map(|&item| {
                            let item_string = item.to_string();
                            let item_string2 = item_string.clone();
                            view! {
                                <ContextMenuCheckboxItem
                                    attr:class=classes::item
                                    checked=Signal::derive(move || {
                                        if selection.get().contains(&item_string.clone()) {
                                            CheckedState::True
                                        } else {
                                            CheckedState::False
                                        }
                                    })
                                    on_checked_change=Callback::new(move |_: bool| {
                                        let opt = item_string2.clone();
                                        set_selection.update(|sel| {
                                            if sel.contains(&opt) {
                                                sel.retain(|s| s != &opt);
                                            } else {
                                                sel.push(opt);
                                            }
                                        });
                                    })
                                >
                                    {item}
                                    <ContextMenuItemIndicator>
                                        <TickIcon />
                                    </ContextMenuItemIndicator>
                                </ContextMenuCheckboxItem>
                            }
                        }).collect_view()}
                        <ContextMenuSeparator />
                        <ContextMenuCheckboxItem attr:class=classes::item disabled=true>
                            "Strikethrough"
                            <ContextMenuItemIndicator>
                                <TickIcon />
                            </ContextMenuItemIndicator>
                        </ContextMenuCheckboxItem>
                    </ContextMenuContent>
                </ContextMenuPortal>
            </ContextMenu>
        </div>
    }
}

#[component]
pub fn RadioItems() -> impl IntoView {
    let files = ["README.md", "index.js", "page.css"];
    let (file, set_file) = signal("index.js".to_string());

    view! {
        <div style="text-align: center; padding: 50px;">
            <ContextMenu>
                <ContextMenuTrigger attr:class=classes::trigger>"Right click here"</ContextMenuTrigger>
                <ContextMenuPortal>
                    <ContextMenuContent attr:class=classes::content>
                        <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"minimize".into()); })>"Minimize window"</ContextMenuItem>
                        <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"zoom".into()); })>"Zoom"</ContextMenuItem>
                        <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"smaller".into()); })>"Smaller"</ContextMenuItem>
                        <ContextMenuSeparator attr:class=classes::separator />
                        <ContextMenuRadioGroup value=file on_value_change=Callback::new(move |v: String| set_file.set(v))>
                            {files.iter().map(|&f| {
                                view! {
                                    <ContextMenuRadioItem attr:class=classes::item value=f>
                                        {f}
                                        <ContextMenuItemIndicator>
                                            <TickIcon />
                                        </ContextMenuItemIndicator>
                                    </ContextMenuRadioItem>
                                }
                            }).collect_view()}
                        </ContextMenuRadioGroup>
                    </ContextMenuContent>
                </ContextMenuPortal>
            </ContextMenu>
            <p>"Selected file: " {move || file.get()}</p>
        </div>
    }
}

#[component]
pub fn PreventClosing() -> impl IntoView {
    view! {
        <div style="text-align: center; padding: 50px;">
            <ContextMenu>
                <ContextMenuTrigger attr:class=classes::trigger>"Right click here"</ContextMenuTrigger>
                <ContextMenuPortal>
                    <ContextMenuContent attr:class=classes::content>
                        <ContextMenuItem
                            attr:class=classes::item
                            on_select=Callback::new(move |_: web_sys::Event| {
                                web_sys::window().unwrap().alert_with_message("action 1").ok();
                            })
                        >
                            "I will close"
                        </ContextMenuItem>
                        <ContextMenuItem
                            attr:class=classes::item
                            on_select=Callback::new(move |event: web_sys::Event| {
                                event.prevent_default();
                                web_sys::window().unwrap().alert_with_message("action 1").ok();
                            })
                        >
                            "I won't close"
                        </ContextMenuItem>
                    </ContextMenuContent>
                </ContextMenuPortal>
            </ContextMenu>
        </div>
    }
}

#[component]
pub fn Nested() -> impl IntoView {
    view! {
        <div style="display: flex; align-items: center; justify-content: center;">
            <ContextMenu>
                <ContextMenuTrigger attr:class=classes::trigger attr:style="padding: 100px; background-color: royalblue;">
                    <ContextMenu>
                        <ContextMenuTrigger attr:class=classes::trigger attr:style="background-color: tomato;">{""}</ContextMenuTrigger>
                        <ContextMenuPortal>
                            <ContextMenuContent attr:class=classes::content>
                                <ContextMenuLabel attr:class=classes::label>"Red box menu"</ContextMenuLabel>
                                <ContextMenuSeparator attr:class=classes::separator />
                                <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"red action1".into()); })>"Red action 1"</ContextMenuItem>
                                <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"red action2".into()); })>"Red action 2"</ContextMenuItem>
                                <ContextMenuSeparator attr:class=classes::separator />
                                <ContextMenuSub>
                                    <ContextMenuSubTrigger attr:class=classes::subTrigger>"Submenu \u{2192}"</ContextMenuSubTrigger>
                                    <ContextMenuPortal>
                                        <ContextMenuSubContent attr:class=classes::content side_offset=12.0>
                                            <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"red sub action 1".into()); })>"Red sub action 1"</ContextMenuItem>
                                            <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"red sub action 2".into()); })>"Red sub action 2"</ContextMenuItem>
                                            <ContextMenuArrow />
                                        </ContextMenuSubContent>
                                    </ContextMenuPortal>
                                </ContextMenuSub>
                            </ContextMenuContent>
                        </ContextMenuPortal>
                    </ContextMenu>
                </ContextMenuTrigger>
                <ContextMenuPortal>
                    <ContextMenuContent attr:class=classes::content>
                        <ContextMenuLabel attr:class=classes::label>"Blue box menu"</ContextMenuLabel>
                        <ContextMenuSeparator attr:class=classes::separator />
                        <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"blue action1".into()); })>"Blue action 1"</ContextMenuItem>
                        <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"blue action2".into()); })>"Blue action 2"</ContextMenuItem>
                        <ContextMenuSeparator attr:class=classes::separator />
                        <ContextMenuSub>
                            <ContextMenuSubTrigger attr:class=classes::subTrigger>"Submenu \u{2192}"</ContextMenuSubTrigger>
                            <ContextMenuPortal>
                                <ContextMenuSubContent attr:class=classes::content side_offset=12.0>
                                    <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"blue sub action 1".into()); })>"Blue sub action 1"</ContextMenuItem>
                                    <ContextMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"blue sub action 2".into()); })>"Blue sub action 2"</ContextMenuItem>
                                    <ContextMenuArrow />
                                </ContextMenuSubContent>
                            </ContextMenuPortal>
                        </ContextMenuSub>
                    </ContextMenuContent>
                </ContextMenuPortal>
            </ContextMenu>
        </div>
    }
}

#[component]
fn TickIcon() -> impl IntoView {
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
