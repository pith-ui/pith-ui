use leptos::prelude::*;
use leptos::either::Either;
use pith_ui::combobox::*;
use web_sys::wasm_bindgen::JsCast;

const FRUITS: &[&str] = &["Apple", "Avocado", "Banana", "Cherry", "Grape", "Mango", "Orange", "Pear"];
const VEGETABLES: &[&str] = &["Carrot", "Celery", "Lettuce", "Potato", "Spinach", "Tomato"];

fn filter_items(items: &[&str], query: &str) -> Vec<String> {
    if query.is_empty() {
        items.iter().map(|s| s.to_string()).collect()
    } else {
        let query_lower = query.to_lowercase();
        items
            .iter()
            .filter(|item| item.to_lowercase().contains(&query_lower))
            .map(|s| s.to_string())
            .collect()
    }
}

fn render_fruit_items(fruits: Vec<String>) -> impl IntoView {
    fruits.into_iter().map(|item| {
        let is_cherry = item == "Cherry";
        let text = item.clone();
        let label = item.clone();
        let label = StoredValue::new(label);
        view! {
            <ComboboxItem attr:class="combobox-item" value=item text_value=text disabled=is_cherry>
                <ComboboxItemIndicator attr:class="combobox-indicator">"✓"</ComboboxItemIndicator>
                {move || label.get_value()}
            </ComboboxItem>
        }
    }).collect_view()
}

fn render_veg_items(vegs: Vec<String>) -> impl IntoView {
    vegs.into_iter().map(|item| {
        let text = item.clone();
        let label = item.clone();
        let label = StoredValue::new(label);
        view! {
            <ComboboxItem attr:class="combobox-item" value=item text_value=text>
                <ComboboxItemIndicator attr:class="combobox-indicator">"✓"</ComboboxItemIndicator>
                {move || label.get_value()}
            </ComboboxItem>
        }
    }).collect_view()
}

#[component]
pub fn ComboboxPage() -> impl IntoView {
    let (value, set_value) = signal(Option::<String>::None);
    let (input_value, set_input_value) = signal(String::new());
    let (disabled, set_disabled) = signal(false);

    let filtered_fruits =
        Memo::new(move |_| filter_items(FRUITS, &input_value.get()));
    let filtered_vegetables =
        Memo::new(move |_| filter_items(VEGETABLES, &input_value.get()));

    view! {
        // ── Single-select combobox (with groups) ──
        <h3>"Single Select"</h3>
        <Combobox
            value=Signal::derive(move || value.get())
            on_value_change=Callback::new(move |v: String| {
                set_value.set(Some(v.clone()));
                set_input_value.set(v);
            })
            input_value=Signal::derive(move || input_value.get())
            on_input_value_change=Callback::new(move |v: String| {
                set_input_value.set(v);
            })
            disabled=disabled
        >
            <ComboboxAnchor
                class:combobox-anchor=true
                attr:data-testid="combobox-anchor"
                attr:data-disabled=move || disabled.get().then_some("")
            >
                <ComboboxInput
                    class:combobox-input=true
                    attr:data-testid="combobox-input"
                    placeholder="Search..."
                />
                <ComboboxClear
                    class:combobox-clear=true
                    attr:data-testid="combobox-clear"
                    attr:aria-label="Clear"
                >
                    "✕"
                </ComboboxClear>
                <ComboboxTrigger
                    class:combobox-trigger=true
                    attr:data-testid="combobox-trigger"
                    attr:aria-label="Toggle"
                >
                    "▼"
                </ComboboxTrigger>
            </ComboboxAnchor>

            <ComboboxPortal>
                <ComboboxContent
                    class:combobox-content=true
                    attr:data-testid="combobox-content"
                    side_offset=4.0
                >
                    <ComboboxViewport class:combobox-viewport=true attr:data-testid="combobox-viewport">
                        <Show when=move || filtered_fruits.get().is_empty() && filtered_vegetables.get().is_empty()>
                            <ComboboxEmpty class:combobox-empty=true attr:data-testid="combobox-empty">
                                "No results found"
                            </ComboboxEmpty>
                        </Show>

                        <Show when=move || !filtered_fruits.get().is_empty()>
                            <ComboboxGroup attr:class="combobox-group">
                                <ComboboxLabel attr:class="combobox-label">"Fruits"</ComboboxLabel>
                                {move || render_fruit_items(filtered_fruits.get())}
                            </ComboboxGroup>
                        </Show>

                        <Show when=move || !filtered_vegetables.get().is_empty()>
                            <ComboboxGroup attr:class="combobox-group">
                                <ComboboxLabel attr:class="combobox-label">"Vegetables"</ComboboxLabel>
                                {move || render_veg_items(filtered_vegetables.get())}
                            </ComboboxGroup>
                        </Show>
                    </ComboboxViewport>
                </ComboboxContent>
            </ComboboxPortal>
        </Combobox>

        <br />
        <br />

        <label>
            <input
                type="checkbox"
                prop:checked=move || disabled.get()
                on:change=move |ev| {
                    let target: web_sys::HtmlInputElement = ev.target().unwrap().unchecked_into();
                    set_disabled.set(target.checked());
                }
            />
            " disabled"
        </label>

        <br />
        <br />

        <span data-testid="combobox-value">{move || {
            match value.get() {
                Some(v) if !v.is_empty() => v,
                _ => "(none)".to_string(),
            }
        }}</span>

        <br />
        <br />

        <button data-testid="outside-button">"outside"</button>
        <input data-testid="outside-input" placeholder="name" />

        <hr />

        // ── Multi-select combobox ──
        <MultiSelectCombobox />

        <hr />

        // ── Default value (uncontrolled) ──
        <DefaultValueCombobox />

        <hr />

        // ── Auto-highlight ──
        <AutoHighlightCombobox />
    }
}

#[component]
fn MultiSelectCombobox() -> impl IntoView {
    let (values, set_values) = signal(Vec::<String>::new());
    let (input_value, set_input_value) = signal(String::new());

    let all_items: Vec<&str> = FRUITS.iter().chain(VEGETABLES.iter()).copied().collect();
    let all_items = StoredValue::new(all_items);

    let filtered_items = Memo::new(move |_| {
        all_items
            .try_with_value(|items| filter_items(items, &input_value.get()))
            .unwrap_or_default()
    });

    view! {
        <h3>"Multi Select"</h3>
        <Combobox
            multiple=true
            values=Signal::derive(move || values.get())
            on_values_change=Callback::new(move |v: Vec<String>| {
                set_values.set(v);
            })
            input_value=Signal::derive(move || input_value.get())
            on_input_value_change=Callback::new(move |v: String| {
                set_input_value.set(v);
            })
        >
            <ComboboxAnchor class:combobox-anchor=true attr:data-testid="multi-anchor">
                <ComboboxChips attr:class="combobox-chips" attr:data-testid="multi-chips">
                    {move || values.get().into_iter().enumerate().map(|(i, val)| {
                        let val_display = StoredValue::new(val.clone());
                        let val_remove = StoredValue::new(val.clone());
                        view! {
                            <ComboboxChip
                                attr:class="combobox-chip"
                                attr:data-testid="multi-chip"
                                value=val
                                index=i
                            >
                                {move || val_display.get_value()}
                                <ComboboxChipRemove
                                    attr:class="combobox-chip-remove"
                                    attr:data-testid="multi-chip-remove"
                                    value=val_remove.get_value()
                                />
                            </ComboboxChip>
                        }
                    }).collect_view()}
                </ComboboxChips>
                <ComboboxInput
                    class:combobox-input=true
                    attr:data-testid="multi-input"
                    placeholder="Search..."
                />
                <ComboboxTrigger
                    class:combobox-trigger=true
                    attr:data-testid="multi-trigger"
                    attr:aria-label="Toggle"
                >
                    "▼"
                </ComboboxTrigger>
            </ComboboxAnchor>

            <ComboboxPortal>
                <ComboboxContent
                    class:combobox-content=true
                    attr:data-testid="multi-content"
                    side_offset=4.0
                >
                    <ComboboxViewport class:combobox-viewport=true attr:data-testid="multi-viewport">
                        {move || {
                            let items = filtered_items.get();
                            if items.is_empty() {
                                return Either::Left(view! {
                                    <ComboboxEmpty attr:class="combobox-empty" attr:data-testid="multi-empty">
                                        "No results found"
                                    </ComboboxEmpty>
                                });
                            }
                            Either::Right(items.into_iter().map(|item| {
                                let text = item.clone();
                                let label = item.clone();
                                let label = StoredValue::new(label);
                                view! {
                                    <ComboboxItem attr:class="combobox-item" value=item text_value=text>
                                        <ComboboxItemIndicator attr:class="combobox-indicator">"✓"</ComboboxItemIndicator>
                                        {move || label.get_value()}
                                    </ComboboxItem>
                                }
                            }).collect_view())
                        }}
                    </ComboboxViewport>
                </ComboboxContent>
            </ComboboxPortal>
        </Combobox>

        <br />

        <span data-testid="multi-value">{move || {
            let vals = values.get();
            if vals.is_empty() {
                "(none)".to_string()
            } else {
                vals.join(", ")
            }
        }}</span>
    }
}

#[component]
fn AutoHighlightCombobox() -> impl IntoView {
    let (value, set_value) = signal(Option::<String>::None);
    let (input_value, set_input_value) = signal(String::new());

    let filtered_items = Memo::new(move |_| filter_items(FRUITS, &input_value.get()));

    view! {
        <h3>"Auto Highlight"</h3>
        <Combobox
            auto_highlight=true
            value=Signal::derive(move || value.get())
            on_value_change=Callback::new(move |v: String| {
                set_value.set(Some(v.clone()));
                set_input_value.set(v);
            })
            input_value=Signal::derive(move || input_value.get())
            on_input_value_change=Callback::new(move |v: String| {
                set_input_value.set(v);
            })
        >
            <ComboboxAnchor class:combobox-anchor=true attr:data-testid="autohighlight-anchor">
                <ComboboxInput
                    class:combobox-input=true
                    attr:data-testid="autohighlight-input"
                    placeholder="Search..."
                />
                <ComboboxTrigger
                    class:combobox-trigger=true
                    attr:data-testid="autohighlight-trigger"
                    attr:aria-label="Toggle"
                >
                    "▼"
                </ComboboxTrigger>
            </ComboboxAnchor>

            <ComboboxPortal>
                <ComboboxContent
                    class:combobox-content=true
                    attr:data-testid="autohighlight-content"
                    side_offset=4.0
                >
                    <ComboboxViewport class:combobox-viewport=true attr:data-testid="autohighlight-viewport">
                        {move || {
                            let items = filtered_items.get();
                            if items.is_empty() {
                                return Either::Left(view! {
                                    <ComboboxEmpty class:combobox-empty=true attr:data-testid="autohighlight-empty">
                                        "No results found"
                                    </ComboboxEmpty>
                                });
                            }
                            Either::Right(items.into_iter().map(|item| {
                                let text = item.clone();
                                let label = item.clone();
                                let label = StoredValue::new(label);
                                view! {
                                    <ComboboxItem attr:class="combobox-item" value=item text_value=text>
                                        <ComboboxItemIndicator attr:class="combobox-indicator">"✓"</ComboboxItemIndicator>
                                        {move || label.get_value()}
                                    </ComboboxItem>
                                }
                            }).collect_view())
                        }}
                    </ComboboxViewport>
                </ComboboxContent>
            </ComboboxPortal>
        </Combobox>

        <br />

        <span data-testid="autohighlight-value">{move || {
            match value.get() {
                Some(v) if !v.is_empty() => v,
                _ => "(none)".to_string(),
            }
        }}</span>
    }
}

#[component]
fn DefaultValueCombobox() -> impl IntoView {
    let (input_value, set_input_value) = signal("Banana".to_string());

    let filtered_items = Memo::new(move |_| filter_items(FRUITS, &input_value.get()));

    view! {
        <h3>"Default Value"</h3>
        <Combobox
            default_value="Banana".to_string()
            input_value=Signal::derive(move || input_value.get())
            on_input_value_change=Callback::new(move |v: String| {
                set_input_value.set(v);
            })
        >
            <ComboboxAnchor class:combobox-anchor=true attr:data-testid="default-anchor">
                <ComboboxInput
                    class:combobox-input=true
                    attr:data-testid="default-input"
                    placeholder="Search..."
                />
                <ComboboxTrigger
                    class:combobox-trigger=true
                    attr:data-testid="default-trigger"
                    attr:aria-label="Toggle"
                >
                    "▼"
                </ComboboxTrigger>
            </ComboboxAnchor>

            <ComboboxPortal>
                <ComboboxContent
                    class:combobox-content=true
                    attr:data-testid="default-content"
                    side_offset=4.0
                >
                    <ComboboxViewport class:combobox-viewport=true attr:data-testid="default-viewport">
                        {move || {
                            filtered_items.get().into_iter().map(|item| {
                                let text = item.clone();
                                let label = item.clone();
                                let label = StoredValue::new(label);
                                view! {
                                    <ComboboxItem attr:class="combobox-item" value=item text_value=text>
                                        <ComboboxItemIndicator attr:class="combobox-indicator">"✓"</ComboboxItemIndicator>
                                        {move || label.get_value()}
                                    </ComboboxItem>
                                }
                            }).collect_view()
                        }}
                    </ComboboxViewport>
                </ComboboxContent>
            </ComboboxPortal>
        </Combobox>
    }
}
