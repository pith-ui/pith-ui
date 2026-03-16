use leptos::prelude::*;
use radix_leptos_primitives::scroll_area::*;

const ITEM_COUNT: usize = 50;

#[component]
pub fn ScrollAreaPage() -> impl IntoView {
    let (show_horizontal, set_show_horizontal) = signal(false);
    let (scroll_type, set_scroll_type) = signal(ScrollAreaType::Hover);

    let scroll_to_bottom = move |_| {
        let document = web_sys::window().unwrap().document().unwrap();
        if let Ok(Some(viewport)) = document.query_selector("[data-radix-scroll-area-viewport]") {
            viewport.set_scroll_top(viewport.scroll_height());
        }
    };

    let scroll_to_top = move |_| {
        let document = web_sys::window().unwrap().document().unwrap();
        if let Ok(Some(viewport)) = document.query_selector("[data-radix-scroll-area-viewport]") {
            viewport.set_scroll_top(0);
        }
    };

    view! {
        {move || {
            let current_type = scroll_type.get();
            let show_h = show_horizontal.get();
            view! {
                <ScrollArea
                    r#type=current_type
                    class:scroll-area-root=true
                    attr:data-testid="scroll-area-root"
                >
                    <ScrollAreaViewport
                        class:scroll-area-viewport=true
                        attr:data-testid="scroll-area-viewport"
                    >
                        <div class={
                            if show_h {
                                "scroll-area-content scroll-area-wide-content"
                            } else {
                                "scroll-area-content"
                            }
                        }>
                            {(1..=ITEM_COUNT)
                                .map(|i| {
                                    view! {
                                        <div class="scroll-area-item">
                                            {format!("Item {i}")}
                                        </div>
                                    }
                                })
                                .collect_view()}
                        </div>
                    </ScrollAreaViewport>
                    <ScrollAreaScrollbar
                        orientation=Orientation::Vertical
                        class="scroll-area-scrollbar"
                        attr:data-testid="scrollbar-vertical"
                    >
                        <ScrollAreaThumb
                            class="scroll-area-thumb"
                            attr:data-testid="thumb-vertical"
                        >""</ScrollAreaThumb>
                    </ScrollAreaScrollbar>
                    {if show_h {
                        Some(view! {
                            <ScrollAreaScrollbar
                                orientation=Orientation::Horizontal
                                class="scroll-area-scrollbar"
                                attr:data-testid="scrollbar-horizontal"
                            >
                                <ScrollAreaThumb
                                    class="scroll-area-thumb"
                                    attr:data-testid="thumb-horizontal"
                                >""</ScrollAreaThumb>
                            </ScrollAreaScrollbar>
                        })
                    } else {
                        None
                    }}
                    <ScrollAreaCorner class:scroll-area-corner=true>""</ScrollAreaCorner>
                </ScrollArea>
            }
        }}

        <br />
        <br />

        <label>
            <input
                type="checkbox"
                prop:checked=move || show_horizontal.get()
                on:change=move |_| set_show_horizontal.update(|v| *v = !*v)
            />
            " show horizontal scrollbar"
        </label>

        <br />
        <br />

        <fieldset>
            <legend>"Scroll Type"</legend>
            <label>
                <input
                    type="radio"
                    name="scroll-type"
                    value="hover"
                    prop:checked=move || scroll_type.get() == ScrollAreaType::Hover
                    on:change=move |_| set_scroll_type.set(ScrollAreaType::Hover)
                />
                " hover"
            </label>
            <br />
            <label>
                <input
                    type="radio"
                    name="scroll-type"
                    value="scroll"
                    prop:checked=move || scroll_type.get() == ScrollAreaType::Scroll
                    on:change=move |_| set_scroll_type.set(ScrollAreaType::Scroll)
                />
                " scroll"
            </label>
            <br />
            <label>
                <input
                    type="radio"
                    name="scroll-type"
                    value="auto"
                    prop:checked=move || scroll_type.get() == ScrollAreaType::Auto
                    on:change=move |_| set_scroll_type.set(ScrollAreaType::Auto)
                />
                " auto"
            </label>
            <br />
            <label>
                <input
                    type="radio"
                    name="scroll-type"
                    value="always"
                    prop:checked=move || scroll_type.get() == ScrollAreaType::Always
                    on:change=move |_| set_scroll_type.set(ScrollAreaType::Always)
                />
                " always"
            </label>
        </fieldset>

        <br />

        <button data-testid="scroll-to-bottom" on:click=scroll_to_bottom>
            "scroll to bottom"
        </button>
        " "
        <button data-testid="scroll-to-top" on:click=scroll_to_top>
            "scroll to top"
        </button>
    }
}
