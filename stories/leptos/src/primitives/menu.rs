use leptos::prelude::*;
use radix_leptos_direction::{Direction, DirectionProvider};
use radix_leptos_menu::*;

stylance::import_crate_style!(classes, "src/primitives/menu.stories.module.css");

struct FoodGroup {
    label: Option<&'static str>,
    foods: &'static [Food],
}

struct Food {
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
        <MenuWithAnchor>
            <MenuItem
                attr:class=classes::item
                on_select=Callback::new(move |_: web_sys::Event| {
                    web_sys::window().unwrap().alert_with_message("undo").ok();
                })
            >
                "Undo"
            </MenuItem>
            <MenuItem
                attr:class=classes::item
                on_select=Callback::new(move |_: web_sys::Event| {
                    web_sys::window().unwrap().alert_with_message("redo").ok();
                })
            >
                "Redo"
            </MenuItem>
            <MenuSeparator attr:class=classes::separator />
            <MenuItem
                attr:class=classes::item
                disabled=true
                on_select=Callback::new(move |_: web_sys::Event| {
                    web_sys::window().unwrap().alert_with_message("cut").ok();
                })
            >
                "Cut"
            </MenuItem>
            <MenuItem
                attr:class=classes::item
                on_select=Callback::new(move |_: web_sys::Event| {
                    web_sys::window().unwrap().alert_with_message("copy").ok();
                })
            >
                "Copy"
            </MenuItem>
            <MenuItem
                attr:class=classes::item
                on_select=Callback::new(move |_: web_sys::Event| {
                    web_sys::window().unwrap().alert_with_message("paste").ok();
                })
            >
                "Paste"
            </MenuItem>
        </MenuWithAnchor>
    }
}

#[component]
pub fn Submenus() -> impl IntoView {
    let (open1, set_open1) = signal(false);
    let (open2, set_open2) = signal(false);
    let (open3, set_open3) = signal(false);
    let (open4, set_open4) = signal(false);
    let (rtl, set_rtl) = signal(false);
    let (animated, set_animated) = signal(false);

    view! {
        <DirectionProvider direction=Signal::derive(move || if rtl.get() { Direction::Rtl } else { Direction::Ltr })>
            <div style="margin-bottom: 8px; display: grid; grid-auto-flow: row; gap: 4px;">
                <label>
                    <input
                        type="checkbox"
                        prop:checked=rtl
                        on:change=move |ev| set_rtl.set(event_target_checked(&ev))
                    />
                    "Right-to-left"
                </label>
                <label>
                    <input
                        type="checkbox"
                        prop:checked=animated
                        on:change=move |ev| set_animated.set(event_target_checked(&ev))
                    />
                    "Animated"
                </label>
            </div>
            <MenuWithAnchor>
                <MenuItem
                    attr:class=classes::item
                    on_select=Callback::new(move |_: web_sys::Event| {
                        web_sys::window().unwrap().alert_with_message("undo").ok();
                    })
                >
                    "Undo"
                </MenuItem>
                <Submenu open=open1 on_open_change=Callback::new(move |v| set_open1.set(v)) animated=animated>
                    <MenuItem attr:class=classes::item disabled=true>"Disabled"</MenuItem>
                    <MenuItem
                        attr:class=classes::item
                        on_select=Callback::new(move |_: web_sys::Event| {
                            web_sys::window().unwrap().alert_with_message("one").ok();
                        })
                    >
                        "One"
                    </MenuItem>
                    <Submenu open=open2 on_open_change=Callback::new(move |v| set_open2.set(v)) animated=animated>
                        <MenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::window().unwrap().alert_with_message("one").ok(); })>"One"</MenuItem>
                        <MenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::window().unwrap().alert_with_message("two").ok(); })>"Two"</MenuItem>
                        <MenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::window().unwrap().alert_with_message("three").ok(); })>"Three"</MenuItem>
                        <MenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::window().unwrap().alert_with_message("four").ok(); })>"Four"</MenuItem>
                        <MenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::window().unwrap().alert_with_message("five").ok(); })>"Five"</MenuItem>
                        <MenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::window().unwrap().alert_with_message("six").ok(); })>"Six"</MenuItem>
                    </Submenu>
                    <Submenu heading="Sub Menu" open=open3 on_open_change=Callback::new(move |v| set_open3.set(v)) animated=animated>
                        <MenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::window().unwrap().alert_with_message("one").ok(); })>"One"</MenuItem>
                        <MenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::window().unwrap().alert_with_message("two").ok(); })>"Two"</MenuItem>
                        <MenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::window().unwrap().alert_with_message("three").ok(); })>"Three"</MenuItem>
                    </Submenu>
                    <MenuItem
                        attr:class=classes::item
                        on_select=Callback::new(move |_: web_sys::Event| {
                            web_sys::window().unwrap().alert_with_message("two").ok();
                        })
                    >
                        "Two"
                    </MenuItem>
                    <Submenu open=open4 on_open_change=Callback::new(move |v| set_open4.set(v)) animated=animated disabled=true>
                        <MenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::window().unwrap().alert_with_message("one").ok(); })>"One"</MenuItem>
                        <MenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::window().unwrap().alert_with_message("two").ok(); })>"Two"</MenuItem>
                        <MenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::window().unwrap().alert_with_message("three").ok(); })>"Three"</MenuItem>
                    </Submenu>
                    <MenuItem
                        attr:class=classes::item
                        on_select=Callback::new(move |_: web_sys::Event| {
                            web_sys::window().unwrap().alert_with_message("three").ok();
                        })
                    >
                        "Three"
                    </MenuItem>
                </Submenu>

                <MenuSeparator attr:class=classes::separator />
                <MenuItem attr:class=classes::item disabled=true on_select=Callback::new(move |_: web_sys::Event| { web_sys::window().unwrap().alert_with_message("cut").ok(); })>"Cut"</MenuItem>
                <MenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::window().unwrap().alert_with_message("copy").ok(); })>"Copy"</MenuItem>
                <MenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::window().unwrap().alert_with_message("paste").ok(); })>"Paste"</MenuItem>
            </MenuWithAnchor>
        </DirectionProvider>
    }
}

#[component]
pub fn WithLabels() -> impl IntoView {
    let last_index = FOOD_GROUPS.len() - 1;

    view! {
        <MenuWithAnchor>
            {FOOD_GROUPS.iter().enumerate().map(|(index, food_group)| {
                view! {
                    <MenuGroup>
                        {food_group.label.map(|label| view! {
                            <MenuLabel attr:class=classes::label>{label}</MenuLabel>
                        })}
                        {food_group.foods.iter().map(|food| {
                            let label = food.label;
                            view! {
                                <MenuItem
                                    attr:class=classes::item
                                    disabled=food.disabled
                                    on_select=Callback::new(move |_: web_sys::Event| {
                                        web_sys::window().unwrap().alert_with_message(label).ok();
                                    })
                                >
                                    {food.label}
                                </MenuItem>
                            }
                        }).collect_view()}
                        {(index < last_index).then(|| view! {
                            <MenuSeparator attr:class=classes::separator />
                        })}
                    </MenuGroup>
                }
            }).collect_view()}
        </MenuWithAnchor>
    }
}

struct Suit {
    emoji: &'static str,
    label: &'static str,
}

const SUITS: &[Suit] = &[
    Suit {
        emoji: "\u{2665}\u{fe0f}",
        label: "Hearts",
    },
    Suit {
        emoji: "\u{2660}\u{fe0f}",
        label: "Spades",
    },
    Suit {
        emoji: "\u{2666}\u{fe0f}",
        label: "Diamonds",
    },
    Suit {
        emoji: "\u{2663}\u{fe0f}",
        label: "Clubs",
    },
];

#[component]
pub fn Typeahead() -> impl IntoView {
    view! {
        <h1>"Testing ground for typeahead behaviour"</h1>

        <div style="display: flex; align-items: flex-start; gap: 100px;">
            <div>
                <h2>"Text labels"</h2>
                <div style="margin-bottom: 20px;">
                    <p>
                        "For comparison"
                        <br />
                        "try the closed select below"
                    </p>
                    <select>
                        {FOOD_GROUPS.iter().map(|food_group| {
                            food_group.foods.iter().map(|food| {
                                view! {
                                    <option value=food.value disabled=food.disabled>
                                        {food.label}
                                    </option>
                                }
                            }).collect_view()
                        }).collect_view()}
                    </select>
                </div>
                <WithLabels />
            </div>

            <div>
                <h2>"Complex children"</h2>
                <p>"(relying on " <code>".textContent"</code> " — default)"</p>
                <MenuWithAnchor>
                    {SUITS.iter().map(|suit| {
                        view! {
                            <MenuItem attr:class=classes::item>
                                {suit.label}
                                <span role="img" aria-label=suit.label>
                                    {suit.emoji}
                                </span>
                            </MenuItem>
                        }
                    }).collect_view()}
                </MenuWithAnchor>
            </div>

            <div>
                <h2>"Complex children"</h2>
                <p>"(with explicit " <code>"textValue"</code> " prop)"</p>
                <MenuWithAnchor>
                    {SUITS.iter().map(|suit| {
                        view! {
                            <MenuItem attr:class=classes::item text_value=suit.label>
                                <span role="img" aria-label=suit.label>
                                    {suit.emoji}
                                </span>
                                {suit.label}
                            </MenuItem>
                        }
                    }).collect_view()}
                </MenuWithAnchor>
            </div>
        </div>
    }
}

#[component]
pub fn CheckboxItems() -> impl IntoView {
    const OPTIONS: &[&str] = &["Crows", "Ravens", "Magpies", "Jackdaws"];
    let (selection, set_selection) = signal::<Vec<String>>(vec![]);

    view! {
        <MenuWithAnchor>
            <MenuCheckboxItem
                attr:class=classes::item
                checked=Signal::derive(move || {
                    let sel = selection.get();
                    if sel.len() == OPTIONS.len() {
                        CheckedState::True
                    } else if sel.is_empty() {
                        CheckedState::False
                    } else {
                        CheckedState::Indeterminate
                    }
                })
                on_checked_change=Callback::new(move |_: bool| {
                    set_selection.update(|sel| {
                        if sel.len() == OPTIONS.len() {
                            sel.clear();
                        } else {
                            *sel = OPTIONS.iter().map(|s| s.to_string()).collect();
                        }
                    });
                })
            >
                "Select all"
                <MenuItemIndicator>
                    {move || {
                        if selection.get().len() == OPTIONS.len() {
                            view! { <TickIcon /> }.into_any()
                        } else {
                            view! { "\u{2014}" }.into_any()
                        }
                    }}
                </MenuItemIndicator>
            </MenuCheckboxItem>
            <MenuSeparator attr:class=classes::separator />
            {OPTIONS.iter().map(|&option| {
                let option_string = option.to_string();
                let option_string2 = option_string.clone();
                view! {
                    <MenuCheckboxItem
                        attr:class=classes::item
                        checked=Signal::derive(move || {
                            if selection.get().contains(&option_string.clone()) {
                                CheckedState::True
                            } else {
                                CheckedState::False
                            }
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
                        <MenuItemIndicator>
                            <TickIcon />
                        </MenuItemIndicator>
                    </MenuCheckboxItem>
                }
            }).collect_view()}
        </MenuWithAnchor>
    }
}

#[component]
pub fn RadioItems() -> impl IntoView {
    const FILES: &[&str] = &["README.md", "index.js", "page.css"];
    let (file, set_file) = signal("index.js".to_string());

    view! {
        <MenuWithAnchor>
            <MenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::window().unwrap().alert_with_message("minimize").ok(); })>"Minimize window"</MenuItem>
            <MenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::window().unwrap().alert_with_message("zoom").ok(); })>"Zoom"</MenuItem>
            <MenuItem attr:class=classes::item on_select=Callback::new(move |_: web_sys::Event| { web_sys::window().unwrap().alert_with_message("smaller").ok(); })>"Smaller"</MenuItem>
            <MenuSeparator attr:class=classes::separator />
            <MenuRadioGroup value=file on_value_change=Callback::new(move |v: String| set_file.set(v))>
                {FILES.iter().map(|&f| {
                    view! {
                        <MenuRadioItem attr:class=classes::item value=f>
                            {f}
                            <MenuItemIndicator>
                                <TickIcon />
                            </MenuItemIndicator>
                        </MenuRadioItem>
                    }
                }).collect_view()}
            </MenuRadioGroup>
        </MenuWithAnchor>
    }
}

#[component]
pub fn Animated() -> impl IntoView {
    const FILES: &[&str] = &["README.md", "index.js", "page.css"];
    let (file, set_file) = signal("index.js".to_string());
    let (open, set_open) = signal(true);
    let (bold, set_bold) = signal(false);
    let (italic, set_italic) = signal(true);
    let (underline, set_underline) = signal(false);
    let (strikethrough, _set_strikethrough) = signal(false);

    let checkbox_items: StoredValue<Vec<(&str, ReadSignal<bool>, Callback<bool>, bool)>> =
        StoredValue::new(vec![
            ("Bold", bold, Callback::new(move |v| set_bold.set(v)), false),
            (
                "Italic",
                italic,
                Callback::new(move |v| set_italic.set(v)),
                false,
            ),
            (
                "Underline",
                underline,
                Callback::new(move |v| set_underline.set(v)),
                false,
            ),
            (
                "Strikethrough",
                strikethrough,
                Callback::new(move |_: bool| {}),
                true,
            ),
        ]);

    view! {
        <label>
            <input
                type="checkbox"
                prop:checked=open
                on:change=move |ev| set_open.set(event_target_checked(&ev))
            />
            " open"
        </label>
        <br />
        <br />
        <MenuWithAnchor attr_class=classes::animatedContent.to_string() open=open>
            {checkbox_items.with_value(|items| items.iter().map(|&(label, checked, on_change, disabled)| {
                view! {
                    <MenuCheckboxItem
                        attr:class=classes::item
                        checked=Signal::derive(move || CheckedState::from(checked.get()))
                        on_checked_change=on_change
                        disabled=disabled
                    >
                        {label}
                        <MenuItemIndicator attr:class=classes::animatedItemIndicator>
                            <TickIcon />
                        </MenuItemIndicator>
                    </MenuCheckboxItem>
                }
            }).collect_view())}
            <MenuRadioGroup value=file on_value_change=Callback::new(move |v: String| set_file.set(v))>
                {FILES.iter().map(|&f| {
                    view! {
                        <MenuRadioItem attr:class=classes::item value=f>
                            {f}
                            <MenuItemIndicator attr:class=classes::animatedItemIndicator>
                                <TickIcon />
                            </MenuItemIndicator>
                        </MenuRadioItem>
                    }
                }).collect_view()}
            </MenuRadioGroup>
        </MenuWithAnchor>
    }
}

#[component]
fn MenuWithAnchor(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] attr_class: Option<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let open = Signal::derive(move || open.get().unwrap_or(true));

    let content_class = StoredValue::new(match attr_class {
        Some(extra) => format!("{} {}", classes::content, extra),
        None => classes::content.to_string(),
    });

    view! {
        <Menu open=open modal=false>
            <MenuAnchor attr:style="display: inline-block">{""}</MenuAnchor>
            <MenuPortal>
                <MenuContent class=Signal::derive(move || content_class.get_value())>
                    {children.with_value(|children| children())}
                </MenuContent>
            </MenuPortal>
        </Menu>
    }
}

#[component]
fn Submenu(
    #[prop(into, optional)] heading: Option<String>,
    #[prop(into)] open: Signal<bool>,
    #[prop(into)] on_open_change: Callback<bool>,
    #[prop(into, optional)] animated: Signal<bool>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let heading = StoredValue::new(heading.unwrap_or_else(|| "Submenu".to_string()));

    let content_class = Signal::derive(move || {
        if animated.get() {
            format!("{} {}", classes::animatedContent, classes::content)
        } else {
            classes::content.to_string()
        }
    });

    view! {
        <MenuSub open=open on_open_change=on_open_change>
            <MenuSubTrigger
                attr:class=format!("{} {}", classes::item, classes::subTrigger)
                disabled=disabled
            >
                {heading.get_value()} " \u{2192}"
            </MenuSubTrigger>
            <MenuPortal>
                <MenuSubContent class=content_class>
                    {children.with_value(|children| children())}
                </MenuSubContent>
            </MenuPortal>
        </MenuSub>
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
