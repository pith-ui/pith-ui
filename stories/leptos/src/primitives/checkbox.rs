use leptos::{ev::Event, prelude::*};
use pith_ui::checkbox::*;
use pith_ui::label::*;

stylance::import_crate_style!(classes, "src/primitives/checkbox.stories.module.css");

#[component]
pub fn Styled() -> impl IntoView {
    view! {
        <p>"This checkbox is nested inside a label. The state is uncontrolled."</p>

        <h1>"Custom label"</h1>
        <Label attr:class=classes::label>
            "Label "
            <Checkbox attr:class=classes::root>
                <CheckboxIndicator attr:class=classes::indicator />
            </Checkbox>
        </Label>

        <br />
        <br />

        <h1>"Native label"</h1>
        <label>
            "Label "
            <Checkbox attr:class=classes::root>
                <CheckboxIndicator attr:class=classes::indicator />
            </Checkbox>
        </label>

        <h1>"Native label + native checkbox"</h1>
        <label>
            "Label " <input type="checkbox" />
        </label>

        <h1>"Custom label + for"</h1>
        <Label attr:r#for="one">"Label"</Label>
        <Checkbox attr:id="one" attr:class=classes::root>
            <CheckboxIndicator attr:class=classes::indicator />
        </Checkbox>

        <br />
        <br />

        <h1>"Native label + for"</h1>
        <label for="two">"Label"</label>
        <Checkbox attr:id="two" attr:class=classes::root>
            <CheckboxIndicator attr:class=classes::indicator />
        </Checkbox>

        <h1>"Native label + native checkbox"</h1>
        <label for="three">"Label"</label>
        <input id="three" type="checkbox" />
    }
}

#[component]
pub fn Controlled() -> impl IntoView {
    let (checked, set_checked) = signal(CheckedState::True);

    view! {
        <p>"This checkbox is placed adjacent to its label. The state is controlled."</p>
        <Label attr:r#for="randBox" attr:class=classes::label>"Label"</Label>{' '}
        <Checkbox
            attr:id="randBox"
            attr:class=classes::root
            checked=checked
            on_checked_change=move |checked| set_checked.set(checked)
        >
            <CheckboxIndicator attr:class=classes::indicator />
        </Checkbox>
    }
}

#[component]
pub fn Indeterminate() -> impl IntoView {
    let (checked, set_checked) = signal(CheckedState::Indeterminate);

    view! {
        <p>
            <Checkbox
                attr:class=classes::root
                checked=checked
                on_checked_change=move |checked| set_checked.set(checked)
            >
                <CheckboxIndicator attr:class=classes::indicator />
            </Checkbox>
        </p>

        <button
            type="button"
            on:click=move |_| {
                set_checked.update(|checked| {
                    *checked = match checked {
                        CheckedState::Indeterminate => CheckedState::False,
                        _ => CheckedState::Indeterminate,
                    };
                })
            }
        >
            "Toggle indeterminate"
        </button>
    }
}

#[component]
pub fn WithinForm() -> impl IntoView {
    struct Data {
        optional: bool,
        required: bool,
        stopprop: bool,
    }

    let (data, set_data) = signal(Data {
        optional: false,
        required: false,
        stopprop: false,
    });
    let (checked, set_checked) = signal(CheckedState::Indeterminate);

    view! {
        <form
            on:submit=move |event| event.prevent_default()
            on:change=move |event: Event| {
                // This event does not exist in the DOM, only in React.
                // To make this story functional, on_checked_change event handlers were used instead.

                let input: web_sys::HtmlInputElement = event_target(&event);

                match input.name().as_str() {
                    "optional" => set_data.update(|data| {
                        data.optional = input.checked();
                    }),
                    "required" => set_data.update(|data| {
                        data.required = input.checked();
                    }),
                    "stopprop" => set_data.update(|data| {
                        data.stopprop = input.checked();
                    }),
                    _ => unreachable!("No other inputs exist."),
                }
            }
        >
            <fieldset>
                <legend>"optional checked: " {move || format!("{}", data.with(|data| data.optional))}</legend>
                <label>
                    <Checkbox
                        attr:class=classes::root
                        name="optional"
                        checked=checked
                        on_checked_change=move |checked| {
                            set_checked.set(checked);
                            set_data.update(|data| {
                                data.optional = checked == CheckedState::True;
                            })
                        }
                    >
                        <CheckboxIndicator attr:class=classes::indicator />
                    </Checkbox>{' '}
                    "with label"
                </label>
                <br />
                <br />

                <button
                    type="button"
                    on:click=move |_| {
                        set_checked.update(|checked| {
                            *checked = match checked {
                                CheckedState::Indeterminate => CheckedState::False,
                                _ => CheckedState::Indeterminate,
                            };
                        })
                    }
                >
                    "Toggle indeterminate"
                </button>
            </fieldset>

            <br />
            <br />

            <fieldset>
                <legend>"required checked: " {move || format!("{}", data.with(|data| data.required))}</legend>
                <Checkbox
                    attr:class=classes::root
                    name="required"
                    required=true
                    on_checked_change=move |checked| {
                        set_data.update(|data| {
                            data.required = checked == CheckedState::True;
                        });
                    }
                >
                    <CheckboxIndicator attr:class=classes::indicator />
                </Checkbox>
            </fieldset>


            <br />
            <br />

            <fieldset>
                <legend>"stop propagation checked: " {move || format!("{}", data.with(|data| data.stopprop))}</legend>
                <Checkbox
                    attr:class=classes::root
                    name="stopprop"
                    on:click=move |event| event.stop_propagation()
                >
                    <CheckboxIndicator attr:class=classes::indicator />
                </Checkbox>
            </fieldset>

            <br />
            <br />

            <button type="reset">"Reset"</button>
            <button>"Submit"</button>
        </form>
    }
}

#[component]
pub fn Animated() -> impl IntoView {
    let (checked, set_checked) = signal(CheckedState::Indeterminate);

    let animated_class = format!("{} {}", classes::indicator, classes::animatedIndicator);

    // TODO: fade out doesn't work, might be an issue with Presence component?

    view! {
        <p>
            <Checkbox
                attr:class=classes::root
                checked=checked
                on_checked_change=move |checked| set_checked.set(checked)
            >
                <CheckboxIndicator attr:class=animated_class.clone() />
            </Checkbox>
        </p>

        <button
            type="button"
            on:click=move |_| {
                set_checked.update(|checked| {
                    *checked = match checked {
                        CheckedState::Indeterminate => CheckedState::False,
                        _ => CheckedState::Indeterminate,
                    };
                })
            }
        >
            "Toggle indeterminate"
        </button>
    }
}

#[component]
pub fn Chromatic() -> impl IntoView {
    view! {
        <h1>"Uncontrolled"</h1>
        <h2>"Unchecked"</h2>
        <Checkbox attr:class=classes::root>
            <CheckboxIndicator attr:class=classes::indicator />
        </Checkbox>

        <h2>"Checked"</h2>
        <Checkbox attr:class=classes::root default_checked=CheckedState::True>
            <CheckboxIndicator attr:class=classes::indicator />
        </Checkbox>

        <h1>"Controlled"</h1>
        <h2>"Unchecked"</h2>
        <Checkbox attr:class=classes::root checked=CheckedState::False>
            <CheckboxIndicator attr:class=classes::indicator />
        </Checkbox>

        <h2>"Checked"</h2>
        <Checkbox attr:class=classes::root checked=CheckedState::True>
            <CheckboxIndicator attr:class=classes::indicator />
        </Checkbox>

        <h1>"Indeterminate"</h1>
        <Checkbox attr:class=classes::root checked=CheckedState::Indeterminate>
            <CheckboxIndicator attr:class=classes::indicator />
        </Checkbox>

        <h1>"Disabled"</h1>
        <Checkbox attr:class=classes::root default_checked=CheckedState::True disabled=true>
            <CheckboxIndicator attr:class=classes::indicator />
        </Checkbox>

        <h1>"Force mounted indicator"</h1>
        <Checkbox attr:class=classes::root>
            <CheckboxIndicator attr:class=classes::indicator force_mount=true />
        </Checkbox>

        <h1>"State attributes"</h1>
        <h2>"Unchecked"</h2>
        <Checkbox attr:class=classes::rootAttr>
            <CheckboxIndicator attr:class=classes::indicatorAttr />
        </Checkbox>

        <h2>"Checked"</h2>
        <Checkbox attr:class=classes::rootAttr default_checked=CheckedState::True>
            <CheckboxIndicator attr:class=classes::indicatorAttr />
        </Checkbox>

        <h2>"Indeterminate"</h2>
        <Checkbox attr:class=classes::rootAttr checked=CheckedState::Indeterminate>
            <CheckboxIndicator attr:class=classes::indicatorAttr />
        </Checkbox>

        <h2>"Disabled"</h2>
        <Checkbox attr:class=classes::rootAttr default_checked=CheckedState::True disabled=true>
            <CheckboxIndicator attr:class=classes::indicatorAttr />
        </Checkbox>

        <h2>"Force mounted indicator"</h2>
        <Checkbox attr:class=classes::rootAttr>
            <CheckboxIndicator attr:class=classes::indicatorAttr force_mount=true />
        </Checkbox>
    }
}
