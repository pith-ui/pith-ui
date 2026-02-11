use leptos::prelude::*;
use radix_leptos_accordion::*;
use web_sys::wasm_bindgen::{JsCast, closure::Closure};

stylance::import_crate_style!(
    accordion_classes,
    "src/primitives/accordion.stories.module.css"
);

fn render_standard_items() -> impl IntoView {
    view! {
        <AccordionItem value="one".to_string() attr:class=accordion_classes::item>
            <AccordionHeader attr:class=accordion_classes::header>
                <AccordionTrigger attr:class=accordion_classes::trigger>"One"</AccordionTrigger>
            </AccordionHeader>
            <AccordionContent attr:class=accordion_classes::content>
                "Per erat orci nostra luctus sociosqu mus risus penatibus, duis elit vulputate viverra integer ullamcorper congue curabitur sociis, nisi malesuada scelerisque quam suscipit habitant sed."
            </AccordionContent>
        </AccordionItem>
        <AccordionItem value="two".to_string() attr:class=accordion_classes::item>
            <AccordionHeader attr:class=accordion_classes::header>
                <AccordionTrigger attr:class=accordion_classes::trigger>"Two"</AccordionTrigger>
            </AccordionHeader>
            <AccordionContent attr:class=accordion_classes::content>
                "Cursus sed mattis commodo fermentum conubia ipsum pulvinar sagittis, diam eget bibendum porta nascetur ac dictum, leo tellus dis integer platea ultrices mi."
            </AccordionContent>
        </AccordionItem>
        <AccordionItem value="three".to_string() disabled=true attr:class=accordion_classes::item>
            <AccordionHeader attr:class=accordion_classes::header>
                <AccordionTrigger attr:class=accordion_classes::trigger>"Three (disabled)"</AccordionTrigger>
            </AccordionHeader>
            <AccordionContent attr:class=accordion_classes::content>
                "Sociis hac sapien turpis conubia sagittis justo dui, inceptos penatibus feugiat himenaeos euismod magna, nec tempor pulvinar eu etiam mattis."
            </AccordionContent>
        </AccordionItem>
        <AccordionItem value="four".to_string() attr:class=accordion_classes::item>
            <AccordionHeader attr:class=accordion_classes::header>
                <AccordionTrigger attr:class=accordion_classes::trigger>"Four"</AccordionTrigger>
            </AccordionHeader>
            <AccordionContent attr:class=accordion_classes::content>
                "Odio placerat "
                <a href="#">"quisque"</a>
                " sapien sagittis non sociis ligula penatibus dignissim vitae, enim vulputate nullam semper potenti etiam volutpat libero."
                <button>"Cool"</button>
            </AccordionContent>
        </AccordionItem>
    }
}

#[component]
pub fn Single() -> impl IntoView {
    let (value_one, set_value_one) = signal("one".to_string());

    view! {
        <h1>"Uncontrolled"</h1>
        <Accordion r#type=AccordionType::Single attr:class=accordion_classes::root>
            {render_standard_items()}
        </Accordion>

        <h1>"Controlled"</h1>
        <Accordion
            r#type=AccordionType::Single
            value=value_one
            on_value_change=Callback::new(move |v: String| set_value_one.set(v))
            attr:class=accordion_classes::root
        >
            {render_standard_items()}
        </Accordion>

        <h1>"Collapsible"</h1>
        <Accordion
            r#type=AccordionType::Single
            default_value="one".to_string()
            collapsible=true
            attr:class=accordion_classes::root
        >
            {render_standard_items()}
        </Accordion>
    }
}

#[component]
pub fn Multiple() -> impl IntoView {
    let (value, set_value) = signal(vec!["one".to_string(), "two".to_string()]);

    view! {
        <h1>"Uncontrolled"</h1>
        <Accordion r#type=AccordionType::Multiple attr:class=accordion_classes::root>
            {render_standard_items()}
        </Accordion>

        <h1>"Controlled"</h1>
        <Accordion
            r#type=AccordionType::Multiple
            values=value
            on_values_change=Callback::new(move |v: Vec<String>| set_value.set(v))
            attr:class=accordion_classes::root
        >
            {render_standard_items()}
        </Accordion>
    }
}

#[component]
pub fn Animated() -> impl IntoView {
    let values: &[&str] = &["One", "Two", "Three", "Four"];

    let (count, set_count) = signal(1u32);
    let (has_dynamic_content, set_has_dynamic_content) = signal(false);
    let timeout_handle = StoredValue::new(None::<i32>);

    Effect::new(move |_| {
        // Clear any previous timeout
        if let Some(handle) = timeout_handle.get_value() {
            web_sys::window()
                .expect("Window should exist.")
                .clear_timeout_with_handle(handle);
            timeout_handle.set_value(None);
        }

        if has_dynamic_content.get() {
            let _count = count.get(); // track count to re-run when it changes

            let closure = Closure::once_into_js(move || {
                set_count.update(|c| {
                    let next = if *c < 5 { *c + 1 } else { *c };
                    if next == 5 {
                        set_has_dynamic_content.set(false);
                    }
                    *c = next;
                });
            });

            let handle = web_sys::window()
                .expect("Window should exist.")
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    closure.unchecked_ref(),
                    3000,
                )
                .expect("setTimeout should succeed.");

            timeout_handle.set_value(Some(handle));
        }
    });

    Owner::on_cleanup(move || {
        if let Some(handle) = timeout_handle.get_value() {
            web_sys::window()
                .expect("Window should exist.")
                .clear_timeout_with_handle(handle);
        }
    });

    view! {
        <label>
            <input
                type="checkbox"
                prop:checked=move || has_dynamic_content.get()
                on:change=move |ev| {
                    let checked = event_target_checked(&ev);
                    if checked {
                        set_count.set(1);
                    }
                    set_has_dynamic_content.set(checked);
                }
            />
            " Dynamic content"
        </label>
        <br />
        <br />

        <h1>"Closed by default"</h1>
        <Accordion r#type=AccordionType::Single attr:class=accordion_classes::root>
            {values.iter().map(|value| {
                let value = value.to_string();
                let value_stored = StoredValue::new(value.clone());
                view! {
                    <AccordionItem value=value attr:class=accordion_classes::item>
                        <AccordionHeader attr:class=accordion_classes::header>
                            <AccordionTrigger attr:class=accordion_classes::trigger>
                                {move || value_stored.get_value()}
                            </AccordionTrigger>
                        </AccordionHeader>
                        <AccordionContent attr:class=accordion_classes::animatedContent>
                            {move || {
                                (0..count.get()).map(|_| {
                                    view! {
                                        <div style="padding: 10px;">
                                            "Per erat orci nostra luctus sociosqu mus risus penatibus, duis elit vulputate viverra integer ullamcorper congue curabitur sociis, nisi malesuada scelerisque quam suscipit habitant sed."
                                        </div>
                                    }
                                }).collect_view()
                            }}
                        </AccordionContent>
                    </AccordionItem>
                }
            }).collect_view()}
        </Accordion>

        <h1>"Open by default"</h1>
        <Accordion r#type=AccordionType::Single default_value="One".to_string() attr:class=accordion_classes::root>
            {values.iter().map(|value| {
                let value = value.to_string();
                let value_stored = StoredValue::new(value.clone());
                view! {
                    <AccordionItem value=value attr:class=accordion_classes::item>
                        <AccordionHeader attr:class=accordion_classes::header>
                            <AccordionTrigger attr:class=accordion_classes::trigger>
                                {move || value_stored.get_value()}
                            </AccordionTrigger>
                        </AccordionHeader>
                        <AccordionContent attr:class=accordion_classes::animatedContent>
                            {move || {
                                (0..count.get()).map(|_| {
                                    view! {
                                        <div style="padding: 10px;">
                                            "Per erat orci nostra luctus sociosqu mus risus penatibus, duis elit vulputate viverra integer ullamcorper congue curabitur sociis, nisi malesuada scelerisque quam suscipit habitant sed."
                                        </div>
                                    }
                                }).collect_view()
                            }}
                        </AccordionContent>
                    </AccordionItem>
                }
            }).collect_view()}
        </Accordion>
    }
}

#[component]
pub fn Animated2D() -> impl IntoView {
    let values: &[&str] = &["One", "Two", "Three", "Four"];

    view! {
        <Accordion r#type=AccordionType::Single attr:class=accordion_classes::root>
            {values.iter().map(|value| {
                let value = value.to_string();
                let value_stored = StoredValue::new(value.clone());
                view! {
                    <AccordionItem value=value attr:class=accordion_classes::item>
                        <AccordionHeader attr:class=accordion_classes::header>
                            <AccordionTrigger attr:class=accordion_classes::trigger>
                                {move || value_stored.get_value()}
                            </AccordionTrigger>
                        </AccordionHeader>
                        <AccordionContent attr:class=accordion_classes::animated2DContent>
                            <div style="padding: 10px; background: whitesmoke; overflow: hidden;">
                                <div style="width: calc(20em - 20px); height: 100px;">
                                    "Per erat orci nostra luctus sociosqu mus risus penatibus, duis elit vulputate viverra integer ullamcorper congue curabitur sociis, nisi malesuada scelerisque quam suscipit habitant sed."
                                </div>
                            </div>
                        </AccordionContent>
                    </AccordionItem>
                }
            }).collect_view()}
        </Accordion>
    }
}

#[component]
pub fn AnimatedControlled() -> impl IntoView {
    let (value, set_value) = signal(vec![
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
    ]);

    view! {
        <Accordion
            r#type=AccordionType::Multiple
            values=value
            on_values_change=Callback::new(move |v: Vec<String>| set_value.set(v))
            attr:class=accordion_classes::root
        >
            <AccordionItem value="one".to_string() attr:class=accordion_classes::item>
                <AccordionHeader attr:class=accordion_classes::header>
                    <AccordionTrigger attr:class=accordion_classes::trigger>"One"</AccordionTrigger>
                </AccordionHeader>
                <AccordionContent attr:class=accordion_classes::animatedContent>
                    "Per erat orci nostra luctus sociosqu mus risus penatibus, duis elit vulputate viverra integer ullamcorper congue curabitur sociis, nisi malesuada scelerisque quam suscipit habitant sed."
                </AccordionContent>
            </AccordionItem>
            <AccordionItem value="two".to_string() attr:class=accordion_classes::item>
                <AccordionHeader attr:class=accordion_classes::header>
                    <AccordionTrigger attr:class=accordion_classes::trigger>"Two"</AccordionTrigger>
                </AccordionHeader>
                <AccordionContent attr:class=accordion_classes::animatedContent>
                    "Cursus sed mattis commodo fermentum conubia ipsum pulvinar sagittis, diam eget bibendum porta nascetur ac dictum, leo tellus dis integer platea ultrices mi."
                </AccordionContent>
            </AccordionItem>
            <AccordionItem value="three".to_string() attr:class=accordion_classes::item>
                <AccordionHeader attr:class=accordion_classes::header>
                    <AccordionTrigger attr:class=accordion_classes::trigger>"Three"</AccordionTrigger>
                </AccordionHeader>
                <AccordionContent attr:class=accordion_classes::animatedContent>
                    "Sociis hac sapien turpis conubia sagittis justo dui, inceptos penatibus feugiat himenaeos euismod magna, nec tempor pulvinar eu etiam mattis."
                </AccordionContent>
            </AccordionItem>
            <AccordionItem value="four".to_string() attr:class=accordion_classes::item>
                <AccordionHeader attr:class=accordion_classes::header>
                    <AccordionTrigger attr:class=accordion_classes::trigger>"Four"</AccordionTrigger>
                </AccordionHeader>
                <AccordionContent attr:class=accordion_classes::animatedContent>
                    "Odio placerat "
                    <a href="#">"quisque"</a>
                    " sapien sagittis non sociis ligula penatibus dignissim vitae, enim vulputate nullam semper potenti etiam volutpat libero."
                    <button>"Cool"</button>
                </AccordionContent>
            </AccordionItem>
        </Accordion>
    }
}

#[component]
pub fn OutsideViewport() -> impl IntoView {
    view! {
        <p>"Scroll down to see tabs"</p>
        <div style="height: 150vh;" />
        <p>"When accordion buttons are focused and the user is navigating via keyboard, the page should not scroll unless the next tab is entering the viewport."</p>
        <Accordion r#type=AccordionType::Single attr:class=accordion_classes::root>
            {render_standard_items()}
        </Accordion>
        <div style="height: 150vh;" />
    }
}

#[component]
pub fn Horizontal() -> impl IntoView {
    view! {
        <h1>"Horizontal Orientation"</h1>
        <Accordion r#type=AccordionType::Single orientation=Orientation::Horizontal attr:class=accordion_classes::root>
            {render_standard_items()}
        </Accordion>
    }
}

#[component]
pub fn Chromatic() -> impl IntoView {
    let text_suffix: &'static str = ": Per erat orci nostra luctus sociosqu mus risus penatibus, duis elit vulputate viverra integer ullamcorper congue curabitur sociis, nisi malesuada scelerisque quam suscipit habitant sed.";
    let items: &[&str] = &["One", "Two", "Three", "Four"];

    view! {
        <h1>"Uncontrolled"</h1>
        <h2>"Single closed"</h2>
        <Accordion r#type=AccordionType::Single attr:class=accordion_classes::root>
            {render_chromatic_items(items, text_suffix, "accordion-item", "accordion-header", "accordion-trigger", "accordion-content")}
        </Accordion>

        <h2>"Single open"</h2>
        <Accordion r#type=AccordionType::Single default_value="Two".to_string() attr:class=accordion_classes::root>
            {render_chromatic_items(items, text_suffix, "accordion-item", "accordion-header", "accordion-trigger", "accordion-content")}
        </Accordion>

        <h2>"Multiple closed"</h2>
        <Accordion r#type=AccordionType::Multiple attr:class=accordion_classes::root>
            {render_chromatic_items(items, text_suffix, "accordion-item", "accordion-header", "accordion-trigger", "accordion-content")}
        </Accordion>

        <h2>"Multiple open"</h2>
        <Accordion r#type=AccordionType::Multiple default_values=vec!["One".to_string(), "Two".to_string()] attr:class=accordion_classes::root>
            {render_chromatic_items(items, text_suffix, "accordion-item", "accordion-header", "accordion-trigger", "accordion-content")}
        </Accordion>

        <h1>"Controlled"</h1>
        <h2>"Single open"</h2>
        <Accordion r#type=AccordionType::Single value="Three".to_string() attr:class=accordion_classes::root>
            {render_chromatic_items(items, text_suffix, "accordion-item", "accordion-header", "accordion-trigger", "accordion-content")}
        </Accordion>

        <h2>"Multiple open"</h2>
        <Accordion r#type=AccordionType::Multiple values=vec!["Two".to_string(), "Three".to_string()] attr:class=accordion_classes::root>
            {render_chromatic_items(items, text_suffix, "accordion-item", "accordion-header", "accordion-trigger", "accordion-content")}
        </Accordion>

        <h1>"Disabled (whole)"</h1>
        <Accordion r#type=AccordionType::Single disabled=true attr:class=accordion_classes::root>
            {render_chromatic_items(items, text_suffix, "accordion-item", "accordion-header", "accordion-trigger", "accordion-content")}
        </Accordion>

        <h1>"Disabled (item)"</h1>
        <h2>"Just item"</h2>
        <Accordion r#type=AccordionType::Single attr:class=accordion_classes::root>
            {items.iter().map(|item| {
                let is_disabled = *item == "Two";
                let label = StoredValue::new(item.to_string());
                let content = StoredValue::new(format!("{item}{text_suffix}"));
                view! {
                    <AccordionItem value=item.to_string() disabled=is_disabled attr:class=accordion_classes::item>
                        <AccordionHeader attr:class=accordion_classes::header>
                            <AccordionTrigger attr:class=accordion_classes::trigger>{move || label.get_value()}</AccordionTrigger>
                        </AccordionHeader>
                        <AccordionContent attr:class=accordion_classes::content>
                            {move || content.get_value()}
                        </AccordionContent>
                    </AccordionItem>
                }
            }).collect_view()}
        </Accordion>

        <h2>"with disabled=false on top-level"</h2>
        <Accordion r#type=AccordionType::Single disabled=false attr:class=accordion_classes::root>
            {items.iter().map(|item| {
                let is_disabled = *item == "Two";
                let label = StoredValue::new(item.to_string());
                let content = StoredValue::new(format!("{item}{text_suffix}"));
                view! {
                    <AccordionItem value=item.to_string() disabled=is_disabled attr:class=accordion_classes::item>
                        <AccordionHeader attr:class=accordion_classes::header>
                            <AccordionTrigger attr:class=accordion_classes::trigger>{move || label.get_value()}</AccordionTrigger>
                        </AccordionHeader>
                        <AccordionContent attr:class=accordion_classes::content>
                            {move || content.get_value()}
                        </AccordionContent>
                    </AccordionItem>
                }
            }).collect_view()}
        </Accordion>

        <h1>"Force mounted contents"</h1>
        <Accordion r#type=AccordionType::Single attr:class=accordion_classes::root>
            {items.iter().map(|item| {
                let label = StoredValue::new(item.to_string());
                let content = StoredValue::new(format!("{item}{text_suffix}"));
                view! {
                    <AccordionItem value=item.to_string() attr:class=accordion_classes::item>
                        <AccordionHeader attr:class=accordion_classes::header>
                            <AccordionTrigger attr:class=accordion_classes::trigger>{move || label.get_value()}</AccordionTrigger>
                        </AccordionHeader>
                        <AccordionContent force_mount=true attr:class=accordion_classes::content>
                            {move || content.get_value()}
                        </AccordionContent>
                    </AccordionItem>
                }
            }).collect_view()}
        </Accordion>

        <h1>"State attributes"</h1>
        <h2>"Accordion disabled"</h2>
        <Accordion r#type=AccordionType::Single default_value="Two".to_string() disabled=true attr:class=accordion_classes::rootAttr>
            {render_chromatic_items(items, text_suffix, "accordion-itemAttr", "accordion-headerAttr", "accordion-triggerAttr", "accordion-contentAttr")}
        </Accordion>

        <h2>"Accordion enabled with item override"</h2>
        <Accordion r#type=AccordionType::Single default_value="Two".to_string() disabled=false attr:class=accordion_classes::rootAttr>
            {items.iter().map(|item| {
                let is_disabled = *item == "Two" || *item == "Four";
                let label = StoredValue::new(item.to_string());
                let content = StoredValue::new(format!("{item}{text_suffix}"));
                view! {
                    <AccordionItem value=item.to_string() disabled=is_disabled attr:class=accordion_classes::itemAttr>
                        <AccordionHeader attr:class=accordion_classes::headerAttr>
                            <AccordionTrigger attr:class=accordion_classes::triggerAttr>{move || label.get_value()}</AccordionTrigger>
                        </AccordionHeader>
                        <AccordionContent attr:class=accordion_classes::contentAttr>
                            {move || content.get_value()}
                        </AccordionContent>
                    </AccordionItem>
                }
            }).collect_view()}
        </Accordion>

        <h2>"Accordion disabled with item override"</h2>
        <Accordion r#type=AccordionType::Single default_value="Two".to_string() disabled=true attr:class=accordion_classes::rootAttr>
            {items.iter().map(|item| {
                // In React: disabled={['Two', 'Four'].includes(item) ? false : undefined}
                // false = explicitly not disabled, undefined = inherit from parent
                // In Leptos: MaybeProp None = inherit, Some(false) = explicitly not disabled
                let disabled_prop: MaybeProp<bool> = if *item == "Two" || *item == "Four" {
                    false.into()
                } else {
                    MaybeProp::default()
                };
                let label = StoredValue::new(item.to_string());
                let content = StoredValue::new(format!("{item}{text_suffix}"));
                view! {
                    <AccordionItem value=item.to_string() disabled=disabled_prop attr:class=accordion_classes::itemAttr>
                        <AccordionHeader attr:class=accordion_classes::headerAttr>
                            <AccordionTrigger attr:class=accordion_classes::triggerAttr>{move || label.get_value()}</AccordionTrigger>
                        </AccordionHeader>
                        <AccordionContent attr:class=accordion_classes::contentAttr>
                            {move || content.get_value()}
                        </AccordionContent>
                    </AccordionItem>
                }
            }).collect_view()}
        </Accordion>
    }
}

fn render_chromatic_items(
    items: &[&str],
    text_suffix: &str,
    item_class: &'static str,
    header_class: &'static str,
    trigger_class: &'static str,
    content_class: &'static str,
) -> impl IntoView {
    items
        .iter()
        .map(|item| {
            let label = StoredValue::new(item.to_string());
            let content = StoredValue::new(format!("{item}{text_suffix}"));
            view! {
                <AccordionItem value=item.to_string() attr:class=item_class>
                    <AccordionHeader attr:class=header_class>
                        <AccordionTrigger attr:class=trigger_class>{move || label.get_value()}</AccordionTrigger>
                    </AccordionHeader>
                    <AccordionContent attr:class=content_class>
                        {move || content.get_value()}
                    </AccordionContent>
                </AccordionItem>
            }
        })
        .collect_view()
}
