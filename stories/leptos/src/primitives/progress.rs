use leptos::prelude::*;
use radix_leptos_progress::*;
use tailwind_fuse::*;

#[component]
pub fn Styled() -> impl IntoView {
    let root_class = Memo::new(move |_| RootClass::default().to_class());
    let indicator_class = Memo::new(move |_| IndicatorClass::default().to_class());

    let max = 150.0;
    let (value, percentage, set_value) = use_progress_value_state(Some(0.0), max);
    let toggle_indeterminate = use_indeterminate_toggle(value, set_value);

    view! {
        <div>
            <Progress attr:class=root_class value=value max=max>
                <ProgressIndicator
                    attr:class=indicator_class
                    attr:style=move || percentage.get().map(|percentage| format!("width: {}%", percentage))
                />
            </Progress>
            <hr />
            <button on:click=move |_| toggle_indeterminate.run(())>Toggle Indeterminate</button>
            <ProgressRange value=value on_value_change=Callback::new(move |val| set_value.set(Some(val))) max=max />
        </div>
    }
}

#[component]
pub fn Chromatic() -> impl IntoView {
    let root_class = Memo::new(move |_| RootClass::default().to_class());
    let chromatic_indicator_class =
        Memo::new(move |_| ChromaticIndicatorClass::default().to_class());
    let root_attr_class = Memo::new(move |_| RootAttrClass::default().to_class());
    let indicator_attr_class = Memo::new(move |_| IndicatorAttrClass::default().to_class());

    view! {
        <>
            <h1>"Loading (not started)"</h1>
            <Progress attr:class=root_class value=0.0>
                <ProgressIndicator attr:class=chromatic_indicator_class>"/"</ProgressIndicator>
            </Progress>

            <h1>"Loading (started)"</h1>
            <Progress attr:class=root_class value=30.0>
                <ProgressIndicator attr:class=chromatic_indicator_class>"/"</ProgressIndicator>
            </Progress>

            <h1>"Indeterminate"</h1>
            <Progress attr:class=root_class>
                <ProgressIndicator attr:class=chromatic_indicator_class>"/"</ProgressIndicator>
            </Progress>

            <h1>"Complete"</h1>
            <Progress attr:class=root_class value=100.0>
                <ProgressIndicator attr:class=chromatic_indicator_class>"/"</ProgressIndicator>
            </Progress>

            <h1>"State attributes"</h1>
            <h2>"Loading (started)"</h2>
            <Progress attr:class=root_attr_class value=30.0>
                <ProgressIndicator attr:class=indicator_attr_class>"/"</ProgressIndicator>
            </Progress>

            <h2>"Indeterminate"</h2>
            <Progress attr:class=root_attr_class>
                <ProgressIndicator attr:class=indicator_attr_class>"/"</ProgressIndicator>
            </Progress>

            <h2>"Complete"</h2>
            <Progress attr:class=root_attr_class value=100.0>
                <ProgressIndicator attr:class=indicator_attr_class>"/"</ProgressIndicator>
            </Progress>
        </>
    }
}

#[component]
fn ProgressRange(
    #[prop(into)] value: Signal<Option<f64>>,
    on_value_change: Callback<f64>,
    #[prop(into)] max: Signal<f64>,
) -> impl IntoView {
    let previous_value = use_previous_value(value);

    view! {
        <input
            type="range"
            disabled=move || value.get().is_none()
            prop:value=move || value.get().unwrap_or(previous_value.get())
            max=move || max.get()
            min="0"
            on:input=move |event: web_sys::Event| {
                use web_sys::wasm_bindgen::JsCast;
                if let Some(val) = event
                    .target()
                    .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                    .and_then(|input| input.value().parse::<f64>().ok())
                {
                    on_value_change.run(val);
                }
            }
        />
    }
}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "w-[400px] h-[20px] max-w-full border-[5px] border-solid border-[#111] box-content")]
struct RootClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "w-0 h-full bg-[crimson] transition-[background] duration-150 ease-[ease-out] data-[state=indeterminate]:bg-[#aaa] data-[state=complete]:bg-[green]"
)]
struct IndicatorClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "w-0 h-full bg-[crimson] transition-[background] duration-150 ease-[ease-out] data-[state=indeterminate]:bg-[#aaa] data-[state=complete]:bg-[green] before:content-[attr(data-value)] after:content-[attr(data-max)]"
)]
struct ChromaticIndicatorClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "bg-[rgba(0,0,255,0.3)] border-[2px] border-solid border-[blue] p-[10px] data-[state=loading]:border-[red] data-[state=indeterminate]:border-[purple] data-[state=complete]:border-[green]"
)]
struct RootAttrClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "bg-[rgba(0,0,255,0.3)] border-[2px] border-solid border-[blue] p-[10px] data-[state=loading]:border-[red] data-[state=indeterminate]:border-[purple] data-[state=complete]:border-[green] before:content-[attr(data-value)] after:content-[attr(data-max)]"
)]
struct IndicatorAttrClass {}

type ProgressValueState = (
    ReadSignal<Option<f64>>,
    Signal<Option<f64>>,
    WriteSignal<Option<f64>>,
);

fn use_progress_value_state(initial_state: Option<f64>, max: f64) -> ProgressValueState {
    let (value, set_value) = signal(initial_state);
    let percentage =
        Signal::derive(move || value.get().map(|value| ((value / max) * 100.0).round()));
    (value, percentage, set_value)
}

fn use_indeterminate_toggle(
    value: ReadSignal<Option<f64>>,
    set_value: WriteSignal<Option<f64>>,
) -> Callback<()> {
    let previous_value = use_previous_value(value.into());

    Callback::new(move |_| {
        set_value.update(move |val| {
            *val = match val {
                Some(_) => None,
                None => Some(previous_value.get()),
            };
        });
    })
}

fn use_previous_value(value: Signal<Option<f64>>) -> ReadSignal<f64> {
    let (previous, set_previous) = signal(value.get_untracked().unwrap_or(0.0));

    Effect::new(move |_| {
        if let Some(value) = value.get() {
            set_previous.set(value);
        }
    });

    previous
}
