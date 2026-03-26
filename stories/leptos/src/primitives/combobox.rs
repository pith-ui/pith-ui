use std::sync::Arc;

use leptos::prelude::*;
use pith_ui::combobox::*;

stylance::import_crate_style!(classes, "src/primitives/combobox.stories.module.css");

const FRUITS: &[&str] = &[
    "Apple",
    "Avocado",
    "Banana",
    "Blueberry",
    "Cherry",
    "Grape",
    "Kiwi",
    "Lemon",
    "Mango",
    "Orange",
    "Peach",
    "Pear",
    "Strawberry",
];
const VEGETABLES: &[&str] = &[
    "Artichoke",
    "Broccoli",
    "Carrot",
    "Celery",
    "Eggplant",
    "Lettuce",
    "Potato",
    "Spinach",
    "Tomato",
    "Zucchini",
];

fn filter(items: &[&str], query: &str) -> Vec<String> {
    if query.is_empty() {
        items.iter().map(|s| s.to_string()).collect()
    } else {
        let q = query.to_lowercase();
        items
            .iter()
            .filter(|i| i.to_lowercase().contains(&q))
            .map(|s| s.to_string())
            .collect()
    }
}

#[component]
fn TickIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32" width="12" height="12"
            fill="none" stroke="currentcolor" stroke-linecap="round" stroke-linejoin="round" stroke-width="3">
            <path d="M2 20 L12 28 30 4" />
        </svg>
    }
}

fn render_items(items: Vec<String>, disabled_item: Option<&str>) -> impl IntoView {
    items.into_iter().map(|item| {
        let is_disabled = disabled_item.is_some_and(|d| d == item);
        let text = item.clone();
        let label = StoredValue::new(item.clone());
        view! {
            <ComboboxItem attr:class=classes::item value=item text_value=text disabled=is_disabled>
                <ComboboxItemIndicator attr:class=classes::indicator><TickIcon /></ComboboxItemIndicator>
                {move || label.get_value()}
            </ComboboxItem>
        }
    }).collect_view()
}

/* -------------------------------------------------------------------------------------------------
 * Styled — Basic single-select combobox
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn Styled() -> impl IntoView {
    let (value, set_value) = signal(Option::<String>::None);
    let (input_value, set_input_value) = signal(String::new());

    let filtered = Memo::new(move |_| filter(FRUITS, &input_value.get()));

    view! {
        <div class=classes::root>
            <h2>"Combobox"</h2>
            <Combobox
                value=Signal::derive(move || value.get())
                on_value_change=Callback::new(move |v: String| {
                    set_value.set(Some(v.clone()));
                    set_input_value.set(v);
                })
                input_value=Signal::derive(move || input_value.get())
                on_input_value_change=Callback::new(move |v: String| set_input_value.set(v))
            >
                <ComboboxAnchor attr:class=classes::anchor>
                    <ComboboxInput attr:class=classes::input placeholder="Select a fruit..." />
                    <ComboboxClear attr:class=classes::clear attr:aria-label="Clear">"✕"</ComboboxClear>
                    <ComboboxTrigger attr:class=classes::trigger attr:aria-label="Toggle">
                        <ComboboxIcon />
                    </ComboboxTrigger>
                </ComboboxAnchor>
                <ComboboxPortal>
                    <ComboboxContent attr:class=classes::content side_offset=4.0>
                        <ComboboxViewport attr:class=classes::viewport>
                            {move || {
                                let items = filtered.get();
                                if items.is_empty() {
                                    return leptos::either::Either::Left(view! {
                                        <ComboboxEmpty attr:class=classes::empty>"No results found"</ComboboxEmpty>
                                    });
                                }
                                leptos::either::Either::Right(render_items(items, Some("Cherry")))
                            }}
                        </ComboboxViewport>
                    </ComboboxContent>
                </ComboboxPortal>
            </Combobox>
            <p>"Selected: " {move || value.get().unwrap_or("(none)".into())}</p>
        </div>
    }
}

/* -------------------------------------------------------------------------------------------------
 * WithGroups — Grouped items with labels
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn WithGroups() -> impl IntoView {
    let (value, set_value) = signal(Option::<String>::None);
    let (input_value, set_input_value) = signal(String::new());

    let filtered_fruits = Memo::new(move |_| filter(FRUITS, &input_value.get()));
    let filtered_vegs = Memo::new(move |_| filter(VEGETABLES, &input_value.get()));

    view! {
        <div class=classes::root>
            <h2>"With Groups"</h2>
            <Combobox
                value=Signal::derive(move || value.get())
                on_value_change=Callback::new(move |v: String| {
                    set_value.set(Some(v.clone()));
                    set_input_value.set(v);
                })
                input_value=Signal::derive(move || input_value.get())
                on_input_value_change=Callback::new(move |v: String| set_input_value.set(v))
            >
                <ComboboxAnchor attr:class=classes::anchor>
                    <ComboboxInput attr:class=classes::input placeholder="Search produce..." />
                    <ComboboxTrigger attr:class=classes::trigger attr:aria-label="Toggle">
                        <ComboboxIcon />
                    </ComboboxTrigger>
                </ComboboxAnchor>
                <ComboboxPortal>
                    <ComboboxContent attr:class=classes::content side_offset=4.0>
                        <ComboboxViewport attr:class=classes::viewport>
                            {move || {
                                let fruits = filtered_fruits.get();
                                let vegs = filtered_vegs.get();
                                if fruits.is_empty() && vegs.is_empty() {
                                    return leptos::either::Either::Left(view! {
                                        <ComboboxEmpty attr:class=classes::empty>"No results found"</ComboboxEmpty>
                                    });
                                }
                                leptos::either::Either::Right(view! {
                                    <Show when=move || !filtered_fruits.get().is_empty()>
                                        <ComboboxGroup>
                                            <ComboboxLabel attr:class=classes::label>"Fruits"</ComboboxLabel>
                                        </ComboboxGroup>
                                    </Show>
                                    {render_items(fruits, None)}
                                    <Show when=move || !filtered_fruits.get().is_empty() && !filtered_vegs.get().is_empty()>
                                        <ComboboxSeparator attr:class=classes::separator />
                                    </Show>
                                    <Show when=move || !filtered_vegs.get().is_empty()>
                                        <ComboboxGroup>
                                            <ComboboxLabel attr:class=classes::label>"Vegetables"</ComboboxLabel>
                                        </ComboboxGroup>
                                    </Show>
                                    {render_items(vegs, None)}
                                })
                            }}
                        </ComboboxViewport>
                    </ComboboxContent>
                </ComboboxPortal>
            </Combobox>
            <p>"Selected: " {move || value.get().unwrap_or("(none)".into())}</p>
        </div>
    }
}

/* -------------------------------------------------------------------------------------------------
 * MultiSelect — Multi-select with chips
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn MultiSelect() -> impl IntoView {
    let (values, set_values) = signal(Vec::<String>::new());
    let (input_value, set_input_value) = signal(String::new());

    let all_items: Vec<&str> = FRUITS.iter().chain(VEGETABLES.iter()).copied().collect();
    let all_items = StoredValue::new(all_items);
    let filtered = Memo::new(move |_| {
        all_items
            .try_with_value(|items| filter(items, &input_value.get()))
            .unwrap_or_default()
    });

    view! {
        <div class=classes::root>
            <h2>"Multi-Select"</h2>
            <Combobox
                multiple=true
                values=Signal::derive(move || values.get())
                on_values_change=Callback::new(move |v: Vec<String>| set_values.set(v))
                input_value=Signal::derive(move || input_value.get())
                on_input_value_change=Callback::new(move |v: String| set_input_value.set(v))
            >
                <ComboboxAnchor attr:class=classes::anchor>
                    <ComboboxChips attr:class=classes::chips>
                        {move || values.get().into_iter().enumerate().map(|(i, val)| {
                            let val_display = StoredValue::new(val.clone());
                            let val_remove = StoredValue::new(val.clone());
                            view! {
                                <ComboboxChip attr:class=classes::chip value=val index=i>
                                    {move || val_display.get_value()}
                                    <ComboboxChipRemove
                                        attr:class=classes::chipRemove
                                        value=val_remove.get_value()
                                    />
                                </ComboboxChip>
                            }
                        }).collect_view()}
                    </ComboboxChips>
                    <ComboboxInput attr:class=classes::input placeholder="Search..." />
                    <ComboboxTrigger attr:class=classes::trigger attr:aria-label="Toggle">
                        <ComboboxIcon />
                    </ComboboxTrigger>
                </ComboboxAnchor>
                <ComboboxPortal>
                    <ComboboxContent attr:class=classes::content side_offset=4.0>
                        <ComboboxViewport attr:class=classes::viewport>
                            {move || {
                                let items = filtered.get();
                                if items.is_empty() {
                                    return leptos::either::Either::Left(view! {
                                        <ComboboxEmpty attr:class=classes::empty>"No results found"</ComboboxEmpty>
                                    });
                                }
                                leptos::either::Either::Right(render_items(items, None))
                            }}
                        </ComboboxViewport>
                    </ComboboxContent>
                </ComboboxPortal>
            </Combobox>
            <p>"Selected: " {move || {
                let v = values.get();
                if v.is_empty() { "(none)".to_string() } else { v.join(", ") }
            }}</p>
            <div style="display: flex; gap: 8px; margin-top: 8px;">
                <button>"Before"</button>
                <input placeholder="Tab target" style="padding: 4px 8px;" />
                <button>"After"</button>
            </div>
        </div>
    }
}

/* -------------------------------------------------------------------------------------------------
 * Controlled — Externally controlled value
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn Controlled() -> impl IntoView {
    let (value, set_value) = signal(Some("Banana".to_string()));
    let (input_value, set_input_value) = signal("Banana".to_string());
    let filtered = Memo::new(move |_| filter(FRUITS, &input_value.get()));

    view! {
        <div class=classes::root>
            <h2>"Controlled"</h2>
            <p>"Use the buttons below to set the value externally."</p>
            <div style="display: flex; gap: 8px; margin-bottom: 8px;">
                <button on:click=move |_| {
                    set_value.set(Some("Apple".to_string()));
                    set_input_value.set("Apple".to_string());
                }>"Set Apple"</button>
                <button on:click=move |_| {
                    set_value.set(Some("Mango".to_string()));
                    set_input_value.set("Mango".to_string());
                }>"Set Mango"</button>
                <button on:click=move |_| {
                    set_value.set(None);
                    set_input_value.set(String::new());
                }>"Clear"</button>
            </div>
            <Combobox
                value=Signal::derive(move || value.get())
                on_value_change=Callback::new(move |v: String| {
                    set_value.set(Some(v.clone()));
                    set_input_value.set(v);
                })
                input_value=Signal::derive(move || input_value.get())
                on_input_value_change=Callback::new(move |v: String| set_input_value.set(v))
            >
                <ComboboxAnchor attr:class=classes::anchor>
                    <ComboboxInput attr:class=classes::input placeholder="Select a fruit..." />
                    <ComboboxTrigger attr:class=classes::trigger attr:aria-label="Toggle">
                        <ComboboxIcon />
                    </ComboboxTrigger>
                </ComboboxAnchor>
                <ComboboxPortal>
                    <ComboboxContent attr:class=classes::content side_offset=4.0>
                        <ComboboxViewport attr:class=classes::viewport>
                            {move || render_items(filtered.get(), None)}
                        </ComboboxViewport>
                    </ComboboxContent>
                </ComboboxPortal>
            </Combobox>
            <p>"Selected: " {move || value.get().unwrap_or("(none)".into())}</p>
        </div>
    }
}

/* -------------------------------------------------------------------------------------------------
 * Disabled — Disabled state
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn Disabled() -> impl IntoView {
    view! {
        <div class=classes::root>
            <h2>"Disabled"</h2>
            <Combobox disabled=true>
                <ComboboxAnchor attr:class=classes::anchor>
                    <ComboboxInput attr:class=classes::input placeholder="Cannot interact..." />
                    <ComboboxTrigger attr:class=classes::trigger attr:aria-label="Toggle">
                        <ComboboxIcon />
                    </ComboboxTrigger>
                </ComboboxAnchor>
                <ComboboxPortal>
                    <ComboboxContent attr:class=classes::content side_offset=4.0>
                        <ComboboxViewport attr:class=classes::viewport>
                            {render_items(FRUITS.iter().map(|s| s.to_string()).collect(), None)}
                        </ComboboxViewport>
                    </ComboboxContent>
                </ComboboxPortal>
            </Combobox>
        </div>
    }
}

/* -------------------------------------------------------------------------------------------------
 * WithEmpty — Empty state when filter matches nothing
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn WithEmpty() -> impl IntoView {
    let (input_value, set_input_value) = signal("zzzzz".to_string());
    let filtered = Memo::new(move |_| filter(FRUITS, &input_value.get()));

    view! {
        <div class=classes::root>
            <h2>"Empty State"</h2>
            <p>"Pre-filled with a query that matches nothing."</p>
            <Combobox
                default_open=true
                input_value=Signal::derive(move || input_value.get())
                on_input_value_change=Callback::new(move |v: String| set_input_value.set(v))
            >
                <ComboboxAnchor attr:class=classes::anchor>
                    <ComboboxInput attr:class=classes::input placeholder="Type to search..." />
                    <ComboboxClear attr:class=classes::clear attr:aria-label="Clear">"✕"</ComboboxClear>
                    <ComboboxTrigger attr:class=classes::trigger attr:aria-label="Toggle">
                        <ComboboxIcon />
                    </ComboboxTrigger>
                </ComboboxAnchor>
                <ComboboxPortal>
                    <ComboboxContent attr:class=classes::content side_offset=4.0>
                        <ComboboxViewport attr:class=classes::viewport>
                            {move || {
                                let items = filtered.get();
                                if items.is_empty() {
                                    return leptos::either::Either::Left(view! {
                                        <ComboboxEmpty attr:class=classes::empty>"No results found"</ComboboxEmpty>
                                    });
                                }
                                leptos::either::Either::Right(render_items(items, None))
                            }}
                        </ComboboxViewport>
                    </ComboboxContent>
                </ComboboxPortal>
            </Combobox>
        </div>
    }
}

/* -------------------------------------------------------------------------------------------------
 * AutoHighlight — Auto-highlight first match while typing
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn AutoHighlight() -> impl IntoView {
    let (value, set_value) = signal(Option::<String>::None);
    let (input_value, set_input_value) = signal(String::new());

    let filtered = Memo::new(move |_| filter(FRUITS, &input_value.get()));

    view! {
        <div class=classes::root>
            <h2>"Auto Highlight"</h2>
            <p>"The first matching item is highlighted automatically as you type."</p>
            <Combobox
                auto_highlight=true
                value=Signal::derive(move || value.get())
                on_value_change=Callback::new(move |v: String| {
                    set_value.set(Some(v.clone()));
                    set_input_value.set(v);
                })
                input_value=Signal::derive(move || input_value.get())
                on_input_value_change=Callback::new(move |v: String| set_input_value.set(v))
            >
                <ComboboxAnchor attr:class=classes::anchor>
                    <ComboboxInput attr:class=classes::input placeholder="Type to search..." />
                    <ComboboxClear attr:class=classes::clear attr:aria-label="Clear">"✕"</ComboboxClear>
                    <ComboboxTrigger attr:class=classes::trigger attr:aria-label="Toggle">
                        <ComboboxIcon />
                    </ComboboxTrigger>
                </ComboboxAnchor>
                <ComboboxPortal>
                    <ComboboxContent attr:class=classes::content side_offset=4.0>
                        <ComboboxViewport attr:class=classes::viewport>
                            {move || {
                                let items = filtered.get();
                                if items.is_empty() {
                                    return leptos::either::Either::Left(view! {
                                        <ComboboxEmpty attr:class=classes::empty>"No results found"</ComboboxEmpty>
                                    });
                                }
                                leptos::either::Either::Right(render_items(items, None))
                            }}
                        </ComboboxViewport>
                    </ComboboxContent>
                </ComboboxPortal>
            </Combobox>
            <p>"Selected: " {move || value.get().unwrap_or("(none)".into())}</p>
        </div>
    }
}

/* -------------------------------------------------------------------------------------------------
 * WithClear — Clear button demo
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn WithClear() -> impl IntoView {
    let (value, set_value) = signal(Some("Orange".to_string()));
    let (input_value, set_input_value) = signal("Orange".to_string());
    let filtered = Memo::new(move |_| filter(FRUITS, &input_value.get()));

    view! {
        <div class=classes::root>
            <h2>"With Clear Button"</h2>
            <p>"Select a value, then use the ✕ button to clear it."</p>
            <Combobox
                value=Signal::derive(move || value.get())
                on_value_change=Callback::new(move |v: String| {
                    set_value.set(Some(v.clone()));
                    set_input_value.set(v);
                })
                input_value=Signal::derive(move || input_value.get())
                on_input_value_change=Callback::new(move |v: String| set_input_value.set(v))
            >
                <ComboboxAnchor attr:class=classes::anchor>
                    <ComboboxInput attr:class=classes::input placeholder="Select a fruit..." />
                    <ComboboxClear attr:class=classes::clear attr:aria-label="Clear">"✕"</ComboboxClear>
                    <ComboboxTrigger attr:class=classes::trigger attr:aria-label="Toggle">
                        <ComboboxIcon />
                    </ComboboxTrigger>
                </ComboboxAnchor>
                <ComboboxPortal>
                    <ComboboxContent attr:class=classes::content side_offset=4.0>
                        <ComboboxViewport attr:class=classes::viewport>
                            {move || render_items(filtered.get(), None)}
                        </ComboboxViewport>
                    </ComboboxContent>
                </ComboboxPortal>
            </Combobox>
            <p>"Selected: " {move || value.get().unwrap_or("(none)".into())}</p>
        </div>
    }
}

/* -------------------------------------------------------------------------------------------------
 * Virtualized — Virtual scrolling with 10,000 items
 * -----------------------------------------------------------------------------------------------*/

fn generate_items(count: usize) -> Vec<String> {
    (0..count).map(|i| format!("Item {}", i + 1)).collect()
}

#[component]
pub fn Virtualized() -> impl IntoView {
    let all_items = StoredValue::new(generate_items(10_000));
    let (value, set_value) = signal(Option::<String>::None);
    let (input_value, set_input_value) = signal(String::new());

    let filtered = Memo::new(move |_| {
        all_items.try_with_value(|items| {
            let q = input_value.get();
            if q.is_empty() {
                items.clone()
            } else {
                let q = q.to_lowercase();
                items.iter().filter(|i| i.to_lowercase().contains(&q)).cloned().collect()
            }
        }).unwrap_or_default()
    });

    let render_item = StoredValue::new(Arc::new(move |vi: VirtualItem| {
        let items = filtered.get();
        let Some(item) = items.get(vi.index).cloned() else {
            return ().into_any();
        };
        let text = item.clone();
        let label = StoredValue::new(item.clone());
        view! {
            <ComboboxItem attr:class=classes::item value=item text_value=text>
                <ComboboxItemIndicator attr:class=classes::indicator><TickIcon /></ComboboxItemIndicator>
                {move || label.get_value()}
            </ComboboxItem>
        }.into_any()
    }) as Arc<dyn Fn(VirtualItem) -> AnyView + Send + Sync>);

    view! {
        <div class=classes::root>
            <h2>"Virtualized"</h2>
            <p>"10,000 items with virtual scrolling. Only visible items are rendered."</p>
            <Combobox
                virtualized=true
                auto_highlight=true
                value=Signal::derive(move || value.get())
                on_value_change=Callback::new(move |v: String| {
                    set_value.set(Some(v.clone()));
                    set_input_value.set(v);
                })
                input_value=Signal::derive(move || input_value.get())
                on_input_value_change=Callback::new(move |v: String| set_input_value.set(v))
            >
                <ComboboxAnchor attr:class=classes::anchor>
                    <ComboboxInput attr:class=classes::input placeholder="Search 10,000 items..." />
                    <ComboboxClear attr:class=classes::clear attr:aria-label="Clear">"✕"</ComboboxClear>
                    <ComboboxTrigger attr:class=classes::trigger attr:aria-label="Toggle">
                        <ComboboxIcon />
                    </ComboboxTrigger>
                </ComboboxAnchor>
                <ComboboxPortal>
                    <ComboboxContent attr:class=classes::content side_offset=4.0>
                        <ComboboxViewport attr:class=classes::viewport>
                            {move || {
                                let count = filtered.get().len();
                                if count == 0 {
                                    return leptos::either::Either::Left(view! {
                                        <ComboboxEmpty attr:class=classes::empty>"No results found"</ComboboxEmpty>
                                    });
                                }
                                leptos::either::Either::Right(view! {
                                    <ComboboxVirtualItems
                                        count=Signal::derive(move || filtered.get().len())
                                        estimate_size=32.0
                                        render_item=render_item.get_value()
                                    />
                                })
                            }}
                        </ComboboxViewport>
                    </ComboboxContent>
                </ComboboxPortal>
            </Combobox>
            <p>"Selected: " {move || value.get().unwrap_or("(none)".into())}</p>
            <p style="color: gray; font-size: 12px;">
                {move || format!("Showing {} filtered items", filtered.get().len())}
            </p>
        </div>
    }
}
