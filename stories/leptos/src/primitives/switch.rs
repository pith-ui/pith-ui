use leptos::{ev::Event, prelude::*};
use radix_leptos_label::*;
use radix_leptos_switch::*;

stylance::import_crate_style!(classes, "src/primitives/switch.stories.module.css");

#[component]
pub fn Styled() -> impl IntoView {
    view! {
        <p>"This switch is nested inside a label. The state is uncontrolled."</p>
        <Label attr:class=classes::label>
            "This is the label "
            <Switch attr:class=classes::root>
                <SwitchThumb attr:class=classes::thumb />
            </Switch>
        </Label>
    }
}

#[component]
pub fn Controlled() -> impl IntoView {
    let (checked, set_checked) = signal(true);

    view! {
        <p>"This switch is placed adjacent to its label. The state is controlled."</p>
        <Label attr:r#for="randBox" attr:class=classes::label>"This is the label"</Label>{' '}
        <Switch
            attr:id="randBox"
            attr:class=classes::root
            checked=checked
            on_checked_change=move |checked| set_checked.set(checked)
        >
            <SwitchThumb attr:class=classes::thumb />
        </Switch>
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
    let (checked, set_checked) = signal(false);

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
                    <Switch
                        attr:class=classes::root
                        name="optional"
                        checked=checked
                        on_checked_change=move |checked| {
                            set_checked.set(checked);
                            set_data.update(|data| {
                                data.optional = checked;
                            })
                        }
                    >
                        <SwitchThumb attr:class=classes::thumb />
                    </Switch>{' '}
                    "with label"
                </label>
            </fieldset>

            <br />
            <br />

            <fieldset>
                <legend>"required checked: " {move || format!("{}", data.with(|data| data.required))}</legend>
                <Switch
                    attr:class=classes::root
                    name="required"
                    required=true
                    on_checked_change=move |checked| {
                        set_data.update(|data| {
                            data.required = checked;
                        });
                    }
                >
                    <SwitchThumb attr:class=classes::thumb />
                </Switch>
            </fieldset>


            <br />
            <br />

            <fieldset>
                <legend>"stop propagation checked: " {move || format!("{}", data.with(|data| data.stopprop))}</legend>
                <Switch
                    attr:class=classes::root
                    name="stopprop"
                    on:click=move |event| event.stop_propagation()
                >
                    <SwitchThumb attr:class=classes::thumb />
                </Switch>
            </fieldset>

            <br />
            <br />

            <button>"Submit"</button>
        </form>
    }
}

#[component]
pub fn Chromatic() -> impl IntoView {
    view! {
        <h1>"Uncontrolled"</h1>
        <h2>"Off"</h2>
        <Switch attr:class=classes::root>
            <SwitchThumb attr:class=classes::thumb />
        </Switch>

        <h2>"On"</h2>
        <Switch attr:class=classes::root default_checked=true>
            <SwitchThumb attr:class=classes::thumb />
        </Switch>

        <h1>"Controlled"</h1>
        <h2>"Off"</h2>
        <Switch attr:class=classes::root checked=false>
            <SwitchThumb attr:class=classes::thumb />
        </Switch>

        <h2>"On"</h2>
        <Switch attr:class=classes::root checked=true>
            <SwitchThumb attr:class=classes::thumb />
        </Switch>

        <h1>"Disabled"</h1>
        <Switch attr:class=classes::root disabled=true>
            <SwitchThumb attr:class=classes::thumb />
        </Switch>

        <h1>"State attributes"</h1>
        <h2>"Unchecked"</h2>
        <Switch attr:class=classes::rootAttr>
            <SwitchThumb attr:class=classes::thumbAttr />
        </Switch>

        <h2>"Checked"</h2>
        <Switch attr:class=classes::rootAttr default_checked=true>
            <SwitchThumb attr:class=classes::thumbAttr />
        </Switch>

        <h2>"Disabled"</h2>
        <Switch attr:class=classes::rootAttr default_checked=true disabled=true>
            <SwitchThumb attr:class=classes::thumbAttr />
        </Switch>
    }
}
