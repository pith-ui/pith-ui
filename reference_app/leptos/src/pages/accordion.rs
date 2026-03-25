use leptos::prelude::*;
use pith_ui::accordion::*;

#[component]
fn AccordionItems() -> impl IntoView {
    view! {
        <AccordionItem
            value="item-1".to_string()
            class:accordion-item=true
            attr:data-testid="item-1"
            attr:data-custom="accordion-item-custom"
        >
            <AccordionHeader class:accordion-header=true attr:data-custom="accordion-header-custom">
                <AccordionTrigger class:accordion-trigger=true attr:data-custom="accordion-trigger-custom">"Item 1"</AccordionTrigger>
            </AccordionHeader>
            <AccordionContent class:accordion-content=true attr:data-custom="accordion-content-custom">"Content 1"</AccordionContent>
        </AccordionItem>
        <AccordionItem
            value="item-2".to_string()
            disabled=true
            class:accordion-item=true
            attr:data-testid="item-2"
        >
            <AccordionHeader class:accordion-header=true>
                <AccordionTrigger class:accordion-trigger=true>"Item 2"</AccordionTrigger>
            </AccordionHeader>
            <AccordionContent class:accordion-content=true>"Content 2"</AccordionContent>
        </AccordionItem>
        <AccordionItem
            value="item-3".to_string()
            class:accordion-item=true
            attr:data-testid="item-3"
        >
            <AccordionHeader class:accordion-header=true>
                <AccordionTrigger class:accordion-trigger=true>"Item 3"</AccordionTrigger>
            </AccordionHeader>
            <AccordionContent class:accordion-content=true>"Content 3"</AccordionContent>
        </AccordionItem>
        <AccordionItem
            value="item-styled".to_string()
            class:accordion-item=true
            attr:data-testid="item-styled"
        >
            <AccordionHeader class:accordion-header=true>
                <AccordionTrigger class:accordion-trigger=true>"Styled Item"</AccordionTrigger>
            </AccordionHeader>
            <AccordionContent
                class:accordion-content=true
                attr:data-testid="styled-content"
                style:background="tomato"
                style:--radix-accordion-content-height="999px"
            >
                "Styled content"
            </AccordionContent>
        </AccordionItem>
    }
}

#[component]
pub fn AccordionPage() -> impl IntoView {
    let (is_multiple, set_is_multiple) = signal(false);
    let (collapsible, set_collapsible) = signal(false);

    view! {
        {move || {
            if is_multiple.get() {
                view! {
                    <AccordionMultiple
                        class:accordion-root=true
                        attr:data-testid="accordion-root"
                    >
                        <AccordionItems />
                    </AccordionMultiple>
                }.into_any()
            } else {
                view! {
                    <AccordionSingle
                        collapsible=collapsible
                        class:accordion-root=true
                        attr:data-testid="accordion-root"
                    >
                        <AccordionItems />
                    </AccordionSingle>
                }.into_any()
            }
        }}

        <br />
        <br />

        <fieldset>
            <legend>"type"</legend>
            <label>
                <input
                    type="radio"
                    name="type"
                    value="single"
                    prop:checked=move || !is_multiple.get()
                    on:change=move |_| set_is_multiple.set(false)
                />
                " single"
            </label>
            <br />
            <label>
                <input
                    type="radio"
                    name="type"
                    value="multiple"
                    prop:checked=move || is_multiple.get()
                    on:change=move |_| set_is_multiple.set(true)
                />
                " multiple"
            </label>
        </fieldset>

        <br />

        <label>
            <input
                type="checkbox"
                prop:checked=move || collapsible.get()
                on:change=move |ev| {
                    use web_sys::wasm_bindgen::JsCast;
                    let target: web_sys::HtmlInputElement = ev.target().unwrap().unchecked_into();
                    set_collapsible.set(target.checked());
                }
            />
            " collapsible"
        </label>

        <br />
        <br />

        <ControlledAccordion />
    }
}

#[component]
fn ControlledAccordion() -> impl IntoView {
    let (controlled_value, set_controlled_value) = signal(String::new());

    view! {
        <h2 data-testid="controlled-heading">"Controlled Accordion"</h2>

        <div data-testid="controlled-value">{move || controlled_value.get()}</div>

        <button
            data-testid="controlled-open-item-1"
            on:click=move |_| set_controlled_value.set("ctrl-item-1".to_string())
        >
            "Open Item 1"
        </button>
        <button
            data-testid="controlled-open-item-2"
            on:click=move |_| set_controlled_value.set("ctrl-item-2".to_string())
        >
            "Open Item 2"
        </button>
        <button
            data-testid="controlled-close-all"
            on:click=move |_| set_controlled_value.set(String::new())
        >
            "Close All"
        </button>

        <AccordionSingle
            collapsible=true
            value=controlled_value
            on_value_change=Callback::new(move |val: String| {
                set_controlled_value.set(val);
            })
            class:accordion-root=true
            attr:data-testid="controlled-accordion-root"
        >
            <AccordionItem
                value="ctrl-item-1".to_string()
                class:accordion-item=true
                attr:data-testid="ctrl-item-1"
            >
                <AccordionHeader class:accordion-header=true>
                    <AccordionTrigger class:accordion-trigger=true>"Ctrl Item 1"</AccordionTrigger>
                </AccordionHeader>
                <AccordionContent class:accordion-content=true>"Controlled Content 1"</AccordionContent>
            </AccordionItem>
            <AccordionItem
                value="ctrl-item-2".to_string()
                class:accordion-item=true
                attr:data-testid="ctrl-item-2"
            >
                <AccordionHeader class:accordion-header=true>
                    <AccordionTrigger class:accordion-trigger=true>"Ctrl Item 2"</AccordionTrigger>
                </AccordionHeader>
                <AccordionContent class:accordion-content=true>"Controlled Content 2"</AccordionContent>
            </AccordionItem>
        </AccordionSingle>
    }
}
