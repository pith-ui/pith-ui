use leptos::prelude::*;
use pith_ui::label::Label;
use pith_ui::select::*;
use wasm_bindgen::JsCast;

stylance::import_crate_style!(classes, "src/primitives/select.stories.module.css");

fn scroll_up_button_class() -> String {
    format!("{} {}", classes::scrollUpButton, classes::scrollButton)
}

fn scroll_down_button_class() -> String {
    format!("{} {}", classes::scrollDownButton, classes::scrollButton)
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

/// Helper component to avoid duplicating select markup for each position mode.
#[component]
fn StyledSelectExample(position: &'static str) -> impl IntoView {
    view! {
        <Label>
            "Choose a number:"
            <Select default_value="two">
                <SelectTrigger attr:class=classes::trigger>
                    <SelectValue />
                    <SelectIcon />
                </SelectTrigger>
                <SelectPortal>
                    <SelectContent attr:class=classes::content position=position side_offset=5.0>
                        <SelectViewport attr:class=classes::viewport>
                            <SelectItem attr:class=classes::item value="one">
                                <SelectItemText>
                                    "One"<span aria-hidden="true">" \u{1F44D}"</span>
                                </SelectItemText>
                                <SelectItemIndicator attr:class=classes::indicator>
                                    <TickIcon />
                                </SelectItemIndicator>
                            </SelectItem>
                            <SelectItem attr:class=classes::item value="two">
                                <SelectItemText>
                                    "Two"<span aria-hidden="true">" \u{1F44C}"</span>
                                </SelectItemText>
                                <SelectItemIndicator attr:class=classes::indicator>
                                    <TickIcon />
                                </SelectItemIndicator>
                            </SelectItem>
                            <SelectItem attr:class=classes::item value="three">
                                <SelectItemText>
                                    "Three"<span aria-hidden="true">" \u{1F918}"</span>
                                </SelectItemText>
                                <SelectItemIndicator attr:class=classes::indicator>
                                    <TickIcon />
                                </SelectItemIndicator>
                            </SelectItem>
                        </SelectViewport>
                        <SelectArrow />
                    </SelectContent>
                </SelectPortal>
            </Select>
        </Label>
    }
}

#[component]
pub fn Styled() -> impl IntoView {
    view! {
        <div style="display: flex; gap: 20px; padding: 50px;">
            <StyledSelectExample position="item-aligned" />
            <StyledSelectExample position="popper" />
        </div>
    }
}

#[component]
fn ControlledSelectExample(
    position: &'static str,
    value: Signal<Option<String>>,
    on_value_change: Callback<String>,
) -> impl IntoView {
    let aria_label = Signal::derive(move || {
        value.get().and_then(|v| match v.as_str() {
            "fr" => Some("France".to_string()),
            "uk" => Some("United Kingdom".to_string()),
            "es" => Some("Spain".to_string()),
            _ => None,
        })
    });
    let flag_emoji = Signal::derive(move || {
        value.get().and_then(|v| match v.as_str() {
            "fr" => Some("\u{1F1EB}\u{1F1F7}"),
            "uk" => Some("\u{1F1EC}\u{1F1E7}"),
            "es" => Some("\u{1F1EA}\u{1F1F8}"),
            _ => None,
        })
    });

    view! {
        <Label>
            "Choose a country:"
            <Select value=value on_value_change=on_value_change>
                <SelectTrigger attr:class=classes::trigger>
                    <SelectValue attr:aria-label=move || aria_label.get()>
                        {move || flag_emoji.get()}
                    </SelectValue>
                    <SelectIcon />
                </SelectTrigger>
                <SelectPortal>
                    <SelectContent attr:class=classes::content position=position side_offset=5.0>
                        <SelectViewport attr:class=classes::viewport>
                            <SelectItem attr:class=classes::item value="fr">
                                <SelectItemText>
                                    "France"<span aria-hidden="true">" \u{1F1EB}\u{1F1F7}"</span>
                                </SelectItemText>
                                <SelectItemIndicator attr:class=classes::indicator>
                                    <TickIcon />
                                </SelectItemIndicator>
                            </SelectItem>
                            <SelectItem attr:class=classes::item value="uk">
                                <SelectItemText>
                                    "United Kingdom"<span aria-hidden="true">" \u{1F1EC}\u{1F1E7}"</span>
                                </SelectItemText>
                                <SelectItemIndicator attr:class=classes::indicator>
                                    <TickIcon />
                                </SelectItemIndicator>
                            </SelectItem>
                            <SelectItem attr:class=classes::item value="es">
                                <SelectItemText>
                                    "Spain"<span aria-hidden="true">" \u{1F1EA}\u{1F1F8}"</span>
                                </SelectItemText>
                                <SelectItemIndicator attr:class=classes::indicator>
                                    <TickIcon />
                                </SelectItemIndicator>
                            </SelectItem>
                        </SelectViewport>
                        <SelectArrow />
                    </SelectContent>
                </SelectPortal>
            </Select>
        </Label>
    }
}

#[component]
pub fn Controlled() -> impl IntoView {
    let (value, set_value) = signal(Some("uk".to_string()));
    let value_signal = Signal::derive(move || value.get());
    let on_change = Callback::new(move |v: String| set_value.set(Some(v)));

    view! {
        <div style="display: flex; gap: 20px; padding: 50px;">
            <ControlledSelectExample position="item-aligned" value=value_signal on_value_change=on_change />
            <ControlledSelectExample position="popper" value=value_signal on_value_change=on_change />
        </div>
    }
}

#[component]
fn PositionSelectExample(position: &'static str) -> impl IntoView {
    view! {
        <Label>
            "Choose an item:"
            <Select default_value="item-25">
                <SelectTrigger attr:class=classes::trigger>
                    <SelectValue />
                    <SelectIcon />
                </SelectTrigger>
                <SelectPortal>
                    <SelectContent attr:class=classes::content position=position side_offset=5.0>
                        <SelectScrollUpButton attr:class=scroll_up_button_class()>
                            {"\u{25B2}"}
                        </SelectScrollUpButton>
                        <SelectViewport attr:class=classes::viewport>
                            {(0..50)
                                .map(|i| {
                                    let value = format!("item-{}", i + 1);
                                    let disabled = i > 5 && i < 9;
                                    view! {
                                        <SelectItem
                                            attr:class=classes::item
                                            value=value
                                            disabled=disabled
                                        >
                                            <SelectItemText>
                                                {format!("item {}", i + 1)}
                                            </SelectItemText>
                                            <SelectItemIndicator attr:class=classes::indicator>
                                                <TickIcon />
                                            </SelectItemIndicator>
                                        </SelectItem>
                                    }
                                })
                                .collect_view()}
                        </SelectViewport>
                        <SelectScrollDownButton attr:class=scroll_down_button_class()>
                            {"\u{25BC}"}
                        </SelectScrollDownButton>
                        <SelectArrow />
                    </SelectContent>
                </SelectPortal>
            </Select>
        </Label>
    }
}

#[component]
pub fn Position() -> impl IntoView {
    view! {
        <div style="display: flex; gap: 20px; align-items: center; justify-content: center; width: 300vw; height: 300vh;">
            <PositionSelectExample position="item-aligned" />
            <PositionSelectExample position="popper" />
        </div>
    }
}

#[component]
fn NoDefaultValueExample(position: &'static str) -> impl IntoView {
    view! {
        <Label>
            "Choose a number:"
            <Select>
                <SelectTrigger attr:class=classes::trigger>
                    <SelectValue placeholder="Pick an option" />
                    <SelectIcon />
                </SelectTrigger>
                <SelectPortal>
                    <SelectContent attr:class=classes::content position=position side_offset=5.0>
                        <SelectViewport attr:class=classes::viewport>
                            <SelectItem attr:class=classes::item value="one" disabled=true>
                                <SelectItemText>"One"</SelectItemText>
                                <SelectItemIndicator attr:class=classes::indicator>
                                    <TickIcon />
                                </SelectItemIndicator>
                            </SelectItem>
                            <SelectItem attr:class=classes::item value="two">
                                <SelectItemText>"Two"</SelectItemText>
                                <SelectItemIndicator attr:class=classes::indicator>
                                    <TickIcon />
                                </SelectItemIndicator>
                            </SelectItem>
                            <SelectItem attr:class=classes::item value="three">
                                <SelectItemText>"Three"</SelectItemText>
                                <SelectItemIndicator attr:class=classes::indicator>
                                    <TickIcon />
                                </SelectItemIndicator>
                            </SelectItem>
                        </SelectViewport>
                        <SelectArrow />
                    </SelectContent>
                </SelectPortal>
            </Select>
        </Label>
    }
}

#[component]
pub fn NoDefaultValue() -> impl IntoView {
    view! {
        <div style="display: flex; gap: 20px; align-items: center; justify-content: center; height: 100vh;">
            <NoDefaultValueExample position="item-aligned" />
            <NoDefaultValueExample position="popper" />
        </div>
    }
}

#[component]
fn TypeaheadSelectExample(position: &'static str) -> impl IntoView {
    // Static food data — use &'static str tuples so they are Copy and can be used in Fn closures.
    let foods: &[(&str, &str)] = &[
        ("apple", "Apple"),
        ("banana", "Banana"),
        ("blueberry", "Blueberry"),
        ("grapes", "Grapes"),
        ("pineapple", "Pineapple"),
        ("aubergine", "Aubergine"),
        ("broccoli", "Broccoli"),
        ("carrot", "Carrot"),
        ("courgette", "Courgette"),
        ("leek", "Leek"),
        ("beef", "Beef"),
        ("beef-with-sauce", "Beef with sauce"),
        ("chicken", "Chicken"),
        ("lamb", "Lamb"),
        ("pork", "Pork"),
        ("candies", "Candies"),
        ("chocolates", "Chocolates"),
    ];

    view! {
        <Label>
            "Favourite food:"
            <Select default_value="banana">
                <SelectTrigger attr:class=classes::trigger>
                    <SelectValue />
                    <SelectIcon />
                </SelectTrigger>
                <SelectPortal>
                    <SelectContent attr:class=classes::content position=position side_offset=5.0>
                        <SelectScrollUpButton attr:class=scroll_up_button_class()>
                            {"\u{25B2}"}
                        </SelectScrollUpButton>
                        <SelectViewport attr:class=classes::viewport>
                            {foods
                                .iter()
                                .map(|(value, label)| {
                                    view! {
                                        <SelectItem attr:class=classes::item value=value.to_string()>
                                            <SelectItemText>{label.to_string()}</SelectItemText>
                                            <SelectItemIndicator attr:class=classes::indicator>
                                                <TickIcon />
                                            </SelectItemIndicator>
                                        </SelectItem>
                                    }
                                })
                                .collect_view()}
                        </SelectViewport>
                        <SelectScrollDownButton attr:class=scroll_down_button_class()>
                            {"\u{25BC}"}
                        </SelectScrollDownButton>
                        <SelectArrow />
                    </SelectContent>
                </SelectPortal>
            </Select>
        </Label>
    }
}

#[component]
pub fn Typeahead() -> impl IntoView {
    view! {
        <div style="display: flex; gap: 20px; align-items: center; justify-content: center; height: 300vh;">
            <TypeaheadSelectExample position="item-aligned" />
            <TypeaheadSelectExample position="popper" />
        </div>
    }
}

/// Single food group for the static data approach
struct StaticFoodItem {
    value: &'static str,
    label: &'static str,
    disabled: bool,
}

struct StaticFoodGroup {
    label: Option<&'static str>,
    items: &'static [StaticFoodItem],
    show_separator: bool,
}

#[component]
fn FoodGroupView(group: &'static StaticFoodGroup) -> impl IntoView {
    let has_label = group.label.is_some();
    view! {
        <SelectGroup attr:class=classes::group>
            {group.label.map(|label| {
                view! {
                    <SelectLabel attr:class=classes::label>{label}</SelectLabel>
                }
            })}
            {group
                .items
                .iter()
                .map(|food| {
                    let item_class = if has_label {
                        format!("{} {}", classes::itemInGroup, classes::item)
                    } else {
                        classes::item.to_string()
                    };
                    view! {
                        <SelectItem
                            attr:class=item_class
                            value=food.value.to_string()
                            disabled=food.disabled
                        >
                            <SelectItemText>{food.label}</SelectItemText>
                            <SelectItemIndicator attr:class=classes::indicator>
                                <TickIcon />
                            </SelectItemIndicator>
                        </SelectItem>
                    }
                })
                .collect_view()}
        </SelectGroup>
        {group.show_separator.then(|| {
            view! { <SelectSeparator attr:class=classes::separator /> }
        })}
    }
}

static FOOD_GROUPS: &[StaticFoodGroup] = &[
    StaticFoodGroup {
        label: Some("Fruits"),
        items: &[
            StaticFoodItem {
                value: "apple",
                label: "Apple",
                disabled: false,
            },
            StaticFoodItem {
                value: "banana",
                label: "Banana",
                disabled: false,
            },
            StaticFoodItem {
                value: "blueberry",
                label: "Blueberry",
                disabled: false,
            },
            StaticFoodItem {
                value: "grapes",
                label: "Grapes",
                disabled: false,
            },
            StaticFoodItem {
                value: "pineapple",
                label: "Pineapple",
                disabled: false,
            },
        ],
        show_separator: true,
    },
    StaticFoodGroup {
        label: Some("Vegetables"),
        items: &[
            StaticFoodItem {
                value: "aubergine",
                label: "Aubergine",
                disabled: false,
            },
            StaticFoodItem {
                value: "broccoli",
                label: "Broccoli",
                disabled: false,
            },
            StaticFoodItem {
                value: "carrot",
                label: "Carrot",
                disabled: false,
            },
            StaticFoodItem {
                value: "courgette",
                label: "Courgette",
                disabled: false,
            },
            StaticFoodItem {
                value: "leek",
                label: "Leek",
                disabled: false,
            },
        ],
        show_separator: true,
    },
    StaticFoodGroup {
        label: Some("Meat"),
        items: &[
            StaticFoodItem {
                value: "beef",
                label: "Beef",
                disabled: false,
            },
            StaticFoodItem {
                value: "beef-with-sauce",
                label: "Beef with sauce",
                disabled: false,
            },
            StaticFoodItem {
                value: "chicken",
                label: "Chicken",
                disabled: false,
            },
            StaticFoodItem {
                value: "lamb",
                label: "Lamb",
                disabled: false,
            },
            StaticFoodItem {
                value: "pork",
                label: "Pork",
                disabled: false,
            },
        ],
        show_separator: true,
    },
    StaticFoodGroup {
        label: None,
        items: &[
            StaticFoodItem {
                value: "candies",
                label: "Candies",
                disabled: false,
            },
            StaticFoodItem {
                value: "chocolates",
                label: "Chocolates",
                disabled: false,
            },
        ],
        show_separator: false,
    },
];

#[component]
fn WithGroupsSelectExample(position: &'static str) -> impl IntoView {
    view! {
        <Label>
            "Favourite food:"
            <Select default_value="banana">
                <SelectTrigger attr:class=classes::trigger>
                    <SelectValue />
                    <SelectIcon />
                </SelectTrigger>
                <SelectPortal>
                    <SelectContent attr:class=classes::content position=position side_offset=5.0>
                        <SelectScrollUpButton attr:class=scroll_up_button_class()>
                            {"\u{25B2}"}
                        </SelectScrollUpButton>
                        <SelectViewport attr:class=classes::viewport>
                            {FOOD_GROUPS
                                .iter()
                                .map(|group| {
                                    view! { <FoodGroupView group=group /> }
                                })
                                .collect_view()}
                        </SelectViewport>
                        <SelectScrollDownButton attr:class=scroll_down_button_class()>
                            {"\u{25BC}"}
                        </SelectScrollDownButton>
                        <SelectArrow />
                    </SelectContent>
                </SelectPortal>
            </Select>
        </Label>
    }
}

#[component]
pub fn WithGroups() -> impl IntoView {
    view! {
        <div style="display: flex; gap: 20px; align-items: center; justify-content: center; height: 300vh;">
            <WithGroupsSelectExample position="item-aligned" />
            <WithGroupsSelectExample position="popper" />
        </div>
    }
}

#[component]
pub fn Labelling() -> impl IntoView {
    view! {
        <div style="padding: 50px;">
            <h1>"`Label` wrapping"</h1>
            <Label>
                "What is your age?"
                <Select default_value="18-40">
                    <SelectTrigger attr:class=classes::trigger>
                        <SelectValue />
                        <SelectIcon />
                    </SelectTrigger>
                    <SelectPortal>
                        <SelectContent attr:class=classes::content>
                            <SelectViewport attr:class=classes::viewport>
                                <SelectItem attr:class=classes::item value="0-18">
                                    <SelectItemText>"0 to 18"</SelectItemText>
                                    <SelectItemIndicator attr:class=classes::indicator>
                                        <TickIcon />
                                    </SelectItemIndicator>
                                </SelectItem>
                                <SelectItem attr:class=classes::item value="18-40">
                                    <SelectItemText>"18 to 40"</SelectItemText>
                                    <SelectItemIndicator attr:class=classes::indicator>
                                        <TickIcon />
                                    </SelectItemIndicator>
                                </SelectItem>
                                <SelectItem attr:class=classes::item value="40+">
                                    <SelectItemText>"Over 40"</SelectItemText>
                                    <SelectItemIndicator attr:class=classes::indicator>
                                        <TickIcon />
                                    </SelectItemIndicator>
                                </SelectItem>
                            </SelectViewport>
                        </SelectContent>
                    </SelectPortal>
                </Select>
            </Label>

            <h1>"`Label` with `html_for`"</h1>
            <Label attr:r#for="age-label">"What is your age?"</Label>
            <Select default_value="18-40">
                <SelectTrigger attr:class=classes::trigger attr:id="age-label">
                    <SelectValue />
                    <SelectIcon />
                </SelectTrigger>
                <SelectPortal>
                    <SelectContent attr:class=classes::content>
                        <SelectViewport attr:class=classes::viewport>
                            <SelectItem attr:class=classes::item value="0-18">
                                <SelectItemText>"0 to 18"</SelectItemText>
                                <SelectItemIndicator attr:class=classes::indicator>
                                    <TickIcon />
                                </SelectItemIndicator>
                            </SelectItem>
                            <SelectItem attr:class=classes::item value="18-40">
                                <SelectItemText>"18 to 40"</SelectItemText>
                                <SelectItemIndicator attr:class=classes::indicator>
                                    <TickIcon />
                                </SelectItemIndicator>
                            </SelectItem>
                            <SelectItem attr:class=classes::item value="40+">
                                <SelectItemText>"Over 40"</SelectItemText>
                                <SelectItemIndicator attr:class=classes::indicator>
                                    <TickIcon />
                                </SelectItemIndicator>
                            </SelectItem>
                        </SelectViewport>
                    </SelectContent>
                </SelectPortal>
            </Select>

            <h1>"`aria-labelledby`"</h1>
            <div id="age-aria-labelledby">"What is your age?"</div>
            <Select default_value="18-40">
                <SelectTrigger attr:class=classes::trigger attr:aria-labelledby="age-aria-labelledby">
                    <SelectValue />
                    <SelectIcon />
                </SelectTrigger>
                <SelectPortal>
                    <SelectContent attr:class=classes::content>
                        <SelectViewport attr:class=classes::viewport>
                            <SelectItem attr:class=classes::item value="0-18">
                                <SelectItemText>"0 to 18"</SelectItemText>
                                <SelectItemIndicator attr:class=classes::indicator>
                                    <TickIcon />
                                </SelectItemIndicator>
                            </SelectItem>
                            <SelectItem attr:class=classes::item value="18-40">
                                <SelectItemText>"18 to 40"</SelectItemText>
                                <SelectItemIndicator attr:class=classes::indicator>
                                    <TickIcon />
                                </SelectItemIndicator>
                            </SelectItem>
                            <SelectItem attr:class=classes::item value="40+">
                                <SelectItemText>"Over 40"</SelectItemText>
                                <SelectItemIndicator attr:class=classes::indicator>
                                    <TickIcon />
                                </SelectItemIndicator>
                            </SelectItem>
                        </SelectViewport>
                    </SelectContent>
                </SelectPortal>
            </Select>

            <h1>"`aria-label`"</h1>
            <Select default_value="18-40">
                <SelectTrigger attr:class=classes::trigger attr:aria-label="What is your age?">
                    <SelectValue />
                    <SelectIcon />
                </SelectTrigger>
                <SelectPortal>
                    <SelectContent attr:class=classes::content>
                        <SelectViewport attr:class=classes::viewport>
                            <SelectItem attr:class=classes::item value="0-18">
                                <SelectItemText>"0 to 18"</SelectItemText>
                                <SelectItemIndicator attr:class=classes::indicator>
                                    <TickIcon />
                                </SelectItemIndicator>
                            </SelectItem>
                            <SelectItem attr:class=classes::item value="18-40">
                                <SelectItemText>"18 to 40"</SelectItemText>
                                <SelectItemIndicator attr:class=classes::indicator>
                                    <TickIcon />
                                </SelectItemIndicator>
                            </SelectItem>
                            <SelectItem attr:class=classes::item value="40+">
                                <SelectItemText>"Over 40"</SelectItemText>
                                <SelectItemIndicator attr:class=classes::indicator>
                                    <TickIcon />
                                </SelectItemIndicator>
                            </SelectItem>
                        </SelectViewport>
                    </SelectContent>
                </SelectPortal>
            </Select>
        </div>
    }
}

#[component]
fn RtlSelectExample(position: &'static str) -> impl IntoView {
    view! {
        <Label>
            {"\u{0627}\u{062E}\u{062A}\u{0631} \u{0641}\u{0627}\u{0643}\u{0647}\u{0629}:"}
            <Select default_value="two" dir=pith_ui::direction::Direction::Rtl>
                <SelectTrigger attr:class=classes::trigger>
                    <SelectValue />
                    <SelectIcon />
                </SelectTrigger>
                <SelectPortal>
                    <SelectContent attr:class=classes::content position=position side_offset=5.0>
                        <SelectViewport attr:class=classes::viewport>
                            <SelectItem attr:class=classes::item value="one">
                                <SelectItemText>
                                    {"\u{062A}\u{0641}\u{0627}\u{062D}"}
                                    <span aria-hidden="true">" \u{1F34E}"</span>
                                </SelectItemText>
                                <SelectItemIndicator attr:class=classes::indicator>
                                    <TickIcon />
                                </SelectItemIndicator>
                            </SelectItem>
                            <SelectItem attr:class=classes::item value="two">
                                <SelectItemText>
                                    {"\u{062D}\u{0641}\u{0646}\u{0629} \u{0645}\u{0646} \u{0627}\u{0644}\u{0645}\u{0648}\u{0632}"}
                                    <span aria-hidden="true">" \u{1F34C}"</span>
                                </SelectItemText>
                                <SelectItemIndicator attr:class=classes::indicator>
                                    <TickIcon />
                                </SelectItemIndicator>
                            </SelectItem>
                            <SelectItem attr:class=classes::item value="three">
                                <SelectItemText>
                                    {"\u{0627}\u{0644}\u{0641}\u{0631}\u{0627}\u{0648}\u{0644}\u{0629}"}
                                    <span aria-hidden="true">" \u{1F353}"</span>
                                </SelectItemText>
                                <SelectItemIndicator attr:class=classes::indicator>
                                    <TickIcon />
                                </SelectItemIndicator>
                            </SelectItem>
                        </SelectViewport>
                        <SelectArrow />
                    </SelectContent>
                </SelectPortal>
            </Select>
        </Label>
    }
}

#[component]
pub fn RightToLeft() -> impl IntoView {
    view! {
        <div style="display: flex; gap: 20px; padding: 50px;" dir="rtl">
            <RtlSelectExample position="item-aligned" />
            <RtlSelectExample position="popper" />
        </div>
    }
}

#[component]
fn FormSelectContent() -> impl IntoView {
    view! {
        <SelectTrigger attr:class=classes::trigger>
            <SelectValue />
            <SelectIcon />
        </SelectTrigger>
        <SelectPortal>
            <SelectContent attr:class=classes::content>
                <SelectViewport attr:class=classes::viewport>
                    <SelectItem attr:class=classes::item value="fr">
                        <SelectItemText>"France"</SelectItemText>
                        <SelectItemIndicator attr:class=classes::indicator>
                            <TickIcon />
                        </SelectItemIndicator>
                    </SelectItem>
                    <SelectItem attr:class=classes::item value="uk">
                        <SelectItemText>"United Kingdom"</SelectItemText>
                        <SelectItemIndicator attr:class=classes::indicator>
                            <TickIcon />
                        </SelectItemIndicator>
                    </SelectItem>
                    <SelectItem attr:class=classes::item value="es">
                        <SelectItemText>"Spain"</SelectItemText>
                        <SelectItemIndicator attr:class=classes::indicator>
                            <TickIcon />
                        </SelectItemIndicator>
                    </SelectItem>
                </SelectViewport>
            </SelectContent>
        </SelectPortal>
    }
}

fn read_form_data(form_el: &web_sys::HtmlFormElement) -> String {
    let form_data = web_sys::FormData::new_with_form(form_el).unwrap();
    let name = form_data.get("name").as_string().unwrap_or_default();
    let country = form_data.get("country").as_string().unwrap_or_default();
    // Match React's JSON.stringify(Object.fromEntries(formData.entries()), null, 2)
    format!("{{\n  \"name\": \"{name}\",\n  \"country\": \"{country}\"\n}}")
}

fn handle_form_change(set_data: WriteSignal<String>) -> impl Fn(leptos::ev::Event) + 'static {
    move |ev: leptos::ev::Event| {
        // Use current_target to get the form element (like React's event.currentTarget)
        if let Some(form_el) = ev.current_target() {
            let form_el: web_sys::HtmlFormElement = form_el.unchecked_into();
            set_data.set(read_form_data(&form_el));
        }
    }
}

#[component]
pub fn WithinForm() -> impl IntoView {
    let (data, set_data) = signal("{}".to_string());
    let on_change = handle_form_change(set_data);
    let on_input = handle_form_change(set_data);

    view! {
        <form
            style="padding: 50px;"
            on:submit=move |ev: leptos::ev::SubmitEvent| {
                ev.prevent_default();
                let form_el: web_sys::HtmlFormElement = ev.current_target().unwrap().unchecked_into();
                set_data.set(read_form_data(&form_el));
            }
            on:change=on_change
            on:input=on_input
        >
            <Label attr:style="display: block;">
                "Name"
                <input name="name" autocomplete="name" style="display: block;" />
            </Label>
            <br />
            <Label attr:style="display: block;">
                "Country"
                <Select name="country" auto_complete="country" default_value="fr">
                    <FormSelectContent />
                </Select>
            </Label>
            <br />
            <button type="submit">"Submit"</button>
            <br />
            <pre>{move || data.get()}</pre>
        </form>
    }
}

#[component]
pub fn WithinDialog() -> impl IntoView {
    use pith_ui::dialog::*;

    view! {
        <div style="height: 120vh;">
            <Dialog>
                <DialogTrigger>"Open Dialog"</DialogTrigger>
                <DialogPortal>
                    <DialogOverlay />
                    <DialogContent attr:style="position: fixed; top: 100px; background: white; padding: 20px; border-radius: 8px; box-shadow: 0 10px 40px rgba(0,0,0,0.2);">
                        <DialogTitle>"A select in a dialog"</DialogTitle>
                        <Label>
                            "Choose a number:"
                            <Select default_value="2">
                                <SelectTrigger attr:class=classes::trigger>
                                    <SelectValue />
                                    <SelectIcon />
                                </SelectTrigger>
                                <SelectPortal>
                                    <SelectContent attr:class=classes::content>
                                        <SelectScrollUpButton attr:class=scroll_up_button_class()>
                                            {"\u{25B2}"}
                                        </SelectScrollUpButton>
                                        <SelectViewport attr:class=classes::viewport>
                                            {(0..30)
                                                .map(|i| {
                                                    view! {
                                                        <SelectItem
                                                            attr:class=classes::item
                                                            value=i.to_string()
                                                        >
                                                            <SelectItemText>
                                                                {format!("Item {i}")}
                                                            </SelectItemText>
                                                            <SelectItemIndicator attr:class=classes::indicator>
                                                                <TickIcon />
                                                            </SelectItemIndicator>
                                                        </SelectItem>
                                                    }
                                                })
                                                .collect_view()}
                                        </SelectViewport>
                                        <SelectScrollDownButton attr:class=scroll_down_button_class()>
                                            {"\u{25BC}"}
                                        </SelectScrollDownButton>
                                    </SelectContent>
                                </SelectPortal>
                            </Select>
                        </Label>
                        <DialogClose>"Close Dialog"</DialogClose>
                    </DialogContent>
                </DialogPortal>
            </Dialog>
        </div>
    }
}

#[component]
pub fn Cypress() -> impl IntoView {
    let (model, set_model) = signal(Some(String::new()));
    let (form_data, set_form_data) = signal(String::new());

    view! {
        <>
            <form
                style="padding: 50px;"
                on:submit=move |ev: leptos::ev::SubmitEvent| {
                    ev.prevent_default();
                    let form_el: web_sys::HtmlFormElement = ev.target().unwrap().unchecked_into();
                    let fd = web_sys::FormData::new_with_form(&form_el).unwrap();
                    let size = fd.get("size").as_string().unwrap_or_default();
                    set_form_data.set(size);
                }
            >
                <Label>
                    "choose a size:"
                    <Select default_value="M" name="size">
                        <SelectTrigger attr:class=classes::trigger>
                            <SelectValue />
                            <SelectIcon />
                        </SelectTrigger>
                        <SelectPortal>
                            <SelectContent attr:class=classes::content>
                                <SelectViewport attr:class=classes::viewport>
                                    <SelectItem attr:class=classes::item value="S">
                                        <SelectItemText>"Small"</SelectItemText>
                                        <SelectItemIndicator attr:class=classes::indicator>
                                            <TickIcon />
                                        </SelectItemIndicator>
                                    </SelectItem>
                                    <SelectItem attr:class=classes::item value="M">
                                        <SelectItemText>"Medium"</SelectItemText>
                                        <SelectItemIndicator attr:class=classes::indicator>
                                            <TickIcon />
                                        </SelectItemIndicator>
                                    </SelectItem>
                                    <SelectItem attr:class=classes::item value="L">
                                        <SelectItemText>"Large"</SelectItemText>
                                        <SelectItemIndicator attr:class=classes::indicator>
                                            <TickIcon />
                                        </SelectItemIndicator>
                                    </SelectItem>
                                </SelectViewport>
                            </SelectContent>
                        </SelectPortal>
                    </Select>
                </Label>
                <button type="submit" style="width: 100px; height: 50px;">
                    "buy"
                </button>
                {move || {
                    let data = form_data.get();
                    (!data.is_empty())
                        .then(|| {
                            view! { <p>{format!("You picked t-shirt size {data}")}</p> }
                        })
                }}
            </form>

            <hr />

            <div style="padding: 50px;">
                <Label>
                    "choose a model"
                    <Select
                        name="model"
                        value=Signal::derive(move || model.get())
                        on_value_change=Callback::new(move |v: String| set_model.set(Some(v)))
                    >
                        <SelectTrigger attr:class=classes::trigger>
                            <SelectValue placeholder="\u{2026}" />
                            <SelectIcon />
                        </SelectTrigger>
                        <SelectPortal>
                            <SelectContent attr:class=classes::content>
                                <SelectViewport attr:class=classes::viewport>
                                    <SelectItem attr:class=classes::item value="S">
                                        <SelectItemText>"Model S"</SelectItemText>
                                        <SelectItemIndicator attr:class=classes::indicator>
                                            <TickIcon />
                                        </SelectItemIndicator>
                                    </SelectItem>
                                    <SelectItem attr:class=classes::item value="3">
                                        <SelectItemText>"Model 3"</SelectItemText>
                                        <SelectItemIndicator attr:class=classes::indicator>
                                            <TickIcon />
                                        </SelectItemIndicator>
                                    </SelectItem>
                                    <SelectItem attr:class=classes::item value="X">
                                        <SelectItemText>"Model X"</SelectItemText>
                                        <SelectItemIndicator attr:class=classes::indicator>
                                            <TickIcon />
                                        </SelectItemIndicator>
                                    </SelectItem>
                                    <SelectItem attr:class=classes::item value="Y">
                                        <SelectItemText>"Model Y"</SelectItemText>
                                        <SelectItemIndicator attr:class=classes::indicator>
                                            <TickIcon />
                                        </SelectItemIndicator>
                                    </SelectItem>
                                </SelectViewport>
                            </SelectContent>
                        </SelectPortal>
                    </Select>
                </Label>
                <button
                    type="button"
                    style="width: 100px; height: 50px;"
                    on:click=move |_| set_model.set(Some(String::new()))
                >
                    "unset"
                </button>
            </div>
        </>
    }
}

#[component]
pub fn DisabledWithinForm() -> impl IntoView {
    let (data, set_data) = signal("{}".to_string());
    let on_change = handle_form_change(set_data);
    let on_input = handle_form_change(set_data);

    view! {
        <form
            style="padding: 50px;"
            on:submit=move |ev: leptos::ev::SubmitEvent| {
                ev.prevent_default();
                let form_el: web_sys::HtmlFormElement = ev.current_target().unwrap().unchecked_into();
                set_data.set(read_form_data(&form_el));
            }
            on:change=on_change
            on:input=on_input
        >
            <Label attr:style="display: block;">
                "Name"
                <input name="name" autocomplete="name" style="display: block;" />
            </Label>
            <br />
            <Label attr:style="display: block;">
                "Country"
                <Select name="country" auto_complete="country" default_value="fr" disabled=true>
                    <FormSelectContent />
                </Select>
            </Label>
            <br />
            <button type="submit">"Submit"</button>
            <br />
            <pre>{move || data.get()}</pre>
        </form>
    }
}

#[component]
pub fn RequiredWithinForm() -> impl IntoView {
    let (data, set_data) = signal("{}".to_string());
    let on_change = handle_form_change(set_data);
    let on_input = handle_form_change(set_data);

    view! {
        <form
            style="padding: 50px;"
            on:submit=move |ev: leptos::ev::SubmitEvent| {
                ev.prevent_default();
                let form_el: web_sys::HtmlFormElement = ev.current_target().unwrap().unchecked_into();
                set_data.set(read_form_data(&form_el));
            }
            on:change=on_change
            on:input=on_input
        >
            <Label attr:style="display: block;">
                "Name"
                <input name="name" autocomplete="name" style="display: block;" />
            </Label>
            <br />
            <Label attr:style="display: block;">
                "Country"
                <Select required=true name="country" auto_complete="country">
                    <SelectTrigger attr:class=classes::trigger>
                        <SelectValue placeholder="Pick an option" />
                        <SelectIcon />
                    </SelectTrigger>
                    <SelectPortal>
                        <SelectContent attr:class=classes::content>
                            <SelectViewport attr:class=classes::viewport>
                                <SelectItem attr:class=classes::item value="fr">
                                    <SelectItemText>"France"</SelectItemText>
                                    <SelectItemIndicator attr:class=classes::indicator>
                                        <TickIcon />
                                    </SelectItemIndicator>
                                </SelectItem>
                                <SelectItem attr:class=classes::item value="uk">
                                    <SelectItemText>"United Kingdom"</SelectItemText>
                                    <SelectItemIndicator attr:class=classes::indicator>
                                        <TickIcon />
                                    </SelectItemIndicator>
                                </SelectItem>
                                <SelectItem attr:class=classes::item value="es">
                                    <SelectItemText>"Spain"</SelectItemText>
                                    <SelectItemIndicator attr:class=classes::indicator>
                                        <TickIcon />
                                    </SelectItemIndicator>
                                </SelectItem>
                            </SelectViewport>
                        </SelectContent>
                    </SelectPortal>
                </Select>
            </Label>
            <br />
            <button type="submit">"Submit"</button>
            <br />
            <pre>{move || data.get()}</pre>
        </form>
    }
}

#[component]
pub fn WithVeryLongSelectItems() -> impl IntoView {
    view! {
        <div style="padding-left: 300px;">
            <Label>
                "What is the meaning of life?"
                <Select default_value="1">
                    <SelectTrigger attr:class=classes::trigger>
                        <SelectValue />
                        <SelectIcon />
                    </SelectTrigger>
                    <SelectPortal>
                        <SelectContent attr:class=classes::content>
                            <SelectScrollUpButton attr:class=scroll_up_button_class()>
                                {"\u{25B2}"}
                            </SelectScrollUpButton>
                            <SelectViewport attr:class=classes::viewport>
                                <SelectItem attr:class=classes::item value="0">
                                    <SelectItemText>
                                        "The meaning of life is a complex topic that has been the subject of much philosophical, scientific, and theological speculation, with no definitive answer. The meaning of life can be interpreted in many different ways, depending on individual beliefs, values, and experiences."
                                    </SelectItemText>
                                    <SelectItemIndicator attr:class=classes::indicator>
                                        <TickIcon />
                                    </SelectItemIndicator>
                                </SelectItem>
                                <SelectItem attr:class=classes::item value="1">
                                    <SelectItemText>"42"</SelectItemText>
                                    <SelectItemIndicator attr:class=classes::indicator>
                                        <TickIcon />
                                    </SelectItemIndicator>
                                </SelectItem>
                            </SelectViewport>
                            <SelectScrollDownButton attr:class=scroll_down_button_class()>
                                {"\u{25BC}"}
                            </SelectScrollDownButton>
                        </SelectContent>
                    </SelectPortal>
                </Select>
            </Label>
        </div>
    }
}

// -- Chromatic stories --

#[component]
fn ChromaticSelect(
    #[prop(default = 5)] count: usize,
    #[prop(default = "content")] padded_element: &'static str,
    selected: usize,
    #[prop(into, optional)] style: String,
) -> impl IntoView {
    let content_class: &'static str = if padded_element == "content" {
        classes::contentWithPadding
    } else {
        ""
    };
    let viewport_class: &'static str = if padded_element == "viewport" {
        classes::viewport
    } else {
        ""
    };
    let scroll_up_style = if padded_element == "content" {
        "margin-top: -5px;"
    } else {
        ""
    };
    let scroll_down_style = if padded_element == "content" {
        "margin-bottom: -5px;"
    } else {
        ""
    };
    let style = StoredValue::new(style);

    view! {
        <Select default_value=selected.to_string() open=true>
            <SelectTrigger attr:class=classes::trigger attr:style=move || style.get_value()>
                <SelectValue />
                <SelectIcon />
            </SelectTrigger>
            <SelectPortal>
                <SelectContent attr:class=content_class attr:style="opacity: 0.7;">
                    <SelectScrollUpButton attr:class=scroll_up_button_class() attr:style=scroll_up_style>
                        {"\u{25B2}"}
                    </SelectScrollUpButton>
                    <SelectViewport attr:class=viewport_class>
                        {(0..count)
                            .map(|i| {
                                view! {
                                    <SelectItem attr:class=classes::item value=i.to_string()>
                                        <SelectItemText>{i.to_string()}</SelectItemText>
                                        <SelectItemIndicator attr:class=classes::indicator>
                                            <TickIcon />
                                        </SelectItemIndicator>
                                    </SelectItem>
                                }
                            })
                            .collect_view()}
                    </SelectViewport>
                    <SelectScrollDownButton attr:class=scroll_down_button_class() attr:style=scroll_down_style>
                        {"\u{25BC}"}
                    </SelectScrollDownButton>
                </SelectContent>
            </SelectPortal>
        </Select>
    }
}

#[component]
fn SelectShort(
    #[prop(default = 9)] count: usize,
    #[prop(default = "content")] padded_element: &'static str,
    selected: usize,
    #[prop(into, optional)] style: String,
) -> impl IntoView {
    view! {
        <ChromaticSelect count=count padded_element=padded_element selected=selected style=style />
    }
}

#[component]
fn SelectLong(
    #[prop(default = 50)] count: usize,
    #[prop(default = "content")] padded_element: &'static str,
    selected: usize,
    #[prop(into, optional)] style: String,
) -> impl IntoView {
    view! {
        <ChromaticSelect count=count padded_element=padded_element selected=selected style=style />
    }
}

#[component]
pub fn ChromaticShortOptionsPaddedContent() -> impl IntoView {
    view! {
        <div style="display: grid; grid-template-columns: repeat(5, 1fr); grid-template-rows: repeat(3, 1fr); height: 100vh; place-items: center;">
            <SelectShort padded_element="content" selected=0 style="align-self: start;" />
            <SelectShort padded_element="content" selected=2 style="align-self: start;" />
            <SelectShort padded_element="content" selected=4 style="align-self: start;" />
            <SelectShort padded_element="content" selected=6 style="align-self: start;" />
            <SelectShort padded_element="content" selected=8 style="align-self: start;" />

            <SelectShort padded_element="content" selected=0 />
            <SelectShort padded_element="content" selected=2 />
            <SelectShort padded_element="content" selected=4 />
            <SelectShort padded_element="content" selected=6 />
            <SelectShort padded_element="content" selected=8 />

            <SelectShort padded_element="content" selected=0 style="align-self: end;" />
            <SelectShort padded_element="content" selected=2 style="align-self: end;" />
            <SelectShort padded_element="content" selected=4 style="align-self: end;" />
            <SelectShort padded_element="content" selected=6 style="align-self: end;" />
            <SelectShort padded_element="content" selected=8 style="align-self: end;" />
        </div>
    }
}

#[component]
pub fn ChromaticShortOptionsPaddedViewport() -> impl IntoView {
    view! {
        <div style="display: grid; grid-template-columns: repeat(5, 1fr); grid-template-rows: repeat(3, 1fr); height: 100vh; place-items: center;">
            <SelectShort padded_element="viewport" selected=0 style="align-self: start;" />
            <SelectShort padded_element="viewport" selected=2 style="align-self: start;" />
            <SelectShort padded_element="viewport" selected=4 style="align-self: start;" />
            <SelectShort padded_element="viewport" selected=6 style="align-self: start;" />
            <SelectShort padded_element="viewport" selected=8 style="align-self: start;" />

            <SelectShort padded_element="viewport" selected=0 />
            <SelectShort padded_element="viewport" selected=2 />
            <SelectShort padded_element="viewport" selected=4 />
            <SelectShort padded_element="viewport" selected=6 />
            <SelectShort padded_element="viewport" selected=8 />

            <SelectShort padded_element="viewport" selected=0 style="align-self: end;" />
            <SelectShort padded_element="viewport" selected=2 style="align-self: end;" />
            <SelectShort padded_element="viewport" selected=4 style="align-self: end;" />
            <SelectShort padded_element="viewport" selected=6 style="align-self: end;" />
            <SelectShort padded_element="viewport" selected=8 style="align-self: end;" />
        </div>
    }
}

#[component]
pub fn ChromaticLongOptionsPaddedContent() -> impl IntoView {
    view! {
        <div style="display: grid; grid-template-columns: repeat(15, 1fr); grid-template-rows: repeat(3, 1fr); height: 100vh; place-items: center;">
            <SelectLong padded_element="content" selected=0 style="align-self: start;" />
            <SelectLong padded_element="content" selected=25 style="align-self: start;" />
            <SelectLong padded_element="content" selected=49 style="align-self: start;" />

            <SelectLong padded_element="content" selected=0 style="grid-row: 1; grid-column: 4;" />
            <SelectLong padded_element="content" selected=25 style="grid-row: 1; grid-column: 5;" />
            <SelectLong padded_element="content" selected=49 style="grid-row: 1; grid-column: 6;" />

            <SelectLong padded_element="content" selected=0 style="grid-row: 2; grid-column: 7;" />
            <SelectLong padded_element="content" selected=25 style="grid-row: 2; grid-column: 8;" />
            <SelectLong padded_element="content" selected=49 style="grid-row: 2; grid-column: 9;" />

            <SelectLong padded_element="content" selected=0 style="grid-row: 3; grid-column: 10;" />
            <SelectLong padded_element="content" selected=25 style="grid-row: 3; grid-column: 11;" />
            <SelectLong padded_element="content" selected=49 style="grid-row: 3; grid-column: 12;" />

            <SelectLong padded_element="content" selected=0 style="grid-row: 3; grid-column: 13; align-self: end;" />
            <SelectLong padded_element="content" selected=25 style="grid-row: 3; grid-column: 14; align-self: end;" />
            <SelectLong padded_element="content" selected=49 style="grid-row: 3; grid-column: 15; align-self: end;" />
        </div>
    }
}

#[component]
pub fn ChromaticLongOptionsPaddedViewport() -> impl IntoView {
    view! {
        <div style="display: grid; grid-template-columns: repeat(15, 1fr); grid-template-rows: repeat(3, 1fr); height: 100vh; place-items: center;">
            <SelectLong padded_element="viewport" selected=0 style="align-self: start;" />
            <SelectLong padded_element="viewport" selected=25 style="align-self: start;" />
            <SelectLong padded_element="viewport" selected=49 style="align-self: start;" />

            <SelectLong padded_element="viewport" selected=0 style="grid-row: 1; grid-column: 4;" />
            <SelectLong padded_element="viewport" selected=25 style="grid-row: 1; grid-column: 5;" />
            <SelectLong padded_element="viewport" selected=49 style="grid-row: 1; grid-column: 6;" />

            <SelectLong padded_element="viewport" selected=0 style="grid-row: 2; grid-column: 7;" />
            <SelectLong padded_element="viewport" selected=25 style="grid-row: 2; grid-column: 8;" />
            <SelectLong padded_element="viewport" selected=49 style="grid-row: 2; grid-column: 9;" />

            <SelectLong padded_element="viewport" selected=0 style="grid-row: 3; grid-column: 10;" />
            <SelectLong padded_element="viewport" selected=25 style="grid-row: 3; grid-column: 11;" />
            <SelectLong padded_element="viewport" selected=49 style="grid-row: 3; grid-column: 12;" />

            <SelectLong padded_element="viewport" selected=0 style="grid-row: 3; grid-column: 13; align-self: end;" />
            <SelectLong padded_element="viewport" selected=25 style="grid-row: 3; grid-column: 14; align-self: end;" />
            <SelectLong padded_element="viewport" selected=49 style="grid-row: 3; grid-column: 15; align-self: end;" />
        </div>
    }
}

#[component]
pub fn ChromaticTopFirstPaddedContent() -> impl IntoView {
    view! {
        <div style="display: flex; height: 100vh;">
            <SelectShort padded_element="content" selected=0 />
        </div>
    }
}

#[component]
pub fn ChromaticTopFirstPaddedViewport() -> impl IntoView {
    view! {
        <div style="display: flex; height: 100vh;">
            <SelectShort padded_element="viewport" selected=0 />
        </div>
    }
}

#[component]
pub fn ChromaticBottomLastPaddedContent() -> impl IntoView {
    view! {
        <div style="display: flex; height: 100vh; align-items: flex-end;">
            <SelectShort padded_element="content" selected=8 />
        </div>
    }
}

#[component]
pub fn ChromaticBottomLastPaddedViewport() -> impl IntoView {
    view! {
        <div style="display: flex; height: 100vh; align-items: flex-end;">
            <SelectShort padded_element="viewport" selected=8 />
        </div>
    }
}

#[component]
pub fn ChromaticNoDefaultValue() -> impl IntoView {
    view! {
        <div style="display: grid; height: 100vh; place-items: center; grid-template-columns: repeat(2, 1fr);">
            <Select open=true>
                <SelectTrigger attr:class=classes::trigger>
                    <SelectValue />
                    <SelectIcon />
                </SelectTrigger>
                <SelectPortal>
                    <SelectContent attr:class=classes::content attr:style="opacity: 0.7;">
                        <SelectScrollUpButton attr:class=scroll_up_button_class()>
                            {"\u{25B2}"}
                        </SelectScrollUpButton>
                        <SelectViewport attr:class=classes::viewport>
                            {(0..10)
                                .map(|i| {
                                    view! {
                                        <SelectItem attr:class=classes::item value=i.to_string() disabled=i < 5>
                                            <SelectItemText>{i.to_string()}</SelectItemText>
                                            <SelectItemIndicator attr:class=classes::indicator>
                                                <TickIcon />
                                            </SelectItemIndicator>
                                        </SelectItem>
                                    }
                                })
                                .collect_view()}
                        </SelectViewport>
                        <SelectScrollDownButton attr:class=scroll_down_button_class()>
                            {"\u{25BC}"}
                        </SelectScrollDownButton>
                    </SelectContent>
                </SelectPortal>
            </Select>

            <Select open=true>
                <SelectTrigger attr:class=classes::trigger>
                    <SelectValue placeholder="Pick an option" />
                    <SelectIcon />
                </SelectTrigger>
                <SelectPortal>
                    <SelectContent attr:class=classes::content attr:style="opacity: 0.7;">
                        <SelectScrollUpButton attr:class=scroll_up_button_class()>
                            {"\u{25B2}"}
                        </SelectScrollUpButton>
                        <SelectViewport attr:class=classes::viewport>
                            {(0..10)
                                .map(|i| {
                                    view! {
                                        <SelectItem attr:class=classes::item value=i.to_string() disabled=i < 5>
                                            <SelectItemText>{i.to_string()}</SelectItemText>
                                            <SelectItemIndicator attr:class=classes::indicator>
                                                <TickIcon />
                                            </SelectItemIndicator>
                                        </SelectItem>
                                    }
                                })
                                .collect_view()}
                        </SelectViewport>
                        <SelectScrollDownButton attr:class=scroll_down_button_class()>
                            {"\u{25BC}"}
                        </SelectScrollDownButton>
                    </SelectContent>
                </SelectPortal>
            </Select>
        </div>
    }
}
