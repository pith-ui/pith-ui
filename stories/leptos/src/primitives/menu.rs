use leptos::prelude::*;
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
            Food { value: "apple", label: "Apple", disabled: false },
            Food { value: "banana", label: "Banana", disabled: false },
            Food { value: "blueberry", label: "Blueberry", disabled: false },
            Food { value: "grapes", label: "Grapes", disabled: false },
            Food { value: "pineapple", label: "Pineapple", disabled: false },
        ],
    },
    FoodGroup {
        label: Some("Vegetables"),
        foods: &[
            Food { value: "aubergine", label: "Aubergine", disabled: false },
            Food { value: "broccoli", label: "Broccoli", disabled: false },
            Food { value: "carrot", label: "Carrot", disabled: true },
            Food { value: "courgette", label: "Courgette", disabled: false },
            Food { value: "leek", label: "Leek", disabled: false },
        ],
    },
    FoodGroup {
        label: Some("Meat"),
        foods: &[
            Food { value: "beef", label: "Beef", disabled: false },
            Food { value: "beef-with-sauce", label: "Beef with sauce", disabled: false },
            Food { value: "chicken", label: "Chicken", disabled: false },
            Food { value: "lamb", label: "Lamb", disabled: false },
            Food { value: "pork", label: "Pork", disabled: false },
        ],
    },
    FoodGroup {
        label: None,
        foods: &[
            Food { value: "candies", label: "Candies", disabled: false },
            Food { value: "chocolates", label: "Chocolates", disabled: false },
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

// TODO: Blocked by unimplemented MenuSub, MenuSubTrigger, MenuSubContent components.
#[component]
pub fn Submenus() -> impl IntoView {
    view! {
        <p>"Submenus story is blocked by unimplemented MenuSub, MenuSubTrigger, and MenuSubContent components."</p>
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
    Suit { emoji: "\u{2665}\u{fe0f}", label: "Hearts" },
    Suit { emoji: "\u{2660}\u{fe0f}", label: "Spades" },
    Suit { emoji: "\u{2666}\u{fe0f}", label: "Diamonds" },
    Suit { emoji: "\u{2663}\u{fe0f}", label: "Clubs" },
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
                            <MenuItem attr:class=classes::item>
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

// TODO: Blocked by unimplemented MenuCheckboxItem and MenuItemIndicator components.
#[component]
pub fn CheckboxItems() -> impl IntoView {
    view! {
        <p>"CheckboxItems story is blocked by unimplemented MenuCheckboxItem and MenuItemIndicator components."</p>
    }
}

// TODO: Blocked by unimplemented MenuRadioGroup, MenuRadioItem, and MenuItemIndicator components.
#[component]
pub fn RadioItems() -> impl IntoView {
    view! {
        <p>"RadioItems story is blocked by unimplemented MenuRadioGroup, MenuRadioItem, and MenuItemIndicator components."</p>
    }
}

// TODO: Blocked by unimplemented MenuCheckboxItem, MenuRadioGroup, MenuRadioItem, and MenuItemIndicator components.
#[component]
pub fn Animated() -> impl IntoView {
    view! {
        <p>"Animated story is blocked by unimplemented MenuCheckboxItem, MenuRadioGroup, MenuRadioItem, and MenuItemIndicator components."</p>
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

    let content_class = match attr_class {
        Some(extra) => format!("{} {}", classes::content, extra),
        None => classes::content.to_string(),
    };

    view! {
        <Menu open=open modal=false>
            <MenuAnchor attr:style="display: inline-block">{""}</MenuAnchor>
            <MenuPortal>
                <MenuContent attr:class=content_class>
                    {children.with_value(|children| children())}
                </MenuContent>
            </MenuPortal>
        </Menu>
    }
}
