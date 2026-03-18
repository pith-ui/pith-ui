use leptos::prelude::*;
use radix_leptos_primitives::toggle_group::*;

#[component]
pub fn ToggleGroupPage() -> impl IntoView {
    let (type_value, set_type_value) = signal("single".to_string());
    let (orientation, set_orientation) = signal("horizontal".to_string());
    let (disabled, set_disabled) = signal(false);
    let (single_value, set_single_value) = signal(String::new());
    let (multiple_value, set_multiple_value) = signal(Vec::<String>::new());

    let orient = Signal::derive(move || match orientation.get().as_str() {
        "vertical" => radix_leptos_primitives::roving_focus::Orientation::Vertical,
        _ => radix_leptos_primitives::roving_focus::Orientation::Horizontal,
    });

    let single_value_signal = Signal::derive(move || single_value.get());
    let multiple_value_signal = Signal::derive(move || multiple_value.get());

    view! {
        <Show
            when=move || type_value.get() == "single"
            fallback=move || {
                view! {
                    <ToggleGroupMultiple
                        orientation=orient
                        disabled=disabled
                        class:toggle-group-root=true
                        attr:aria-label="Options"
                        value=multiple_value_signal
                        on_value_change=Callback::new(move |v: Vec<String>| set_multiple_value.set(v))
                    >
                        <ToggleGroupItem value="1" class:toggle-group-item=true>
                            "Item 1"
                        </ToggleGroupItem>
                        <ToggleGroupItem value="2" disabled=true class:toggle-group-item=true>
                            "Item 2"
                        </ToggleGroupItem>
                        <ToggleGroupItem value="3" class:toggle-group-item=true>
                            "Item 3"
                        </ToggleGroupItem>
                    </ToggleGroupMultiple>
                }
            }
        >
            <ToggleGroupSingle
                orientation=orient
                disabled=disabled
                class:toggle-group-root=true
                attr:aria-label="Options"
                value=single_value_signal
                on_value_change=Callback::new(move |v: String| set_single_value.set(v))
            >
                <ToggleGroupItem value="1" class:toggle-group-item=true>
                    "Item 1"
                </ToggleGroupItem>
                <ToggleGroupItem value="2" disabled=true class:toggle-group-item=true>
                    "Item 2"
                </ToggleGroupItem>
                <ToggleGroupItem value="3" class:toggle-group-item=true>
                    "Item 3"
                </ToggleGroupItem>
            </ToggleGroupSingle>
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

        <br />

        <span data-testid="toggle-value">{move || {
            if type_value.get() == "single" {
                single_value.get()
            } else {
                multiple_value.get().join(",")
            }
        }}</span>
        <button data-testid="set-item3" on:click=move |_| {
            if type_value.get() == "single" {
                set_single_value.set("3".to_string());
            } else {
                set_multiple_value.set(vec!["3".to_string()]);
            }
        }>
            "set item 3"
        </button>
        <button data-testid="clear-value" on:click=move |_| {
            if type_value.get() == "single" {
                set_single_value.set(String::new());
            } else {
                set_multiple_value.set(vec![]);
            }
        }>
            "clear"
        </button>
    }
}
