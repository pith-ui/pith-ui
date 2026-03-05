use std::cell::Cell;
use std::rc::Rc;

use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use radix_leptos_primitives::dialog::*;
use radix_leptos_primitives::direction::{Direction, DirectionProvider};
use radix_leptos_primitives::dropdown_menu::*;
use radix_leptos_primitives::tooltip::*;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::JsCast;

stylance::import_crate_style!(
    #[allow(dead_code)]
    classes,
    "src/primitives/dropdown_menu.stories.module.css"
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

fn sub_trigger_class() -> String {
    format!("{} {}", classes::item, classes::subTrigger)
}

#[component]
pub fn Styled() -> impl IntoView {
    view! {
        <div style="display: flex; align-items: center; justify-content: center; height: 200vh;">
            <DropdownMenu>
                <DropdownMenuTrigger attr:class=classes::trigger>"Open"</DropdownMenuTrigger>
                <DropdownMenuPortal>
                    <DropdownMenuContent class=classes::content side_offset=5.0>
                        <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"undo".into()); })>"Undo"</DropdownMenuItem>
                        <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"redo".into()); })>"Redo"</DropdownMenuItem>
                        <DropdownMenuSeparator attr:class=classes::separator />
                        <DropdownMenuItem attr:class=classes::item disabled=true on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"cut".into()); })>"Cut"</DropdownMenuItem>
                        <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"copy".into()); })>"Copy"</DropdownMenuItem>
                        <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"paste".into()); })>"Paste"</DropdownMenuItem>
                        <DropdownMenuArrow />
                    </DropdownMenuContent>
                </DropdownMenuPortal>
            </DropdownMenu>
        </div>
    }
}

#[component]
pub fn Modality() -> impl IntoView {
    let sub_trigger_class = StoredValue::new(sub_trigger_class());

    view! {
        <div style="display: flex; align-items: center; justify-content: center; height: 110vh;">
            <div style="display: grid; gap: 50px;">
                <div style="display: inline-flex; align-items: center; flex-direction: column;">
                    <h1>"Modal (default)"</h1>
                    <DropdownMenu>
                        <DropdownMenuTrigger attr:class=classes::trigger>"Open"</DropdownMenuTrigger>
                        <DropdownMenuPortal>
                            <DropdownMenuContent class=classes::content side_offset=5.0>
                                <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"undo".into()); })>"Undo"</DropdownMenuItem>
                                <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"redo".into()); })>"Redo"</DropdownMenuItem>
                                <DropdownMenuSeparator attr:class=classes::separator />
                                <DropdownMenuSub>
                                    <DropdownMenuSubTrigger attr:class=sub_trigger_class.get_value()>"Submenu \u{2192}"</DropdownMenuSubTrigger>
                                    <DropdownMenuPortal>
                                        <DropdownMenuSubContent class=classes::content side_offset=12.0>
                                            <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"one".into()); })>"One"</DropdownMenuItem>
                                            <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"two".into()); })>"Two"</DropdownMenuItem>
                                            <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"three".into()); })>"Three"</DropdownMenuItem>
                                            <DropdownMenuArrow />
                                        </DropdownMenuSubContent>
                                    </DropdownMenuPortal>
                                </DropdownMenuSub>
                                <DropdownMenuSeparator attr:class=classes::separator />
                                <DropdownMenuItem attr:class=classes::item disabled=true on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"cut".into()); })>"Cut"</DropdownMenuItem>
                                <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"copy".into()); })>"Copy"</DropdownMenuItem>
                                <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"paste".into()); })>"Paste"</DropdownMenuItem>
                                <DropdownMenuArrow />
                            </DropdownMenuContent>
                        </DropdownMenuPortal>
                    </DropdownMenu>
                    <textarea
                        style="width: 500px; height: 100px; margin-top: 10px;"
                    >"Lorem ipsum dolor sit amet consectetur adipisicing elit. Quaerat nobis at ipsa, nihil tempora debitis maxime dignissimos non amet."</textarea>
                </div>
                <div style="display: inline-flex; align-items: center; flex-direction: column;">
                    <h1>"Non modal"</h1>
                    <DropdownMenu modal=false>
                        <DropdownMenuTrigger attr:class=classes::trigger>"Open"</DropdownMenuTrigger>
                        <DropdownMenuPortal>
                            <DropdownMenuContent class=classes::content side_offset=5.0>
                                <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"undo".into()); })>"Undo"</DropdownMenuItem>
                                <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"redo".into()); })>"Redo"</DropdownMenuItem>
                                <DropdownMenuSeparator attr:class=classes::separator />
                                <DropdownMenuSub>
                                    <DropdownMenuSubTrigger attr:class=sub_trigger_class.get_value()>"Submenu \u{2192}"</DropdownMenuSubTrigger>
                                    <DropdownMenuPortal>
                                        <DropdownMenuSubContent class=classes::content side_offset=12.0>
                                            <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"one".into()); })>"One"</DropdownMenuItem>
                                            <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"two".into()); })>"Two"</DropdownMenuItem>
                                            <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"three".into()); })>"Three"</DropdownMenuItem>
                                            <DropdownMenuArrow />
                                        </DropdownMenuSubContent>
                                    </DropdownMenuPortal>
                                </DropdownMenuSub>
                                <DropdownMenuSeparator attr:class=classes::separator />
                                <DropdownMenuItem attr:class=classes::item disabled=true on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"cut".into()); })>"Cut"</DropdownMenuItem>
                                <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"copy".into()); })>"Copy"</DropdownMenuItem>
                                <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"paste".into()); })>"Paste"</DropdownMenuItem>
                                <DropdownMenuArrow />
                            </DropdownMenuContent>
                        </DropdownMenuPortal>
                    </DropdownMenu>
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
    let sub_trigger_class = StoredValue::new(sub_trigger_class());

    view! {
        <div style="display: flex; align-items: center; justify-content: center; height: 100vh; gap: 20px;">
            <div style="display: flex; flex-direction: column; justify-content: center;">
                <label style="margin-bottom: 10px;">
                    <input
                        type="checkbox"
                        prop:checked=rtl
                        on:change=move |ev| set_rtl.set(event_target_checked(&ev))
                    />
                    " Right-to-left"
                </label>
                <DirectionProvider direction=Signal::derive(move || if rtl.get() { Direction::Rtl } else { Direction::Ltr })>
                    <DropdownMenu dir=Signal::derive(move || if rtl.get() { Direction::Rtl } else { Direction::Ltr })>
                        <DropdownMenuTrigger attr:class=classes::trigger>"Open"</DropdownMenuTrigger>
                        <DropdownMenuPortal>
                            <DropdownMenuContent class=classes::content side_offset=5.0>
                                <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"new-tab".into()); })>"New Tab"</DropdownMenuItem>
                                <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"new-window".into()); })>"New Window"</DropdownMenuItem>
                                <DropdownMenuSeparator attr:class=classes::separator />
                                <DropdownMenuSub>
                                    <DropdownMenuSubTrigger attr:class=sub_trigger_class.get_value()>"Bookmarks \u{2192}"</DropdownMenuSubTrigger>
                                    <DropdownMenuPortal>
                                        <DropdownMenuSubContent class=classes::content side_offset=12.0>
                                            <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"index".into()); })>"Inbox"</DropdownMenuItem>
                                            <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"calendar".into()); })>"Calendar"</DropdownMenuItem>
                                            <DropdownMenuSeparator attr:class=classes::separator />
                                            <DropdownMenuSub>
                                                <DropdownMenuSubTrigger attr:class=sub_trigger_class.get_value()>"WorkOS \u{2192}"</DropdownMenuSubTrigger>
                                                <DropdownMenuPortal>
                                                    <DropdownMenuSubContent class=classes::content side_offset=12.0>
                                                        <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"stitches".into()); })>"Stitches"</DropdownMenuItem>
                                                        <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"composer".into()); })>"Composer"</DropdownMenuItem>
                                                        <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"radix".into()); })>"Radix"</DropdownMenuItem>
                                                        <DropdownMenuArrow />
                                                    </DropdownMenuSubContent>
                                                </DropdownMenuPortal>
                                            </DropdownMenuSub>
                                            <DropdownMenuSeparator attr:class=classes::separator />
                                            <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"notion".into()); })>"Notion"</DropdownMenuItem>
                                            <DropdownMenuArrow />
                                        </DropdownMenuSubContent>
                                    </DropdownMenuPortal>
                                </DropdownMenuSub>
                                <DropdownMenuSub>
                                    <DropdownMenuSubTrigger attr:class=sub_trigger_class.get_value() disabled=true>"History \u{2192}"</DropdownMenuSubTrigger>
                                    <DropdownMenuPortal>
                                        <DropdownMenuSubContent class=classes::content side_offset=12.0>
                                            <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"github".into()); })>"Github"</DropdownMenuItem>
                                            <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"google".into()); })>"Google"</DropdownMenuItem>
                                            <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"stack-overflow".into()); })>"Stack Overflow"</DropdownMenuItem>
                                            <DropdownMenuArrow />
                                        </DropdownMenuSubContent>
                                    </DropdownMenuPortal>
                                </DropdownMenuSub>
                                <DropdownMenuSub>
                                    <DropdownMenuSubTrigger attr:class=sub_trigger_class.get_value()>"Tools \u{2192}"</DropdownMenuSubTrigger>
                                    <DropdownMenuPortal>
                                        <DropdownMenuSubContent class=classes::content side_offset=12.0>
                                            <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"extensions".into()); })>"Extensions"</DropdownMenuItem>
                                            <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"task-manager".into()); })>"Task Manager"</DropdownMenuItem>
                                            <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"developer-tools".into()); })>"Developer Tools"</DropdownMenuItem>
                                            <DropdownMenuArrow />
                                        </DropdownMenuSubContent>
                                    </DropdownMenuPortal>
                                </DropdownMenuSub>
                                <DropdownMenuSeparator attr:class=classes::separator />
                                <DropdownMenuItem attr:class=classes::item disabled=true on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"print".into()); })>"Print\u{2026}"</DropdownMenuItem>
                                <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"cast".into()); })>"Cast\u{2026}"</DropdownMenuItem>
                                <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"find".into()); })>"Find\u{2026}"</DropdownMenuItem>
                                <DropdownMenuArrow />
                            </DropdownMenuContent>
                        </DropdownMenuPortal>
                    </DropdownMenu>
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
            <DropdownMenu>
                <DropdownMenuTrigger attr:class=classes::trigger>"Open"</DropdownMenuTrigger>
                <DropdownMenuPortal>
                    <DropdownMenuContent class=classes::content side_offset=5.0>
                        {FOOD_GROUPS.iter().enumerate().map(|(index, food_group)| {
                            view! {
                                <DropdownMenuGroup>
                                    {food_group.label.map(|label| view! {
                                        <DropdownMenuLabel attr:class=classes::label>{label}</DropdownMenuLabel>
                                    })}
                                    {food_group.foods.iter().map(|food| {
                                        let label = food.label;
                                        view! {
                                            <DropdownMenuItem
                                                attr:class=classes::item
                                                disabled=food.disabled
                                                on_select=Callback::new(move |_: web_sys::Event| {
                                                    web_sys::console::log_1(&label.into());
                                                })
                                            >
                                                {food.label}
                                            </DropdownMenuItem>
                                        }
                                    }).collect_view()}
                                    {(index < last_index).then(|| view! {
                                        <DropdownMenuSeparator attr:class=classes::separator />
                                    })}
                                </DropdownMenuGroup>
                            }
                        }).collect_view()}
                        <DropdownMenuArrow />
                    </DropdownMenuContent>
                </DropdownMenuPortal>
            </DropdownMenu>
        </div>
    }
}

#[component]
pub fn CheckboxItems() -> impl IntoView {
    let options = ["Crows", "Ravens", "Magpies", "Jackdaws"];
    let (selection, set_selection) = signal::<Vec<String>>(vec![]);

    let all_selected = move || selection.get().len() == options.len();

    view! {
        <div style="text-align: center; padding: 50px;">
            <DropdownMenu>
                <DropdownMenuTrigger attr:class=classes::trigger>"Open"</DropdownMenuTrigger>
                <DropdownMenuPortal>
                    <DropdownMenuContent class=classes::content side_offset=5.0>
                        <DropdownMenuGroup>
                            <DropdownMenuCheckboxItem
                                attr:class=classes::item
                                checked=Signal::derive(move || {
                                    let sel = selection.get();
                                    if sel.len() == options.len() {
                                        CheckedState::True
                                    } else if !sel.is_empty() {
                                        CheckedState::Indeterminate
                                    } else {
                                        CheckedState::False
                                    }
                                })
                                on_select=Callback::new(move |event: web_sys::Event| {
                                    event.prevent_default();
                                })
                                on_checked_change=Callback::new(move |_: bool| {
                                    set_selection.update(|sel| {
                                        if sel.len() == options.len() {
                                            sel.clear();
                                        } else {
                                            *sel = options.iter().map(|s| s.to_string()).collect();
                                        }
                                    });
                                })
                            >
                                "Select all"
                                <DropdownMenuItemIndicator>
                                    {move || if all_selected() {
                                        view! { <TickIcon /> }.into_any()
                                    } else {
                                        view! { "\u{2014}" }.into_any()
                                    }}
                                </DropdownMenuItemIndicator>
                            </DropdownMenuCheckboxItem>
                            <DropdownMenuSeparator attr:class=classes::separator />
                            {options.iter().map(|&option| {
                                let option_string = option.to_string();
                                let option_string2 = option_string.clone();
                                view! {
                                    <DropdownMenuCheckboxItem
                                        attr:class=classes::item
                                        checked=Signal::derive(move || {
                                            if selection.get().contains(&option_string.clone()) {
                                                CheckedState::True
                                            } else {
                                                CheckedState::False
                                            }
                                        })
                                        on_select=Callback::new(move |event: web_sys::Event| {
                                            event.prevent_default();
                                        })
                                        on_checked_change=Callback::new(move |_: bool| {
                                            let opt = option_string2.clone();
                                            set_selection.update(|sel| {
                                                if sel.contains(&opt) {
                                                    sel.retain(|s| s != &opt);
                                                } else {
                                                    sel.push(opt);
                                                }
                                            });
                                        })
                                    >
                                        {option}
                                        <DropdownMenuItemIndicator>
                                            <TickIcon />
                                        </DropdownMenuItemIndicator>
                                    </DropdownMenuCheckboxItem>
                                }
                            }).collect_view()}
                        </DropdownMenuGroup>
                        <DropdownMenuArrow />
                    </DropdownMenuContent>
                </DropdownMenuPortal>
            </DropdownMenu>
        </div>
    }
}

#[component]
pub fn RadioItems() -> impl IntoView {
    let files = ["README.md", "index.js", "page.css"];
    let (file, set_file) = signal("index.js".to_string());

    view! {
        <div style="text-align: center; padding: 50px;">
            <DropdownMenu>
                <DropdownMenuTrigger attr:class=classes::trigger>"Open"</DropdownMenuTrigger>
                <DropdownMenuPortal>
                    <DropdownMenuContent class=classes::content side_offset=5.0>
                        <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"minimize".into()); })>"Minimize window"</DropdownMenuItem>
                        <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"zoom".into()); })>"Zoom"</DropdownMenuItem>
                        <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"smaller".into()); })>"Smaller"</DropdownMenuItem>
                        <DropdownMenuSeparator attr:class=classes::separator />
                        <DropdownMenuRadioGroup value=file on_value_change=Callback::new(move |v: String| set_file.set(v))>
                            {files.iter().map(|&f| {
                                view! {
                                    <DropdownMenuRadioItem attr:class=classes::item value=f>
                                        {f}
                                        <DropdownMenuItemIndicator>
                                            <TickIcon />
                                        </DropdownMenuItemIndicator>
                                    </DropdownMenuRadioItem>
                                }
                            }).collect_view()}
                        </DropdownMenuRadioGroup>
                        <DropdownMenuArrow />
                    </DropdownMenuContent>
                </DropdownMenuPortal>
            </DropdownMenu>
            <p>"Selected file: " {move || file.get()}</p>
        </div>
    }
}

#[component]
pub fn PreventClosing() -> impl IntoView {
    view! {
        <div style="text-align: center; padding: 50px;">
            <DropdownMenu>
                <DropdownMenuTrigger attr:class=classes::trigger>"Open"</DropdownMenuTrigger>
                <DropdownMenuPortal>
                    <DropdownMenuContent class=classes::content side_offset=5.0>
                        <DropdownMenuItem
                            attr:class=classes::item
                            on_select=Callback::new(move |_: web_sys::Event| {
                                web_sys::window().unwrap().alert_with_message("action 1").ok();
                            })
                        >
                            "I will close"
                        </DropdownMenuItem>
                        <DropdownMenuItem
                            attr:class=classes::item
                            on_select=Callback::new(move |event: web_sys::Event| {
                                event.prevent_default();
                                web_sys::window().unwrap().alert_with_message("action 1").ok();
                            })
                        >
                            "I won't close"
                        </DropdownMenuItem>
                        <DropdownMenuArrow />
                    </DropdownMenuContent>
                </DropdownMenuPortal>
            </DropdownMenu>
        </div>
    }
}

#[component]
pub fn WithTooltip() -> impl IntoView {
    view! {
        <div style="display: flex; align-items: center; justify-content: center; height: 200vh;">
            <DropdownMenu>
                <TooltipProvider>
                    <Tooltip>
                        <TooltipTrigger as_child=true>
                            <DropdownMenuTrigger attr:class=classes::trigger>"Open"</DropdownMenuTrigger>
                        </TooltipTrigger>
                        <TooltipContent>"Tooltip content"</TooltipContent>
                    </Tooltip>
                </TooltipProvider>
                <DropdownMenuPortal>
                    <DropdownMenuContent class=classes::content side_offset=5.0>
                        <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"undo".into()); })>"Undo"</DropdownMenuItem>
                        <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"redo".into()); })>"Redo"</DropdownMenuItem>
                        <DropdownMenuSeparator attr:class=classes::separator />
                        <DropdownMenuItem attr:class=classes::item disabled=true on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"cut".into()); })>"Cut"</DropdownMenuItem>
                        <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"copy".into()); })>"Copy"</DropdownMenuItem>
                        <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"paste".into()); })>"Paste"</DropdownMenuItem>
                        <DropdownMenuArrow />
                    </DropdownMenuContent>
                </DropdownMenuPortal>
            </DropdownMenu>
        </div>
    }
}

#[component]
pub fn NestedComposition() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh;">
            <DropdownMenu>
                <DropdownMenuTrigger attr:class=classes::trigger>"Open"</DropdownMenuTrigger>
                <DropdownMenuPortal>
                    <DropdownMenuContent class=classes::content side_offset=5.0>
                        <Dialog>
                            <DialogTrigger attr:class=classes::item as_child=true>
                                <DropdownMenuItem on_select=Callback::new(move |event: web_sys::Event| { event.prevent_default(); })>
                                    "Open dialog"
                                </DropdownMenuItem>
                            </DialogTrigger>

                            <DialogPortal>
                                <DialogContent attr:class=classes::dialog>
                                    <DialogTitle>"Nested dropdown"</DialogTitle>
                                    <DropdownMenu>
                                        <DropdownMenuTrigger attr:class=classes::trigger attr:style="width: 100%; margin-bottom: 20px;">
                                            "Open"
                                        </DropdownMenuTrigger>
                                        <DropdownMenuPortal>
                                            <DropdownMenuContent class=classes::content side_offset=5.0>
                                                <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"undo".into()); })>"Undo"</DropdownMenuItem>
                                                <DropdownMenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::console::log_1(&"redo".into()); })>"Redo"</DropdownMenuItem>
                                                <DropdownMenuArrow />
                                            </DropdownMenuContent>
                                        </DropdownMenuPortal>
                                    </DropdownMenu>
                                    <DialogClose>"Close"</DialogClose>
                                </DialogContent>
                            </DialogPortal>
                        </Dialog>
                        <DropdownMenuItem attr:class=classes::item>"Test"</DropdownMenuItem>
                        <DropdownMenuArrow />
                    </DropdownMenuContent>
                </DropdownMenuPortal>
            </DropdownMenu>
        </div>
    }
}

#[component]
pub fn SingleItemAsDialogTrigger() -> impl IntoView {
    let dropdown_trigger_ref = AnyNodeRef::new();
    let dropdown_trigger_ref2 = AnyNodeRef::new();
    let is_dialog_open_ref = SendWrapper::new(Rc::new(Cell::new(false)));

    let close_auto_focus_modal = {
        let dropdown_trigger_ref = dropdown_trigger_ref;
        Callback::new(move |event: web_sys::Event| {
            if let Some(trigger) = dropdown_trigger_ref
                .try_read_untracked()
                .and_then(|guard| (*guard).clone())
            {
                let el: web_sys::HtmlElement = trigger.unchecked_into();
                el.focus().ok();
            }
            event.prevent_default();
        })
    };

    let content_close_auto_focus = {
        let is_dialog_open_ref = is_dialog_open_ref.clone();
        Callback::new(move |event: web_sys::Event| {
            if is_dialog_open_ref.get() {
                event.prevent_default();
            }
        })
    };

    let on_select_delete = {
        let is_dialog_open_ref = is_dialog_open_ref.clone();
        Callback::new(move |_: web_sys::Event| {
            is_dialog_open_ref.set(true);
        })
    };

    let close_auto_focus_non_modal = {
        let dropdown_trigger_ref2 = dropdown_trigger_ref2;
        let is_dialog_open_ref = is_dialog_open_ref;
        Callback::new(move |event: web_sys::Event| {
            if let Some(trigger) = dropdown_trigger_ref2
                .try_read_untracked()
                .and_then(|guard| (*guard).clone())
            {
                let el: web_sys::HtmlElement = trigger.unchecked_into();
                el.focus().ok();
            }
            event.prevent_default();
            is_dialog_open_ref.set(false);
        })
    };

    view! {
        <div style="display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh;">
            <h1>"Modal"</h1>
            <Dialog>
                <DropdownMenu>
                    <DropdownMenuTrigger attr:class=classes::trigger node_ref=dropdown_trigger_ref>
                        "Open"
                    </DropdownMenuTrigger>

                    <DropdownMenuPortal>
                        <DropdownMenuContent class=classes::content side_offset=5.0>
                            <DialogTrigger attr:class=classes::item as_child=true>
                                <DropdownMenuItem>"Delete"</DropdownMenuItem>
                            </DialogTrigger>
                            <DropdownMenuItem attr:class=classes::item>"Test"</DropdownMenuItem>
                            <DropdownMenuArrow />
                        </DropdownMenuContent>
                    </DropdownMenuPortal>
                </DropdownMenu>

                <DialogContent attr:class=classes::dialog on_close_auto_focus=close_auto_focus_modal>
                    <DialogTitle>"Are you sure?"</DialogTitle>
                    <DialogClose>"Close"</DialogClose>
                </DialogContent>
            </Dialog>

            <h1>"Non-modal"</h1>
            <Dialog modal=false>
                <DropdownMenu modal=false>
                    <DropdownMenuTrigger attr:class=classes::trigger node_ref=dropdown_trigger_ref2>
                        "Open"
                    </DropdownMenuTrigger>

                    <DropdownMenuPortal>
                        <DropdownMenuContent class=classes::content side_offset=5.0
                            on_close_auto_focus=content_close_auto_focus
                        >
                            <DialogTrigger attr:class=classes::item as_child=true>
                                <DropdownMenuItem on_select=on_select_delete>
                                    "Delete"
                                </DropdownMenuItem>
                            </DialogTrigger>
                            <DropdownMenuItem attr:class=classes::item>"Test"</DropdownMenuItem>
                            <DropdownMenuArrow />
                        </DropdownMenuContent>
                    </DropdownMenuPortal>
                </DropdownMenu>

                <DialogContent attr:class=classes::dialog on_close_auto_focus=close_auto_focus_non_modal>
                    <DialogTitle>"Are you sure?"</DialogTitle>
                    <DialogClose>"Close"</DialogClose>
                </DialogContent>
            </Dialog>
        </div>
    }
}

#[component]
pub fn MultipleItemsAsDialogTriggers() -> impl IntoView {
    let (delete_open, set_delete_open) = signal(false);
    let (switch_accounts_open, set_switch_accounts_open) = signal(false);
    let (delete_open2, set_delete_open2) = signal(false);
    let (switch_accounts_open2, set_switch_accounts_open2) = signal(false);
    let dropdown_trigger_ref = AnyNodeRef::new();
    let dropdown_trigger_ref2 = AnyNodeRef::new();

    view! {
        <div style="display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh;">
            <h1>"Modal"</h1>
            <Dialog on_open_change=Callback::new(move |open: bool| {
                if !open {
                    set_delete_open.set(false);
                    set_switch_accounts_open.set(false);
                }
            })>
                <DropdownMenu>
                    <DropdownMenuTrigger attr:class=classes::trigger node_ref=dropdown_trigger_ref>
                        "Open"
                    </DropdownMenuTrigger>

                    <DropdownMenuPortal>
                        <DropdownMenuContent class=classes::content side_offset=5.0>
                            <DialogTrigger as_child=true attr:class=classes::item>
                                <DropdownMenuItem on_select=Callback::new(move |_: web_sys::Event| { set_switch_accounts_open.set(true); })>
                                    "Switch Accounts"
                                </DropdownMenuItem>
                            </DialogTrigger>
                            <DialogTrigger as_child=true attr:class=classes::item>
                                <DropdownMenuItem on_select=Callback::new(move |_: web_sys::Event| { set_delete_open.set(true); })>
                                    "Delete"
                                </DropdownMenuItem>
                            </DialogTrigger>
                            <DropdownMenuArrow />
                        </DropdownMenuContent>
                    </DropdownMenuPortal>
                </DropdownMenu>

                <DialogContent attr:class=classes::dialog on_close_auto_focus={
                    let dropdown_trigger_ref = dropdown_trigger_ref;
                    Callback::new(move |event: web_sys::Event| {
                        if let Some(trigger) = dropdown_trigger_ref.try_read_untracked().and_then(|guard| (*guard).clone()) {
                            let el: web_sys::HtmlElement = trigger.unchecked_into();
                            el.focus().ok();
                        }
                        event.prevent_default();
                    })
                }>
                    {move || switch_accounts_open.get().then(|| view! { <DialogTitle>"Switch accounts"</DialogTitle> })}
                    {move || delete_open.get().then(|| view! { <DialogTitle>"Are you sure?"</DialogTitle> })}
                    <DialogClose>"Close"</DialogClose>
                </DialogContent>
            </Dialog>

            <h1>"Non-modal"</h1>
            <Dialog modal=false on_open_change=Callback::new(move |open: bool| {
                if !open {
                    set_delete_open2.set(false);
                    set_switch_accounts_open2.set(false);
                }
            })>
                <DropdownMenu modal=false>
                    <DropdownMenuTrigger attr:class=classes::trigger node_ref=dropdown_trigger_ref2>
                        "Open"
                    </DropdownMenuTrigger>

                    <DropdownMenuPortal>
                        <DropdownMenuContent class=classes::content side_offset=5.0
                            on_close_auto_focus=Callback::new(move |event: web_sys::Event| {
                                if delete_open2.get_untracked() || switch_accounts_open2.get_untracked() {
                                    event.prevent_default();
                                }
                            })
                        >
                            <DialogTrigger as_child=true attr:class=classes::item>
                                <DropdownMenuItem on_select=Callback::new(move |_: web_sys::Event| { set_switch_accounts_open2.set(true); })>
                                    "Switch Accounts"
                                </DropdownMenuItem>
                            </DialogTrigger>
                            <DialogTrigger as_child=true attr:class=classes::item>
                                <DropdownMenuItem on_select=Callback::new(move |_: web_sys::Event| { set_delete_open2.set(true); })>
                                    "Delete"
                                </DropdownMenuItem>
                            </DialogTrigger>
                            <DropdownMenuArrow />
                        </DropdownMenuContent>
                    </DropdownMenuPortal>
                </DropdownMenu>

                <DialogContent attr:class=classes::dialog on_close_auto_focus={
                    let dropdown_trigger_ref2 = dropdown_trigger_ref2;
                    Callback::new(move |event: web_sys::Event| {
                        if let Some(trigger) = dropdown_trigger_ref2.try_read_untracked().and_then(|guard| (*guard).clone()) {
                            let el: web_sys::HtmlElement = trigger.unchecked_into();
                            el.focus().ok();
                        }
                        event.prevent_default();
                    })
                }>
                    {move || switch_accounts_open2.get().then(|| view! { <DialogTitle>"Switch accounts"</DialogTitle> })}
                    {move || delete_open2.get().then(|| view! { <DialogTitle>"Are you sure?"</DialogTitle> })}
                    <DialogClose>"Close"</DialogClose>
                </DialogContent>
            </Dialog>
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
