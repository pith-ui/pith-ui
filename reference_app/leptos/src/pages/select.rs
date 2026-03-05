use leptos::prelude::*;
use radix_leptos_primitives::select::*;
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
            <SelectTrigger attr:class="select-trigger" attr:data-testid="select-trigger">
                <SelectValue placeholder="Select a fruit..." />
                <SelectIcon attr:class="select-icon">"▼"</SelectIcon>
            </SelectTrigger>
            <SelectPortal>
                <SelectContent attr:class="select-content" position="popper" side_offset=4.0>
                    <SelectScrollUpButton attr:class="select-scroll-button">"▲"</SelectScrollUpButton>
                    <SelectViewport attr:class="select-viewport">
                        <SelectGroup>
                            <SelectLabel attr:class="select-label">"Fruits"</SelectLabel>
                            <SelectItem attr:class="select-item" value="apple">
                                <SelectItemIndicator attr:class="select-indicator">"✓"</SelectItemIndicator>
                                <SelectItemText>"Apple"</SelectItemText>
                            </SelectItem>
                            <SelectItem attr:class="select-item" value="avocado">
                                <SelectItemIndicator attr:class="select-indicator">"✓"</SelectItemIndicator>
                                <SelectItemText>"Avocado"</SelectItemText>
                            </SelectItem>
                            <SelectItem attr:class="select-item" value="banana">
                                <SelectItemIndicator attr:class="select-indicator">"✓"</SelectItemIndicator>
                                <SelectItemText>"Banana"</SelectItemText>
                            </SelectItem>
                            <SelectItem attr:class="select-item" value="cherry" disabled=true>
                                <SelectItemIndicator attr:class="select-indicator">"✓"</SelectItemIndicator>
                                <SelectItemText>"Cherry"</SelectItemText>
                            </SelectItem>
                        </SelectGroup>

                        <SelectSeparator attr:class="select-separator" />

                        <SelectGroup>
                            <SelectLabel attr:class="select-label">"Vegetables"</SelectLabel>
                            <SelectItem attr:class="select-item" value="carrot">
                                <SelectItemIndicator attr:class="select-indicator">"✓"</SelectItemIndicator>
                                <SelectItemText>"Carrot"</SelectItemText>
                            </SelectItem>
                            <SelectItem attr:class="select-item" value="potato">
                                <SelectItemIndicator attr:class="select-indicator">"✓"</SelectItemIndicator>
                                <SelectItemText>"Potato"</SelectItemText>
                            </SelectItem>
                        </SelectGroup>
                    </SelectViewport>
                    <SelectScrollDownButton attr:class="select-scroll-button">"▼"</SelectScrollDownButton>
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
            <SelectTrigger attr:class="select-trigger" attr:data-testid="default-trigger">
                <SelectValue />
                <SelectIcon attr:class="select-icon">"▼"</SelectIcon>
            </SelectTrigger>
            <SelectPortal>
                <SelectContent attr:class="select-content" position="popper" side_offset=4.0>
                    <SelectViewport attr:class="select-viewport">
                        <SelectItem attr:class="select-item" value="apple">
                            <SelectItemIndicator attr:class="select-indicator">"✓"</SelectItemIndicator>
                            <SelectItemText>"Apple"</SelectItemText>
                        </SelectItem>
                        <SelectItem attr:class="select-item" value="banana">
                            <SelectItemIndicator attr:class="select-indicator">"✓"</SelectItemIndicator>
                            <SelectItemText>"Banana"</SelectItemText>
                        </SelectItem>
                        <SelectItem attr:class="select-item" value="cherry">
                            <SelectItemIndicator attr:class="select-indicator">"✓"</SelectItemIndicator>
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
                    <SelectTrigger attr:class="select-trigger" attr:data-testid="controlled-trigger-a">
                        <SelectValue />
                        <SelectIcon attr:class="select-icon">"▼"</SelectIcon>
                    </SelectTrigger>
                    <SelectPortal>
                        <SelectContent attr:class="select-content" position="popper" side_offset=4.0>
                            <SelectViewport attr:class="select-viewport">
                                <SelectItem attr:class="select-item" value="fr">
                                    <SelectItemIndicator attr:class="select-indicator">"✓"</SelectItemIndicator>
                                    <SelectItemText>"France"</SelectItemText>
                                </SelectItem>
                                <SelectItem attr:class="select-item" value="uk">
                                    <SelectItemIndicator attr:class="select-indicator">"✓"</SelectItemIndicator>
                                    <SelectItemText>"United Kingdom"</SelectItemText>
                                </SelectItem>
                                <SelectItem attr:class="select-item" value="es">
                                    <SelectItemIndicator attr:class="select-indicator">"✓"</SelectItemIndicator>
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
                    <SelectTrigger attr:class="select-trigger" attr:data-testid="controlled-trigger-b">
                        <SelectValue />
                        <SelectIcon attr:class="select-icon">"▼"</SelectIcon>
                    </SelectTrigger>
                    <SelectPortal>
                        <SelectContent attr:class="select-content" position="popper" side_offset=4.0>
                            <SelectViewport attr:class="select-viewport">
                                <SelectItem attr:class="select-item" value="fr">
                                    <SelectItemIndicator attr:class="select-indicator">"✓"</SelectItemIndicator>
                                    <SelectItemText>"France"</SelectItemText>
                                </SelectItem>
                                <SelectItem attr:class="select-item" value="uk">
                                    <SelectItemIndicator attr:class="select-indicator">"✓"</SelectItemIndicator>
                                    <SelectItemText>"United Kingdom"</SelectItemText>
                                </SelectItem>
                                <SelectItem attr:class="select-item" value="es">
                                    <SelectItemIndicator attr:class="select-indicator">"✓"</SelectItemIndicator>
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
            <SelectTrigger attr:class="select-trigger" attr:data-testid="aligned-trigger">
                <SelectValue />
                <SelectIcon attr:class="select-icon">"▼"</SelectIcon>
            </SelectTrigger>
            <SelectPortal>
                <SelectContent attr:class="select-content" attr:data-testid="aligned-content">
                    <SelectViewport attr:class="select-viewport">
                        <SelectItem attr:class="select-item" value="apple">
                            <SelectItemIndicator attr:class="select-indicator">"✓"</SelectItemIndicator>
                            <SelectItemText>"Apple"</SelectItemText>
                        </SelectItem>
                        <SelectItem attr:class="select-item" value="banana">
                            <SelectItemIndicator attr:class="select-indicator">"✓"</SelectItemIndicator>
                            <SelectItemText>"Banana"</SelectItemText>
                        </SelectItem>
                        <SelectItem attr:class="select-item" value="cherry">
                            <SelectItemIndicator attr:class="select-indicator">"✓"</SelectItemIndicator>
                            <SelectItemText>"Cherry"</SelectItemText>
                        </SelectItem>
                        <SelectItem attr:class="select-item" value="date">
                            <SelectItemIndicator attr:class="select-indicator">"✓"</SelectItemIndicator>
                            <SelectItemText>"Date"</SelectItemText>
                        </SelectItem>
                        <SelectItem attr:class="select-item" value="elderberry">
                            <SelectItemIndicator attr:class="select-indicator">"✓"</SelectItemIndicator>
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
                    <SelectTrigger attr:class="select-trigger" attr:data-testid="form-trigger">
                        <SelectValue />
                        <SelectIcon attr:class="select-icon">"▼"</SelectIcon>
                    </SelectTrigger>
                    <SelectPortal>
                        <SelectContent attr:class="select-content" position="popper" side_offset=4.0>
                            <SelectViewport attr:class="select-viewport">
                                <SelectItem attr:class="select-item" value="fr">
                                    <SelectItemIndicator attr:class="select-indicator">"✓"</SelectItemIndicator>
                                    <SelectItemText>"France"</SelectItemText>
                                </SelectItem>
                                <SelectItem attr:class="select-item" value="uk">
                                    <SelectItemIndicator attr:class="select-indicator">"✓"</SelectItemIndicator>
                                    <SelectItemText>"United Kingdom"</SelectItemText>
                                </SelectItem>
                                <SelectItem attr:class="select-item" value="es">
                                    <SelectItemIndicator attr:class="select-indicator">"✓"</SelectItemIndicator>
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
                <SelectTrigger attr:class="select-trigger" attr:data-testid="forced-novalue-trigger">
                    <SelectValue placeholder="Pick an option" />
                    <SelectIcon attr:class="select-icon">"▼"</SelectIcon>
                </SelectTrigger>
                <SelectPortal>
                    <SelectContent attr:class="select-content" attr:data-testid="forced-novalue-content" attr:style="opacity: 0.7;">
                        <SelectViewport attr:class="select-viewport">
                            <SelectItem attr:class="select-item" value="apple">
                                <SelectItemIndicator attr:class="select-indicator">"✓"</SelectItemIndicator>
                                <SelectItemText>"Apple"</SelectItemText>
                            </SelectItem>
                            <SelectItem attr:class="select-item" value="banana">
                                <SelectItemIndicator attr:class="select-indicator">"✓"</SelectItemIndicator>
                                <SelectItemText>"Banana"</SelectItemText>
                            </SelectItem>
                            <SelectItem attr:class="select-item" value="cherry">
                                <SelectItemIndicator attr:class="select-indicator">"✓"</SelectItemIndicator>
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
                <SelectTrigger attr:class="select-trigger" attr:data-testid="forced-trigger">
                    <SelectValue />
                    <SelectIcon attr:class="select-icon">"▼"</SelectIcon>
                </SelectTrigger>
                <SelectPortal>
                    <SelectContent attr:class="select-content" attr:data-testid="forced-content" position="popper" side_offset=4.0>
                        <SelectViewport attr:class="select-viewport">
                            <SelectItem attr:class="select-item" value="apple">
                                <SelectItemIndicator attr:class="select-indicator">"✓"</SelectItemIndicator>
                                <SelectItemText>"Apple"</SelectItemText>
                            </SelectItem>
                            <SelectItem attr:class="select-item" value="banana">
                                <SelectItemIndicator attr:class="select-indicator">"✓"</SelectItemIndicator>
                                <SelectItemText>"Banana"</SelectItemText>
                            </SelectItem>
                            <SelectItem attr:class="select-item" value="cherry">
                                <SelectItemIndicator attr:class="select-indicator">"✓"</SelectItemIndicator>
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
                <SelectTrigger attr:class="select-trigger" attr:data-testid="forced-aligned-trigger">
                    <SelectValue />
                    <SelectIcon attr:class="select-icon">"▼"</SelectIcon>
                </SelectTrigger>
                <SelectPortal>
                    <SelectContent attr:class="select-content" attr:data-testid="forced-aligned-content">
                        <SelectViewport attr:class="select-viewport">
                            <SelectItem attr:class="select-item" value="apple">
                                <SelectItemIndicator attr:class="select-indicator">"✓"</SelectItemIndicator>
                                <SelectItemText>"Apple"</SelectItemText>
                            </SelectItem>
                            <SelectItem attr:class="select-item" value="banana">
                                <SelectItemIndicator attr:class="select-indicator">"✓"</SelectItemIndicator>
                                <SelectItemText>"Banana"</SelectItemText>
                            </SelectItem>
                            <SelectItem attr:class="select-item" value="cherry">
                                <SelectItemIndicator attr:class="select-indicator">"✓"</SelectItemIndicator>
                                <SelectItemText>"Cherry"</SelectItemText>
                            </SelectItem>
                        </SelectViewport>
                    </SelectContent>
                </SelectPortal>
            </Select>
        </div>
    }
}
