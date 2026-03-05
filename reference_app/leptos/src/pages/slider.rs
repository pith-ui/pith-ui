use leptos::prelude::*;
use radix_leptos_primitives::slider::*;

#[component]
pub fn SliderPage() -> impl IntoView {
    let (disabled, set_disabled) = signal(false);
    let (orientation, set_orientation) = signal(Orientation::Horizontal);
    let (value, set_value) = signal(vec![50.0]);

    view! {
        // Wrap in reactive closure so the Slider remounts when orientation changes,
        // ensuring attr:class is properly applied to the new DOM element.
        {move || {
            let o = orientation.get();
            view! {
                <Slider
                    attr:class="slider-root"
                    disabled=disabled
                    orientation=o
                    value=value
                    on_value_change=Callback::new(move |v: Vec<f64>| set_value.set(v))
                    min=0.0
                    max=100.0
                    step=1.0
                >
                    <SliderTrack attr:class="slider-track">
                        <SliderRange attr:class="slider-range" />
                    </SliderTrack>
                    <SliderThumb attr:class="slider-thumb" attr:aria-label="Volume" />
                </Slider>
            }
        }}

        <br />
        <br />

        <span data-testid="slider-value">
            {move || value.get()[0] as i64}
        </span>

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
