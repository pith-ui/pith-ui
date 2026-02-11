use leptos::prelude::*;
use radix_leptos_toggle::*;

stylance::import_crate_style!(classes, "src/primitives/toggle.stories.module.css");

#[component]
pub fn Styled() -> impl IntoView {
    view! {
        <Toggle attr:class=classes::root>"Toggle"</Toggle>
    }
}

#[component]
pub fn Controlled() -> impl IntoView {
    let (pressed, set_pressed) = signal(true);

    view! {
        <Toggle
            attr:class=classes::root
            pressed=pressed
            on_pressed_change=move |value: bool| set_pressed.set(value)
        >
            {move || match pressed.get() {
                true => "On",
                false => "Off",
            }}
        </Toggle>
    }
}

#[component]
pub fn Chromatic() -> impl IntoView {
    view! {
        <h1>"Uncontrolled"</h1>
        <h2>"Off"</h2>
        <Toggle attr:class=classes::root>"Toggle"</Toggle>

        <h2>"On"</h2>
        <Toggle attr:class=classes::root default_pressed=true>"Toggle"</Toggle>

        <h1>"Controlled"</h1>
        <h2>"Off"</h2>
        <Toggle attr:class=classes::root pressed=false>"Toggle"</Toggle>

        <h2>"On"</h2>
        <Toggle attr:class=classes::root pressed=true>"Toggle"</Toggle>

        <h1>"Disabled"</h1>
        <Toggle attr:class=classes::root disabled=true>"Toggle"</Toggle>

        <h1>"State attributes"</h1>
        <Toggle attr:class=classes::rootAttr>"Toggle"</Toggle>
        <Toggle attr:class=classes::rootAttr disabled=true>"Toggle"</Toggle>
    }
}
