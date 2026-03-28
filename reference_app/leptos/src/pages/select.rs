use leptos::prelude::*;
use pith_ui::select::*;
use web_sys::wasm_bindgen::JsCast;

#[component]
pub fn SelectPage() -> impl IntoView {
    let (value, set_value) = signal(String::new());
    let (disabled, set_disabled) = signal(false);
    let (controlled_value, set_controlled_value) = signal("uk".to_string());

    view! {
        // ── Main select (controlled, with groups) ──
        <Select
            value=Signal::derive(move || {
                let v = value.get();
                if v.is_empty() { None } else { Some(v) }
            })
            on_value_change=Callback::new(move |v: String| set_value.set(v))
            disabled=disabled
        >
            <SelectTrigger class:select-trigger=true attr:data-testid="select-trigger" attr:aria-label="Select a fruit">
                <SelectValue placeholder="Select a fruit..." />
                <SelectIcon class:select-icon=true>"▼"</SelectIcon>
            </SelectTrigger>
            <SelectPortal>
                <SelectContent class:select-content=true position="popper" side_offset=4.0>
                    <SelectScrollUpButton class:select-scroll-button=true>"▲"</SelectScrollUpButton>
                    <SelectViewport class:select-viewport=true>
                        <SelectGroup>
                            <SelectLabel class:select-label=true>"Fruits"</SelectLabel>
                            <SelectItem class:select-item=true value="apple">
                                <SelectItemIndicator class:select-indicator=true>"✓"</SelectItemIndicator>
                                <SelectItemText>"Apple"</SelectItemText>
                            </SelectItem>
                            <SelectItem class:select-item=true value="avocado">
                                <SelectItemIndicator class:select-indicator=true>"✓"</SelectItemIndicator>
                                <SelectItemText>"Avocado"</SelectItemText>
                            </SelectItem>
                            <SelectItem class:select-item=true value="banana">
                                <SelectItemIndicator class:select-indicator=true>"✓"</SelectItemIndicator>
                                <SelectItemText>"Banana"</SelectItemText>
                            </SelectItem>
                            <SelectItem class:select-item=true value="cherry" disabled=true>
                                <SelectItemIndicator class:select-indicator=true>"✓"</SelectItemIndicator>
                                <SelectItemText>"Cherry"</SelectItemText>
                            </SelectItem>
                        </SelectGroup>

                        <SelectSeparator class:select-separator=true />

                        <SelectGroup>
                            <SelectLabel class:select-label=true>"Vegetables"</SelectLabel>
                            <SelectItem class:select-item=true value="carrot">
                                <SelectItemIndicator class:select-indicator=true>"✓"</SelectItemIndicator>
                                <SelectItemText>"Carrot"</SelectItemText>
                            </SelectItem>
                            <SelectItem class:select-item=true value="potato">
                                <SelectItemIndicator class:select-indicator=true>"✓"</SelectItemIndicator>
                                <SelectItemText>"Potato"</SelectItemText>
                            </SelectItem>
                        </SelectGroup>
                    </SelectViewport>
                    <SelectScrollDownButton class:select-scroll-button=true>"▼"</SelectScrollDownButton>
                </SelectContent>
            </SelectPortal>
        </Select>

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

        <span data-testid="select-value">{move || {
            let v = value.get();
            if v.is_empty() { "(none)".to_string() } else { v }
        }}</span>

        <br />
        <br />

        <button data-testid="outside-button">"outside"</button>
        <input data-testid="outside-input" placeholder="name" />

        <hr />

        // ── Default value select ──
        <h3>"Default Value"</h3>
        <Select default_value="banana">
            <SelectTrigger class:select-trigger=true attr:data-testid="default-trigger" attr:aria-label="Select a fruit">
                <SelectValue />
                <SelectIcon class:select-icon=true>"▼"</SelectIcon>
            </SelectTrigger>
            <SelectPortal>
                <SelectContent class:select-content=true position="popper" side_offset=4.0>
                    <SelectViewport class:select-viewport=true>
                        <SelectItem class:select-item=true value="apple">
                            <SelectItemIndicator class:select-indicator=true>"✓"</SelectItemIndicator>
                            <SelectItemText>"Apple"</SelectItemText>
                        </SelectItem>
                        <SelectItem class:select-item=true value="banana">
                            <SelectItemIndicator class:select-indicator=true>"✓"</SelectItemIndicator>
                            <SelectItemText>"Banana"</SelectItemText>
                        </SelectItem>
                        <SelectItem class:select-item=true value="cherry">
                            <SelectItemIndicator class:select-indicator=true>"✓"</SelectItemIndicator>
                            <SelectItemText>"Cherry"</SelectItemText>
                        </SelectItem>
                    </SelectViewport>
                </SelectContent>
            </SelectPortal>
        </Select>

        <hr />

        // ── Controlled dual selects (shared state) ──
        <h3>"Controlled Pair"</h3>
        <div style="display: flex; gap: 20px;">
            <label>
                "Select A:"
                <Select
                    value=Signal::derive(move || Some(controlled_value.get()))
                    on_value_change=Callback::new(move |v: String| set_controlled_value.set(v))
                >
                    <SelectTrigger class:select-trigger=true attr:data-testid="controlled-trigger-a">
                        <SelectValue />
                        <SelectIcon class:select-icon=true>"▼"</SelectIcon>
                    </SelectTrigger>
                    <SelectPortal>
                        <SelectContent class:select-content=true position="popper" side_offset=4.0>
                            <SelectViewport class:select-viewport=true>
                                <SelectItem class:select-item=true value="fr">
                                    <SelectItemIndicator class:select-indicator=true>"✓"</SelectItemIndicator>
                                    <SelectItemText>"France"</SelectItemText>
                                </SelectItem>
                                <SelectItem class:select-item=true value="uk">
                                    <SelectItemIndicator class:select-indicator=true>"✓"</SelectItemIndicator>
                                    <SelectItemText>"United Kingdom"</SelectItemText>
                                </SelectItem>
                                <SelectItem class:select-item=true value="es">
                                    <SelectItemIndicator class:select-indicator=true>"✓"</SelectItemIndicator>
                                    <SelectItemText>"Spain"</SelectItemText>
                                </SelectItem>
                            </SelectViewport>
                        </SelectContent>
                    </SelectPortal>
                </Select>
            </label>

            <label>
                "Select B:"
                <Select
                    value=Signal::derive(move || Some(controlled_value.get()))
                    on_value_change=Callback::new(move |v: String| set_controlled_value.set(v))
                >
                    <SelectTrigger class:select-trigger=true attr:data-testid="controlled-trigger-b">
                        <SelectValue />
                        <SelectIcon class:select-icon=true>"▼"</SelectIcon>
                    </SelectTrigger>
                    <SelectPortal>
                        <SelectContent class:select-content=true position="popper" side_offset=4.0>
                            <SelectViewport class:select-viewport=true>
                                <SelectItem class:select-item=true value="fr">
                                    <SelectItemIndicator class:select-indicator=true>"✓"</SelectItemIndicator>
                                    <SelectItemText>"France"</SelectItemText>
                                </SelectItem>
                                <SelectItem class:select-item=true value="uk">
                                    <SelectItemIndicator class:select-indicator=true>"✓"</SelectItemIndicator>
                                    <SelectItemText>"United Kingdom"</SelectItemText>
                                </SelectItem>
                                <SelectItem class:select-item=true value="es">
                                    <SelectItemIndicator class:select-indicator=true>"✓"</SelectItemIndicator>
                                    <SelectItemText>"Spain"</SelectItemText>
                                </SelectItem>
                            </SelectViewport>
                        </SelectContent>
                    </SelectPortal>
                </Select>
            </label>
        </div>
        <span data-testid="controlled-value">{move || controlled_value.get()}</span>

        <hr />

        // ── Form integration ──
        <h3>"Form Integration"</h3>
        <FormSection />

        <hr />

        // ── Item-aligned positioning (default) ──
        <h3>"Item Aligned"</h3>
        <Select default_value="banana">
            <SelectTrigger class:select-trigger=true attr:data-testid="aligned-trigger" attr:aria-label="Select a fruit">
                <SelectValue />
                <SelectIcon class:select-icon=true>"▼"</SelectIcon>
            </SelectTrigger>
            <SelectPortal>
                <SelectContent class:select-content=true attr:data-testid="aligned-content">
                    <SelectViewport class:select-viewport=true>
                        <SelectItem class:select-item=true value="apple">
                            <SelectItemIndicator class:select-indicator=true>"✓"</SelectItemIndicator>
                            <SelectItemText>"Apple"</SelectItemText>
                        </SelectItem>
                        <SelectItem class:select-item=true value="banana">
                            <SelectItemIndicator class:select-indicator=true>"✓"</SelectItemIndicator>
                            <SelectItemText>"Banana"</SelectItemText>
                        </SelectItem>
                        <SelectItem class:select-item=true value="cherry">
                            <SelectItemIndicator class:select-indicator=true>"✓"</SelectItemIndicator>
                            <SelectItemText>"Cherry"</SelectItemText>
                        </SelectItem>
                        <SelectItem class:select-item=true value="date">
                            <SelectItemIndicator class:select-indicator=true>"✓"</SelectItemIndicator>
                            <SelectItemText>"Date"</SelectItemText>
                        </SelectItem>
                        <SelectItem class:select-item=true value="elderberry">
                            <SelectItemIndicator class:select-indicator=true>"✓"</SelectItemIndicator>
                            <SelectItemText>"Elderberry"</SelectItemText>
                        </SelectItem>
                    </SelectViewport>
                </SelectContent>
            </SelectPortal>
        </Select>
    }
}

#[component]
fn FormSection() -> impl IntoView {
    let (data, set_data) = signal("{}".to_string());

    let handle_change = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.current_target() {
            let form_el: web_sys::HtmlFormElement = target.unchecked_into();
            if let Ok(form_data) = web_sys::FormData::new_with_form(&form_el) {
                let country = form_data.get("country").as_string().unwrap_or_default();
                set_data.set(format!("{{\"country\":\"{country}\"}}"));
            }
        }
    };

    view! {
        <form
            data-testid="select-form"
            on:submit=move |ev: leptos::ev::SubmitEvent| {
                ev.prevent_default();
                handle_change(ev.into());
            }
            on:change=handle_change
        >
            <label>
                "Country: "
                <Select name="country" default_value="fr">
                    <SelectTrigger class:select-trigger=true attr:data-testid="form-trigger">
                        <SelectValue />
                        <SelectIcon class:select-icon=true>"▼"</SelectIcon>
                    </SelectTrigger>
                    <SelectPortal>
                        <SelectContent class:select-content=true position="popper" side_offset=4.0>
                            <SelectViewport class:select-viewport=true>
                                <SelectItem class:select-item=true value="fr">
                                    <SelectItemIndicator class:select-indicator=true>"✓"</SelectItemIndicator>
                                    <SelectItemText>"France"</SelectItemText>
                                </SelectItem>
                                <SelectItem class:select-item=true value="uk">
                                    <SelectItemIndicator class:select-indicator=true>"✓"</SelectItemIndicator>
                                    <SelectItemText>"United Kingdom"</SelectItemText>
                                </SelectItem>
                                <SelectItem class:select-item=true value="es">
                                    <SelectItemIndicator class:select-indicator=true>"✓"</SelectItemIndicator>
                                    <SelectItemText>"Spain"</SelectItemText>
                                </SelectItem>
                            </SelectViewport>
                        </SelectContent>
                    </SelectPortal>
                </Select>
            </label>
            <button type="submit">"Submit"</button>
            <pre data-testid="form-data">{move || data.get()}</pre>
        </form>
    }
}

#[component]
pub fn SelectForcedOpenPage() -> impl IntoView {
    view! {
        // ── Forced open, no default value, item-aligned ──
        <h3>"Forced Open No Value (Item Aligned)"</h3>
        <div style="position: relative; min-height: 200px;">
            <Select open=true>
                <SelectTrigger class:select-trigger=true attr:data-testid="forced-novalue-trigger">
                    <SelectValue placeholder="Pick an option" />
                    <SelectIcon class:select-icon=true>"▼"</SelectIcon>
                </SelectTrigger>
                <SelectPortal>
                    <SelectContent class:select-content=true attr:data-testid="forced-novalue-content" style:opacity="0.7">
                        <SelectViewport class:select-viewport=true>
                            <SelectItem class:select-item=true value="apple">
                                <SelectItemIndicator class:select-indicator=true>"✓"</SelectItemIndicator>
                                <SelectItemText>"Apple"</SelectItemText>
                            </SelectItem>
                            <SelectItem class:select-item=true value="banana">
                                <SelectItemIndicator class:select-indicator=true>"✓"</SelectItemIndicator>
                                <SelectItemText>"Banana"</SelectItemText>
                            </SelectItem>
                            <SelectItem class:select-item=true value="cherry">
                                <SelectItemIndicator class:select-indicator=true>"✓"</SelectItemIndicator>
                                <SelectItemText>"Cherry"</SelectItemText>
                            </SelectItem>
                        </SelectViewport>
                    </SelectContent>
                </SelectPortal>
            </Select>
        </div>

        <hr />

        // ── Forced open, popper (controlled open=true) ──
        <h3>"Forced Open (Popper)"</h3>
        <div style="position: relative; min-height: 200px;">
            <Select default_value="banana" open=true>
                <SelectTrigger class:select-trigger=true attr:data-testid="forced-trigger">
                    <SelectValue />
                    <SelectIcon class:select-icon=true>"▼"</SelectIcon>
                </SelectTrigger>
                <SelectPortal>
                    <SelectContent class:select-content=true attr:data-testid="forced-content" position="popper" side_offset=4.0 style:background="tomato">
                        <SelectViewport class:select-viewport=true>
                            <SelectItem class:select-item=true value="apple">
                                <SelectItemIndicator class:select-indicator=true>"✓"</SelectItemIndicator>
                                <SelectItemText>"Apple"</SelectItemText>
                            </SelectItem>
                            <SelectItem class:select-item=true value="banana">
                                <SelectItemIndicator class:select-indicator=true>"✓"</SelectItemIndicator>
                                <SelectItemText>"Banana"</SelectItemText>
                            </SelectItem>
                            <SelectItem class:select-item=true value="cherry">
                                <SelectItemIndicator class:select-indicator=true>"✓"</SelectItemIndicator>
                                <SelectItemText>"Cherry"</SelectItemText>
                            </SelectItem>
                        </SelectViewport>
                    </SelectContent>
                </SelectPortal>
            </Select>
        </div>

        <hr />

        // ── Forced open, item-aligned (controlled open=true, no position="popper") ──
        <h3>"Forced Open (Item Aligned)"</h3>
        <div style="position: relative; min-height: 200px;">
            <Select default_value="banana" open=true>
                <SelectTrigger class:select-trigger=true attr:data-testid="forced-aligned-trigger">
                    <SelectValue />
                    <SelectIcon class:select-icon=true>"▼"</SelectIcon>
                </SelectTrigger>
                <SelectPortal>
                    <SelectContent class:select-content=true attr:data-testid="forced-aligned-content">
                        <SelectViewport class:select-viewport=true>
                            <SelectItem class:select-item=true value="apple">
                                <SelectItemIndicator class:select-indicator=true>"✓"</SelectItemIndicator>
                                <SelectItemText>"Apple"</SelectItemText>
                            </SelectItem>
                            <SelectItem class:select-item=true value="banana">
                                <SelectItemIndicator class:select-indicator=true>"✓"</SelectItemIndicator>
                                <SelectItemText>"Banana"</SelectItemText>
                            </SelectItem>
                            <SelectItem class:select-item=true value="cherry">
                                <SelectItemIndicator class:select-indicator=true>"✓"</SelectItemIndicator>
                                <SelectItemText>"Cherry"</SelectItemText>
                            </SelectItem>
                        </SelectViewport>
                    </SelectContent>
                </SelectPortal>
            </Select>
        </div>
    }
}
