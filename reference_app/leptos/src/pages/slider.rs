use leptos::prelude::*;
use cardo_ui::slider::*;

#[component]
pub fn SliderPage() -> impl IntoView {
    let (disabled, set_disabled) = signal(false);
    let (orientation, set_orientation) = signal(Orientation::Horizontal);
    let (value, set_value) = signal(vec![50.0]);

    view! {
        // Wrap in reactive closure so the Slider remounts when orientation changes,
        // ensuring class: is properly applied to the new DOM element.
        {move || {
            let o = orientation.get();
            view! {
                <Slider
                    class:slider-root=true
                    disabled=disabled
                    orientation=o
                    value=value
                    on_value_change=Callback::new(move |v: Vec<f64>| set_value.set(v))
                    min=0.0
                    max=100.0
                    step=1.0
                >
                    <SliderTrack class:slider-track=true>
                        <SliderRange class:slider-range=true />
                    </SliderTrack>
                    <SliderThumb class:slider-thumb=true attr:aria-label="Volume" />
                </Slider>
            }
        }}

        <br />
        <br />

        <span data-testid="slider-value">
            {move || value.get()[0] as i64}
        </span>

        <hr />

        <h3>"Uncontrolled"</h3>
        <Slider
            class:slider-root=true
            default_value=vec![30.0]
            min=0.0
            max=100.0
            step=1.0
            attr:data-testid="uncontrolled-slider"
        >
            <SliderTrack class:slider-track=true>
                <SliderRange class:slider-range=true />
            </SliderTrack>
            <SliderThumb class:slider-thumb=true attr:aria-label="Uncontrolled volume" attr:data-testid="uncontrolled-thumb" />
        </Slider>

        <hr />

        <br />
        <br />

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

        <fieldset>
            <legend>"orientation"</legend>
            <label>
                <input
                    type="radio"
                    name="orientation"
                    value="horizontal"
                    prop:checked=move || matches!(orientation.get(), Orientation::Horizontal)
                    on:change=move |_| set_orientation.set(Orientation::Horizontal)
                />
                " horizontal"
            </label>
            <br />
            <label>
                <input
                    type="radio"
                    name="orientation"
                    value="vertical"
                    prop:checked=move || matches!(orientation.get(), Orientation::Vertical)
                    on:change=move |_| set_orientation.set(Orientation::Vertical)
                />
                " vertical"
            </label>
        </fieldset>
    }
}
