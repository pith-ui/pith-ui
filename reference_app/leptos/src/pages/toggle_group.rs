use leptos::prelude::*;
use radix_leptos_primitives::toggle_group::*;

#[component]
pub fn ToggleGroupPage() -> impl IntoView {
    let (type_value, set_type_value) = signal("single".to_string());
    let (orientation, set_orientation) = signal("horizontal".to_string());
    let (disabled, set_disabled) = signal(false);

    let toggle_type = Signal::derive(move || match type_value.get().as_str() {
        "multiple" => ToggleGroupType::Multiple,
        _ => ToggleGroupType::Single,
    });

    let orient = Signal::derive(move || match orientation.get().as_str() {
        "vertical" => radix_leptos_primitives::roving_focus::Orientation::Vertical,
        _ => radix_leptos_primitives::roving_focus::Orientation::Horizontal,
    });

    view! {
        <Show
            when=move || matches!(toggle_type.get(), ToggleGroupType::Single)
            fallback=move || {
                view! {
                    <ToggleGroup
                        r#type=ToggleGroupType::Multiple
                        orientation=orient.get()
                        disabled=disabled
                        attr:class="toggle-group-root"
                        attr:aria-label="Options"
                    >
                        <ToggleGroupItem value="1" attr:class="toggle-group-item">
                            "Item 1"
                        </ToggleGroupItem>
                        <ToggleGroupItem value="2" disabled=true attr:class="toggle-group-item">
                            "Item 2"
                        </ToggleGroupItem>
                        <ToggleGroupItem value="3" attr:class="toggle-group-item">
                            "Item 3"
                        </ToggleGroupItem>
                    </ToggleGroup>
                }
            }
        >
            <ToggleGroup
                r#type=ToggleGroupType::Single
                orientation=orient.get()
                disabled=disabled
                attr:class="toggle-group-root"
                attr:aria-label="Options"
            >
                <ToggleGroupItem value="1" attr:class="toggle-group-item">
                    "Item 1"
                </ToggleGroupItem>
                <ToggleGroupItem value="2" disabled=true attr:class="toggle-group-item">
                    "Item 2"
                </ToggleGroupItem>
                <ToggleGroupItem value="3" attr:class="toggle-group-item">
                    "Item 3"
                </ToggleGroupItem>
            </ToggleGroup>
        </Show>

        <br /><br />

        <fieldset>
            <legend>"type"</legend>
            <label>
                <input
                    type="radio"
                    name="type"
                    value="single"
                    prop:checked=move || type_value.get() == "single"
                    on:change=move |_| set_type_value.set("single".to_string())
                />
                " single"
            </label>
            <br />
            <label>
                <input
                    type="radio"
                    name="type"
                    value="multiple"
                    prop:checked=move || type_value.get() == "multiple"
                    on:change=move |_| set_type_value.set("multiple".to_string())
                />
                " multiple"
            </label>
        </fieldset>

        <fieldset>
            <legend>"orientation"</legend>
            <label>
                <input
                    type="radio"
                    name="orientation"
                    value="horizontal"
                    prop:checked=move || orientation.get() == "horizontal"
                    on:change=move |_| set_orientation.set("horizontal".to_string())
                />
                " horizontal"
            </label>
            <br />
            <label>
                <input
                    type="radio"
                    name="orientation"
                    value="vertical"
                    prop:checked=move || orientation.get() == "vertical"
                    on:change=move |_| set_orientation.set("vertical".to_string())
                />
                " vertical"
            </label>
        </fieldset>

        <label>
            <input
                type="checkbox"
                prop:checked=move || disabled.get()
                on:change=move |ev| {
                    use web_sys::wasm_bindgen::JsCast;
                    let target: web_sys::HtmlInputElement = ev.target().unwrap().unchecked_into();
                    set_disabled.set(target.checked());
                }
            />
            " disabled"
        </label>
    }
}
