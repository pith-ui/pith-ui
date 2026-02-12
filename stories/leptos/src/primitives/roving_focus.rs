use leptos::{context::Provider, ev, prelude::*};
use radix_leptos_direction::Direction;
use radix_leptos_roving_focus::*;
use web_sys::wasm_bindgen::JsCast;

#[derive(Clone)]
struct ButtonGroupContextValue {
    value: ReadSignal<Option<String>>,
    set_value: WriteSignal<Option<String>>,
}

#[component]
fn ButtonGroup(
    #[prop(into, optional)] orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] r#loop: MaybeProp<bool>,
    #[prop(into, optional)] default_value: Option<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let (value, set_value) = signal(default_value);
    let children = StoredValue::new(children);

    let flex_direction = Signal::derive(move || match orientation.get() {
        Some(Orientation::Vertical) => "column",
        _ => "row",
    });

    view! {
        <Provider value=ButtonGroupContextValue { value, set_value }>
            <RovingFocusGroup
                orientation=orientation
                dir=dir
                r#loop=r#loop
                attr:style=move || format!(
                    "display: inline-flex; flex-direction: {}; gap: 10px;",
                    flex_direction.get()
                )
            >
                {children.with_value(|children| children())}
            </RovingFocusGroup>
        </Provider>
    }
}

#[component]
fn Button(
    #[prop(into, optional)] value: Option<String>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] extra_style: MaybeProp<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let ctx = expect_context::<ButtonGroupContextValue>();
    let value_clone = value.clone();
    let is_selected = Signal::derive(move || {
        ctx.value
            .get()
            .is_some_and(|v| value_clone.as_ref().is_some_and(|val| v == *val))
    });
    let disabled_val = Signal::derive(move || disabled.get().unwrap_or(false));

    let btn_value = StoredValue::new(value);

    view! {
        <RovingFocusGroupItem
            as_child=true
            active=is_selected
            focusable=Signal::derive(move || !disabled_val.get())
        >
            <button
                disabled=move || disabled_val.get()
                style=move || {
                    let extra = extra_style.get().unwrap_or_default();
                    let selected_styles = if is_selected.get() {
                        "border-color: black; background-color: black; color: white;"
                    } else {
                        ""
                    };
                    format!(
                        "border: 1px solid; border-color: #ccc; padding: 5px 10px; border-radius: 5px; {selected_styles} {extra}"
                    )
                }
                on:click=move |_| {
                    if !disabled_val.get() {
                        ctx.set_value.set(btn_value.get_value());
                    }
                }
                on:focus=move |event: ev::FocusEvent| {
                    if ctx.value.get_untracked().is_some() {
                        let target: web_sys::HtmlElement = event
                            .target()
                            .expect("Event should have target.")
                            .unchecked_into();
                        target.click();
                    }
                }
            >
                {children.with_value(|children| children())}
            </button>
        </RovingFocusGroupItem>
    }
}

#[component]
pub fn Basic() -> impl IntoView {
    let (dir, set_dir) = signal(Direction::Ltr);

    view! {
        <div dir=move || match dir.get() {
            Direction::Ltr => "ltr",
            Direction::Rtl => "rtl",
        }>
            <h1>
                "Direction: "
                {move || match dir.get() {
                    Direction::Ltr => "ltr",
                    Direction::Rtl => "rtl",
                }}
                " "
                <button
                    type="button"
                    on:click=move |_| set_dir.update(|d| {
                        *d = match d {
                            Direction::Ltr => Direction::Rtl,
                            Direction::Rtl => Direction::Ltr,
                        };
                    })
                >
                    {move || match dir.get() {
                        Direction::Ltr => "Toggle to rtl",
                        Direction::Rtl => "Toggle to ltr",
                    }}
                </button>
            </h1>

            <h2>"no orientation (both) + no looping"</h2>
            <ButtonGroup dir=dir default_value="two".to_string()>
                <Button value="one".to_string()>"One"</Button>
                <Button value="two".to_string()>"Two"</Button>
                <Button disabled=true value="three".to_string()>"Three"</Button>
                <Button value="four".to_string()>"Four"</Button>
            </ButtonGroup>

            <h2>"no orientation (both) + looping"</h2>
            <ButtonGroup dir=dir r#loop=true>
                <Button value="hidden".to_string() extra_style="display: none">"Hidden"</Button>
                <Button value="one".to_string()>"One"</Button>
                <Button value="two".to_string()>"Two"</Button>
                <Button disabled=true value="three".to_string()>"Three"</Button>
                <Button value="four".to_string()>"Four"</Button>
            </ButtonGroup>

            <h2>"horizontal orientation + no looping"</h2>
            <ButtonGroup orientation=Orientation::Horizontal dir=dir>
                <Button value="one".to_string()>"One"</Button>
                <Button value="two".to_string()>"Two"</Button>
                <Button disabled=true value="three".to_string()>"Three"</Button>
                <Button value="four".to_string()>"Four"</Button>
            </ButtonGroup>

            <h2>"horizontal orientation + looping"</h2>
            <ButtonGroup orientation=Orientation::Horizontal dir=dir r#loop=true>
                <Button value="one".to_string()>"One"</Button>
                <Button value="two".to_string()>"Two"</Button>
                <Button disabled=true value="three".to_string()>"Three"</Button>
                <Button value="four".to_string()>"Four"</Button>
            </ButtonGroup>

            <h2>"vertical orientation + no looping"</h2>
            <ButtonGroup orientation=Orientation::Vertical dir=dir>
                <Button value="one".to_string()>"One"</Button>
                <Button value="two".to_string()>"Two"</Button>
                <Button disabled=true value="three".to_string()>"Three"</Button>
                <Button value="four".to_string()>"Four"</Button>
            </ButtonGroup>

            <h2>"vertical orientation + looping"</h2>
            <ButtonGroup orientation=Orientation::Vertical dir=dir r#loop=true>
                <Button value="one".to_string()>"One"</Button>
                <Button value="two".to_string()>"Two"</Button>
                <Button disabled=true value="three".to_string()>"Three"</Button>
                <Button value="four".to_string()>"Four"</Button>
            </ButtonGroup>
        </div>
    }
}

#[component]
pub fn Nested() -> impl IntoView {
    view! {
        <ButtonGroup orientation=Orientation::Vertical r#loop=true>
            <Button value="1".to_string()>"1"</Button>

            <div style="display: flex; flex-direction: column;">
                <Button value="2".to_string() extra_style="margin-bottom: 10px">"2"</Button>

                <ButtonGroup orientation=Orientation::Horizontal r#loop=true>
                    <Button value="2.1".to_string()>"2.1"</Button>
                    <Button value="2.2".to_string()>"2.2"</Button>
                    <Button disabled=true value="2.3".to_string()>"2.3"</Button>
                    <Button value="2.4".to_string()>"2.4"</Button>
                </ButtonGroup>
            </div>

            <Button value="3".to_string() disabled=true>"3"</Button>
            <Button value="4".to_string()>"4"</Button>
        </ButtonGroup>
    }
}

#[component]
pub fn EdgeCases() -> impl IntoView {
    let (extra, set_extra) = signal(false);
    let (disabled, set_disabled) = signal(false);
    let (hidden, set_hidden) = signal(false);
    let (disabled_3_to_5, set_disabled_3_to_5) = signal(false);

    view! {
        <button on:click=move |_| set_extra.update(|x| *x = !*x)>"Add/remove extra"</button>
        <button on:click=move |_| set_disabled.update(|x| *x = !*x)>"Disable/Enable \"One\""</button>
        <button on:click=move |_| set_hidden.update(|x| *x = !*x)>"Hide/show \"One\""</button>
        <button on:click=move |_| set_disabled_3_to_5.update(|x| *x = !*x)>"Disable/Enable \"Three\" to \"Five\""</button>
        <hr />

        <ButtonGroup>
            {move || extra.get().then(|| view! {
                <Button value="extra".to_string()>"Extra"</Button>
            })}
            <Button
                value="one".to_string()
                disabled=disabled
                extra_style=Signal::derive(move || match hidden.get() {
                    true => "display: none".to_string(),
                    false => String::new(),
                })
            >
                "One"
            </Button>
            <Button value="two".to_string() disabled=true>"Two"</Button>
            <Button value="three".to_string() disabled=disabled_3_to_5>"Three"</Button>
            <Button value="four".to_string() disabled=disabled_3_to_5 extra_style="display: none">"Four"</Button>
            <Button value="five".to_string() disabled=disabled_3_to_5>"Five"</Button>
        </ButtonGroup>

        <hr />
        <button type="button">"Focusable outside of group"</button>
    }
}
