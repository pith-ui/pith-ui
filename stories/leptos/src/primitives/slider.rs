use leptos::prelude::*;
use radix_leptos_direction::DirectionProvider;
use radix_leptos_slider::{
    Orientation, Slider as SliderRoot, SliderRange, SliderThumb, SliderTrack,
};
use web_sys::wasm_bindgen::JsCast;

stylance::import_crate_style!(classes, "src/primitives/slider.stories.module.css");

#[component]
pub fn Styled() -> impl IntoView {
    view! {
        <SliderRoot attr:class=classes::root>
            <SliderTrack attr:class=classes::track>
                <SliderRange attr:class=classes::range />
            </SliderTrack>
            <SliderThumb attr:class=classes::thumb />
        </SliderRoot>
    }
}

#[component]
pub fn WithOnValueCommit() -> impl IntoView {
    view! {
        <>
            <SliderRoot
                attr:class=classes::root
                default_value=vec![20.0]
                on_value_commit=Callback::new(move |value: Vec<f64>| {
                    web_sys::console::log_1(&format!("onValueCommit: {:?}", value).into());
                })
            >
                <SliderTrack attr:class=classes::track>
                    <SliderRange attr:class=classes::range />
                </SliderTrack>
                <SliderThumb attr:class=classes::thumb />
            </SliderRoot>
            <p>"Check the console for the onValueCommit log"</p>
        </>
    }
}

#[component]
pub fn RightToLeft() -> impl IntoView {
    view! {
        <SliderRoot attr:class=classes::root dir=radix_leptos_direction::Direction::Rtl>
            <SliderTrack attr:class=classes::track>
                <SliderRange attr:class=classes::range />
            </SliderTrack>
            <SliderThumb attr:class=classes::thumb />
        </SliderRoot>
    }
}

#[component]
pub fn Horizontal() -> impl IntoView {
    view! {
        <div style:display="flex" style:flex-direction="column" style:gap="50px">
            <SliderRoot
                attr:class=classes::root
                default_value=vec![10.0, 30.0]
                min_steps_between_thumbs=1.0
                on_value_change=Callback::new(move |value: Vec<f64>| {
                    web_sys::console::log_1(&format!("{:?}", value).into());
                })
            >
                <SliderTrack attr:class=classes::track>
                    <SliderRange attr:class=classes::range />
                </SliderTrack>
                <SliderThumb attr:class=classes::thumb />
                <SliderThumb attr:class=classes::thumb />
            </SliderRoot>

            <SliderRoot attr:class=classes::root default_value=vec![10.0]>
                <SliderTrack attr:class=classes::track>
                    <SliderRange attr:class=classes::range />
                </SliderTrack>
                <SliderThumb attr:class=classes::thumb />
            </SliderRoot>
        </div>
    }
}

#[component]
pub fn Vertical() -> impl IntoView {
    view! {
        <div style:display="flex" style:gap="50px">
            <SliderRoot
                attr:class=classes::root
                default_value=vec![10.0, 30.0]
                orientation=Orientation::Vertical
            >
                <SliderTrack attr:class=classes::track>
                    <SliderRange attr:class=classes::range />
                </SliderTrack>
                <SliderThumb attr:class=classes::thumb />
                <SliderThumb attr:class=classes::thumb />
            </SliderRoot>

            <SliderRoot
                attr:class=classes::root
                default_value=vec![10.0]
                orientation=Orientation::Vertical
            >
                <SliderTrack attr:class=classes::track>
                    <SliderRange attr:class=classes::range />
                </SliderTrack>
                <SliderThumb attr:class=classes::thumb />
            </SliderRoot>
        </div>
    }
}

#[component]
pub fn Inversions() -> impl IntoView {
    view! {
        <>
            <h1>"Inversions"</h1>
            <h2>"Horizontal"</h2>
            <div style:display="flex" style:gap="50px">
                <div style:flex="1">
                    <h3>"LTR"</h3>
                    <h4>"default"</h4>
                    <SliderRoot attr:class=classes::root default_value=vec![20.0]>
                        <SliderTrack attr:class=classes::track>
                            <SliderRange attr:class=classes::range />
                        </SliderTrack>
                        <SliderThumb attr:class=classes::thumb />
                    </SliderRoot>

                    <h4>"Inverted"</h4>
                    <SliderRoot attr:class=classes::root default_value=vec![20.0] inverted=true>
                        <SliderTrack attr:class=classes::track>
                            <SliderRange attr:class=classes::range />
                        </SliderTrack>
                        <SliderThumb attr:class=classes::thumb />
                    </SliderRoot>
                </div>

                <div style:flex="1">
                    <h3>"RTL"</h3>
                    <h4>"Default"</h4>
                    <SliderRoot attr:class=classes::root default_value=vec![20.0] dir=radix_leptos_direction::Direction::Rtl>
                        <SliderTrack attr:class=classes::track>
                            <SliderRange attr:class=classes::range />
                        </SliderTrack>
                        <SliderThumb attr:class=classes::thumb />
                    </SliderRoot>

                    <h4>"Inverted"</h4>
                    <SliderRoot attr:class=classes::root default_value=vec![20.0] dir=radix_leptos_direction::Direction::Rtl inverted=true>
                        <SliderTrack attr:class=classes::track>
                            <SliderRange attr:class=classes::range />
                        </SliderTrack>
                        <SliderThumb attr:class=classes::thumb />
                    </SliderRoot>
                </div>
            </div>

            <h2>"Vertical"</h2>
            <div style:display="flex" style:gap="50px">
                <div style:flex="1">
                    <h3>"LTR"</h3>
                    <div style:display="flex" style:gap="50px">
                        <div>
                            <h4>"Default"</h4>
                            <SliderRoot attr:class=classes::root default_value=vec![20.0] orientation=Orientation::Vertical>
                                <SliderTrack attr:class=classes::track>
                                    <SliderRange attr:class=classes::range />
                                </SliderTrack>
                                <SliderThumb attr:class=classes::thumb />
                            </SliderRoot>
                        </div>
                        <div>
                            <h4>"Inverted"</h4>
                            <SliderRoot attr:class=classes::root default_value=vec![20.0] orientation=Orientation::Vertical inverted=true>
                                <SliderTrack attr:class=classes::track>
                                    <SliderRange attr:class=classes::range />
                                </SliderTrack>
                                <SliderThumb attr:class=classes::thumb />
                            </SliderRoot>
                        </div>
                    </div>
                </div>

                <div style:flex="1">
                    <h3>"RTL"</h3>
                    <div style:display="flex" style:gap="50px">
                        <div>
                            <h4>"Default"</h4>
                            <SliderRoot attr:class=classes::root default_value=vec![20.0] dir=radix_leptos_direction::Direction::Rtl orientation=Orientation::Vertical>
                                <SliderTrack attr:class=classes::track>
                                    <SliderRange attr:class=classes::range />
                                </SliderTrack>
                                <SliderThumb attr:class=classes::thumb />
                            </SliderRoot>
                        </div>
                        <div>
                            <h4>"Inverted"</h4>
                            <SliderRoot attr:class=classes::root default_value=vec![20.0] dir=radix_leptos_direction::Direction::Rtl orientation=Orientation::Vertical inverted=true>
                                <SliderTrack attr:class=classes::track>
                                    <SliderRange attr:class=classes::range />
                                </SliderTrack>
                                <SliderThumb attr:class=classes::thumb />
                            </SliderRoot>
                        </div>
                    </div>
                </div>
            </div>
        </>
    }
}

#[component]
pub fn WithMinimumStepsBetweenThumbs() -> impl IntoView {
    view! {
        <SliderRoot attr:class=classes::root default_value=vec![10.0, 30.0] min_steps_between_thumbs=3.0>
            <SliderTrack attr:class=classes::track>
                <SliderRange attr:class=classes::range />
            </SliderTrack>
            <SliderThumb attr:class=classes::thumb />
            <SliderThumb attr:class=classes::thumb />
        </SliderRoot>
    }
}

#[component]
pub fn WithMultipleRanges() -> impl IntoView {
    let (min_steps, set_min_steps) = signal(0.0_f64);

    view! {
        <>
            <label>
                "Minimum steps between thumbs: "
                <input
                    type="number"
                    prop:value=move || min_steps.get()
                    on:input=move |ev| {
                        if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                            set_min_steps.set(val);
                        }
                    }
                    style:width="30px"
                />
            </label>

            <br /><br />

            <SliderRoot
                attr:class=classes::root
                default_value=vec![10.0, 15.0, 20.0, 80.0]
                min_steps_between_thumbs=min_steps
            >
                <SliderTrack attr:class=classes::track>
                    <SliderRange attr:class=classes::range />
                </SliderTrack>
                <SliderThumb attr:class=classes::thumb />
                <SliderThumb attr:class=classes::thumb />
                <SliderThumb attr:class=classes::thumb />
                <SliderThumb attr:class=classes::thumb />
            </SliderRoot>
        </>
    }
}

#[component]
pub fn SmallSteps() -> impl IntoView {
    let (value, set_value) = signal(vec![0.1]);

    view! {
        <>
            <SliderRoot
                attr:class=classes::root
                value=value
                on_value_change=Callback::new(move |v: Vec<f64>| set_value.set(v))
                min=0.1
                max=0.2
                step=0.003
            >
                <SliderTrack attr:class=classes::track>
                    <SliderRange attr:class=classes::range />
                </SliderTrack>
                <SliderThumb attr:class=classes::thumb />
            </SliderRoot>
            <div>{move || format!("{:?}", value.get())}</div>
        </>
    }
}

#[component]
pub fn WithinForm() -> impl IntoView {
    let (single, set_single) = signal("0".to_string());
    let (multiple, set_multiple) = signal("10,15,20,80".to_string());
    let (price, set_price) = signal("{\"min\":\"30\",\"max\":\"70\"}".to_string());

    let serialize_form = move |form: &web_sys::HtmlFormElement| {
        let form_data = web_sys::FormData::new_with_form(form).unwrap();
        if let Some(val) = form_data.get("single").as_string() {
            set_single.set(val);
        }
        let multiple_vals: Vec<String> = form_data
            .get_all("multiple")
            .iter()
            .filter_map(|v| v.as_string())
            .collect();
        if !multiple_vals.is_empty() {
            set_multiple.set(multiple_vals.join(","));
        }
        let price_min = form_data.get("price[min]").as_string().unwrap_or_default();
        let price_max = form_data.get("price[max]").as_string().unwrap_or_default();
        set_price.set(format!(
            "{{\"min\":\"{price_min}\",\"max\":\"{price_max}\"}}"
        ));
    };

    view! {
        <form
            on:submit=move |ev: leptos::ev::SubmitEvent| {
                ev.prevent_default();
                let form: web_sys::HtmlFormElement = ev.target().unwrap().unchecked_into();
                let form_data = web_sys::FormData::new_with_form(&form).unwrap();
                web_sys::console::log_1(&format!("{:?}", form_data).into());
            }
            on:input=move |ev: leptos::ev::Event| {
                let form: web_sys::HtmlFormElement = ev.current_target().unwrap().unchecked_into();
                serialize_form(&form);
            }
        >
            <fieldset>
                <legend>"Single value: " {move || single.get()}</legend>
                <SliderRoot name="single" default_value=vec![0.0] attr:class=classes::root>
                    <SliderTrack attr:class=classes::track>
                        <SliderRange attr:class=classes::range />
                    </SliderTrack>
                    <SliderThumb attr:class=classes::thumb />
                </SliderRoot>
            </fieldset>

            <br /><br />

            <fieldset>
                <legend>"Multiple value: " {move || multiple.get()}</legend>
                <SliderRoot name="multiple" default_value=vec![10.0, 15.0, 20.0, 80.0] attr:class=classes::root>
                    <SliderTrack attr:class=classes::track>
                        <SliderRange attr:class=classes::range />
                    </SliderTrack>
                    <SliderThumb attr:class=classes::thumb />
                    <SliderThumb attr:class=classes::thumb />
                    <SliderThumb attr:class=classes::thumb />
                    <SliderThumb attr:class=classes::thumb />
                </SliderRoot>
            </fieldset>

            <br /><br />

            <fieldset>
                <legend>"Multiple values (with named thumbs): " {move || price.get()}</legend>
                <SliderRoot default_value=vec![30.0, 70.0] attr:class=classes::root>
                    <SliderTrack attr:class=classes::track>
                        <SliderRange attr:class=classes::range />
                    </SliderTrack>
                    <SliderThumb attr:class=classes::thumb name="price[min]" />
                    <SliderThumb attr:class=classes::thumb name="price[max]" />
                </SliderRoot>
            </fieldset>

            <button type="submit">"Submit"</button>
        </form>
    }
}

/// In React, this story wraps sliders in `React.StrictMode` to verify the component handles
/// double-invoked effects and lifecycle methods correctly. Leptos has no equivalent strict mode,
/// so this story simply renders the same slider configurations to maintain story parity.
#[component]
pub fn Strict() -> impl IntoView {
    view! {
        <>
            <SliderRoot attr:class=classes::root>
                <SliderTrack attr:class=classes::track>
                    <SliderRange attr:class=classes::range />
                </SliderTrack>
                <SliderThumb attr:class=classes::thumb />
            </SliderRoot>
            <SliderRoot attr:class=classes::root default_value=vec![10.0, 15.0, 20.0, 80.0]>
                <SliderTrack attr:class=classes::track>
                    <SliderRange attr:class=classes::range />
                </SliderTrack>
                <SliderThumb attr:class=classes::thumb />
                <SliderThumb attr:class=classes::thumb />
                <SliderThumb attr:class=classes::thumb />
                <SliderThumb attr:class=classes::thumb />
            </SliderRoot>
        </>
    }
}

#[component]
pub fn Chromatic() -> impl IntoView {
    view! {
        <>
            <h1>"Uncontrolled"</h1>
            <h2>"LTR"</h2>
            <SliderRoot attr:class=classes::root default_value=vec![20.0]>
                <SliderTrack attr:class=classes::track>
                    <SliderRange attr:class=classes::range />
                </SliderTrack>
                <SliderThumb attr:class=classes::thumb />
            </SliderRoot>
            <SliderRoot attr:class=classes::root default_value=vec![10.0, 30.0]>
                <SliderTrack attr:class=classes::track>
                    <SliderRange attr:class=classes::range />
                </SliderTrack>
                <SliderThumb attr:class=classes::thumb />
                <SliderThumb attr:class=classes::thumb />
            </SliderRoot>

            <h2>"RTL"</h2>
            <SliderRoot attr:class=classes::root default_value=vec![20.0] dir=radix_leptos_direction::Direction::Rtl>
                <SliderTrack attr:class=classes::track>
                    <SliderRange attr:class=classes::range />
                </SliderTrack>
                <SliderThumb attr:class=classes::thumb />
            </SliderRoot>
            <SliderRoot attr:class=classes::root default_value=vec![10.0, 30.0] dir=radix_leptos_direction::Direction::Rtl>
                <SliderTrack attr:class=classes::track>
                    <SliderRange attr:class=classes::range />
                </SliderTrack>
                <SliderThumb attr:class=classes::thumb />
                <SliderThumb attr:class=classes::thumb />
            </SliderRoot>

            <h2>"Multiple ranges"</h2>
            <SliderRoot attr:class=classes::root default_value=vec![10.0, 15.0, 20.0, 80.0]>
                <SliderTrack attr:class=classes::track>
                    <SliderRange attr:class=classes::range />
                </SliderTrack>
                <SliderThumb attr:class=classes::thumb />
                <SliderThumb attr:class=classes::thumb />
                <SliderThumb attr:class=classes::thumb />
                <SliderThumb attr:class=classes::thumb />
            </SliderRoot>

            <h1>"Controlled"</h1>
            <h2>"LTR"</h2>
            <SliderRoot attr:class=classes::root value=vec![20.0]>
                <SliderTrack attr:class=classes::track>
                    <SliderRange attr:class=classes::range />
                </SliderTrack>
                <SliderThumb attr:class=classes::thumb />
            </SliderRoot>
            <SliderRoot attr:class=classes::root value=vec![10.0, 30.0]>
                <SliderTrack attr:class=classes::track>
                    <SliderRange attr:class=classes::range />
                </SliderTrack>
                <SliderThumb attr:class=classes::thumb />
                <SliderThumb attr:class=classes::thumb />
            </SliderRoot>

            <h1>"Direction"</h1>
            <h2>"Prop"</h2>
            <SliderRoot attr:class=classes::root value=vec![20.0] dir=radix_leptos_direction::Direction::Rtl>
                <SliderTrack attr:class=classes::track>
                    <SliderRange attr:class=classes::range />
                </SliderTrack>
                <SliderThumb attr:class=classes::thumb />
            </SliderRoot>
            <SliderRoot attr:class=classes::root value=vec![10.0, 30.0] dir=radix_leptos_direction::Direction::Rtl>
                <SliderTrack attr:class=classes::track>
                    <SliderRange attr:class=classes::range />
                </SliderTrack>
                <SliderThumb attr:class=classes::thumb />
                <SliderThumb attr:class=classes::thumb />
            </SliderRoot>

            <h2>"Inherited"</h2>
            <DirectionProvider direction=radix_leptos_direction::Direction::Rtl>
                <SliderRoot attr:class=classes::root value=vec![20.0]>
                    <SliderTrack attr:class=classes::track>
                        <SliderRange attr:class=classes::range />
                    </SliderTrack>
                    <SliderThumb attr:class=classes::thumb />
                </SliderRoot>
                <SliderRoot attr:class=classes::root value=vec![10.0, 30.0]>
                    <SliderTrack attr:class=classes::track>
                        <SliderRange attr:class=classes::range />
                    </SliderTrack>
                    <SliderThumb attr:class=classes::thumb />
                    <SliderThumb attr:class=classes::thumb />
                </SliderRoot>
            </DirectionProvider>

            <h1>"Scenarios"</h1>
            <h2>"Extremes"</h2>
            <SliderRoot attr:class=classes::root default_value=vec![0.0, 100.0]>
                <SliderTrack attr:class=classes::track>
                    <SliderRange attr:class=classes::range />
                </SliderTrack>
                <SliderThumb attr:class=classes::thumb />
                <SliderThumb attr:class=classes::thumb />
            </SliderRoot>

            <h2>"0 case"</h2>
            <SliderRoot attr:class=classes::root default_value=vec![0.0] min=-100.0 max=100.0>
                <SliderTrack attr:class=classes::track>
                    <SliderRange attr:class=classes::range />
                </SliderTrack>
                <SliderThumb attr:class=classes::thumb />
            </SliderRoot>

            <h2>"Multiple ranges"</h2>
            <SliderRoot attr:class=classes::root value=vec![10.0, 15.0, 20.0, 80.0]>
                <SliderTrack attr:class=classes::track>
                    <SliderRange attr:class=classes::range />
                </SliderTrack>
                <SliderThumb attr:class=classes::thumb />
                <SliderThumb attr:class=classes::thumb />
                <SliderThumb attr:class=classes::thumb />
                <SliderThumb attr:class=classes::thumb />
            </SliderRoot>

            <h2>"Vertical"</h2>
            <div style:display="flex">
                <SliderRoot attr:class=classes::root default_value=vec![10.0, 30.0] orientation=Orientation::Vertical>
                    <SliderTrack attr:class=classes::track>
                        <SliderRange attr:class=classes::range />
                    </SliderTrack>
                    <SliderThumb attr:class=classes::thumb />
                    <SliderThumb attr:class=classes::thumb />
                </SliderRoot>
                <SliderRoot attr:class=classes::root default_value=vec![20.0] orientation=Orientation::Vertical>
                    <SliderTrack attr:class=classes::track>
                        <SliderRange attr:class=classes::range />
                    </SliderTrack>
                    <SliderThumb attr:class=classes::thumb />
                </SliderRoot>
            </div>

            <h2>"Out of bound value (negative)"</h2>
            <SliderRoot attr:class=classes::root default_value=vec![-9000.0] min=0.0 max=100.0>
                <SliderTrack attr:class=classes::track>
                    <SliderRange attr:class=classes::range />
                </SliderTrack>
                <SliderThumb attr:class=classes::thumb />
            </SliderRoot>

            <h2>"Out of bound value (positive)"</h2>
            <SliderRoot attr:class=classes::root default_value=vec![9000.0] min=0.0 max=100.0>
                <SliderTrack attr:class=classes::track>
                    <SliderRange attr:class=classes::range />
                </SliderTrack>
                <SliderThumb attr:class=classes::thumb />
            </SliderRoot>

            <h1>"Disabled"</h1>
            <SliderRoot attr:class=classes::root default_value=vec![20.0] disabled=true>
                <SliderTrack attr:class=classes::track>
                    <SliderRange attr:class=classes::range />
                </SliderTrack>
                <SliderThumb attr:class=classes::thumb />
            </SliderRoot>

            <Inversions />

            <h1>"State attributes"</h1>
            <h2>"Default"</h2>
            <SliderRoot attr:class=classes::rootAttr default_value=vec![20.0]>
                <SliderTrack attr:class=classes::trackAttr>
                    <SliderRange attr:class=classes::rangeAttr />
                </SliderTrack>
                <SliderThumb attr:class=classes::thumbAttr />
            </SliderRoot>

            <h2>"Disabled"</h2>
            <SliderRoot attr:class=classes::rootAttr default_value=vec![20.0] disabled=true>
                <SliderTrack attr:class=classes::trackAttr>
                    <SliderRange attr:class=classes::rangeAttr />
                </SliderTrack>
                <SliderThumb attr:class=classes::thumbAttr />
            </SliderRoot>
        </>
    }
}
