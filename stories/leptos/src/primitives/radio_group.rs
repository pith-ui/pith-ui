use leptos::prelude::*;
use radix_leptos_direction::DirectionProvider;
use radix_leptos_label::Label;
use radix_leptos_radio_group::*;

stylance::import_crate_style!(classes, "src/primitives/radio_group.stories.module.css");

#[component]
pub fn LegacyStyled() -> impl IntoView {
    view! {
        <Label attr:class=classes::label>
            "Favourite pet"
            <RadioGroup default_value="1" attr:class=classes::root>
                <Label attr:class=classes::label>
                    <RadioGroupItem value="1" attr:class=classes::item>
                        <RadioGroupIndicator attr:class=classes::indicator />
                    </RadioGroupItem>
                    " Cat"
                </Label>
                " "
                <Label attr:class=classes::label>
                    <RadioGroupItem value="2" attr:class=classes::item>
                        <RadioGroupIndicator attr:class=classes::indicator />
                    </RadioGroupItem>
                    " Dog"
                </Label>
                " "
                <Label attr:class=classes::label>
                    <RadioGroupItem value="3" attr:class=classes::item>
                        <RadioGroupIndicator attr:class=classes::indicator />
                    </RadioGroupItem>
                    " Rabbit"
                </Label>
            </RadioGroup>
        </Label>
    }
}

#[component]
pub fn LegacyControlled() -> impl IntoView {
    let (value, set_value) = signal("2".to_string());

    view! {
        <RadioGroup
            value=Signal::derive(move || value.get())
            on_value_change=Callback::new(move |v: String| set_value.set(v))
            attr:class=classes::root
        >
            <RadioGroupItem value="1" attr:class=classes::item>
                <RadioGroupIndicator attr:class=classes::indicator />
            </RadioGroupItem>
            <RadioGroupItem value="2" attr:class=classes::item>
                <RadioGroupIndicator attr:class=classes::indicator />
            </RadioGroupItem>
            <RadioGroupItem value="3" attr:class=classes::item>
                <RadioGroupIndicator attr:class=classes::indicator />
            </RadioGroupItem>
        </RadioGroup>
    }
}

#[component]
pub fn LegacyUnset() -> impl IntoView {
    view! {
        <Label attr:class=classes::label>
            "Favourite pet"
            <RadioGroup attr:class=classes::root>
                <Label attr:class=classes::label>
                    <RadioGroupItem value="1" attr:class=classes::item>
                        <RadioGroupIndicator attr:class=classes::indicator />
                    </RadioGroupItem>
                    " Cat"
                </Label>
                " "
                <Label attr:class=classes::label>
                    <RadioGroupItem value="2" disabled=true attr:class=classes::item>
                        <RadioGroupIndicator attr:class=classes::indicator />
                    </RadioGroupItem>
                    " Dog"
                </Label>
                " "
                <Label attr:class=classes::label>
                    <RadioGroupItem value="3" attr:class=classes::item>
                        <RadioGroupIndicator attr:class=classes::indicator />
                    </RadioGroupItem>
                    " Rabbit"
                </Label>
            </RadioGroup>
        </Label>
    }
}

#[component]
pub fn LegacyWithinForm() -> impl IntoView {
    let (optional, set_optional) = signal(String::new());
    let (required, set_required) = signal(String::new());
    let (stopprop, set_stopprop) = signal(String::new());

    view! {
        <form
            on:submit=move |event: leptos::ev::SubmitEvent| event.prevent_default()
            on:change=move |event: leptos::ev::Event| {
                let target = event.target().and_then(|t| {
                    use web_sys::wasm_bindgen::JsCast;
                    t.dyn_into::<web_sys::HtmlInputElement>().ok()
                });
                if let Some(radio) = target {
                    let name = radio.name();
                    let value = radio.value();
                    match name.as_str() {
                        "optional" => set_optional.set(value),
                        "required" => set_required.set(value),
                        "stopprop" => set_stopprop.set(value),
                        _ => {}
                    }
                }
            }
        >
            <fieldset>
                <legend>"optional value: " {move || optional.get()}</legend>
                <RadioGroup name="optional" attr:class=classes::root>
                    <RadioGroupItem value="1" attr:class=classes::item>
                        <RadioGroupIndicator attr:class=classes::indicator />
                    </RadioGroupItem>
                    <RadioGroupItem value="2" attr:class=classes::item>
                        <RadioGroupIndicator attr:class=classes::indicator />
                    </RadioGroupItem>
                    <RadioGroupItem value="3" attr:class=classes::item>
                        <RadioGroupIndicator attr:class=classes::indicator />
                    </RadioGroupItem>
                </RadioGroup>
            </fieldset>

            <br /><br />

            <fieldset>
                <legend>"required value: " {move || required.get()}</legend>
                <RadioGroup name="required" required=true attr:class=classes::root>
                    <RadioGroupItem value="1" attr:class=classes::item>
                        <RadioGroupIndicator attr:class=classes::indicator />
                    </RadioGroupItem>
                    <RadioGroupItem value="2" attr:class=classes::item>
                        <RadioGroupIndicator attr:class=classes::indicator />
                    </RadioGroupItem>
                    <RadioGroupItem value="3" attr:class=classes::item>
                        <RadioGroupIndicator attr:class=classes::indicator />
                    </RadioGroupItem>
                </RadioGroup>
            </fieldset>

            <br /><br />

            <fieldset>
                <legend>"stop propagation value: " {move || stopprop.get()}</legend>
                <RadioGroup name="stopprop" attr:class=classes::root>
                    <RadioGroupItem
                        value="1"
                        on_click=Callback::new(|event: leptos::ev::MouseEvent| event.stop_propagation())
                        attr:class=classes::item
                    >
                        <RadioGroupIndicator attr:class=classes::indicator />
                    </RadioGroupItem>
                    <RadioGroupItem
                        value="2"
                        on_click=Callback::new(|event: leptos::ev::MouseEvent| event.stop_propagation())
                        attr:class=classes::item
                    >
                        <RadioGroupIndicator attr:class=classes::indicator />
                    </RadioGroupItem>
                    <RadioGroupItem
                        value="3"
                        on_click=Callback::new(|event: leptos::ev::MouseEvent| event.stop_propagation())
                        attr:class=classes::item
                    >
                        <RadioGroupIndicator attr:class=classes::indicator />
                    </RadioGroupItem>
                </RadioGroup>
            </fieldset>

            <br /><br />

            <button>"Submit"</button>
        </form>
    }
}

#[component]
pub fn LegacyAnimated() -> impl IntoView {
    let indicator_class = format!("{} {}", classes::indicator, classes::animatedIndicator);
    let indicator_class = StoredValue::new(indicator_class);

    view! {
        <Label attr:class=classes::label>
            "Favourite pet"
            <RadioGroup default_value="1" attr:class=classes::root>
                <Label attr:class=classes::label>
                    <RadioGroupItem value="1" attr:class=classes::item>
                        <RadioGroupIndicator attr:class=indicator_class.get_value() />
                    </RadioGroupItem>
                    " Cat"
                </Label>
                " "
                <Label attr:class=classes::label>
                    <RadioGroupItem value="2" attr:class=classes::item>
                        <RadioGroupIndicator attr:class=indicator_class.get_value() />
                    </RadioGroupItem>
                    " Dog"
                </Label>
                " "
                <Label attr:class=classes::label>
                    <RadioGroupItem value="3" attr:class=classes::item>
                        <RadioGroupIndicator attr:class=indicator_class.get_value() />
                    </RadioGroupItem>
                    " Rabbit"
                </Label>
            </RadioGroup>
        </Label>
    }
}

#[component]
pub fn LegacyChromatic() -> impl IntoView {
    view! {
        <h1>"Uncontrolled"</h1>
        <h2>"Unset"</h2>
        <RadioGroup attr:class=classes::root>
            <RadioGroupItem value="1" attr:class=classes::item>
                <RadioGroupIndicator attr:class=classes::indicator />
            </RadioGroupItem>
            <RadioGroupItem value="2" attr:class=classes::item>
                <RadioGroupIndicator attr:class=classes::indicator />
            </RadioGroupItem>
            <RadioGroupItem value="3" attr:class=classes::item>
                <RadioGroupIndicator attr:class=classes::indicator />
            </RadioGroupItem>
        </RadioGroup>

        <h2>"Set"</h2>
        <RadioGroup default_value="3" attr:class=classes::root>
            <RadioGroupItem value="1" attr:class=classes::item>
                <RadioGroupIndicator attr:class=classes::indicator />
            </RadioGroupItem>
            <RadioGroupItem value="2" attr:class=classes::item>
                <RadioGroupIndicator attr:class=classes::indicator />
            </RadioGroupItem>
            <RadioGroupItem value="3" attr:class=classes::item>
                <RadioGroupIndicator attr:class=classes::indicator />
            </RadioGroupItem>
        </RadioGroup>

        <h1>"Controlled"</h1>
        <h2>"Unset"</h2>
        <RadioGroup value="" attr:class=classes::root>
            <RadioGroupItem value="1" attr:class=classes::item>
                <RadioGroupIndicator attr:class=classes::indicator />
            </RadioGroupItem>
            <RadioGroupItem value="2" attr:class=classes::item>
                <RadioGroupIndicator attr:class=classes::indicator />
            </RadioGroupItem>
            <RadioGroupItem value="3" attr:class=classes::item>
                <RadioGroupIndicator attr:class=classes::indicator />
            </RadioGroupItem>
        </RadioGroup>

        <h2>"Set"</h2>
        <RadioGroup value="3" attr:class=classes::root>
            <RadioGroupItem value="1" attr:class=classes::item>
                <RadioGroupIndicator attr:class=classes::indicator />
            </RadioGroupItem>
            <RadioGroupItem value="2" attr:class=classes::item>
                <RadioGroupIndicator attr:class=classes::indicator />
            </RadioGroupItem>
            <RadioGroupItem value="3" attr:class=classes::item>
                <RadioGroupIndicator attr:class=classes::indicator />
            </RadioGroupItem>
        </RadioGroup>

        <h1>"Disabled item"</h1>
        <RadioGroup attr:class=classes::root>
            <RadioGroupItem value="1" attr:class=classes::item>
                <RadioGroupIndicator attr:class=classes::indicator />
            </RadioGroupItem>
            <RadioGroupItem value="2" disabled=true attr:class=classes::item>
                <RadioGroupIndicator attr:class=classes::indicator />
            </RadioGroupItem>
            <RadioGroupItem value="3" attr:class=classes::item>
                <RadioGroupIndicator attr:class=classes::indicator />
            </RadioGroupItem>
        </RadioGroup>

        <h1>"Disabled root"</h1>
        <RadioGroup disabled=true attr:class=classes::root>
            <RadioGroupItem value="1" attr:class=classes::item>
                <RadioGroupIndicator attr:class=classes::indicator />
            </RadioGroupItem>
            // Not possible to set `disabled` back to `false` since it's set on the root
            // (this item should still be disabled).
            <RadioGroupItem value="2" disabled=false attr:class=classes::item>
                <RadioGroupIndicator attr:class=classes::indicator />
            </RadioGroupItem>
            <RadioGroupItem value="3" attr:class=classes::item>
                <RadioGroupIndicator attr:class=classes::indicator />
            </RadioGroupItem>
        </RadioGroup>

        <h1>"All items disabled"</h1>
        <RadioGroup attr:class=classes::root>
            <RadioGroupItem value="1" disabled=true attr:class=classes::item>
                <RadioGroupIndicator attr:class=classes::indicator />
            </RadioGroupItem>
            <RadioGroupItem value="2" disabled=true attr:class=classes::item>
                <RadioGroupIndicator attr:class=classes::indicator />
            </RadioGroupItem>
            <RadioGroupItem value="3" disabled=true attr:class=classes::item>
                <RadioGroupIndicator attr:class=classes::indicator />
            </RadioGroupItem>
        </RadioGroup>

        <h1>"Force mounted indicator"</h1>
        <RadioGroup attr:class=classes::root>
            <RadioGroupItem value="1" attr:class=classes::item>
                <RadioGroupIndicator force_mount=true attr:class=classes::indicator />
            </RadioGroupItem>
            <RadioGroupItem value="2" attr:class=classes::item>
                <RadioGroupIndicator force_mount=true attr:class=classes::indicator />
            </RadioGroupItem>
            <RadioGroupItem value="3" attr:class=classes::item>
                <RadioGroupIndicator force_mount=true attr:class=classes::indicator />
            </RadioGroupItem>
        </RadioGroup>

        <h1>"Direction"</h1>
        <h2>"Prop"</h2>
        <RadioGroup default_value="1" dir=radix_leptos_direction::Direction::Rtl attr:class=classes::root>
            <RadioGroupItem value="1" attr:class=classes::item>
                <RadioGroupIndicator attr:class=classes::indicator />
            </RadioGroupItem>
            <RadioGroupItem value="2" attr:class=classes::item>
                <RadioGroupIndicator attr:class=classes::indicator />
            </RadioGroupItem>
            <RadioGroupItem value="3" attr:class=classes::item>
                <RadioGroupIndicator attr:class=classes::indicator />
            </RadioGroupItem>
        </RadioGroup>

        <h2>"Inherited"</h2>
        <DirectionProvider direction=Signal::derive(|| radix_leptos_direction::Direction::Rtl)>
            <RadioGroup default_value="1" attr:class=classes::root>
                <RadioGroupItem value="1" attr:class=classes::item>
                    <RadioGroupIndicator attr:class=classes::indicator />
                </RadioGroupItem>
                <RadioGroupItem value="2" attr:class=classes::item>
                    <RadioGroupIndicator attr:class=classes::indicator />
                </RadioGroupItem>
                <RadioGroupItem value="3" attr:class=classes::item>
                    <RadioGroupIndicator attr:class=classes::indicator />
                </RadioGroupItem>
            </RadioGroup>
        </DirectionProvider>

        <h1>"State attributes"</h1>
        <h2>"Default"</h2>
        <RadioGroup default_value="3" attr:class=classes::rootAttr>
            <RadioGroupItem value="1" attr:class=classes::itemAttr>
                <RadioGroupIndicator attr:class=classes::indicatorAttr />
            </RadioGroupItem>
            <RadioGroupItem value="2" attr:class=classes::itemAttr>
                <RadioGroupIndicator attr:class=classes::indicatorAttr />
            </RadioGroupItem>
            <RadioGroupItem value="3" attr:class=classes::itemAttr>
                <RadioGroupIndicator attr:class=classes::indicatorAttr />
            </RadioGroupItem>
        </RadioGroup>

        <h2>"Disabled item"</h2>
        <RadioGroup default_value="3" attr:class=classes::rootAttr>
            <RadioGroupItem value="1" attr:class=classes::itemAttr>
                <RadioGroupIndicator attr:class=classes::indicatorAttr />
            </RadioGroupItem>
            <RadioGroupItem value="2" disabled=true attr:class=classes::itemAttr>
                <RadioGroupIndicator attr:class=classes::indicatorAttr />
            </RadioGroupItem>
            <RadioGroupItem value="3" attr:class=classes::itemAttr>
                <RadioGroupIndicator attr:class=classes::indicatorAttr />
            </RadioGroupItem>
        </RadioGroup>

        <RadioGroup default_value="2" attr:class=classes::rootAttr>
            <RadioGroupItem value="1" attr:class=classes::itemAttr>
                <RadioGroupIndicator attr:class=classes::indicatorAttr />
            </RadioGroupItem>
            <RadioGroupItem value="2" disabled=true attr:class=classes::itemAttr>
                <RadioGroupIndicator attr:class=classes::indicatorAttr />
            </RadioGroupItem>
            <RadioGroupItem value="3" attr:class=classes::itemAttr>
                <RadioGroupIndicator attr:class=classes::indicatorAttr />
            </RadioGroupItem>
        </RadioGroup>

        <h2>"Disabled root"</h2>
        <RadioGroup default_value="3" disabled=true attr:class=classes::rootAttr>
            <RadioGroupItem value="1" attr:class=classes::itemAttr>
                <RadioGroupIndicator attr:class=classes::indicatorAttr />
            </RadioGroupItem>
            <RadioGroupItem value="2" attr:class=classes::itemAttr>
                <RadioGroupIndicator attr:class=classes::indicatorAttr />
            </RadioGroupItem>
            <RadioGroupItem value="3" attr:class=classes::itemAttr>
                <RadioGroupIndicator attr:class=classes::indicatorAttr />
            </RadioGroupItem>
        </RadioGroup>

        <h2>"All items disabled"</h2>
        <RadioGroup default_value="3" attr:class=classes::rootAttr>
            <RadioGroupItem value="1" disabled=true attr:class=classes::itemAttr>
                <RadioGroupIndicator attr:class=classes::indicatorAttr />
            </RadioGroupItem>
            <RadioGroupItem value="2" disabled=true attr:class=classes::itemAttr>
                <RadioGroupIndicator attr:class=classes::indicatorAttr />
            </RadioGroupItem>
            <RadioGroupItem value="3" disabled=true attr:class=classes::itemAttr>
                <RadioGroupIndicator attr:class=classes::indicatorAttr />
            </RadioGroupItem>
        </RadioGroup>
    }
}
