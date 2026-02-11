use leptos::prelude::*;
use radix_leptos_direction::DirectionProvider;
use radix_leptos_scroll_area::*;
use web_sys::wasm_bindgen::{closure::Closure, JsCast};

stylance::import_crate_style!(classes, "src/primitives/scroll_area.stories.module.css");

const COPY_TEXT: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Fusce sit amet eros iaculis, bibendum tellus ac, lobortis odio. Aliquam bibendum elit est, in iaculis est commodo id. Donec pulvinar est libero. Proin consectetur pellentesque molestie. Fusce mi ante, ullamcorper eu ante finibus, finibus pellentesque turpis. Mauris convallis, leo in vulputate varius, sapien lectus suscipit eros, ac semper odio sapien sit amet magna. Sed mattis turpis et lacinia ultrices. Nulla a commodo mauris. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Pellentesque id tempor metus. Pellentesque faucibus tortor non nisi maximus dignissim. Etiam leo nisi, molestie a porttitor at, euismod a libero. Nullam placerat tristique enim nec pulvinar. Sed eleifend dictum nulla a aliquam. Sed tempus ipsum eget urna posuere aliquam. Nulla maximus tortor dui, sed laoreet odio aliquet ac. Vestibulum dolor orci, lacinia finibus vehicula eget, posuere ac lectus. Quisque non felis at ipsum scelerisque condimentum. In pharetra semper arcu, ut hendrerit sem auctor vel. Aliquam non lacinia elit, a facilisis ante. Praesent eget eros augue. Praesent nunc orci, ullamcorper non pulvinar eu, elementum id nibh. Nam id lorem euismod, sodales augue quis, porttitor magna. Vivamus ut nisl velit. Nam ultrices maximus felis, quis ullamcorper quam luctus et.";

#[component]
fn Copy(#[prop(into, optional)] style: MaybeProp<String>) -> impl IntoView {
    let style = Memo::new(move |_| {
        let base = "width: 4000px; margin-top: 0;".to_string();
        match style.get() {
            Some(s) => format!("{base} {s}"),
            None => base,
        }
    });

    view! {
        <p style=move || style.get()>{COPY_TEXT}</p>
    }
}

#[component]
fn ScrollAreaStory(
    #[prop(optional)] r#type: ScrollAreaType,
    #[prop(into, optional)] dir: MaybeProp<&'static str>,
    #[prop(optional)] scroll_hide_delay: u32,
    #[prop(optional, default = true)] vertical: bool,
    #[prop(optional, default = true)] horizontal: bool,
    #[prop(optional)] animated: bool,
    #[prop(into, optional)] style: MaybeProp<String>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let style = Memo::new(move |_| {
        let base = "width: 200px; height: 200px;".to_string();
        match style.get() {
            Some(s) => format!("{base} {s}"),
            None => base,
        }
    });

    let dir_value = dir.get_untracked().map(|d| match d {
        "rtl" => radix_leptos_direction::Direction::Rtl,
        _ => radix_leptos_direction::Direction::Ltr,
    });

    let thumb_class = StoredValue::new(if animated {
        format!("{} {}", classes::animatedThumb, classes::thumb)
    } else {
        classes::thumb.to_string()
    });

    view! {
        <div style=move || style.get()>
            <ScrollArea
                r#type=r#type
                dir=dir_value
                scroll_hide_delay=scroll_hide_delay
                attr:class=classes::scrollArea
            >
                <ScrollAreaViewport attr:class=classes::scrollAreaViewport>
                    {children.with_value(|children| children())}
                </ScrollAreaViewport>
                {vertical.then(|| view! {
                    <ScrollAreaScrollbar orientation=Orientation::Vertical class=classes::scrollbar>
                        <ScrollAreaThumb class=thumb_class.get_value()>""</ScrollAreaThumb>
                    </ScrollAreaScrollbar>
                })}
                {horizontal.then(|| view! {
                    <ScrollAreaScrollbar orientation=Orientation::Horizontal class=classes::scrollbar>
                        <ScrollAreaThumb class=thumb_class.get_value()>""</ScrollAreaThumb>
                    </ScrollAreaScrollbar>
                })}
                <ScrollAreaCorner class=classes::corner>""</ScrollAreaCorner>
            </ScrollArea>
        </div>
    }
}

#[component]
pub fn Basic() -> impl IntoView {
    let (scroll_area_type, set_scroll_area_type) = signal(ScrollAreaType::default());
    let (dir, set_dir) = signal::<Option<&'static str>>(None);
    let (scroll_hide_delay, set_scroll_hide_delay) = signal(600u32);

    view! {
        <div class=classes::root>
            <div style="margin: 20px auto; width: max-content; text-align: center;">
                <label>
                    "type: "
                    <select on:change=move |ev| {
                        use web_sys::wasm_bindgen::JsCast;
                        let value = ev.target()
                            .and_then(|t| t.dyn_into::<web_sys::HtmlSelectElement>().ok())
                            .map(|s| s.value())
                            .unwrap_or_default();
                        set_scroll_area_type.set(match value.as_str() {
                            "always" => ScrollAreaType::Always,
                            "auto" => ScrollAreaType::Auto,
                            "scroll" => ScrollAreaType::Scroll,
                            "hover" => ScrollAreaType::Hover,
                            _ => ScrollAreaType::default(),
                        });
                    }>
                        <option value="">" "</option>
                        <option value="always">"always"</option>
                        <option value="auto">"auto"</option>
                        <option value="scroll">"scroll"</option>
                        <option value="hover">"hover"</option>
                    </select>
                </label>
                " "
                <label>
                    "dir: "
                    <select on:change=move |ev| {
                        use web_sys::wasm_bindgen::JsCast;
                        let value = ev.target()
                            .and_then(|t| t.dyn_into::<web_sys::HtmlSelectElement>().ok())
                            .map(|s| s.value())
                            .unwrap_or_default();
                        set_dir.set(match value.as_str() {
                            "ltr" => Some("ltr"),
                            "rtl" => Some("rtl"),
                            _ => None,
                        });
                    }>
                        <option value="">" "</option>
                        <option value="ltr">"ltr"</option>
                        <option value="rtl">"rtl"</option>
                    </select>
                </label>
                " "
                <label>
                    "scrollHideDelay: "
                    <input type="number" on:change=move |ev| {
                        use web_sys::wasm_bindgen::JsCast;
                        let value = ev.target()
                            .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                            .map(|i| i.value())
                            .unwrap_or_default();
                        set_scroll_hide_delay.set(value.parse::<u32>().unwrap_or(600));
                    } />
                </label>
            </div>

            // Wrapping in a reactive closure forces remount when signals change,
            // matching React's key={props.type} behavior.
            {move || {
                let t = scroll_area_type.get();
                let d = dir.get();
                let delay = scroll_hide_delay.get();
                view! {
                    <ScrollAreaStory
                        r#type=t
                        dir=d.unwrap_or("ltr")
                        scroll_hide_delay=delay
                        style="width: 800px; height: 800px; margin: 30px auto;"
                    >
                        {(0..30).map(|_| view! { <Copy /> }).collect_view()}
                    </ScrollAreaStory>
                }
            }}
        </div>
    }
}

#[component]
pub fn Resizable() -> impl IntoView {
    view! {
        <div class=classes::root style="width: 800px; height: 800px; padding: 20px; resize: both; border: 1px solid gray; overflow: hidden;">
            <ScrollAreaStory style="width: 100%; height: 100%;">
                {(0..30).map(|_| view! { <Copy /> }).collect_view()}
            </ScrollAreaStory>
        </div>
    }
}

#[component]
pub fn ContentChange() -> impl IntoView {
    let (vertical_count, set_vertical_count) = signal(1usize);
    let (horizontal_count, set_horizontal_count) = signal(1usize);

    view! {
        <div class=classes::root>
            <button on:click=move |_| set_vertical_count.update(|c| *c += 1)>
                "Add vertical content"
            </button>
            <button on:click=move |_| set_horizontal_count.update(|c| *c += 1)>
                "Increase horizontal size"
            </button>
            <ScrollAreaStory r#type=ScrollAreaType::Always style="width: 800px; height: 800px;">
                {move || {
                    let vc = vertical_count.get();
                    let hc = horizontal_count.get();
                    let width = format!("width: {}px;", 300 * hc);
                    (0..vc).map(|_| {
                        let w = width.clone();
                        view! { <Copy style=w /> }
                    }).collect_view()
                }}
            </ScrollAreaStory>
        </div>
    }
}

#[component]
pub fn Animated() -> impl IntoView {
    view! {
        <div class=classes::root>
            <ScrollAreaStory animated=true style="width: 800px; height: 800px;">
                {(0..30).map(|_| view! { <Copy /> }).collect_view()}
            </ScrollAreaStory>
        </div>
    }
}

#[component]
pub fn Chromatic() -> impl IntoView {
    view! {
        <div class=classes::root>
            <h1>"Vertical"</h1>
            <h2>"Auto with overflow"</h2>
            <ScrollAreaStory r#type=ScrollAreaType::Auto horizontal=false>
                {(0..10).map(|_| view! { <Copy /> }).collect_view()}
            </ScrollAreaStory>

            <h2>"Auto without overflow"</h2>
            <ScrollAreaStory r#type=ScrollAreaType::Auto horizontal=false>
                <Copy style="height: 50px; overflow: hidden;" />
            </ScrollAreaStory>

            <h2>"Always with overflow"</h2>
            <ScrollAreaStory r#type=ScrollAreaType::Always horizontal=false>
                {(0..10).map(|_| view! { <Copy /> }).collect_view()}
            </ScrollAreaStory>

            <h2>"Always without overflow"</h2>
            <ScrollAreaStory r#type=ScrollAreaType::Always horizontal=false>
                <Copy style="height: 50px; overflow: hidden;" />
            </ScrollAreaStory>

            <h2>"Scroll with overflow"</h2>
            <ScrollAreaStory r#type=ScrollAreaType::Scroll horizontal=false>
                {(0..10).map(|_| view! { <Copy /> }).collect_view()}
            </ScrollAreaStory>

            <h2>"Scroll without overflow"</h2>
            <ScrollAreaStory r#type=ScrollAreaType::Scroll horizontal=false>
                <Copy style="height: 50px; overflow: hidden;" />
            </ScrollAreaStory>

            <h2>"Hover with overflow"</h2>
            <ScrollAreaStory r#type=ScrollAreaType::Hover horizontal=false>
                {(0..10).map(|_| view! { <Copy /> }).collect_view()}
            </ScrollAreaStory>

            <h2>"Hover without overflow"</h2>
            <ScrollAreaStory r#type=ScrollAreaType::Hover horizontal=false>
                <Copy style="height: 50px; overflow: hidden;" />
            </ScrollAreaStory>

            <h1>"Horizontal"</h1>
            <h2>"Auto with overflow"</h2>
            <ScrollAreaStory r#type=ScrollAreaType::Auto vertical=false>
                {(0..10).map(|_| view! { <Copy /> }).collect_view()}
            </ScrollAreaStory>

            <h2>"Auto without overflow"</h2>
            <ScrollAreaStory r#type=ScrollAreaType::Auto vertical=false>
                <Copy style="width: 50px; overflow: hidden;" />
            </ScrollAreaStory>

            <h2>"Always with overflow"</h2>
            <ScrollAreaStory r#type=ScrollAreaType::Always vertical=false>
                {(0..10).map(|_| view! { <Copy /> }).collect_view()}
            </ScrollAreaStory>

            <h2>"Always without overflow"</h2>
            <ScrollAreaStory r#type=ScrollAreaType::Always vertical=false>
                <Copy style="width: 50px; overflow: hidden;" />
            </ScrollAreaStory>

            <h2>"Scroll with overflow"</h2>
            <ScrollAreaStory r#type=ScrollAreaType::Scroll vertical=false>
                {(0..10).map(|_| view! { <Copy /> }).collect_view()}
            </ScrollAreaStory>

            <h2>"Scroll without overflow"</h2>
            <ScrollAreaStory r#type=ScrollAreaType::Scroll vertical=false>
                <Copy style="width: 50px; overflow: hidden;" />
            </ScrollAreaStory>

            <h2>"Hover with overflow"</h2>
            <ScrollAreaStory r#type=ScrollAreaType::Hover vertical=false>
                {(0..10).map(|_| view! { <Copy /> }).collect_view()}
            </ScrollAreaStory>

            <h2>"Hover without overflow"</h2>
            <ScrollAreaStory r#type=ScrollAreaType::Hover vertical=false>
                <Copy style="width: 50px; overflow: hidden;" />
            </ScrollAreaStory>

            <h1>"Both"</h1>
            <h2>"Auto with overflow"</h2>
            <ScrollAreaStory r#type=ScrollAreaType::Auto>
                {(0..10).map(|_| view! { <Copy /> }).collect_view()}
            </ScrollAreaStory>

            <h2>"Auto with horizontal overflow"</h2>
            <ScrollAreaStory r#type=ScrollAreaType::Auto>
                {(0..1).map(|_| view! { <Copy /> }).collect_view()}
            </ScrollAreaStory>

            <h2>"Auto with vertical overflow"</h2>
            <ScrollAreaStory r#type=ScrollAreaType::Auto>
                {(0..10).map(|_| view! { <Copy style="width: 50px; overflow: hidden;" /> }).collect_view()}
            </ScrollAreaStory>

            <h2>"Auto without overflow"</h2>
            <ScrollAreaStory r#type=ScrollAreaType::Auto>
                <Copy style="width: 50px; height: 50px; overflow: hidden;" />
            </ScrollAreaStory>

            <h2>"Always with overflow"</h2>
            <ScrollAreaStory r#type=ScrollAreaType::Always>
                {(0..10).map(|_| view! { <Copy /> }).collect_view()}
            </ScrollAreaStory>

            <h2>"Always without overflow"</h2>
            <ScrollAreaStory r#type=ScrollAreaType::Always>
                <Copy style="width: 50px; height: 50px; overflow: hidden;" />
            </ScrollAreaStory>

            <h2>"Scroll with overflow"</h2>
            <ScrollAreaStory r#type=ScrollAreaType::Scroll>
                {(0..10).map(|_| view! { <Copy /> }).collect_view()}
            </ScrollAreaStory>

            <h2>"Scroll without overflow"</h2>
            <ScrollAreaStory r#type=ScrollAreaType::Scroll>
                <Copy style="width: 50px; height: 50px; overflow: hidden;" />
            </ScrollAreaStory>

            <h2>"Hover with overflow"</h2>
            <ScrollAreaStory r#type=ScrollAreaType::Hover>
                {(0..10).map(|_| view! { <Copy /> }).collect_view()}
            </ScrollAreaStory>

            <h2>"Hover without overflow"</h2>
            <ScrollAreaStory r#type=ScrollAreaType::Hover>
                <Copy style="width: 50px; height: 50px; overflow: hidden;" />
            </ScrollAreaStory>

            <h2>"Hover with horizontal overflow"</h2>
            <ScrollAreaStory r#type=ScrollAreaType::Hover>
                {(0..1).map(|_| view! { <Copy /> }).collect_view()}
            </ScrollAreaStory>

            <h2>"Hover with vertical overflow"</h2>
            <ScrollAreaStory r#type=ScrollAreaType::Hover>
                {(0..10).map(|_| view! { <Copy style="width: 50px; overflow: hidden;" /> }).collect_view()}
            </ScrollAreaStory>

            <h1>"Min thumb size"</h1>
            <ScrollAreaStory r#type=ScrollAreaType::Always>
                {(0..100).map(|_| view! { <Copy style="width: 10000px;" /> }).collect_view()}
            </ScrollAreaStory>

            <h1>"RTL"</h1>
            <h2>"Prop"</h2>
            <ScrollAreaStory r#type=ScrollAreaType::Always dir="rtl">
                {(0..10).map(|_| view! { <Copy /> }).collect_view()}
            </ScrollAreaStory>

            <h2>"Inherited"</h2>
            <DirectionProvider direction=Signal::derive(|| radix_leptos_direction::Direction::Rtl)>
                <ScrollAreaStory r#type=ScrollAreaType::Always>
                    {(0..10).map(|_| view! { <Copy /> }).collect_view()}
                </ScrollAreaStory>
            </DirectionProvider>
        </div>
    }
}

const DYNAMIC_CONTENT_DELAY: i32 = 2000;

#[component]
fn DynamicContentStory(#[prop(optional)] start_loaded: bool) -> impl IntoView {
    let (show_content, set_show_content) = signal(start_loaded);

    if !start_loaded {
        let cb = Closure::once(move || {
            set_show_content.set(true);
        });
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(),
                DYNAMIC_CONTENT_DELAY,
            )
            .unwrap();
        cb.forget();
    }

    view! {
        <div class=classes::root>
            <h1>"Always"</h1>
            <ScrollAreaStory r#type=ScrollAreaType::Always style="width: 500px; height: 250px;">
                {move || if show_content.get() {
                    (0..30).map(|_| view! { <Copy /> }).collect_view().into_any()
                } else {
                    view! { <h1>"Loading..."</h1> }.into_any()
                }}
            </ScrollAreaStory>

            <h1>"Hover"</h1>
            <ScrollAreaStory r#type=ScrollAreaType::Hover style="width: 500px; height: 250px;">
                {move || if show_content.get() {
                    (0..30).map(|_| view! { <Copy /> }).collect_view().into_any()
                } else {
                    view! { <h1>"Loading..."</h1> }.into_any()
                }}
            </ScrollAreaStory>

            <h1>"Scroll"</h1>
            <ScrollAreaStory r#type=ScrollAreaType::Scroll style="width: 500px; height: 250px;">
                {move || if show_content.get() {
                    (0..30).map(|_| view! { <Copy /> }).collect_view().into_any()
                } else {
                    view! { <h1>"Loading..."</h1> }.into_any()
                }}
            </ScrollAreaStory>

            <h1>"Auto"</h1>
            <ScrollAreaStory r#type=ScrollAreaType::Auto style="width: 500px; height: 250px;">
                {move || if show_content.get() {
                    (0..30).map(|_| view! { <Copy /> }).collect_view().into_any()
                } else {
                    view! { <h1>"Loading..."</h1> }.into_any()
                }}
            </ScrollAreaStory>
        </div>
    }
}

#[component]
pub fn ChromaticDynamicContentBeforeLoaded() -> impl IntoView {
    view! { <DynamicContentStory /> }
}

#[component]
pub fn ChromaticDynamicContentAfterLoaded() -> impl IntoView {
    view! { <DynamicContentStory start_loaded=true /> }
}
