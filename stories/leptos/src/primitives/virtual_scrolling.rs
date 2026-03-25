use std::rc::Rc;

use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use pith_virtual_leptos::{
    ScrollAlignment, ScrollBehavior, ScrollToOptions, UseVirtualizerOptions, use_virtualizer,
};
use web_sys::wasm_bindgen::JsCast;

stylance::import_crate_style!(
    classes,
    "src/primitives/virtual_scrolling.stories.module.css"
);

fn scroll_element_signal(node_ref: AnyNodeRef) -> Signal<Option<web_sys::Element>> {
    Signal::derive(move || {
        node_ref
            .get()
            .and_then(|n| n.dyn_into::<web_sys::Element>().ok())
    })
}

/// Helper: render a standard vertical virtual list with default row rendering.
#[component]
fn VirtualList(
    virtualizer: pith_virtual_leptos::VirtualizerHandle,
    scroll_ref: AnyNodeRef,
) -> impl IntoView {
    let v1 = virtualizer.clone();
    let v2 = virtualizer.clone();
    let v3 = virtualizer.clone();

    view! {
        <div node_ref=scroll_ref class=classes::container>
            <div
                class=classes::totalSizer
                style:height=move || format!("{}px", v1.get_total_size())
            >
                <div
                    class=classes::itemList
                    style:transform=move || {
                        let items = v2.get_virtual_items();
                        let offset = items.first().map(|i| i.start).unwrap_or(0.0);
                        format!("translateY({}px)", offset)
                    }
                >
                    {move || {
                        v3.get_virtual_items()
                            .into_iter()
                            .map(|item| {
                                view! {
                                    <div
                                        class=classes::item
                                        style:height=format!("{}px", item.size)
                                        data-index=item.index
                                    >
                                        <span class=classes::itemIndex>{item.index}</span>
                                        {format!("Row {}", item.index)}
                                    </div>
                                }
                            })
                            .collect_view()
                    }}
                </div>
            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// Fixed Size — 10,000 rows with uniform 35px height
// ---------------------------------------------------------------------------

#[component]
pub fn FixedSize() -> impl IntoView {
    let scroll_ref = AnyNodeRef::new();
    let virtualizer = use_virtualizer(UseVirtualizerOptions {
        count: Signal::from(10_000),
        get_scroll_element: scroll_element_signal(scroll_ref),
        estimate_size: Rc::new(|_| 35.0),
        overscan: Some(5),
        ..default_options()
    });

    let v_stats1 = virtualizer.clone();
    let v_stats2 = virtualizer.clone();
    let v_stats3 = virtualizer.clone();

    view! {
        <div class=classes::storySection>
            <p class=classes::storyTitle>"Fixed Size List"</p>
            <p class=classes::storyDescription>"10,000 rows, each 35px tall."</p>
            <VirtualList virtualizer=virtualizer scroll_ref=scroll_ref />
            <div class=classes::stats>
                <span>{move || format!("Rendered: {}", v_stats1.get_virtual_items().len())}</span>
                <span>{move || format!("Total: {}px", v_stats2.get_total_size())}</span>
                <span>{move || if v_stats3.is_scrolling() { "Scrolling..." } else { "Idle" }}</span>
            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// Variable Size — rows with different heights
// ---------------------------------------------------------------------------

const SENTENCES: [&str; 6] = [
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
    "Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
    "Ut enim ad minim veniam, quis nostrud exercitation.",
    "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum.",
    "Excepteur sint occaecat cupidatat non proident.",
    "Sunt in culpa qui officia deserunt mollit anim id est laborum.",
];

fn sentence_count(index: usize) -> usize {
    1 + (index.wrapping_mul(2654435761) % 4)
}

fn sentence_text(index: usize, n: usize) -> String {
    (0..n)
        .map(|i| SENTENCES[(index + i) % SENTENCES.len()])
        .collect::<Vec<_>>()
        .join(" ")
}

#[component]
pub fn VariableSize() -> impl IntoView {
    let scroll_ref = AnyNodeRef::new();
    let list_ref = AnyNodeRef::new();

    let virtualizer = use_virtualizer(UseVirtualizerOptions {
        count: Signal::from(5_000),
        get_scroll_element: scroll_element_signal(scroll_ref),
        estimate_size: Rc::new(|_| 60.0),
        overscan: Some(5),
        ..default_options()
    });

    // After each render, scan the list container for item elements and
    // measure them. The version counter triggers re-runs.
    let v_measure = virtualizer.clone();
    Effect::new(move |_| {
        // Subscribe to version changes so this re-runs after scroll/resize.
        v_measure.track();

        let Some(list_node) = list_ref.get() else {
            return;
        };
        let Ok(list_el) = list_node.dyn_into::<web_sys::HtmlElement>() else {
            return;
        };

        let vh = v_measure.clone();
        let closure: wasm_bindgen::closure::Closure<dyn FnMut()> =
            wasm_bindgen::closure::Closure::once(move || {
                let children = list_el.children();
                for i in 0..children.length() {
                    if let Some(wrapper) = children.item(i) {
                        if let Some(item_el) = wrapper.first_element_child() {
                            if let Ok(html_el) = item_el.dyn_into::<web_sys::HtmlElement>() {
                                vh.measure_element(Some(&html_el));
                            }
                        }
                    }
                }
            });

        if let Some(window) = web_sys::window() {
            window
                .request_animation_frame(closure.as_ref().unchecked_ref())
                .ok();
        }
        closure.forget();
    });

    let v1 = virtualizer.clone();
    let v2 = virtualizer.clone();

    view! {
        <div class=classes::storySection>
            <p class=classes::storyTitle>"Variable Size List (Measured)"</p>
            <p class=classes::storyDescription>
                "5,000 rows with varying content. Each item is measured via ResizeObserver for accurate positioning."
            </p>

            <div node_ref=scroll_ref class=classes::container>
                <div
                    class=classes::totalSizer
                    style:height=move || format!("{}px", v1.get_total_size())
                >
                    <div node_ref=list_ref class=classes::itemList>
                        {move || {
                            v2.get_virtual_items()
                                .into_iter()
                                .map(|item| {
                                    let count = sentence_count(item.index);
                                    let text = sentence_text(item.index, count);
                                    view! {
                                        <div style=format!(
                                            "position: absolute; top: 0; left: 0; width: 100%; transform: translateY({}px);",
                                            item.start
                                        )>
                                            <div class=classes::dynamicItem data-index=item.index>
                                                <div class=classes::dynamicTitle>
                                                    {format!("Item #{}", item.index)}
                                                    " "
                                                    <span class=format!("{} {}", classes::badge, classes::badgeBlue)>
                                                        {format!("{} line{}", count, if count > 1 { "s" } else { "" })}
                                                    </span>
                                                </div>
                                                <div class=classes::dynamicBody>{text}</div>
                                            </div>
                                        </div>
                                    }
                                })
                                .collect_view()
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// Horizontal — horizontal scrolling
// ---------------------------------------------------------------------------

#[component]
pub fn Horizontal() -> impl IntoView {
    let scroll_ref = AnyNodeRef::new();
    let virtualizer = use_virtualizer(UseVirtualizerOptions {
        count: Signal::from(10_000),
        get_scroll_element: scroll_element_signal(scroll_ref),
        estimate_size: Rc::new(|_| 120.0),
        overscan: Some(5),
        horizontal: Some(true),
        ..default_options()
    });

    let v1 = virtualizer.clone();
    let v2 = virtualizer.clone();
    let v3 = virtualizer.clone();

    view! {
        <div class=classes::storySection>
            <p class=classes::storyTitle>"Horizontal List"</p>
            <p class=classes::storyDescription>"10,000 columns scrolling horizontally."</p>

            <div node_ref=scroll_ref class=classes::horizontalContainer>
                <div
                    class=classes::horizontalSizer
                    style:width=move || format!("{}px", v1.get_total_size())
                >
                    <div
                        class=classes::horizontalItemList
                        style:transform=move || {
                            let items = v2.get_virtual_items();
                            let offset = items.first().map(|i| i.start).unwrap_or(0.0);
                            format!("translateX({}px)", offset)
                        }
                    >
                        {move || {
                            v3.get_virtual_items()
                                .into_iter()
                                .map(|item| {
                                    view! {
                                        <div
                                            class=classes::horizontalItem
                                            style:width=format!("{}px", item.size)
                                            data-index=item.index
                                        >
                                            <span class=classes::itemIndex>{item.index}</span>
                                            {format!("Col {}", item.index)}
                                        </div>
                                    }
                                })
                                .collect_view()
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// Grid — multi-lane layout
// ---------------------------------------------------------------------------

#[component]
fn GridDemo() -> impl IntoView {
    let scroll_ref = AnyNodeRef::new();
    let cols = 4;
    let virtualizer = use_virtualizer(UseVirtualizerOptions {
        count: Signal::from(10_000),
        get_scroll_element: scroll_element_signal(scroll_ref),
        estimate_size: Rc::new(|_| 80.0),
        overscan: Some(5),
        lanes: Some(cols),
        ..default_options()
    });

    let v1 = virtualizer.clone();
    let v2 = virtualizer.clone();

    view! {
        <div class=classes::storySection>
            <p class=classes::storyTitle>"Grid Layout"</p>
            <p class=classes::storyDescription>
                {format!("10,000 items in a {}-column grid using multi-lane virtualization.", cols)}
            </p>

            <div node_ref=scroll_ref class=classes::gridContainer>
                <div
                    class=classes::gridSizer
                    style:height=move || format!("{}px", v1.get_total_size())
                >
                    {move || {
                        v2.get_virtual_items()
                            .into_iter()
                            .map(|item| {
                                view! {
                                    <div
                                        class=classes::gridItem
                                        style=format!(
                                            "position: absolute; top: 0; left: 0; width: {}%; height: {}px; transform: translateY({}px) translateX({}%)",
                                            100.0 / cols as f64,
                                            item.size,
                                            item.start,
                                            item.lane as f64 * 100.0 / cols as f64 * cols as f64 / 1.0
                                        )
                                        data-index=item.index
                                    >
                                        <span class=classes::itemIndex>{item.index}</span>
                                        <span style="font-size: 11px; color: var(--gray-9);">
                                            {format!("lane {}", item.lane)}
                                        </span>
                                    </div>
                                }
                            })
                            .collect_view()
                    }}
                </div>
            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// Scroll To — programmatic scrolling
// ---------------------------------------------------------------------------

#[component]
pub fn ScrollTo() -> impl IntoView {
    let scroll_ref = AnyNodeRef::new();
    let (target_index, set_target_index) = signal(String::from("500"));

    let virtualizer = use_virtualizer(UseVirtualizerOptions {
        count: Signal::from(10_000),
        get_scroll_element: scroll_element_signal(scroll_ref),
        estimate_size: Rc::new(|_| 35.0),
        overscan: Some(5),
        ..default_options()
    });

    let v_s1 = virtualizer.clone();
    let v_s2 = virtualizer.clone();
    let v_s3 = virtualizer.clone();
    let v_s4 = virtualizer.clone();

    let v1 = virtualizer.clone();
    let v2 = virtualizer.clone();
    let v3 = virtualizer.clone();

    view! {
        <div class=classes::storySection>
            <p class=classes::storyTitle>"Scroll To Index"</p>
            <p class=classes::storyDescription>"Programmatic scrolling with different alignments."</p>

            <div class=classes::controls>
                <span class=classes::label>"Index:"</span>
                <input
                    class=classes::input
                    r#type="number"
                    prop:value=target_index
                    on:input=move |ev| set_target_index.set(event_target_value(&ev))
                />
                <button class=classes::button on:click=move |_| {
                    let i = target_index.get_untracked().parse::<usize>().unwrap_or(500);
                    v_s1.scroll_to_index(i, ScrollToOptions { align: ScrollAlignment::Start, behavior: ScrollBehavior::Auto });
                }>"Start"</button>
                <button class=classes::button on:click=move |_| {
                    let i = target_index.get_untracked().parse::<usize>().unwrap_or(500);
                    v_s2.scroll_to_index(i, ScrollToOptions { align: ScrollAlignment::Center, behavior: ScrollBehavior::Auto });
                }>"Center"</button>
                <button class=classes::button on:click=move |_| {
                    let i = target_index.get_untracked().parse::<usize>().unwrap_or(500);
                    v_s3.scroll_to_index(i, ScrollToOptions { align: ScrollAlignment::End, behavior: ScrollBehavior::Auto });
                }>"End"</button>
                <button class=classes::button on:click=move |_| {
                    let i = target_index.get_untracked().parse::<usize>().unwrap_or(500);
                    v_s4.scroll_to_index(i, ScrollToOptions { align: ScrollAlignment::Start, behavior: ScrollBehavior::Smooth });
                }>"Smooth"</button>
            </div>

            <div node_ref=scroll_ref class=classes::container>
                <div
                    class=classes::totalSizer
                    style:height=move || format!("{}px", v1.get_total_size())
                >
                    <div
                        class=classes::itemList
                        style:transform=move || {
                            let items = v2.get_virtual_items();
                            let offset = items.first().map(|i| i.start).unwrap_or(0.0);
                            format!("translateY({}px)", offset)
                        }
                    >
                        {move || {
                            let target = target_index.get().parse::<usize>().unwrap_or(usize::MAX);
                            v3.get_virtual_items()
                                .into_iter()
                                .map(|item| {
                                    let is_target = item.index == target;
                                    let bg = if is_target { "background: var(--blue-3);" } else { "" };
                                    view! {
                                        <div
                                            class=classes::item
                                            style=format!("height: {}px; {}", item.size, bg)
                                            data-index=item.index
                                        >
                                            <span class=classes::itemIndex>{item.index}</span>
                                            {format!("Row {}", item.index)}
                                            {is_target.then(|| view! {
                                                <span class=format!("{} {}", classes::badge, classes::badgeGreen) style="margin-left: 8px;">
                                                    "target"
                                                </span>
                                            })}
                                        </div>
                                    }
                                })
                                .collect_view()
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// Padding and Gap
// ---------------------------------------------------------------------------

#[component]
fn PaddingAndGapDemo() -> impl IntoView {
    let scroll_ref = AnyNodeRef::new();
    let virtualizer = use_virtualizer(UseVirtualizerOptions {
        count: Signal::from(1_000),
        get_scroll_element: scroll_element_signal(scroll_ref),
        estimate_size: Rc::new(|_| 40.0),
        overscan: Some(5),
        padding_start: Some(20.0),
        padding_end: Some(20.0),
        gap: Some(8.0),
        ..default_options()
    });

    let v1 = virtualizer.clone();
    let v2 = virtualizer.clone();

    view! {
        <div class=classes::storySection>
            <p class=classes::storyTitle>"Padding & Gap"</p>
            <p class=classes::storyDescription>"1,000 items with 20px padding and 8px gap."</p>

            <div node_ref=scroll_ref class=classes::container>
                <div
                    class=classes::totalSizer
                    style:height=move || format!("{}px", v1.get_total_size())
                >
                    <div class=classes::itemList>
                        {move || {
                            v2.get_virtual_items()
                                .into_iter()
                                .map(|item| {
                                    view! {
                                        <div
                                            class=classes::item
                                            style=format!(
                                                "position: absolute; top: 0; left: 0; width: 100%; height: {}px; transform: translateY({}px);",
                                                item.size, item.start
                                            )
                                            data-index=item.index
                                        >
                                            <span class=classes::itemIndex>{item.index}</span>
                                            {format!("Row {} (y: {:.0}px)", item.index, item.start)}
                                        </div>
                                    }
                                })
                                .collect_view()
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// Dynamic Count
// ---------------------------------------------------------------------------

#[component]
fn DynamicCountDemo() -> impl IntoView {
    let scroll_ref = AnyNodeRef::new();
    let (count, set_count) = signal(100_usize);

    let virtualizer = use_virtualizer(UseVirtualizerOptions {
        count: count.into(),
        get_scroll_element: scroll_element_signal(scroll_ref),
        estimate_size: Rc::new(|_| 35.0),
        overscan: Some(5),
        ..default_options()
    });

    view! {
        <div class=classes::storySection>
            <p class=classes::storyTitle>"Dynamic Count"</p>
            <p class=classes::storyDescription>"Add or remove items at runtime."</p>

            <div class=classes::controls>
                <button class=classes::button on:click=move |_| set_count.update(|c| *c = c.saturating_sub(100))>"-100"</button>
                <button class=classes::button on:click=move |_| set_count.update(|c| *c = c.saturating_sub(10))>"-10"</button>
                <span class=classes::label>{move || format!("{} items", count.get())}</span>
                <button class=classes::button on:click=move |_| set_count.update(|c| *c += 10)>"+10"</button>
                <button class=classes::button on:click=move |_| set_count.update(|c| *c += 100)>"+100"</button>
                <button class=classes::button on:click=move |_| set_count.set(100_000)>"100k"</button>
            </div>

            <VirtualList virtualizer=virtualizer scroll_ref=scroll_ref />
        </div>
    }
}

// ---------------------------------------------------------------------------
// Showcase — all demos on one page
// ---------------------------------------------------------------------------

#[component]
pub fn Showcase() -> impl IntoView {
    view! {
        <FixedSize />
        <VariableSize />
        <Horizontal />
        <GridDemo />
        <ScrollTo />
        <PaddingAndGapDemo />
        <DynamicCountDemo />
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn default_options() -> UseVirtualizerOptions {
    UseVirtualizerOptions {
        count: Signal::from(0),
        get_scroll_element: Signal::derive(|| None),
        estimate_size: Rc::new(|_| 50.0),
        overscan: None,
        horizontal: None,
        padding_start: None,
        padding_end: None,
        scroll_padding_start: None,
        scroll_padding_end: None,
        initial_offset: None,
        initial_rect: None,
        get_item_key: None,
        range_extractor: None,
        scroll_margin: None,
        gap: None,
        index_attribute: None,
        initial_measurements_cache: None,
        lanes: None,
        is_scrolling_reset_delay: None,
        enabled: None,
        is_rtl: None,
        use_scrollend_event: None,
        debug: None,
    }
}
